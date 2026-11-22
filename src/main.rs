//! phdoc — a project scanner for shipped artifacts.
//!
//! phdoc reads a project from its own records — its manifests, its git
//! log, its source text — and writes documentation in two forms that
//! carry the same words: a markdown file for humans, and a phext for
//! agents and for the record. Every public method and command it finds
//! is explained in plain English, from its doc comment when the code
//! carries one, and from its signature when it does not. When neither
//! yields a summary, phdoc does not invent one: the item is marked
//! awaiting-summary and named in an instruction file, so the user's
//! agent can supply the missing words.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use libphext::phext;

// ---------------------------------------------------------------------------
// time: the Exocortical Epoch clock and a civil date, computed, never stored
// ---------------------------------------------------------------------------

/// EE anchor: EE 1 begins 2026-11-22 00:00:00 CST = unix 1795327200.
/// EE(t) = floor((t - anchor)/420) + 1; EE 0 before the anchor.
fn epoch_of(unix: u64) -> u64 {
    if unix < 1_795_327_200 {
        0
    } else {
        (unix - 1_795_327_200) / 420 + 1
    }
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Days-from-epoch to (year, month, day); Howard Hinnant's algorithm.
fn civil_date(unix: u64) -> (u64, u64, u64) {
    let z = (unix / 86_400) as i64 + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y } as u64, m, d)
}

fn date_string(unix: u64) -> String {
    let (y, m, d) = civil_date(unix);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

// ---------------------------------------------------------------------------
// the scan record
// ---------------------------------------------------------------------------

#[derive(Default)]
struct DepInfo {
    ecosystem: String,
    manifest: String,
    direct: Vec<(String, String)>,
    total_resolved: Option<usize>,
    license: Option<String>,
}

#[derive(Default)]
struct RfcEntry {
    number: String,
    sha: String,
    source: String,
    status: String,
}

#[derive(Default, Clone)]
struct PubItem {
    kind: String, // function | method | struct | enum | trait | const | type | re-export | command | flag | entry point
    name: String,
    signature: String,
    doc: String,
    extra: String, // fields / variants / value, when read
    file: String,
    line: usize,
}

#[derive(Default)]
struct Scan {
    name: String,
    root: String,
    date: String,
    ee: u64,
    deps: Vec<DepInfo>,
    authors: Vec<(String, usize)>, // name <email>, commits
    total_commits: usize,
    lang_lines: BTreeMap<String, (usize, usize, usize, usize, usize)>, // lang -> (files,total,blank,comment,code)
    largest_files: Vec<(String, usize)>,
    fn_count: usize,
    largest_fns: Vec<(String, String, usize)>, // name, file, span lines
    test_files: usize,
    test_count: usize,
    ci: Vec<String>,
    lint_configs: Vec<String>,
    todo_count: usize,
    fixme_count: usize,
    items: Vec<PubItem>,
    rfcs: Option<Vec<RfcEntry>>,
    rfcs_note: String,
    origin: String,
    branch: String,
    head: String,
    dirty: usize,
    is_git: bool,
}

// ---------------------------------------------------------------------------
// small helpers
// ---------------------------------------------------------------------------

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

fn git(root: &Path, args: &[&str]) -> String {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

const SKIP_DIRS: [&str; 6] = [".git", "target", "node_modules", "vendor", ".sq", "__pycache__"];

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(rd) = fs::read_dir(&dir) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    if !SKIP_DIRS.contains(&name.as_str()) {
                        stack.push(p);
                    }
                } else if p.is_file() {
                    out.push(p);
                }
            }
        }
    }
    out.sort();
    out
}

fn lang_of(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|e| e.to_str())? {
        "rs" => Some("Rust"),
        "py" => Some("Python"),
        "js" => Some("JavaScript"),
        "ts" => Some("TypeScript"),
        "go" => Some("Go"),
        "c" => Some("C"),
        "h" => Some("C header"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("C++"),
        "java" => Some("Java"),
        "rb" => Some("Ruby"),
        "sh" | "bash" => Some("Shell"),
        "md" => Some("Markdown"),
        "toml" => Some("TOML"),
        _ => None,
    }
}

fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root).unwrap_or(p).to_string_lossy().to_string()
}

// ---------------------------------------------------------------------------
// dependencies
// ---------------------------------------------------------------------------

fn parse_cargo_toml(text: &str) -> (Vec<(String, String)>, Option<String>, Option<String>) {
    let mut deps = Vec::new();
    let mut license = None;
    let mut pkg = None;
    let mut section = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            section = t.trim_matches(|c| c == '[' || c == ']').to_string();
            continue;
        }
        if section == "package" {
            if let Some(v) = t.strip_prefix("license =") {
                license = Some(v.trim().trim_matches('"').to_string());
            }
            if let Some(v) = t.strip_prefix("name =") {
                pkg = Some(v.trim().trim_matches('"').to_string());
            }
        }
        if section == "dependencies" || section == "dev-dependencies" || section == "build-dependencies" {
            if let Some(eq) = t.find('=') {
                let name = t[..eq].trim().trim_matches('"').to_string();
                let rest = t[eq + 1..].trim();
                let ver = if let Some(v) = rest.strip_prefix('"') {
                    v.trim_end_matches('"').to_string()
                } else if let Some(pos) = rest.find("version") {
                    let tail = &rest[pos..];
                    tail.split('"').nth(1).unwrap_or("").to_string()
                } else {
                    "(path or git)".to_string()
                };
                if !name.is_empty() {
                    deps.push((format!("{} [{}]", name, section), ver));
                }
            }
        }
    }
    (deps, license, pkg)
}

fn count_lock_packages(text: &str) -> usize {
    text.matches("[[package]]").count()
}

fn parse_package_json(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for key in ["\"dependencies\"", "\"devDependencies\""] {
        if let Some(start) = text.find(key) {
            let rest = &text[start..];
            if let Some(open) = rest.find('{') {
                let mut depth = 0i32;
                let mut end = open;
                for (i, ch) in rest[open..].char_indices() {
                    match ch {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                end = open + i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                let block = &rest[open..=end.min(rest.len() - 1)];
                for part in block.split(',') {
                    let mut it = part.splitn(2, ':');
                    if let (Some(k), Some(v)) = (it.next(), it.next()) {
                        let name = k.trim().trim_matches(|c| c == '"' || c == '{' || c == '}').to_string();
                        let ver = v.trim().trim_matches(|c| c == '"' || c == '{' || c == '}').to_string();
                        if !name.is_empty() && !ver.is_empty() {
                            out.push((name, ver));
                        }
                    }
                }
            }
        }
    }
    out
}

fn parse_requirements(text: &str) -> Vec<(String, String)> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            for sep in ["==", ">=", "~=", "<="] {
                if let Some(i) = l.find(sep) {
                    return (l[..i].trim().to_string(), l[i..].trim().to_string());
                }
            }
            (l.to_string(), "(unpinned)".to_string())
        })
        .collect()
}

fn parse_go_mod(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut in_req = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("require (") {
            in_req = true;
            continue;
        }
        if in_req && t == ")" {
            in_req = false;
            continue;
        }
        let body = if let Some(r) = t.strip_prefix("require ") { r } else if in_req { t } else { continue };
        let mut parts = body.split_whitespace();
        if let (Some(n), Some(v)) = (parts.next(), parts.next()) {
            out.push((n.to_string(), v.to_string()));
        }
    }
    out
}

fn scan_deps(root: &Path, files: &[PathBuf]) -> Vec<DepInfo> {
    let mut out = Vec::new();
    for f in files {
        let name = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        match name.as_str() {
            "Cargo.toml" => {
                let text = read(f);
                let (deps, license, _pkg) = parse_cargo_toml(&text);
                let total = files
                    .iter()
                    .find(|p| p.file_name().map(|n| n == "Cargo.lock").unwrap_or(false))
                    .map(|p| count_lock_packages(&read(p)));
                out.push(DepInfo { ecosystem: "Rust (cargo)".into(), manifest: rel(root, f), direct: deps, total_resolved: total, license });
            }
            "package.json" => {
                let text = read(f);
                let lic = text.find("\"license\"").and_then(|i| text[i..].split('"').nth(3)).map(|s| s.to_string());
                out.push(DepInfo { ecosystem: "JavaScript (npm)".into(), manifest: rel(root, f), direct: parse_package_json(&text), total_resolved: None, license: lic });
            }
            "requirements.txt" => {
                out.push(DepInfo { ecosystem: "Python (pip)".into(), manifest: rel(root, f), direct: parse_requirements(&read(f)), total_resolved: None, license: None });
            }
            "go.mod" => {
                out.push(DepInfo { ecosystem: "Go".into(), manifest: rel(root, f), direct: parse_go_mod(&read(f)), total_resolved: None, license: None });
            }
            _ => {}
        }
    }
    out
}

// ---------------------------------------------------------------------------
// declared RFCs: the target rfcs.json
// ---------------------------------------------------------------------------

fn parse_rfcs_json(text: &str) -> Vec<RfcEntry> {
    let mut out = Vec::new();
    // objects are flat in the rfcs.json schema: split on '{' and read keys.
    for chunk in text.split('{').skip(1) {
        let get_str = |key: &str| -> String {
            for pat in [format!("\"{}\"", key)] {
                if let Some(i) = chunk.find(&pat) {
                    let tail = &chunk[i + pat.len()..];
                    if let Some(c) = tail.find(':') {
                        let v = tail[c + 1..].trim_start();
                        if let Some(stripped) = v.strip_prefix('"') {
                            return stripped.split('"').next().unwrap_or("").to_string();
                        }
                        return v.split(|ch| ch == ',' || ch == '}' || ch == '\n').next().unwrap_or("").trim().to_string();
                    }
                }
            }
            String::new()
        };
        let number = {
            let n = get_str("rfc");
            if !n.is_empty() {
                n
            } else {
                let n2 = get_str("number");
                if n2.is_empty() {
                    get_str("id")
                } else {
                    n2
                }
            }
        };
        if number.chars().all(|c| c.is_ascii_digit()) && !number.is_empty() {
            let status = {
                let s = get_str("status");
                if !s.is_empty() {
                    s
                } else {
                    let sb = get_str("superseded_by");
                    if sb.is_empty() {
                        "current (as declared)".to_string()
                    } else {
                        format!("superseded by RFC {}", sb)
                    }
                }
            };
            out.push(RfcEntry {
                number,
                sha: get_str("sha256"),
                source: get_str("source"),
                status,
            });
        }
    }
    out
}

// ---------------------------------------------------------------------------
// public items and commands
// ---------------------------------------------------------------------------

fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_rust_items(text: &str, file: &str, items: &mut Vec<PubItem>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut doc: Vec<String> = Vec::new();
    let mut impl_name: Option<String> = None;
    let mut i = 0;
    while i < lines.len() {
        let raw = lines[i];
        let t = raw.trim();
        if let Some(d) = t.strip_prefix("///") {
            doc.push(d.trim().to_string());
            i += 1;
            continue;
        }
        if t.starts_with("#[") || t.is_empty() {
            if t.is_empty() {
                doc.clear();
            }
            i += 1;
            continue;
        }
        if t.starts_with("impl") && t.contains('{') {
            // impl [Trait for] Type {  /  impl Type {
            let body = t.trim_start_matches("impl").trim();
            let body = body.split('{').next().unwrap_or("").trim();
            let ty = body.rsplit(" for ").next().unwrap_or(body).trim();
            let ty = ty.split('<').next().unwrap_or(ty).trim().split_whitespace().last().unwrap_or(ty);
            impl_name = Some(ty.to_string());
            doc.clear();
            i += 1;
            continue;
        }
        if t == "}" {
            impl_name = None;
        }
        let kinds: [(&str, &str); 11] = [
            ("pub async fn ", "function"),
            ("pub unsafe fn ", "function"),
            ("pub fn ", "function"),
            ("pub struct ", "struct"),
            ("pub enum ", "enum"),
            ("pub trait ", "trait"),
            ("pub const ", "const"),
            ("pub static ", "const"),
            ("pub type ", "type"),
            ("pub use ", "re-export"),
            ("fn main(", "entry point"),
        ];
        let mut matched: Option<(&str, &str)> = None;
        for (pat, kind) in kinds {
            if t.starts_with(pat) {
                matched = Some((pat, kind));
                break;
            }
        }
        if let Some((pat, kind)) = matched {
            // gather the full signature: until '{' or ';'
            let mut sig = t.to_string();
            let mut j = i;
            while !sig.contains('{') && !sig.contains(';') && j + 1 < lines.len() && j - i < 12 {
                j += 1;
                sig.push(' ');
                sig.push_str(lines[j].trim());
            }
            let sig = collapse(&sig);
            let after = sig[pat.len()..].trim();
            let name = if kind == "entry point" {
                "main".to_string()
            } else if kind == "re-export" {
                after.trim_end_matches(';').to_string()
            } else {
                after
                    .split(|c: char| c == '(' || c == '<' || c == '{' || c == ';' || c.is_whitespace() || c == ':')
                    .next()
                    .unwrap_or("")
                    .to_string()
            };
            // extra: struct fields / enum variants / const value
            let mut extra = String::new();
            if (kind == "struct" || kind == "enum") && sig.contains('{') {
                let mut parts = Vec::new();
                let mut k = j;
                while k < lines.len() {
                    let lt = lines[k].trim();
                    if k > j && lt.starts_with('}') {
                        break;
                    }
                    if kind == "struct" && lt.starts_with("pub ") && lt.contains(':') {
                        parts.push(collapse(lt.trim_end_matches(',')));
                    }
                    if kind == "enum" && !lt.is_empty() && !lt.starts_with("//") && !lt.starts_with('#') && k != j {
                        let v = lt.trim_end_matches(',').split('(').next().unwrap_or("").split('{').next().unwrap_or("").trim();
                        if !v.is_empty() && v.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false) {
                            parts.push(v.to_string());
                        }
                    }
                    k += 1;
                    if k - j > 60 {
                        break;
                    }
                }
                extra = parts.join("; ");
            }
            if kind == "const" {
                if let Some(eq) = sig.find('=') {
                    let v = sig[eq + 1..].trim().trim_end_matches(';');
                    extra = v.chars().take(60).collect();
                }
            }
            let qualified = if kind == "function" {
                if let Some(owner) = &impl_name {
                    format!("{}::{}", owner, name)
                } else {
                    name.clone()
                }
            } else {
                name.clone()
            };
            items.push(PubItem {
                kind: kind.to_string(),
                name: qualified,
                signature: sig.chars().take(300).collect(),
                doc: doc.join(" "),
                extra,
                file: file.to_string(),
                line: i + 1,
            });
            doc.clear();
            i = j + 1;
            continue;
        }
        doc.clear();
        i += 1;
    }
}

fn extract_python_items(text: &str, file: &str, items: &mut Vec<PubItem>) {
    let lines: Vec<&str> = text.lines().collect();
    for (i, raw) in lines.iter().enumerate() {
        let t = raw.trim_start();
        if let Some(rest) = t.strip_prefix("def ") {
            if rest.starts_with('_') && !rest.starts_with("__") {
                continue; // private by the one-underscore mark
            }
            let sig = collapse(rest.split('{').next().unwrap_or(rest));
            let name = rest.split('(').next().unwrap_or("").trim().to_string();
            let mut doc = String::new();
            if i + 1 < lines.len() {
                let next = lines[i + 1].trim();
                if let Some(d) = next.strip_prefix("\"\"\"") {
                    doc = d.trim_end_matches("\"\"\"").trim().to_string();
                    if doc.is_empty() {
                        // multi-line docstring: take the next line
                        if i + 2 < lines.len() {
                            doc = lines[i + 2].trim().to_string();
                        }
                    }
                }
            }
            items.push(PubItem {
                kind: "function".into(),
                name,
                signature: format!("def {}", sig),
                doc,
                extra: String::new(),
                file: file.to_string(),
                line: i + 1,
            });
        }
    }
}

fn extract_shell_items(text: &str, file: &str, items: &mut Vec<PubItem>) {
    for (i, raw) in text.lines().enumerate() {
        let t = raw.trim();
        // name() {  or  name () {
        if t.ends_with("() {") || t.ends_with("(){") {
            let name = t.trim_end_matches("() {").trim_end_matches("(){").trim().to_string();
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
                items.push(PubItem {
                    kind: "function".into(),
                    name: name.clone(),
                    signature: format!("{}()", name),
                    doc: String::new(),
                    extra: String::new(),
                    file: file.to_string(),
                    line: i + 1,
                });
            }
        }
    }
}

fn extract_cli(text: &str, file: &str, items: &mut Vec<PubItem>) {
    let mut seen: Vec<String> = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let t = raw.trim();
        // match arms: "word" =>
        if t.contains("=>") {
            if let Some(q1) = t.find('"') {
                if let Some(q2rel) = t[q1 + 1..].find('"') {
                    let lit = &t[q1 + 1..q1 + 1 + q2rel];
                    if !lit.is_empty()
                        && lit.len() <= 24
                        && lit.chars().all(|c| c.is_ascii_lowercase() || c == '-' || c == '_')
                        && !seen.contains(&lit.to_string())
                    {
                        seen.push(lit.to_string());
                        items.push(PubItem {
                            kind: "command".into(),
                            name: lit.to_string(),
                            signature: format!("command \"{}\"", lit),
                            doc: String::new(),
                            extra: String::new(),
                            file: file.to_string(),
                            line: i + 1,
                        });
                    }
                }
            }
        }
        // flags: "--word" literals
        let mut rest = t;
        while let Some(p) = rest.find("\"--") {
            let tail = &rest[p + 1..];
            if let Some(e) = tail.find('"') {
                let flag = &tail[..e];
                if flag.len() > 2 && flag.len() <= 30 && !seen.contains(&flag.to_string()) {
                    seen.push(flag.to_string());
                    items.push(PubItem {
                        kind: "flag".into(),
                        name: flag.to_string(),
                        signature: format!("flag {}", flag),
                        doc: String::new(),
                        extra: String::new(),
                        file: file.to_string(),
                        line: i + 1,
                    });
                }
                rest = &tail[e..];
            } else {
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// explanations: doc comment first, then the signature, then awaiting
// ---------------------------------------------------------------------------

/// Returns (explanation, source) where source is
/// "doc comment" | "signature" | "agent" | "awaiting".
fn explain(item: &PubItem, agent: &BTreeMap<String, String>) -> (String, &'static str) {
    if let Some(s) = agent.get(&item.name).or_else(|| agent.get(&format!("{}:{}", item.file, item.name))) {
        return (s.clone(), "agent");
    }
    let doc = item.doc.trim();
    if !doc.is_empty() && !doc.eq_ignore_ascii_case("todo") {
        return (doc.to_string(), "doc comment");
    }
    if item.name.is_empty() {
        return (String::new(), "awaiting");
    }
    let sig = &item.signature;
    let text = match item.kind.as_str() {
        "function" => {
            let params = sig
                .split('(')
                .nth(1)
                .map(|p| p.split(')').next().unwrap_or("").trim().to_string())
                .unwrap_or_default();
            let ret = if let Some(r) = sig.split("->").nth(1) {
                format!(" It gives back {}.", collapse(r.split('{').next().unwrap_or("").trim()))
            } else {
                " It gives back nothing; it does its work on what it is given.".to_string()
            };
            let ret = ret.trim_start_matches(" It gives back ").trim_start_matches(" It gives back nothing; it does its work on what it is given.").trim_end_matches('.').to_string();
            if params.is_empty() || params == "self" || params == "&self" || params == "&mut self" {
                if ret.is_empty() {
                    format!("Function {} takes no inputs beyond the value it is called on, and gives back nothing; it does its work on what it is given.", item.name)
                } else {
                    format!("Function {} takes no inputs beyond the value it is called on, and gives back {}.", item.name, ret)
                }
            } else if ret.is_empty() {
                format!("Function {} takes {} and gives back nothing; it does its work on what it is given.", item.name, params)
            } else {
                format!("Function {} takes {} and gives back {}.", item.name, params, ret)
            }
        }
        "struct" => {
            if item.extra.is_empty() {
                format!("A data shape named {}. The scanner did not read its fields.", item.name)
            } else {
                format!("A data shape named {}. Its fields are: {}.", item.name, item.extra)
            }
        }
        "enum" => {
            if item.extra.is_empty() {
                format!("A named choice named {}. The scanner did not read its variants.", item.name)
            } else {
                format!("A named choice named {}. It is one of: {}.", item.name, item.extra)
            }
        }
        "trait" => format!("A trait named {}: a set of methods that a type can promise to give.", item.name),
        "const" => {
            if item.extra.is_empty() {
                format!("A fixed value named {}.", item.name)
            } else {
                format!("A fixed value named {}, set to {}.", item.name, item.extra)
            }
        }
        "type" => format!("A type name, {}, that stands for another type.", item.name),
        "re-export" => format!("A re-export: {} is made reachable through this module, so other code can use it from here.", item.name),
        "entry point" => "The entry point: the program starts here. It reads the command line and runs the command the user named.".to_string(),
        "command" => format!("A command named {}. The program runs it when the user types this word on the command line.", item.name),
        "flag" => format!("A command line flag, {}. It is read by the program when the user passes it.", item.name),
        _ => return (String::new(), "awaiting"),
    };
    (text, "signature")
}

// ---------------------------------------------------------------------------
// the scan itself
// ---------------------------------------------------------------------------

fn scan_project(root: &Path, name_override: &str) -> Scan {
    let files = walk(root);
    let mut s = Scan::default();
    let now = now_unix();
    s.date = date_string(now);
    s.ee = epoch_of(now);
    s.root = root.to_string_lossy().to_string();

    // upstream git repo
    s.is_git = root.join(".git").exists();
    if s.is_git {
        s.origin = git(root, &["remote", "get-url", "origin"]);
        s.branch = git(root, &["branch", "--show-current"]);
        s.head = git(root, &["rev-parse", "HEAD"]);
        let dirty = git(root, &["status", "--porcelain"]);
        s.dirty = if dirty.is_empty() { 0 } else { dirty.lines().count() };
        // authors
        let log = git(root, &["log", "--format=%an <%ae>"]);
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for line in log.lines() {
            if !line.trim().is_empty() {
                *counts.entry(line.trim().to_string()).or_insert(0) += 1;
                s.total_commits += 1;
            }
        }
        let mut authors: Vec<(String, usize)> = counts.into_iter().collect();
        authors.sort_by(|a, b| b.1.cmp(&a.1));
        s.authors = authors;
    }
    s.name = if !name_override.is_empty() {
        name_override.to_string()
    } else if !s.origin.is_empty() {
        s.origin.rsplit('/').next().unwrap_or("project").trim_end_matches(".git").to_string()
    } else {
        root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "project".into())
    };

    // dependencies
    s.deps = scan_deps(root, &files);

    // declared RFCs
    let rfcs_path = root.join("rfcs.json");
    if rfcs_path.exists() {
        let text = read(&rfcs_path);
        let entries = parse_rfcs_json(&text);
        if entries.is_empty() {
            s.rfcs_note = format!("An rfcs.json file is present ({} bytes) but no RFC entries could be read from it by the scanner's reader.", text.len());
        } else {
            s.rfcs_note = format!("Declared in rfcs.json ({} entries).", entries.len());
        }
        s.rfcs = Some(entries);
    } else {
        s.rfcs = None;
        s.rfcs_note = "No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.".to_string();
    }

    // complexity + quality + items
    let mut largest: Vec<(String, usize)> = Vec::new();
    let mut fns: Vec<(String, String, usize, usize)> = Vec::new(); // name,file,start,end-span computed later
    for f in &files {
        let fname = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        // CI + lint signals by filename
        let relp = rel(root, f);
        if fname == "Jenkinsfile" {
            s.ci.push("Jenkins file present (Jenkinsfile) — CI is declared in the project.".to_string());
        }
        if relp.contains(".github/workflows/") {
            s.ci.push(format!("GitHub Actions workflow found at {} — house rule: CI runs on ranch Jenkins, never GitHub Actions. This is flagged, not judged.", relp));
        }
        for cfg in ["rustfmt.toml", ".rustfmt.toml", "clippy.toml", ".clippy.toml", ".eslintrc", ".eslintrc.json", ".eslintrc.js", ".prettierrc", "Makefile", ".editorconfig"] {
            if fname == cfg || fname.starts_with(".eslintrc") && cfg == ".eslintrc" {
                if !s.lint_configs.contains(&relp) {
                    s.lint_configs.push(relp.clone());
                }
            }
        }
        if fname.starts_with("test") || relp.contains("/tests/") || fname.contains("_test.") || fname.ends_with("_test.go") {
            s.test_files += 1;
        }
        let lang = match lang_of(f) {
            Some(l) => l,
            None => continue,
        };
        let text = read(f);
        let mut total = 0usize;
        let mut blank = 0usize;
        let mut comment = 0usize;
        let mut in_block = false;
        let mut fn_starts: Vec<(String, usize)> = Vec::new();
        for (ln, line) in text.lines().enumerate() {
            total += 1;
            let t = line.trim();
            if t.is_empty() {
                blank += 1;
                continue;
            }
            let is_comment = if in_block {
                if t.contains("*/") {
                    in_block = false;
                }
                true
            } else if t.starts_with("/*") {
                if !t.contains("*/") {
                    in_block = true;
                }
                true
            } else {
                t.starts_with("//") || (lang != "Rust" && t.starts_with('#') && lang != "Markdown") || t.starts_with("--")
            };
            if is_comment {
                comment += 1;
            }
            if t.contains("TODO") {
                s.todo_count += 1;
            }
            if t.contains("FIXME") {
                s.fixme_count += 1;
            }
            if t.contains("#[test]") || t.contains("#[tokio::test]") {
                s.test_count += 1;
            }
            if lang == "Rust" {
                let body = t.trim_start_matches("pub ").trim_start_matches("async ").trim_start_matches("unsafe ");
                if body.starts_with("fn ") || body.starts_with("fn\t") {
                    let nm = body[3..].split('(').next().unwrap_or("").trim().to_string();
                    if !nm.is_empty() {
                        fn_starts.push((nm, ln));
                    }
                }
            }
            if lang == "Python" {
                if let Some(rest) = t.strip_prefix("def ") {
                    let nm = rest.split('(').next().unwrap_or("").trim().to_string();
                    if nm.starts_with("test_") {
                        s.test_count += 1;
                    }
                    if !nm.is_empty() {
                        fn_starts.push((nm, ln));
                    }
                }
            }
            if lang == "Shell" && (t.ends_with("() {") || t.ends_with("(){")) {
                let nm = t.trim_end_matches("() {").trim_end_matches("(){").trim().to_string();
                if !nm.is_empty() {
                    fn_starts.push((nm, ln));
                }
            }
        }
        let code = total - blank - comment;
        let e = s.lang_lines.entry(lang.to_string()).or_insert((0, 0, 0, 0, 0));
        e.0 += 1;
        e.1 += total;
        e.2 += blank;
        e.3 += comment;
        e.4 += code;
        largest.push((relp.clone(), total));
        for (idx, (nm, start)) in fn_starts.iter().enumerate() {
            let end = if idx + 1 < fn_starts.len() { fn_starts[idx + 1].1 } else { total };
            fns.push((nm.clone(), relp.clone(), *start, end - start));
        }
        // public items
        match lang {
            "Rust" => {
                extract_rust_items(&text, &relp, &mut s.items);
                let base = f.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                if base == "main.rs" || relp.contains("/bin/") {
                    extract_cli(&text, &relp, &mut s.items);
                }
            }
            "Python" => extract_python_items(&text, &relp, &mut s.items),
            "Shell" => extract_shell_items(&text, &relp, &mut s.items),
            _ => {}
        }
    }
    s.fn_count = fns.len();
    fns.sort_by(|a, b| b.3.cmp(&a.3));
    s.largest_fns = fns.iter().take(5).map(|(n, f, _st, span)| (n.clone(), f.clone(), *span)).collect();
    largest.sort_by(|a, b| b.1.cmp(&a.1));
    s.largest_files = largest.into_iter().take(5).collect();
    s
}

// ---------------------------------------------------------------------------
// rendering: the same words in markdown and in phext
// ---------------------------------------------------------------------------

fn sections(scan: &Scan, agent: &BTreeMap<String, String>) -> (Vec<(String, String)>, Vec<PubItem>) {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut awaiting: Vec<PubItem> = Vec::new();

    let mut home = String::new();
    home.push_str(&format!("# {} — project documentation\n\n", scan.name));
    home.push_str(&format!(
        "Scanned by phdoc {} on {} (Exocortical Epoch {}). Project root: {}. ",
        env!("CARGO_PKG_VERSION"),
        scan.date,
        scan.ee,
        scan.root
    ));
    if scan.is_git {
        home.push_str(&format!("Scanned at commit {}. ", scan.head));
    }
    home.push_str("Every fact in this document was read from its own records: its manifests, its git log, and its source text. Nothing here was guessed.\n\n");
    home.push_str("This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.\n");
    out.push(("Home".into(), home));

    // dependencies
    let mut d = String::new();
    d.push_str("## Dependencies\n\n");
    if scan.deps.is_empty() {
        d.push_str("No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.\n");
    }
    for dep in &scan.deps {
        d.push_str(&format!("### {} — {}\n\n", dep.ecosystem, dep.manifest));
        if let Some(lic) = &dep.license {
            d.push_str(&format!("License stated in the manifest: {}.\n\n", lic));
        }
        d.push_str(&format!("Direct dependencies: {}.\n\n", dep.direct.len()));
        for (n, v) in &dep.direct {
            d.push_str(&format!("- {} — {}\n", n, v));
        }
        if let Some(t) = dep.total_resolved {
            d.push_str(&format!("\nTotal packages in the lock file, this one included: {}.\n", t));
        }
        d.push('\n');
    }
    out.push(("Dependencies".into(), d));

    // authors
    let mut a = String::new();
    a.push_str("## Authors\n\n");
    if scan.authors.is_empty() {
        a.push_str("No git history was read, so no authors are recorded.\n");
    } else {
        a.push_str(&format!("The git log holds {} commits. The authors, by commit count:\n\n", scan.total_commits));
        for (who, n) in &scan.authors {
            let share = if scan.total_commits > 0 { (*n as f64) * 100.0 / (scan.total_commits as f64) } else { 0.0 };
            a.push_str(&format!("- {} — {} commit{} ({:.1}%)\n", who, n, if *n == 1 { "" } else { "s" }, share));
        }
    }
    out.push(("Authors".into(), a));

    // complexity
    let mut c = String::new();
    c.push_str("## Complexity\n\n");
    c.push_str("The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.\n\n");
    let mut tf = 0usize;
    let mut tt = 0usize;
    let mut tc = 0usize;
    for (lang, (files, total, blank, comment, code)) in &scan.lang_lines {
        c.push_str(&format!("- {}: {} files, {} lines total, {} blank, {} comment, {} code\n", lang, files, total, blank, comment, code));
        tf += files;
        tt += total;
        tc += code;
    }
    c.push_str(&format!("\nAll languages: {} files, {} lines total, {} lines of code. Functions read: {}.\n\n", tf, tt, tc, scan.fn_count));
    c.push_str("Largest files:\n\n");
    for (f, n) in &scan.largest_files {
        c.push_str(&format!("- {} — {} lines\n", f, n));
    }
    c.push_str("\nLargest functions, by span in lines:\n\n");
    for (n, f, span) in &scan.largest_fns {
        c.push_str(&format!("- {} in {} — about {} lines\n", n, f, span));
    }
    out.push(("Complexity".into(), c));

    // quality
    let mut q = String::new();
    q.push_str("## Code quality signals\n\n");
    q.push_str("Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.\n\n");
    q.push_str(&format!("- Tests: {} test functions counted in the source, across {} files whose names mark them as test files.\n", scan.test_count, scan.test_files));
    if scan.ci.is_empty() {
        q.push_str("- CI: no CI configuration was found in the project.\n");
    }
    for line in &scan.ci {
        q.push_str(&format!("- CI: {}\n", line));
    }
    if scan.lint_configs.is_empty() {
        q.push_str("- Lint and format: no lint or format configuration file was found.\n");
    } else {
        q.push_str(&format!("- Lint and format configuration files: {}.\n", scan.lint_configs.join(", ")));
    }
    q.push_str(&format!("- TODO marks in the source: {}. FIXME marks: {}.\n", scan.todo_count, scan.fixme_count));
    let total_items = scan.items.len();
    let documented = scan.items.iter().filter(|i| !i.doc.trim().is_empty()).count();
    if total_items > 0 {
        q.push_str(&format!(
            "- Public items with a doc comment: {} of {} ({:.0}%).\n",
            documented,
            total_items,
            (documented as f64) * 100.0 / (total_items as f64)
        ));
    }
    out.push(("Code quality".into(), q));

    // declared RFCs
    let mut r = String::new();
    r.push_str("## Declared RFCs\n\n");
    r.push_str(&format!("{}\n\n", scan.rfcs_note));
    if let Some(entries) = &scan.rfcs {
        for e in entries {
            r.push_str(&format!(
                "- RFC {} — status: {}; hash: {}; source: {}\n",
                e.number,
                e.status,
                if e.sha.is_empty() { "(none recorded)" } else { &e.sha },
                if e.source.is_empty() { "(none recorded)" } else { &e.source }
            ));
        }
    }
    out.push(("Declared RFCs".into(), r));

    // upstream
    let mut u = String::new();
    u.push_str("## Upstream git repo\n\n");
    if !scan.is_git {
        u.push_str("This project is not a git repo, so it has no upstream to record.\n");
    } else {
        u.push_str(&format!("- Origin: {}\n", if scan.origin.is_empty() { "(no origin remote set)" } else { &scan.origin }));
        u.push_str(&format!("- Branch: {}\n", if scan.branch.is_empty() { "(detached or unnamed)" } else { &scan.branch }));
        u.push_str(&format!("- Head commit: {}\n", scan.head));
        u.push_str(&format!(
            "- Working tree: {}\n",
            if scan.dirty == 0 { "clean".to_string() } else { format!("{} changed paths", scan.dirty) }
        ));
    }
    out.push(("Upstream git repo".into(), u));

    // public items index + per-item sections
    let mut idx = String::new();
    idx.push_str("## Public methods and commands\n\n");
    idx.push_str("Every public method and command the scanner found, each with a plain English note. A note says where its words came from: the code's own doc comment, the item's signature, the user's agent, or it is marked awaiting a summary.\n\n");
    for item in &scan.items {
        let (text, source) = explain(item, agent);
        if source == "awaiting" {
            awaiting.push(item.clone());
            idx.push_str(&format!("- {} `{}` ({}:{}) — awaiting a summary\n", item.kind, item.name, item.file, item.line));
        } else {
            idx.push_str(&format!("- {} `{}` ({}:{}) — {}\n", item.kind, item.name, item.file, item.line, first_sentence(&text)));
        }
    }
    out.push(("Public methods and commands".into(), idx));

    for item in &scan.items {
        let (text, source) = explain(item, agent);
        let mut b = String::new();
        b.push_str(&format!("## {} — {}\n\n", item.kind, item.name));
        b.push_str(&format!("Found in {} at line {}.\n\n", item.file, item.line));
        b.push_str(&format!("Signature: `{}`\n\n", item.signature));
        if source == "awaiting" {
            b.push_str("Summary: awaiting a summary. This item is named in the instruction file that came with this scan. No words were invented for it.\n");
        } else {
            b.push_str(&format!("What it does: {}\n\n", text));
            b.push_str(&format!("(Where these words came from: {}.)\n", source));
        }
        out.push((format!("{} {}", item.kind, item.name), b));
    }
    (out, awaiting)
}

fn first_sentence(text: &str) -> String {
    let t = text.trim();
    match t.find(". ") {
        Some(i) => t[..=i].to_string(),
        None => t.chars().take(120).collect(),
    }
}

fn render_markdown(sections: &[(String, String)]) -> String {
    let mut out = String::new();
    for (i, (_title, body)) in sections.iter().enumerate() {
        if i > 0 {
            out.push_str("\n---\n\n");
        }
        out.push_str(body);
        if !body.ends_with('\n') {
            out.push('\n');
        }
    }
    out
}

fn render_phext(sections: &[(String, String)]) -> String {
    let mut text = String::new();
    for (i, (_title, body)) in sections.iter().enumerate() {
        let coord = if i < 7 {
            phext::to_coordinate(&format!("1.1.1/1.1.1/1.1.{}", i + 1))
        } else {
            phext::to_coordinate(&format!("1.1.1/1.2.1/1.1.{}", i - 6))
        };
        text = phext::insert(text, coord, body);
    }
    text
}

fn instruction_file(scan: &Scan, awaiting: &[PubItem]) -> String {
    let mut s = String::new();
    s.push_str("# Summaries needed — phdoc instruction file\n\n");
    s.push_str(&format!(
        "phdoc scanned {} on {} (Exocortical Epoch {}) and could not write a summary for the items below from the code's doc comments or signatures alone. No words were invented for them.\n\n",
        scan.name, scan.date, scan.ee
    ));
    s.push_str("## What to do\n\n");
    s.push_str("Have your agent read each item in its file, write one to three plain sentences that say what it does, what it takes, and what it gives back, and put them in a summaries file: one line per item, in the form `name :: the summary`. Then run phdoc again with `--summaries <that file>` and the summaries will be carried into the documentation, marked as supplied by the user's agent.\n\n");
    s.push_str("An LLM endpoint of your choice can also be set in phdoc's config to draft these, but that is never required and never on by default.\n\n");
    s.push_str("## Items that need a summary\n\n");
    for item in awaiting {
        s.push_str(&format!("- {} `{}` — {} at line {}. Signature: `{}`\n", item.kind, item.name, item.file, item.line, item.signature));
    }
    s
}

fn load_summaries(path: &Path) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in read(path).lines() {
        if let Some((k, v)) = line.split_once("::") {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    map
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn usage() -> String {
    "phdoc — a project scanner for shipped artifacts.\n\nCommands:\n  phdoc scan <project-dir> --out <output-dir> [--name <name>] [--summaries <file>]\n      Scan a project once and write docs.md and docs.phext into the output dir.\n      With --summaries, a file of `name :: summary` lines from the user's\n      agent is carried into the documentation.\n  phdoc audit <project-dir> [--repo <phdoc-repo-dir>] [--out <dir>] [--name <name>] [--hold-push]\n      Audit every unaudited commit, root to head: append one universe per\n      commit to the project's multiversal phext chain, render the latest\n      and historical references, update the project's .phdoc.json ledger,\n      report the unreviewed backlog, and export the changes to the export\n      repo (commit and push) unless --hold-push is given. The export repo\n      and the doc site are the project's own configuration, held in its\n      .phdoc.json as \"export-repo\" and \"doc-site\" (defaults: wbic16/phdoc\n      and https://docs.phext.io); when an export repo is configured and no\n      --repo is given, phdoc keeps a managed clone of that repo to export to.\n  phdoc review <project-dir> <commit> --by <name>\n      Mark a commit reviewed in the project's .phdoc.json.\n  phdoc render <project-dir> --at <commit|latest> [--repo <phdoc-repo-dir>] [--out <dir>]\n      Render the reference as of a commit by folding the chain.\n  phdoc help\n      Show this text.\n  phdoc version\n      Show the version.\n".to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    match cmd {
        "help" | "--help" | "-h" => print!("{}", usage()),
        "version" | "--version" => println!("phdoc {}", env!("CARGO_PKG_VERSION")),
        "scan" => {
            let mut project: Option<String> = None;
            let mut out_dir = String::from("phdoc-out");
            let mut name = String::new();
            let mut summaries: Option<PathBuf> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--out" => {
                        i += 1;
                        out_dir = args.get(i).cloned().unwrap_or(out_dir);
                    }
                    "--name" => {
                        i += 1;
                        name = args.get(i).cloned().unwrap_or_default();
                    }
                    "--summaries" => {
                        i += 1;
                        summaries = args.get(i).map(PathBuf::from);
                    }
                    other => {
                        if !other.starts_with("--") && project.is_none() {
                            project = Some(other.to_string());
                        }
                    }
                }
                i += 1;
            }
            let project = match project {
                Some(p) => p,
                None => {
                    eprintln!("phdoc scan: a project dir is required.\n\n{}", usage());
                    std::process::exit(2);
                }
            };
            let root = PathBuf::from(&project);
            if !root.is_dir() {
                eprintln!("phdoc scan: {} is not a directory.", project);
                std::process::exit(2);
            }
            let agent = summaries.as_ref().map(|p| load_summaries(p)).unwrap_or_default();
            let scan = scan_project(&root, &name);
            let (mut sections, awaiting) = sections(&scan, &agent);
            let cfg = load_ledger(&root);
            sections.push(provenance_section(&scan.name, eff_export_repo(&cfg), eff_doc_site(&cfg)));
            let out = PathBuf::from(&out_dir);
            if let Err(e) = fs::create_dir_all(&out) {
                eprintln!("phdoc scan: cannot create {}: {}", out_dir, e);
                std::process::exit(1);
            }
            let md = render_markdown(&sections);
            let px = render_phext(&sections);
            fs::write(out.join("docs.md"), &md).expect("write docs.md");
            fs::write(out.join("docs.phext"), &px).expect("write docs.phext");
            println!(
                "phdoc: scanned {} — {} files of code, {} public items, {} dependencies across {} manifests.",
                scan.name,
                scan.lang_lines.values().map(|v| v.0).sum::<usize>(),
                scan.items.len(),
                scan.deps.iter().map(|d| d.direct.len()).sum::<usize>(),
                scan.deps.len()
            );
            println!("phdoc: wrote {} ({} bytes) and {} ({} bytes, checksum {}).", out.join("docs.md").display(), md.len(), out.join("docs.phext").display(), px.len(), phext::checksum(&px));
            if awaiting.is_empty() {
                println!("phdoc: every public item was explained from a doc comment or its signature. No instruction file was needed.");
            } else {
                let instr = instruction_file(&scan, &awaiting);
                fs::write(out.join("SUMMARY-INSTRUCTIONS.md"), &instr).expect("write instructions");
                println!(
                    "phdoc: {} items need a summary the scanner could not write. They are listed in {}. Have your agent write them and re-run with --summaries.",
                    awaiting.len(),
                    out.join("SUMMARY-INSTRUCTIONS.md").display()
                );
            }
        }
        "audit" => {
            let mut a = AuditArgs { project: String::new(), repo: None, out: None, name: String::new(), hold_push: false };
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--repo" => {
                        i += 1;
                        a.repo = args.get(i).map(PathBuf::from);
                    }
                    "--out" => {
                        i += 1;
                        a.out = args.get(i).map(PathBuf::from);
                    }
                    "--name" => {
                        i += 1;
                        a.name = args.get(i).cloned().unwrap_or_default();
                    }
                    "--hold-push" => a.hold_push = true,
                    other => {
                        if !other.starts_with("--") && a.project.is_empty() {
                            a.project = other.to_string();
                        }
                    }
                }
                i += 1;
            }
            if a.project.is_empty() {
                eprintln!("phdoc audit: a project dir is required.\n\n{}", usage());
                std::process::exit(2);
            }
            std::process::exit(cmd_audit(a));
        }
        "review" => {
            let mut project = String::new();
            let mut sha = String::new();
            let mut by = String::new();
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--by" => {
                        i += 1;
                        by = args.get(i).cloned().unwrap_or_default();
                    }
                    other => {
                        if !other.starts_with("--") {
                            if project.is_empty() {
                                project = other.to_string();
                            } else if sha.is_empty() {
                                sha = other.to_string();
                            }
                        }
                    }
                }
                i += 1;
            }
            if project.is_empty() || sha.is_empty() || by.is_empty() {
                eprintln!("phdoc review: use phdoc review <project-dir> <commit> --by <name>.");
                std::process::exit(2);
            }
            std::process::exit(cmd_review(&project, &sha, &by));
        }
        "render" => {
            let mut project = String::new();
            let mut at = String::from("latest");
            let mut repo: Option<PathBuf> = None;
            let mut out: Option<PathBuf> = None;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--at" => {
                        i += 1;
                        at = args.get(i).cloned().unwrap_or(at);
                    }
                    "--repo" => {
                        i += 1;
                        repo = args.get(i).map(PathBuf::from);
                    }
                    "--out" => {
                        i += 1;
                        out = args.get(i).map(PathBuf::from);
                    }
                    other => {
                        if !other.starts_with("--") && project.is_empty() {
                            project = other.to_string();
                        }
                    }
                }
                i += 1;
            }
            if project.is_empty() {
                eprintln!("phdoc render: a project dir is required.");
                std::process::exit(2);
            }
            std::process::exit(cmd_render(&project, &at, &repo, &out));
        }
        other => {
            eprintln!("phdoc: unknown command \"{}\".\n\n{}", other, usage());
            std::process::exit(2);
        }
    }
}

// ---------------------------------------------------------------------------
// the multiversal chain: one universe per audited commit, deltas in order
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
struct FoldState {
    items: BTreeMap<String, PubItem>, // key: file :: name
    item_source: BTreeMap<String, String>,
    deps: BTreeMap<String, Vec<(String, String)>>, // manifest -> (name, version)
    authors: BTreeMap<String, usize>,
    total_commits: usize,
    lang: BTreeMap<String, (usize, usize, usize, usize, usize)>,
    rfcs_count: Option<usize>,
    head_sha: String,
    head_date: String,
}

fn git_bytes(root: &Path, args: &[&str]) -> Option<Vec<u8>> {
    let o = Command::new("git").arg("-C").arg(root).args(args).output().ok()?;
    if o.status.success() {
        Some(o.stdout)
    } else {
        None
    }
}

fn git_str(root: &Path, args: &[&str]) -> String {
    git_bytes(root, args).map(|b| String::from_utf8_lossy(&b).trim().to_string()).unwrap_or_default()
}

fn file_at(root: &Path, sha: &str, path: &str) -> Option<String> {
    let bytes = git_bytes(root, &["show", &format!("{}:{}", sha, path)])?;
    if bytes.contains(&0) {
        return None; // binary: not a text the scanner reads
    }
    Some(String::from_utf8_lossy(&bytes).to_string())
}

fn items_at(root: &Path, sha: &str, path: &str) -> Vec<PubItem> {
    let text = match file_at(root, sha, path) {
        Some(t) => t,
        None => return Vec::new(),
    };
    let p = Path::new(path);
    let mut items = Vec::new();
    match lang_of(p) {
        Some("Rust") => {
            extract_rust_items(&text, path, &mut items);
            let base = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if base == "main.rs" || path.contains("/bin/") {
                extract_cli(&text, path, &mut items);
            }
        }
        Some("Python") => extract_python_items(&text, path, &mut items),
        Some("Shell") => extract_shell_items(&text, path, &mut items),
        _ => {}
    }
    items
}

fn line_classes(text: &str, lang: &str) -> (usize, usize, usize, usize) {
    let mut total = 0usize;
    let mut blank = 0usize;
    let mut comment = 0usize;
    let mut in_block = false;
    for line in text.lines() {
        total += 1;
        let t = line.trim();
        if t.is_empty() {
            blank += 1;
            continue;
        }
        let is_comment = if in_block {
            if t.contains("*/") {
                in_block = false;
            }
            true
        } else if t.starts_with("/*") {
            if !t.contains("*/") {
                in_block = true;
            }
            true
        } else {
            t.starts_with("//") || (lang != "Rust" && t.starts_with('#') && lang != "Markdown")
        };
        if is_comment {
            comment += 1;
        }
    }
    (total, blank, comment, total - blank - comment)
}

const EMPTY_TREE: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

/// Audit one commit against its parent. Returns (delta scroll text, ok).
fn audit_commit(root: &Path, sha: &str, parent: Option<&str>, checkpoint: bool, state: &mut FoldState) -> String {
    let info = git_str(root, &["show", "-s", "--format=%an <%ae>%n%at%n%s", sha]);
    let mut info_lines = info.lines();
    let author = info_lines.next().unwrap_or("").to_string();
    let at: u64 = info_lines.next().unwrap_or("0").parse().unwrap_or(0);
    let subject = info_lines.next().unwrap_or("").to_string();
    let parent_str = parent.unwrap_or("none");

    // changed files
    let ns = if parent.is_some() {
        git_str(root, &["diff-tree", "--no-commit-id", "--name-status", "-r", sha])
    } else {
        git_str(root, &["diff-tree", "--root", "--no-commit-id", "--name-status", "-r", sha])
    };
    let mut changed: Vec<(String, String)> = Vec::new();
    for line in ns.lines() {
        let mut parts = line.split('\t');
        if let (Some(st), Some(p)) = (parts.next(), parts.next()) {
            changed.push((st.to_string(), p.to_string()));
        }
    }

    let mut delta: Vec<String> = Vec::new();
    let mut n_add = 0usize;
    let mut n_del = 0usize;
    let mut n_chg = 0usize;
    let mut dep_notes: Vec<String> = Vec::new();

    // item + dependency deltas from changed files
    for (status, path) in &changed {
        let p = Path::new(path);
        let fname = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if ["Cargo.toml", "package.json", "requirements.txt", "go.mod"].contains(&fname.as_str()) {
            let new_deps = if status == "D" {
                Vec::new()
            } else {
                let t = file_at(root, sha, path).unwrap_or_default();
                match fname.as_str() {
                    "Cargo.toml" => parse_cargo_toml(&t).0.into_iter().map(|(n, v)| (n.split(" [").next().unwrap_or(&n).to_string(), v)).collect(),
                    "package.json" => parse_package_json(&t),
                    "requirements.txt" => parse_requirements(&t),
                    _ => parse_go_mod(&t),
                }
            };
            let old_deps = match parent {
                Some(par) if status != "A" => {
                    let t = file_at(root, par, path).unwrap_or_default();
                    match fname.as_str() {
                        "Cargo.toml" => parse_cargo_toml(&t).0.into_iter().map(|(n, v)| (n.split(" [").next().unwrap_or(&n).to_string(), v)).collect(),
                        "package.json" => parse_package_json(&t),
                        "requirements.txt" => parse_requirements(&t),
                        _ => parse_go_mod(&t),
                    }
                }
                _ => Vec::new(),
            };
            let old_map: BTreeMap<String, String> = old_deps.iter().cloned().collect();
            let new_map: BTreeMap<String, String> = new_deps.iter().cloned().collect();
            for (n, v) in &new_map {
                match old_map.get(n) {
                    None => {
                        delta.push(format!("dep+ {} :: {} :: {}", path, n, v));
                        dep_notes.push(format!("{} added at {}", n, v));
                    }
                    Some(ov) if ov != v => {
                        delta.push(format!("dep~ {} :: {} :: {} -> {}", path, n, ov, v));
                        dep_notes.push(format!("{} {} -> {}", n, ov, v));
                    }
                    _ => {}
                }
            }
            for n in old_map.keys() {
                if !new_map.contains_key(n) {
                    delta.push(format!("dep- {} :: {}", path, n));
                    dep_notes.push(format!("{} removed", n));
                }
            }
            if status == "D" {
                state.deps.remove(path);
            } else {
                state.deps.insert(path.clone(), new_deps);
            }
            continue;
        }
        // rfcs.json movement (checked before the language gate: json has no language entry)
        if fname == "rfcs.json" {
            let count = if status == "D" {
                None
            } else {
                Some(parse_rfcs_json(&file_at(root, sha, path).unwrap_or_default()).len())
            };
            state.rfcs_count = count;
            delta.push(format!("rfcs {}", count.map(|c| c.to_string()).unwrap_or_else(|| "none".into())));
            continue;
        }
        if lang_of(p).is_none() {
            continue;
        }
        let new_items = if status == "D" { Vec::new() } else { items_at(root, sha, path) };
        let old_items = match parent {
            Some(par) if status != "A" => items_at(root, par, path),
            _ => Vec::new(),
        };
        let old_map: BTreeMap<String, PubItem> = old_items.iter().map(|i| (format!("{} :: {}", i.file, i.name), i.clone())).collect();
        let new_map: BTreeMap<String, PubItem> = new_items.iter().map(|i| (format!("{} :: {}", i.file, i.name), i.clone())).collect();
        for (k, item) in &new_map {
            match old_map.get(k) {
                None => {
                    let (text, source) = explain(item, &BTreeMap::new());
                    delta.push(format!("item+ {} {} :: {} :: {} :: {} :: {}", item.kind.replace(' ', "-"), item.file, item.name, item.signature, text.replace('\n', " "), source));
                    state.items.insert(k.clone(), item.clone());
                    state.item_source.insert(k.clone(), source.to_string());
                    n_add += 1;
                }
                Some(old) if old.signature != item.signature || old.doc != item.doc => {
                    let (text, source) = explain(item, &BTreeMap::new());
                    delta.push(format!("item~ {} {} :: {} :: {} :: {} :: {}", item.kind.replace(' ', "-"), item.file, item.name, item.signature, text.replace('\n', " "), source));
                    state.items.insert(k.clone(), item.clone());
                    state.item_source.insert(k.clone(), source.to_string());
                    n_chg += 1;
                }
                _ => {}
            }
        }
        for k in old_map.keys() {
            if !new_map.contains_key(k) {
                let old = &old_map[k];
                delta.push(format!("item- {} {} :: {}", old.kind.replace(' ', "-"), old.file, old.name));
                state.items.remove(k);
                state.item_source.remove(k);
                n_del += 1;
            }
        }
    }

    // complexity: exact at checkpoints, numstat-adjusted between them
    if checkpoint {
        let tree = git_str(root, &["ls-tree", "-r", "--name-only", sha]);
        let mut lang: BTreeMap<String, (usize, usize, usize, usize, usize)> = BTreeMap::new();
        for path in tree.lines() {
            let p = Path::new(path);
            if let Some(l) = lang_of(p) {
                if let Some(text) = file_at(root, sha, path) {
                    let (t, b, c, code) = line_classes(&text, l);
                    let e = lang.entry(l.to_string()).or_insert((0, 0, 0, 0, 0));
                    e.0 += 1;
                    e.1 += t;
                    e.2 += b;
                    e.3 += c;
                    e.4 += code;
                }
            }
        }
        state.lang = lang;
    } else {
        let base = parent.unwrap_or(EMPTY_TREE);
        let numstat = git_str(root, &["diff", "--numstat", base, sha]);
        let mut file_delta: BTreeMap<String, i64> = BTreeMap::new();
        for line in numstat.lines() {
            let mut parts = line.split('\t');
            let add: i64 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
            let del: i64 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(0);
            if let Some(path) = parts.next() {
                if let Some(l) = lang_of(Path::new(path)) {
                    let e = state.lang.entry(l.to_string()).or_insert((0, 0, 0, 0, 0));
                    let net = add - del;
                    e.1 = (e.1 as i64 + net).max(0) as usize;
                    e.4 = (e.4 as i64 + net).max(0) as usize;
                    *file_delta.entry(l.to_string()).or_insert(0) += net;
                }
            }
        }
        // file counts from name-status
        for (status, path) in &changed {
            if let Some(l) = lang_of(Path::new(path)) {
                let e = state.lang.entry(l.to_string()).or_insert((0, 0, 0, 0, 0));
                if status == "A" {
                    e.0 += 1;
                } else if status == "D" {
                    e.0 = e.0.saturating_sub(1);
                }
            }
        }
    }
    for (l, (f, t, b, c, code)) in &state.lang {
        delta.push(format!("stat {} files {} total {} blank {} comment {} code {}", l, f, t, b, c, code));
    }

    // author
    if !author.is_empty() {
        let n = {
            let e = state.authors.entry(author.clone()).or_insert(0);
            *e += 1;
            *e
        };
        if n == 1 {
            delta.push(format!("author+ {}", author));
        } else {
            delta.push(format!("author~ {} commits {}", author, n));
        }
    }
    state.total_commits += 1;
    state.head_sha = sha.to_string();
    state.head_date = date_string(at);

    // the scroll: a plain summary, then the delta lines the fold reads
    let code_now: usize = state.lang.values().map(|v| v.4).sum();
    let mut scroll = String::new();
    scroll.push_str(&format!("Commit {} by {} on {} (EE {}).\n\n", &sha[..sha.len().min(12)], author, date_string(at), epoch_of(at)));
    scroll.push_str(&format!("Subject: {}\n\n", subject));
    scroll.push_str(&format!(
        "Public items: {} added, {} removed, {} changed. ",
        n_add, n_del, n_chg
    ));
    if dep_notes.is_empty() {
        scroll.push_str("Dependencies did not change. ");
    } else {
        scroll.push_str(&format!("Dependencies: {}. ", dep_notes.join("; ")));
    }
    scroll.push_str(&format!("Code lines after this commit: {}.\n\n", code_now));
    if checkpoint {
        scroll.push_str("This commit is a checkpoint: the line counts were read from the whole tree, not from the diff.\n\n");
    }
    scroll.push_str(&format!("commit {}\nparent {}\nauthor {}\ndate {} {}\n", sha, parent_str, author, at, date_string(at)));
    for line in &delta {
        scroll.push_str(line);
        scroll.push('\n');
    }
    scroll
}

/// Fold a chain text up to universe `upto` (1-based count of universes).
fn fold_chain(chain: &str, upto: usize) -> FoldState {
    let mut state = FoldState::default();
    for u in 1..=upto {
        let coord = phext::to_coordinate(&format!("{}.1.1/1.1.1/1.1.1", u));
        let scroll = phext::fetch(chain, coord);
        apply_delta(&scroll, &mut state);
    }
    state
}

fn apply_delta(scroll: &str, state: &mut FoldState) {
    for line in scroll.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("commit ") {
            state.head_sha = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("date ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                state.head_date = parts[1].to_string();
            }
            state.total_commits += 1;
        } else if let Some(rest) = line.strip_prefix("item+ ") {
            // kind file :: name :: signature :: explanation :: source
            let parts: Vec<&str> = rest.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let kind = if parts[0] == "entry-point" { "entry point" } else { parts[0] };
                let fields: Vec<&str> = parts[1].split(" :: ").collect();
                if fields.len() >= 5 {
                    let item = PubItem {
                        kind: kind.to_string(),
                        file: fields[0].to_string(),
                        name: fields[1].to_string(),
                        signature: fields[2].to_string(),
                        doc: String::new(),
                        extra: String::new(),
                        line: 0,
                    };
                    let key = format!("{} :: {}", item.file, item.name);
                    state.items.insert(key.clone(), item);
                    state.item_source.insert(key, fields[4].to_string());
                    // explanation is fields[3]; kept via a pseudo doc for rendering
                    if let Some(it) = state.items.get_mut(&format!("{} :: {}", fields[0], fields[1])) {
                        it.doc = fields[3].to_string();
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("item~ ") {
            apply_delta(&line.replacen("item~ ", "item+ ", 1), state);
            let _ = rest;
        } else if let Some(rest) = line.strip_prefix("item- ") {
            let fields: Vec<&str> = rest.splitn(2, ' ').collect();
            if parts_ok(&fields) {
                let f: Vec<&str> = fields[1].split(" :: ").collect();
                if f.len() >= 2 {
                    state.items.remove(&format!("{} :: {}", f[0], f[1]));
                }
            }
        } else if let Some(rest) = line.strip_prefix("dep+ ") {
            let fields: Vec<&str> = rest.split(" :: ").collect();
            if fields.len() >= 3 {
                state.deps.entry(fields[0].to_string()).or_default().push((fields[1].to_string(), fields[2].to_string()));
            }
        } else if let Some(rest) = line.strip_prefix("dep~ ") {
            let fields: Vec<&str> = rest.split(" :: ").collect();
            if fields.len() >= 3 {
                if let Some(list) = state.deps.get_mut(fields[0]) {
                    let newv = fields[2].split(" -> ").nth(1).unwrap_or("").to_string();
                    for e in list.iter_mut() {
                        if e.0 == fields[1] {
                            e.1 = newv.clone();
                        }
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("dep- ") {
            let fields: Vec<&str> = rest.split(" :: ").collect();
            if fields.len() >= 2 {
                if let Some(list) = state.deps.get_mut(fields[0]) {
                    list.retain(|e| e.0 != fields[1]);
                }
            }
        } else if let Some(rest) = line.strip_prefix("stat ") {
            // stat <lang...> files N total N blank N comment N code N
            let toks: Vec<&str> = rest.split_whitespace().collect();
            if toks.len() >= 11 {
                let lang = toks[..toks.len() - 10].join(" ");
                let nums: Vec<usize> = toks[toks.len() - 10..]
                    .iter()
                    .filter_map(|t| t.parse().ok())
                    .collect();
                if nums.len() == 5 {
                    state.lang.insert(lang, (nums[0], nums[1], nums[2], nums[3], nums[4]));
                }
            }
        } else if let Some(rest) = line.strip_prefix("author+ ") {
            state.authors.insert(rest.trim().to_string(), 1);
        } else if let Some(rest) = line.strip_prefix("author~ ") {
            if let Some(pos) = rest.rfind(" commits ") {
                let who = rest[..pos].to_string();
                let n: usize = rest[pos + 9..].parse().unwrap_or(0);
                state.authors.insert(who, n);
            }
        } else if let Some(rest) = line.strip_prefix("rfcs ") {
            state.rfcs_count = if rest.trim() == "none" { None } else { rest.trim().parse().ok() };
        }
    }
}

fn parts_ok(fields: &[&str]) -> bool {
    fields.len() == 2
}

/// The provenance section for the snapshot renders (docs.md / docs.phext /
/// latest.md): it names the export repo and the doc site this documentation
/// was produced for, from the project's configuration in its .phdoc.json.
fn provenance_section(name: &str, export_repo: &str, doc_site: &str) -> (String, String) {
    (
        "Provenance".into(),
        format!(
            "## Provenance\n\nThis documentation is produced for the export repo {} (its chain of record lives at docs/{}/chain.phext) and for the doc site {}. Both targets are the project's own configuration, held in its .phdoc.json as \"export-repo\" and \"doc-site\"; when a project does not name them, the defaults are {} and {}.\n",
            export_repo, name, doc_site, DEFAULT_EXPORT_REPO, DEFAULT_DOC_SITE
        ),
    )
}

/// Render the folded reference (what the doc site shows per universe).
/// The provenance block names the targets the documentation was produced
/// for: the export repo that keeps the chain, and the doc site it is
/// rendered for — both from the project's own configuration.
fn render_folded(name: &str, state: &FoldState, at_label: &str, export_repo: &str, doc_site: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {} — reference as of {}\n\n", name, at_label));
    out.push_str(&format!("Commit {}. {} commit{} folded. Every fact here was folded from the chain's per-commit deltas.\n\n", state.head_sha, state.total_commits, if state.total_commits == 1 { "" } else { "s" }));
    out.push_str("## Provenance\n\n");
    out.push_str(&format!(
        "This reference is folded from the project's documentation chain, kept in the export repo {} at docs/{}/chain.phext, and rendered for the doc site {}. Both targets are named in the project's own .phdoc.json; the defaults are {} and {}.\n\n",
        export_repo, name, doc_site, DEFAULT_EXPORT_REPO, DEFAULT_DOC_SITE
    ));
    out.push_str("## Dependencies\n\n");
    if state.deps.is_empty() {
        out.push_str("No dependencies recorded at this point.\n\n");
    }
    for (manifest, list) in &state.deps {
        out.push_str(&format!("### {}\n\n", manifest));
        for (n, v) in list {
            out.push_str(&format!("- {} — {}\n", n, v));
        }
        out.push('\n');
    }
    out.push_str("## Authors\n\n");
    let mut authors: Vec<(&String, &usize)> = state.authors.iter().collect();
    authors.sort_by(|a, b| b.1.cmp(a.1));
    for (who, n) in authors {
        out.push_str(&format!("- {} — {} commits\n", who, n));
    }
    out.push_str("\n## Complexity\n\n");
    for (lang, (f, t, b, c, code)) in &state.lang {
        out.push_str(&format!("- {}: {} files, {} lines total, {} blank, {} comment, {} code\n", lang, f, t, b, c, code));
    }
    out.push_str(&format!(
        "\nDeclared RFCs at this point: {}.\n\n",
        state.rfcs_count.map(|c| c.to_string()).unwrap_or_else(|| "none declared".into())
    ));
    out.push_str("## Public methods and commands\n\n");
    for (key, item) in &state.items {
        let _ = key;
        out.push_str(&format!(
            "- {} `{}` ({}) — {}\n",
            item.kind,
            item.name,
            item.file,
            first_sentence(&item.doc)
        ));
    }
    out
}

// ---------------------------------------------------------------------------
// .phdoc.json — the target project's own ledger
// ---------------------------------------------------------------------------

#[derive(Default)]
struct LedgerEntry {
    audited: Option<String>, // stamp text
    failed: Option<String>,  // reason
    reviewed: Option<String>, // stamp text
}

/// The default export repo: where doc changes are exported when the
/// project's .phdoc.json does not name one (Will's own setup).
const DEFAULT_EXPORT_REPO: &str = "wbic16/phdoc";
/// The default doc site: the base URL the documentation is rendered for
/// when the project's .phdoc.json does not name one.
const DEFAULT_DOC_SITE: &str = "https://docs.phext.io";

struct Ledger {
    chain_location: String,
    project: String,
    export_repo: String, // "" = not configured; the default applies
    doc_site: String,    // "" = not configured; the default applies
    export_repo_set: bool, // the field was present (non-empty) in .phdoc.json
    doc_site_set: bool,
    entries: BTreeMap<String, LedgerEntry>,
    order: Vec<String>,
}

/// The export repo in effect: the project's configured value, or the default.
fn eff_export_repo(l: &Ledger) -> &str {
    if l.export_repo.is_empty() {
        DEFAULT_EXPORT_REPO
    } else {
        &l.export_repo
    }
}

/// The doc site in effect: the project's configured value, or the default.
fn eff_doc_site(l: &Ledger) -> &str {
    if l.doc_site.is_empty() {
        DEFAULT_DOC_SITE
    } else {
        &l.doc_site
    }
}

fn ledger_path(root: &Path) -> PathBuf {
    root.join(".phdoc.json")
}

fn load_ledger(root: &Path) -> Ledger {
    let path = ledger_path(root);
    let mut ledger = Ledger {
        chain_location: String::new(),
        project: String::new(),
        export_repo: String::new(),
        doc_site: String::new(),
        export_repo_set: false,
        doc_site_set: false,
        entries: BTreeMap::new(),
        order: Vec::new(),
    };
    if !path.exists() {
        return ledger;
    }
    let text = read(&path);
    let mut current: Option<String> = None;
    for line in text.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("\"chain\":") {
            ledger.chain_location = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
        }
        if let Some(rest) = t.strip_prefix("\"project\":") {
            ledger.project = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
        }
        if let Some(rest) = t.strip_prefix("\"export-repo\":") {
            let v = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
            if !v.is_empty() {
                ledger.export_repo = v;
                ledger.export_repo_set = true;
            }
        }
        if let Some(rest) = t.strip_prefix("\"doc-site\":") {
            let v = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
            if !v.is_empty() {
                ledger.doc_site = v;
                ledger.doc_site_set = true;
            }
        }
        // entry start: "<40 hex>": {
        let key = t.trim_end_matches(": {").trim_matches('"');
        if key.len() == 40 && key.chars().all(|c| c.is_ascii_hexdigit()) && t.ends_with(": {") {
            current = Some(key.to_string());
            ledger.order.push(key.to_string());
            ledger.entries.insert(key.to_string(), LedgerEntry::default());
            continue;
        }
        if let Some(sha) = &current {
            if t == "}" || t == "}," {
                current = None;
                continue;
            }
            let entry = ledger.entries.get_mut(sha).expect("entry");
            if let Some(rest) = t.strip_prefix("\"audited\":") {
                let v = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
                if v != "null" {
                    entry.audited = Some(v);
                }
            }
            if let Some(rest) = t.strip_prefix("\"audit-failed\":") {
                let v = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
                if v != "null" {
                    entry.failed = Some(v);
                }
            }
            if let Some(rest) = t.strip_prefix("\"reviewed\":") {
                let v = rest.trim().trim_matches(|c| c == '"' || c == ',').to_string();
                if v != "null" {
                    entry.reviewed = Some(v);
                }
            }
        }
    }
    ledger
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn save_ledger(root: &Path, ledger: &Ledger) {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"project\": \"{}\",\n", json_escape(&ledger.project)));
    out.push_str(&format!("  \"chain\": \"{}\",\n", json_escape(&ledger.chain_location)));
    if !ledger.export_repo.is_empty() {
        out.push_str(&format!("  \"export-repo\": \"{}\",\n", json_escape(&ledger.export_repo)));
    }
    if !ledger.doc_site.is_empty() {
        out.push_str(&format!("  \"doc-site\": \"{}\",\n", json_escape(&ledger.doc_site)));
    }
    out.push_str("  \"commits\": {\n");
    let n = ledger.order.len();
    for (i, sha) in ledger.order.iter().enumerate() {
        let e = &ledger.entries[sha];
        out.push_str(&format!("    \"{}\": {{\n", sha));
        match (&e.audited, &e.failed) {
            (Some(a), _) => out.push_str(&format!("      \"audited\": \"{}\",\n", json_escape(a))),
            (None, Some(f)) => out.push_str(&format!("      \"audit-failed\": \"{}\",\n", json_escape(f))),
            (None, None) => out.push_str("      \"audited\": null,\n"),
        }
        match &e.reviewed {
            Some(r) => out.push_str(&format!("      \"reviewed\": \"{}\"\n", json_escape(r))),
            None => out.push_str("      \"reviewed\": null\n"),
        }
        out.push_str(if i + 1 < n { "    },\n" } else { "    }\n" });
    }
    out.push_str("  }\n}\n");
    fs::write(ledger_path(root), out).expect("write .phdoc.json");
}

fn stamp() -> String {
    let now = now_unix();
    format!("{} EE {} by phdoc {}", date_string(now), epoch_of(now), env!("CARGO_PKG_VERSION"))
}

// ---------------------------------------------------------------------------
// audit / review / render / export
// ---------------------------------------------------------------------------

fn chain_coord(u: usize) -> phext::Coordinate {
    phext::to_coordinate(&format!("{}.1.1/1.1.1/1.1.1", u))
}

fn chain_universe_count(chain: &str) -> usize {
    // probe by fetch: universes are written in order from 1, never empty
    let mut n = 0;
    while n < 100_000 {
        let scroll = phext::fetch(chain, chain_coord(n + 1));
        if scroll.is_empty() {
            break;
        }
        n += 1;
    }
    n
}

fn chain_commits(chain: &str) -> Vec<String> {
    let n = chain_universe_count(chain);
    let mut out = Vec::new();
    for u in 1..=n {
        let scroll = phext::fetch(chain, chain_coord(u));
        for line in scroll.lines() {
            if let Some(rest) = line.trim().strip_prefix("commit ") {
                out.push(rest.trim().to_string());
                break;
            }
        }
    }
    out
}

/// The clone source for an export-repo setting: a local path, a full URL,
/// or an owner/name pair (read as a GitHub repo over SSH).
fn clone_source(spec: &str) -> String {
    if spec.starts_with('/') || spec.starts_with('.') || spec.starts_with('~') || spec.contains("://") || spec.starts_with("git@") {
        spec.to_string()
    } else if spec.matches('/').count() == 1 {
        format!("git@github.com:{}.git", spec)
    } else {
        spec.to_string()
    }
}

/// Ensure phdoc has a working clone of the configured export repo at dir:
/// clone it if absent, and give the clone a commit identity if it has none
/// (an export commit must have an author). Returns the dir on success.
fn ensure_managed_clone(spec: &str, dir: &Path) -> Result<PathBuf, String> {
    if dir.join(".git").exists() {
        return Ok(dir.to_path_buf());
    }
    if let Some(parent) = dir.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let src = clone_source(spec);
    let o = Command::new("git").args(["clone", &src]).arg(dir).output().map_err(|e| e.to_string())?;
    if !o.status.success() {
        return Err(format!("git clone {} failed: {}", src, String::from_utf8_lossy(&o.stderr).trim()));
    }
    if git_str(dir, &["config", "user.email"]).is_empty() {
        let _ = git_str(dir, &["config", "user.name", "phdoc"]);
        let _ = git_str(dir, &["config", "user.email", "phdoc@localhost"]);
    }
    Ok(dir.to_path_buf())
}

struct AuditArgs {
    project: String,
    repo: Option<PathBuf>,
    out: Option<PathBuf>,
    name: String,
    hold_push: bool,
}

fn cmd_audit(a: AuditArgs) -> i32 {
    let root = PathBuf::from(&a.project);
    if !root.join(".git").exists() {
        eprintln!("phdoc audit: {} is not a git repo; there is no history to audit.", a.project);
        return 2;
    }
    let scan = scan_project(&root, &a.name);
    let name = scan.name.clone();

    // the project's own ledger carries its target configuration: which repo
    // doc changes export to, and which doc site the documentation is for
    let mut ledger = load_ledger(&root);
    let export_repo_cfg = eff_export_repo(&ledger).to_string();
    let doc_site_cfg = eff_doc_site(&ledger).to_string();

    // where the chain lives: the --repo checkout if given; else, when the
    // project configures an export repo, a managed clone of that repo;
    // else the plain out dir and the export is held
    let local_dir: PathBuf = a.out.clone().unwrap_or_else(|| PathBuf::from(format!("phdoc-out/{}", name)));
    let export_dir: Option<PathBuf> = match &a.repo {
        Some(r) => Some(r.clone()),
        None if ledger.export_repo_set => {
            let dir = local_dir.join("export-checkout");
            match ensure_managed_clone(&export_repo_cfg, &dir) {
                Ok(d) => Some(d),
                Err(e) => {
                    eprintln!("phdoc audit: the project configures export repo {} but phdoc could not clone it: {}. Nothing was exported.", export_repo_cfg, e);
                    return 1;
                }
            }
        }
        None => None,
    };
    let chain_home = export_dir.clone().unwrap_or_else(|| local_dir.clone());
    let proj_dir = chain_home.join("docs").join(&name);
    let chain_path = proj_dir.join("chain.phext");
    let mut chain = if chain_path.exists() { read(&chain_path) } else { String::new() };

    // repo hygiene before writing: fetch + rebase onto the fresh remote head
    // of the checkout's own branch (exo for wbic16/phdoc; a configured repo
    // may name another). A remote with no commits yet has nothing to rebase.
    let mut export_branch = "exo".to_string();
    if let Some(repo) = &export_dir {
        let dirty = git_str(repo, &["status", "--porcelain"]);
        if !dirty.is_empty() {
            eprintln!("phdoc audit: the phdoc repo at {} has uncommitted changes that are not phdoc's; export aborted, nothing was written there.", repo.display());
            return 1;
        }
        let _ = git_str(repo, &["fetch", "origin"]);
        let b = git_str(repo, &["symbolic-ref", "--short", "HEAD"]);
        if !b.is_empty() {
            export_branch = b;
        }
        let remote_ref = format!("origin/{}", export_branch);
        if !git_str(repo, &["rev-parse", "--verify", &remote_ref]).is_empty() {
            let rb = Command::new("git").arg("-C").arg(repo).args(["rebase", &remote_ref]).output();
            if let Ok(o) = rb {
                if !o.status.success() {
                    eprintln!("phdoc audit: rebase onto {} failed in {}; export aborted.", remote_ref, repo.display());
                    return 1;
                }
            }
        }
    }

    // the commits to audit: ledger + chain agree; anything in history not in the chain is new
    let history = git_str(&root, &["rev-list", "--reverse", "HEAD"]);
    let commits: Vec<String> = history.lines().map(|s| s.to_string()).collect();
    let already = chain_commits(&chain);
    ledger.project = name.clone();
    ledger.chain_location = if export_dir.is_some() {
        format!("{}:docs/{}/chain.phext", export_repo_cfg, name)
    } else {
        chain_path.to_string_lossy().to_string()
    };
    for sha in &commits {
        if !ledger.order.contains(sha) {
            ledger.order.push(sha.clone());
            ledger.entries.insert(sha.clone(), LedgerEntry::default());
        }
    }

    let mut state = fold_chain(&chain, already.len());
    let mut new_universes = 0usize;
    let total = commits.len();
    for (idx, sha) in commits.iter().enumerate() {
        if already.contains(sha) {
            continue;
        }
        let parent = if idx == 0 {
            None
        } else {
            Some(commits[idx - 1].as_str())
        };
        let checkpoint = idx == 0 || idx + 1 == total || (idx + 1) % 25 == 0;
        // an audit can fail: git refusing to show the commit is recorded, never skipped silently
        if git_bytes(&root, &["cat-file", "-e", &format!("{}^{{commit}}", sha)]).is_none() {
            let entry = ledger.entries.get_mut(sha).expect("entry");
            entry.failed = Some("git cat-file could not read the commit object".to_string());
            let u = already.len() + new_universes + 1;
            let scroll = format!(
                "Commit {} could not be audited.\n\ncommit {}\nparent {}\naudit-failed git cat-file could not read the commit object\n",
                &sha[..sha.len().min(12)],
                sha,
                parent.unwrap_or("none")
            );
            chain = phext::insert(chain, chain_coord(u), &scroll);
            new_universes += 1;
            continue;
        }
        let scroll = audit_commit(&root, sha, parent, checkpoint, &mut state);
        let u = already.len() + new_universes + 1;
        chain = phext::insert(chain, chain_coord(u), &scroll);
        new_universes += 1;
        let entry = ledger.entries.get_mut(sha).expect("entry");
        entry.audited = Some(stamp());
        entry.failed = None;
    }

    // render: latest (full working-tree scan) + one history page per universe
    let (mut sections, awaiting) = sections(&scan, &BTreeMap::new());
    sections.push(provenance_section(&name, &export_repo_cfg, &doc_site_cfg));
    let latest_md = render_markdown(&sections);
    let hist_dir = proj_dir.join("history");
    if let Err(e) = fs::create_dir_all(&hist_dir) {
        eprintln!("phdoc audit: cannot create {}: {}", hist_dir.display(), e);
        return 1;
    }
    let all = chain_commits(&chain);
    for (i, sha) in all.iter().enumerate() {
        let st = fold_chain(&chain, i + 1);
        let page = render_folded(&name, &st, &format!("commit {}", &sha[..sha.len().min(12)]), &export_repo_cfg, &doc_site_cfg);
        let _ = fs::write(hist_dir.join(format!("{}.md", &sha[..sha.len().min(12)])), page);
    }
    let _ = fs::write(proj_dir.join("latest.md"), &latest_md);
    let _ = fs::write(&chain_path, &chain);
    // the snapshot doors stay available beside the chain when no repo is used
    if export_dir.is_none() {
        let _ = fs::write(local_dir.join("docs.md"), &latest_md);
        let _ = fs::write(local_dir.join("docs.phext"), render_phext(&sections));
    }
    if !awaiting.is_empty() {
        let instr = instruction_file(&scan, &awaiting);
        let _ = fs::write(proj_dir.join("SUMMARY-INSTRUCTIONS.md"), &instr);
    }

    save_ledger(&root, &ledger);

    // the unreviewed backlog, reported on every run
    let unreviewed: Vec<&String> = ledger
        .order
        .iter()
        .filter(|sha| ledger.entries[*sha].audited.is_some() && ledger.entries[*sha].reviewed.is_none())
        .collect();
    let failed_count = ledger.entries.values().filter(|e| e.failed.is_some()).count();

    println!(
        "phdoc audit: {} — {} commits in history, {} newly audited, {} universes in the chain (checksum {}).",
        name,
        total,
        new_universes,
        chain_universe_count(&chain),
        phext::checksum(&chain)
    );
    if failed_count > 0 {
        println!("phdoc audit: {} commits are marked audit-failed in .phdoc.json, each with its reason.", failed_count);
    }
    if unreviewed.is_empty() {
        println!("phdoc audit: unreviewed backlog: 0.");
    } else {
        println!(
            "phdoc audit: unreviewed backlog: {} (oldest: {}). Mark one with: phdoc review {} <commit> --by <name>.",
            unreviewed.len(),
            &unreviewed[0][..unreviewed[0].len().min(12)],
            a.project
        );
    }
    if !awaiting.is_empty() {
        println!(
            "phdoc audit: {} public items are awaiting a summary; see {}.",
            awaiting.len(),
            proj_dir.join("SUMMARY-INSTRUCTIONS.md").display()
        );
    }

    // export: commit + push the phdoc repo, or report the held state
    match &export_dir {
        None => {
            println!("phdoc audit: export held — no phdoc repo was given, so the chain and renders stay in {}. Nothing was dropped; pass --repo <phdoc-repo-dir>, or name an \"export-repo\" in the project's .phdoc.json, to export.", proj_dir.display());
        }
        Some(repo) => {
            let status = git_str(repo, &["status", "--porcelain"]);
            let mut exported = false;
            if !status.is_empty() {
                let _ = git_str(repo, &["add", "docs"]);
                let msg = format!(
                    "phdoc export: {} @ {} — audited to EE {} (phdoc {})",
                    name,
                    &scan.head[..scan.head.len().min(12)],
                    scan.ee,
                    env!("CARGO_PKG_VERSION")
                );
                let c = Command::new("git").arg("-C").arg(repo).args(["commit", "-m", &msg]).output();
                match c {
                    Ok(o) if o.status.success() => exported = true,
                    Ok(o) => eprintln!("phdoc audit: export commit failed: {}", String::from_utf8_lossy(&o.stderr)),
                    Err(e) => eprintln!("phdoc audit: export commit failed: {}", e),
                }
            }
            // push any unpushed commits (this run's or an earlier held run's);
            // a remote with no commits on the branch yet counts every commit
            let remote_ref = format!("origin/{}", export_branch);
            let ahead = if git_str(repo, &["rev-parse", "--verify", &remote_ref]).is_empty() {
                git_str(repo, &["rev-list", "--count", "HEAD"])
            } else {
                git_str(repo, &["rev-list", "--count", &format!("{}..HEAD", remote_ref)])
            };
            if ahead != "0" && !ahead.is_empty() {
                if a.hold_push {
                    println!("phdoc audit: export committed locally ({} commit(s) ahead); push held by --hold-push.", ahead);
                } else {
                    let p = Command::new("git").arg("-C").arg(repo).args(["push", "origin", &export_branch]).output();
                    match p {
                        Ok(o) if o.status.success() => {
                            let remote = git_str(repo, &["ls-remote", "origin", &export_branch]);
                            let remote_sha = remote.split_whitespace().next().unwrap_or("");
                            let local_sha = git_str(repo, &["rev-parse", "HEAD"]);
                            if remote_sha == local_sha {
                                println!("phdoc audit: exported to {}@{} and verified — remote head {} matches the local head.", export_repo_cfg, export_branch, &local_sha[..local_sha.len().min(12)]);
                            } else {
                                eprintln!("phdoc audit: push reported success but the remote head does not match the local head; treat the export as unverified.");
                                return 1;
                            }
                        }
                        Ok(o) => {
                            eprintln!("phdoc audit: export push failed: {}", String::from_utf8_lossy(&o.stderr));
                            return 1;
                        }
                        Err(e) => {
                            eprintln!("phdoc audit: export push failed: {}", e);
                            return 1;
                        }
                    }
                }
            } else if exported {
                println!("phdoc audit: export committed and already on the remote.");
            } else {
                println!("phdoc audit: no changes to export — the chain and renders on the remote are already current.");
            }
        }
    }
    0
}

fn cmd_review(project: &str, sha_prefix: &str, by: &str) -> i32 {
    let root = PathBuf::from(project);
    let mut ledger = load_ledger(&root);
    let full = ledger.order.iter().find(|s| s.starts_with(sha_prefix)).cloned();
    match full {
        Some(sha) => {
            let entry = ledger.entries.get_mut(&sha).expect("entry");
            entry.reviewed = Some(format!("{} by {}", stamp(), by));
            save_ledger(&root, &ledger);
            println!("phdoc review: {} marked reviewed by {} in {}.", &sha[..sha.len().min(12)], by, ledger_path(&root).display());
            0
        }
        None => {
            eprintln!("phdoc review: no commit starting with {} is in the ledger at {}. Run phdoc audit first.", sha_prefix, ledger_path(&root).display());
            2
        }
    }
}

fn cmd_render(project: &str, at: &str, repo: &Option<PathBuf>, out: &Option<PathBuf>) -> i32 {
    let root = PathBuf::from(project);
    let ledger = load_ledger(&root);
    let name = if ledger.project.is_empty() {
        root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
    } else {
        ledger.project.clone()
    };
    let chain_path = match repo {
        Some(r) => r.join("docs").join(&name).join("chain.phext"),
        None => PathBuf::from(format!("phdoc-out/{}/chain.phext", name)),
    };
    if !chain_path.exists() {
        eprintln!("phdoc render: no chain at {}. Run phdoc audit first.", chain_path.display());
        return 2;
    }
    let chain = read(&chain_path);
    let commits = chain_commits(&chain);
    let upto = if at == "latest" {
        commits.len()
    } else {
        match commits.iter().position(|s| s.starts_with(at)) {
            Some(i) => i + 1,
            None => {
                eprintln!("phdoc render: commit {} is not in the chain.", at);
                return 2;
            }
        }
    };
    let state = fold_chain(&chain, upto);
    let page = render_folded(&name, &state, &format!("commit {}", &commits[upto - 1][..commits[upto - 1].len().min(12)]), eff_export_repo(&ledger), eff_doc_site(&ledger));
    match out {
        Some(p) => {
            let _ = fs::write(p, &page);
            println!("phdoc render: wrote {} ({} bytes).", p.display(), page.len());
        }
        None => print!("{}", page),
    }
    0
}

// ---------------------------------------------------------------------------
// tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_anchor() {
        assert_eq!(epoch_of(1_795_327_199), 0);
        assert_eq!(epoch_of(1_795_327_200), 1);
        assert_eq!(epoch_of(1_795_327_200 + 420 * 2537), 2538);
    }

    #[test]
    fn date_of_anchor() {
        assert_eq!(date_string(1_795_327_200), "2026-11-22");
    }

    #[test]
    fn cargo_deps_parse() {
        let text = "[package]\nname = \"x\"\nlicense = \"MIT\"\n\n[dependencies]\nlibphext = \"0.6.6\"\nserde = { version = \"1.0\", features = [\"derive\"] }\nlocal = { path = \"../local\" }\n";
        let (deps, lic, pkg) = parse_cargo_toml(text);
        assert_eq!(lic.as_deref(), Some("MIT"));
        assert_eq!(pkg.as_deref(), Some("x"));
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].1, "0.6.6");
        assert_eq!(deps[1].1, "1.0");
    }

    #[test]
    fn rfcs_json_parse() {
        let text = r#"{"rfcs":[{"rfc":8446,"sha256":"abc","source":"rfc-editor","status":"current"},{"rfc":"9000","superseded_by":"9001"}]}"#;
        let entries = parse_rfcs_json(text);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].number, "8446");
        assert_eq!(entries[0].sha, "abc");
        assert_eq!(entries[1].status, "superseded by RFC 9001");
    }

    #[test]
    fn explain_from_signature() {
        let item = PubItem {
            kind: "function".into(),
            name: "max_weight".into(),
            signature: "pub fn max_weight(ty: &str) -> u8".into(),
            doc: String::new(),
            extra: String::new(),
            file: "src/schema.rs".into(),
            line: 59,
        };
        let (text, source) = explain(&item, &BTreeMap::new());
        assert_eq!(source, "signature");
        assert!(text.contains("max_weight"));
        assert!(text.contains("u8"));
    }

    #[test]
    fn explain_prefers_doc_comment() {
        let item = PubItem {
            kind: "function".into(),
            name: "build".into(),
            signature: "pub fn build() -> Graph".into(),
            doc: "Builds the graph from the homes.".into(),
            extra: String::new(),
            file: "src/builder.rs".into(),
            line: 68,
        };
        let (text, source) = explain(&item, &BTreeMap::new());
        assert_eq!(source, "doc comment");
        assert_eq!(text, "Builds the graph from the homes.");
    }

    #[test]
    fn phext_roundtrip() {
        let sections = vec![
            ("Home".to_string(), "# Home\n".to_string()),
            ("Dependencies".to_string(), "## Dependencies\n".to_string()),
        ];
        let px = render_phext(&sections);
        assert_eq!(phext::fetch(&px, phext::to_coordinate("1.1.1/1.1.1/1.1.1")), "# Home\n");
        assert_eq!(phext::fetch(&px, phext::to_coordinate("1.1.1/1.1.1/1.1.2")), "## Dependencies\n");
        assert!(!phext::checksum(&px).is_empty());
    }

    #[test]
    fn fold_applies_deltas() {
        let mut chain = String::new();
        let d1 = "Commit abc by A <a@x> on 2026-11-22 (EE 1).\n\ncommit aaaa\ndate 1795327200 2026-11-22\nitem+ function src/lib.rs :: hello :: pub fn hello() :: Says hello. :: doc comment\nauthor+ A <a@x>\nstat Rust files 1 total 10 blank 1 comment 1 code 8\n";
        chain = phext::insert(chain, chain_coord(1), d1);
        let d2 = "Commit def by A <a@x> on 2026-11-22 (EE 2).\n\ncommit bbbb\ndate 1795327620 2026-11-22\nitem- function src/lib.rs :: hello\nauthor~ A <a@x> commits 2\nstat Rust files 1 total 4 blank 0 comment 0 code 4\n";
        chain = phext::insert(chain, chain_coord(2), d2);
        let st1 = fold_chain(&chain, 1);
        assert_eq!(st1.items.len(), 1);
        assert_eq!(st1.lang["Rust"].4, 8);
        let st2 = fold_chain(&chain, 2);
        assert_eq!(st2.items.len(), 0);
        assert_eq!(st2.authors["A <a@x>"], 2);
        assert_eq!(st2.total_commits, 2);
        assert_eq!(chain_commits(&chain), vec!["aaaa".to_string(), "bbbb".to_string()]);
    }

    #[test]
    fn ledger_roundtrip() {
        let dir = std::env::temp_dir().join("phdoc-ledger-test");
        let _ = fs::create_dir_all(&dir);
        let sha = "a".repeat(40);
        let mut ledger = Ledger {
            chain_location: "wbic16/phdoc:docs/x/chain.phext".into(),
            project: "x".into(),
            export_repo: "/tmp/example-docs.git".into(),
            doc_site: "https://docs.example.test".into(),
            export_repo_set: true,
            doc_site_set: true,
            entries: BTreeMap::new(),
            order: vec![sha.clone()],
        };
        ledger.entries.insert(sha.clone(), LedgerEntry { audited: Some("2026-11-22 EE 77 by phdoc 0.1.0".into()), failed: None, reviewed: None });
        save_ledger(&dir, &ledger);
        let back = load_ledger(&dir);
        assert_eq!(back.project, "x");
        assert_eq!(back.export_repo, "/tmp/example-docs.git");
        assert_eq!(back.doc_site, "https://docs.example.test");
        assert!(back.export_repo_set && back.doc_site_set);
        assert_eq!(eff_export_repo(&back), "/tmp/example-docs.git");
        assert_eq!(eff_doc_site(&back), "https://docs.example.test");
        assert_eq!(back.entries[&sha].audited.as_deref(), Some("2026-11-22 EE 77 by phdoc 0.1.0"));
        assert!(back.entries[&sha].reviewed.is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn config_defaults_when_fields_absent() {
        // no .phdoc.json at all: the defaults apply and nothing is "set"
        let dir = std::env::temp_dir().join("phdoc-config-absent-test");
        let _ = fs::create_dir_all(&dir);
        let _ = fs::remove_file(ledger_path(&dir));
        let l = load_ledger(&dir);
        assert!(!l.export_repo_set && !l.doc_site_set);
        assert_eq!(eff_export_repo(&l), "wbic16/phdoc");
        assert_eq!(eff_doc_site(&l), "https://docs.phext.io");
        // a ledger file without the two fields reads the same way
        fs::write(ledger_path(&dir), "{\n  \"project\": \"y\",\n  \"chain\": \"somewhere\",\n  \"commits\": {\n  }\n}\n").expect("write ledger");
        let l2 = load_ledger(&dir);
        assert!(!l2.export_repo_set && !l2.doc_site_set);
        assert_eq!(eff_export_repo(&l2), DEFAULT_EXPORT_REPO);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn audit_exports_to_configured_bare_repo() {
        // A project that names its own export repo (a local bare repo) and
        // its own doc site in .phdoc.json is documented end to end from
        // configuration alone: no --repo, no code changes. The export must
        // land in the bare repo, and the rendered pages must name the
        // configured targets in their provenance.
        let base = std::env::temp_dir().join(format!("phdoc-config-export-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let proj = base.join("proj");
        let bare = base.join("bare.git");
        let out = base.join("out");
        fs::create_dir_all(proj.join("src")).expect("fixture dirs");
        let git = |dir: &Path, args: &[&str]| {
            let o = Command::new("git")
                .arg("-C")
                .arg(dir)
                .args(args)
                .env("GIT_AUTHOR_NAME", "Test")
                .env("GIT_AUTHOR_EMAIL", "test@localhost")
                .env("GIT_COMMITTER_NAME", "Test")
                .env("GIT_COMMITTER_EMAIL", "test@localhost")
                .output()
                .expect("git runs");
            assert!(o.status.success(), "git {:?} failed: {}", args, String::from_utf8_lossy(&o.stderr));
        };
        git(&base, &["init", "proj"]);
        git(&base, &["init", "--bare", "bare.git"]);
        fs::write(proj.join("Cargo.toml"), "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nlicense = \"MIT\"\n\n[dependencies]\n").expect("write manifest");
        fs::write(proj.join("src/lib.rs"), "/// Says hello.\npub fn hello() -> &'static str { \"hello\" }\n").expect("write source");
        git(&proj, &["add", "."]);
        git(&proj, &["commit", "-m", "first"]);
        fs::write(proj.join("src/lib.rs"), "/// Says hello.\npub fn hello() -> &'static str { \"hello\" }\n\n/// Says bye.\npub fn bye() -> &'static str { \"bye\" }\n").expect("write source 2");
        git(&proj, &["add", "."]);
        git(&proj, &["commit", "-m", "second"]);
        // the project's own target configuration
        let bare_str = bare.to_string_lossy().to_string();
        fs::write(
            ledger_path(&proj),
            format!(
                "{{\n  \"project\": \"fixture\",\n  \"export-repo\": \"{}\",\n  \"doc-site\": \"https://docs.example.test\",\n  \"commits\": {{\n  }}\n}}\n",
                bare_str
            ),
        )
        .expect("write .phdoc.json");

        let rc = cmd_audit(AuditArgs { project: proj.to_string_lossy().to_string(), repo: None, out: Some(out.clone()), name: "fixture".into(), hold_push: false });
        assert_eq!(rc, 0);

        // the export landed in the configured bare repo
        let branch = git_str(&bare, &["symbolic-ref", "--short", "HEAD"]);
        assert!(!branch.is_empty());
        let tree = git_str(&bare, &["ls-tree", "-r", "--name-only", &branch]);
        assert!(tree.contains("docs/fixture/chain.phext"), "chain in bare repo tree: {}", tree);
        assert!(tree.contains("docs/fixture/latest.md"), "latest in bare repo tree: {}", tree);
        let hist = tree.lines().find(|l| l.contains("docs/fixture/history/")).expect("a history page").to_string();
        let page = String::from_utf8(git_bytes(&bare, &["show", &format!("{}:{}", branch, hist)]).expect("show page")).expect("page is utf8");
        assert!(page.contains("https://docs.example.test"), "provenance names the configured doc site");
        assert!(page.contains(&bare_str), "provenance names the configured export repo");
        // the project's ledger kept its configuration and names the chain's home
        let led = read(&ledger_path(&proj));
        assert!(led.contains(&bare_str), "ledger keeps the export-repo setting");
        assert!(led.contains("https://docs.example.test"), "ledger keeps the doc-site setting");
        assert!(led.contains(&format!("\"chain\": \"{}:docs/fixture/chain.phext\"", bare_str)), "chain location names the configured repo: {}", led);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn rust_items_extract() {
        let src = "/// Says hello.\npub fn hello(name: &str) -> String { name.to_string() }\npub struct Point {\n    pub x: i64,\n    pub y: i64,\n}\n";
        let mut items = Vec::new();
        extract_rust_items(src, "src/lib.rs", &mut items);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "hello");
        assert_eq!(items[0].doc, "Says hello.");
        assert_eq!(items[1].kind, "struct");
        assert!(items[1].extra.contains("pub x: i64"));
    }
}
