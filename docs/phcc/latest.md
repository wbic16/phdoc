# phcc — project documentation

Scanned by phdoc 0.1.0 on 2026-11-22 (Exocortical Epoch 148). Scanned at commit 0628c00360a22aafcaf4d0bf26a4991f9d29b900. Everything in this document was extracted from the revision named above: its manifests, its git log, and its source text. Descriptions are labeled by how they were obtained — direct (the source's own words), inferred (read off a signature), agent-supplied (written by a reviewer), verified (backed by a validated contract), or unverified — and only validated contracts are marked verified.

This document holds: what the project depends on, who wrote it, how large and how complex it is, the state of its quality signals, the RFCs it declares, where its upstream repo lives, and a plain English note for every public method and command.

---

## Summary

A small, fast C compiler — a fork of [TinyCC](https://bellard.org/tcc/) developed as a **phext-native** project: the compiler's sources, design, tests, proof receipts and build artifacts also live in a single phext file (`phcc.phext`) whose claims are hash-bound and machine-checked by its own gate. phcc compiles itself, compiles most C99 code at many times the speed of an optimising compiler, and can execute C directly with `-run`, like a scripting language.

(The opening of the project's README, quoted as written.)

---

## Dependencies

No dependency manifest was found (no Cargo.toml, package.json, requirements.txt, or go.mod), so no dependencies are declared in a form the scanner reads.

---

## Authors

The git log holds 3888 commits.

The author of record is **Will Bickford** — 99 commits folded into his account. Will's ruling (2026-11-22): all Exocortical authors fold into his account — "I speak for the cluster." The fold is presentation only: the git history is unchanged, and the per-persona detail is kept as annotation.

Annotation, by persona (all folded into Will Bickford):

- Theia — 46 commits
- Emise — 32 commits
- Alveus — 12 commits
- Will Bickford (his own identities) — 9 commits

Other authors, kept as themselves (not Exocortical cluster personas; never folded):

- grischka <grischka> — 555 commits
- Michael Matz <matz@suse.de> — 462 commits
- bellard <bellard> — 393 commits
- herman ten brugge <hermantenbrugge@home.nl> — 352 commits
- Thomas Preud'homme <robotux@celest.fr> — 239 commits
- seyko <seyko2@gmail.com> — 194 commits
- Christian Jullien <jullien@eligis.com> — 121 commits
- Danny Milosavljevic <dannym@scratchpost.org> — 78 commits
- Edmund Grimley Evans <Edmund.Grimley.Evans@gmail.com> — 72 commits
- Joe Soroka <gits@joesoroka.com> — 67 commits
- Thomas Preud'homme <thomas.preudhomme@celest.fr> — 64 commits
- Henry Kroll III <henry@comptune.com> — 63 commits
- Avi Halachmi (:avih) <avihpit@yahoo.com> — 56 commits
- jiang <30155751@qq.com> — 49 commits
- Shinichiro Hamaji <shinichiro.hamaji _at_ gmail.com> — 48 commits
- Daniel Glöckner <daniel-gl@gmx.net> — 42 commits
- Detlef Riekenberg <wine.dev@web.de> — 42 commits
- Christian Jullien <Christian Jullien> — 40 commits
- Meng Zhuo <mengzhuo@iscas.ac.cn> — 31 commits
- Petr Skocik <pskocik@gmail.com> — 30 commits
- Vlad Vissoultchev <wqweto@gmail.com> — 27 commits
- Philip <pipcet@gmail.com> — 25 commits
- Ekaitz Zarraga <ekaitz@elenq.tech> — 20 commits
- James Lyon <jamesly0n@hotmail.com> — 20 commits
- noneofyourbusiness <noneofyourbusiness@danwin1210.de> — 19 commits
- Aleksi Hannula <ahannula4+nixgit@gmail.com> — 18 commits
- mingodad <mingodad@gmail.com> — 18 commits
- matthias <uso.cosmo.ray@gmail.com> — 17 commits
- Kirill Smelkov <kirr@navytux.spb.ru> — 16 commits
- wanjochan <wanjochan@gmail.com> — 16 commits
- Reimar Döffinger <Reimar.Doeffinger@gmx.de> — 15 commits
- Vincent Lefevre <vincent@vinc17.net> — 15 commits
- Dmitry Selyutin <ghostmansd@gmail.com> — 14 commits
- Mounir IDRASSI <mounir.idrassi@amcrypto.jp> — 14 commits
- Steffen Nurpmeso <steffen@sdaoden.eu> — 13 commits
- gus knight <waddlesplash@gmail.com> — 13 commits
- Matthias Gatto <uso.cosmo.ray@gmail.com> — 12 commits
- Benjamin Oldenburg <benjamin.oldenburg@ordis.co.th> — 11 commits
- Detlef Riekenberg <tcc.dev@web.de> — 11 commits
- Larry Doolittle <ldoolitt@recycle.lbl.gov> — 11 commits
- Pavlas, Zdenek <zden3k@gmail.com> — 11 commits
- Shinichiro Hamaji <shinichiro.hamaji@gmail.com> — 11 commits
- Stefan <stefan-guix@vodafonemail.de> — 11 commits
- Christopher Bazley <chris.bazley.wg14@gmail.com> — 9 commits
- minux <minux.ma@gmail.com> — 9 commits
- noneofyourbusiness <stpavlenko@kolabnow.ch> — 9 commits
- Akim Demaille <akim@lrde.epita.fr> — 8 commits
- Kirill Smelkov <kirr@landau.phys.spbu.ru> — 8 commits
- Tyge Løvset <tylo@norceresearch.no> — 8 commits
- Urs Janssen <urs@hq.tin.org> — 8 commits
- Carlos Montiers <carlos@thefunsouth.com> — 7 commits
- Christian Jullien <jullien@coltrane.eligis.com> — 7 commits
- David Mertens <dcmertens.perl@gmail.com> — 7 commits
- Frederic Feret <tommy.planet@free.fr> — 7 commits
- Kirill Smelkov <kirr@mns.spb.ru> — 7 commits
- Zhang Boyang <zhangboyang.id@gmail.com> — 7 commits
- kbkpbot <kbkpbot@sina.com> — 7 commits
- Kyryl Melekhin <k.melekhin@gmail.com> — 6 commits
- Lee Duhem <lee.duhem@gmail.com> — 6 commits
- Roy <roytam@gmail.com> — 6 commits
- Sushant Pandurangi <sushpa@alumni.ethz.ch> — 6 commits
- Ziyao <tswuyin_st127@163.com> — 6 commits
- Arthur Williams <taaparthur@gmail.com> — 5 commits
- Jean-Claude Beaudoin <jean.claude.beaudoin@gmail.com> — 5 commits
- Matteo Cypriani <mcy@lm7.fr> — 5 commits
- Raphael Cohn <raphael.cohn@stormmq.com> — 5 commits
- Sergei Trofimovich <st@anti-virus.by> — 5 commits
- Udo <Udo@Udos-PC> — 5 commits
- YX Hao <lifenjoiner@163.com> — 5 commits
- Bernhard Reutner-Fischer <rep.dot.nop@gmail.com> — 4 commits
- Christian Jullien <jullien@freebsd64.eligis.com> — 4 commits
- Cyan Ogilvie <cyan.ogilvie@gmail.com> — 4 commits
- Jonathan M. Wilbur <jonathan@wilbur.space> — 4 commits
- Milutin Jovanović <jovanovic.milutin@gmail.com> — 4 commits
- Pursuer <1596067968@qq.com> — 4 commits
- Ramsay Jones <ramsay@ramsay1.demon.co.uk> — 4 commits
- Sergey Sushilin <sushilinsergey@gmail.com> — 4 commits
- remph <lhr@disroot.org> — 4 commits
- yuanbin <Administrator@yuan.(none)> — 4 commits
- Andrei Warkentin <andrey.warkentin@gmail.com> — 3 commits
- Changming Xu <xcmhit@gmail.com> — 3 commits
- Christian Jullien <jullien@openbsd.eligis.com> — 3 commits
- Christian Jullien <jullien@openbsd.home> — 3 commits
- Christian Jullien <jullien@openbsd64.eligis.com> — 3 commits
- Dylan Fei <feidylan23@gmail.com> — 3 commits
- Gabriel Corneanu <gabrielcorneanu@gmail.com> — 3 commits
- Jaroslav Kysela <perex@perex.cz> — 3 commits
- Kamil Rytarowski <n54@gmx.com> — 3 commits
- Milutin Jovanovic <jovanovic.milutin@gmail.com> — 3 commits
- Riccardo Schirone <sirmy15@gmail.com> — 3 commits
- Steven G. Messervey <nuke48386@yahoo.com> — 3 commits
- coltrane <jullien@coltrane.eligis.com> — 3 commits
- Andrew Mulbrook <andrew262@gmail.com> — 2 commits
- Archidemon <_dangerdl@mail.ru> — 2 commits
- Aron BARATH <baratharon@caesar.elte.hu> — 2 commits
- Austin English <austinenglish@gmail.com> — 2 commits
- Brian Callahan <bcallah@openbsd.org> — 2 commits
- Christian Jullien <jullien@sims3.eligis.com> — 2 commits
- CodeHz <codehz@outlook.com> — 2 commits
- Domingo Alvarez Duarte <mingodad@gmail.com> — 2 commits
- Eric Raible <raible@gmail.com> — 2 commits
- Giovanni Mascellani <gio@debian.org> — 2 commits
- H-language <hydrogen.language@gmail.com> — 2 commits
- Iavael <iavaelooeyt@gmail.com> — 2 commits
- John Nunley <dev@notgull.net> — 2 commits
- John Scott <jscott@posteo.net> — 2 commits
- Marc Vertes <mvertes@free.fr> — 2 commits
- OpenCode[AI] <opencode-ai@example.com> — 2 commits
- Pascal Cuoq <cuoq@trust-in-soft.com> — 2 commits
- Pursuer2 <1596067968@qq.com> — 2 commits
- Reini Urban <reini.urban@gmail.com> — 2 commits
- Scott Graham <scott.tinycc@h4ck3r.net> — 2 commits
- Thomas Preud'homme <robotux@armelhome.rsr.lip6.fr> — 2 commits
- Thomas Stalder <git@netsolux.ch> — 2 commits
- Timo VJ Lahde <timovj.lahde@gmail.com> — 2 commits
- Tyge <tylo@norceresearch.no> — 2 commits
- U-NELSON\jullien <jullien@nelson.eligis.com> — 2 commits
- kbkpbot <kbkpbot@gmail.com> — 2 commits
- keren <keren@bluebird.(none)> — 2 commits
- matthias gatto <uso.cosmo.ray@gmail.com> — 2 commits
- waltje <decwiz@yahoo.com> — 2 commits
- waterlens <waterlens@waterlens.moe> — 2 commits
- Adam Sampson <ats@offog.org> — 1 commit
- Alexander Egorenkov <ae@linux-pc-2.(none)> — 1 commit
- Alexander Medvednikov <alexander@medvednikov.com> — 1 commit
- Alexander Sosedkin <monk@unboiled.info> — 1 commit
- Alexandre Becoulet <diaxen@ganymede.localdomain> — 1 commit
- Ali Gholami Rudi <ali@rudi.ir> — 1 commit
- Amine Najahi <mohaminaj@gmail.com> — 1 commit
- Andrew Aladjev <aladjev.andrew@gmail.com> — 1 commit
- Andrey Gursky <andrey.gursky@e-mail.ua> — 1 commit
- Andrius Štikonas <andrius@stikonas.eu> — 1 commit
- Anon <tsoding> — 1 commit
- Arthur Williams <taaparthur@disroot.org> — 1 commit
- Austin English <me@example.com> — 1 commit
- Balazs Kezes <rlblaster@gmail.com> — 1 commit
- Ben Bacarisse <software@bsb.me.uk> — 1 commit
- Ben C <ben.c.dev@outlook.com> — 1 commit
- Ben Noordhuis <info@bnoordhuis.nl> — 1 commit
- Boian Berberov <bberberov@ieee.org> — 1 commit
- Brian Callahan <bcallah@posteo.net> — 1 commit
- Christian Jullien <jullien@byas.local> — 1 commit
- Christian Jullien <jullien@debian64.eligis.com> — 1 commit
- Christian Jullien <jullien@fedora64.eligis.com> — 1 commit
- Christian Jullien <jullien@freebsd64.home> — 1 commit
- Christian Jullien <jullien@sims.eligis.com> — 1 commit
- Christian Jullien <none> — 1 commit
- Claudio Bley <b_l_e_y@ml1.net> — 1 commit
- DAVID <76133201+majestuoso@users.noreply.github.com> — 1 commit
- DFP <dfp@disroot.org> — 1 commit
- Devin Hussey <husseydevin@gmail.com> — 1 commit
- Edmund Grimley Evans <Edmund.Grimley-Evans@arm.com> — 1 commit
- Emil <ez@la.mine.nu> — 1 commit
- Fabrice Bellard <bellard.org> — 1 commit
- Feret@.(none) <Feret@.(none)> — 1 commit
- Frédéric Féret <frederic.feret@outlook.com> — 1 commit
- Guest0x0 <guest0x0@outlook.com> — 1 commit
- Gynt <Gynt@users.noreply.github.com> — 1 commit
- Haelwenn (lanodan) Monnier <contact@hacktivis.me> — 1 commit
- Henry K <henry@desk.local> — 1 commit
- Hitoshi Mitake <h.mitake@gmail.com> — 1 commit
- Huang Zhaobin <zhaob1n@foxmail.com> — 1 commit
- James Buren <ryu0@ymail.com> — 1 commit
- Jan Boon <jan.boon@kaetemi.be> — 1 commit
- Jonathan Newman <jonathannewman314@gmail.com> — 1 commit
- Jookia <166291@gmail.com> — 1 commit
- Joseph Poirier <jdpoirier@gmail.com> — 1 commit
- Jov <amutu@amutu.com> — 1 commit
- Keith Thompson <Keith.S.Thompson@gmail.com> — 1 commit
- Konstantin Baltsat <baltsat2002@mail.ru> — 1 commit
- Kurt Nalty <kurt.nalty@kurtnalty.com> — 1 commit
- Luc Everse <luc@wukl.net> — 1 commit
- Luigi Rizzo <rizzo> — 1 commit
- Manuel Simoni <msimoni@gmail.com> — 1 commit
- Marek Knápek <knapek.mar@gmail.com> — 1 commit
- Mathias Krause <minipli@grsecurity.net> — 1 commit
- Maxim Logaev <maxlogaev@proton.me> — 1 commit
- Meng Zhuo <mengzhuo1203@gmail.com> — 1 commit
- Michael Wilder <mwilder.w@gmail.com> — 1 commit
- Nicolas Limare <nicolas.limare@cmla.ens-cachan.fr> — 1 commit
- Niklas Rosencrantz <niklasr@pm.me> — 1 commit
- Nikos Mavrogiannopoulos <nmav@gnutls.org> — 1 commit
- OldWorldOrdr <joey.t.reinhart@gmail.com> — 1 commit
- Per Nordlöw <PNordlow@symmetryinvestments.com> — 1 commit
- Per Nordlöw <per.nordlow@gmail.com> — 1 commit
- Petr Skocik <borgx.space@gmail.com> — 1 commit
- Raven <citrons@mondecitronne.com> — 1 commit
- Reinhard Urban <reinhard.urban@nubix.de> — 1 commit
- Reini Urban <reini@spexai.com> — 1 commit
- Rob Pilling <robpilling@gmail.com> — 1 commit
- Robert Hoelzl <robert.hoelzl@posteo.de> — 1 commit
- Romain Francoise <rfrancoise@debian.org> — 1 commit
- Roy Tam <roytam@gmail.com> — 1 commit
- Ryan Burns <rtburns@protonmail.com> — 1 commit
- Sam Ellicott <sellicott@cedarville.edu> — 1 commit
- Sam Watkins <sam@nikas.(none)> — 1 commit
- Sergey Sushilin <sergeysushilin@protonmail.com> — 1 commit
- Sergey Vinokurov <serg.foo@gmail.com> — 1 commit
- Siddhesh Poyarekar <siddhesh@gotplt.org> — 1 commit
- Sigrid Solveig Haflínudóttir <sigrid@ftrv.se> — 1 commit
- Sizhe Zhao <prc.zhao@outlook.com> — 1 commit
- Soloist Deng <soloist.deng-gmail.com> — 1 commit
- Sylvain BERTRAND <sylvain.bertrand@legeek.net> — 1 commit
- Theodore Dubois <tblodt@icloud.com> — 1 commit
- Thomas Coudray <amanone@gmail.com> — 1 commit
- Thomas Preud'homme <robotux@mallow.celest.fr> — 1 commit
- Thomas Stalder <thomas@netsolux.ch> — 1 commit
- Timo VJ Lähde <timovj.lahde@gmail.com> — 1 commit
- U-UNIT1\dennis <dennis@UNIT1.(none)> — 1 commit
- Urja Rannikko <urjaman@gmail.com> — 1 commit
- Urs Janssen <urs@akk.org> — 1 commit
- Urs Janssen <urs@niko.tin.org> — 1 commit
- Vittorio Giovara <vittorio.giovara@gmail.com> — 1 commit
- Vlad Vissoultchev <vlado@vis.bg> — 1 commit
- Willy Tarreau <w@1wt.eu> — 1 commit
- Wirtos_new <Wirtos.new@gmail.com> — 1 commit
- Yao Zi <ziyao@disroot.org> — 1 commit
- Zdenek Pavlas <zden3k@gmail.com> — 1 commit
- Ziga Lenarcic <ziga.lenarcic@protonmail.com> — 1 commit
- Ziyao <ziyao@disroot.org> — 1 commit
- Zuhaitz-dev <zuhaitz.zechhub@gmail.com> — 1 commit
- ayush-varshney <ayush.kumar@upc.edu> — 1 commit
- bobbl <bobbl@gmx.de> — 1 commit
- checkroom <checkroom@lenovos-novos.lan> — 1 commit
- diannaaa <diannaaav@gmail.com> — 1 commit
- emekoi <emekankurumeh@outlook.com> — 1 commit
- foobar <foobar@redchan.it> — 1 commit
- general <general> — 1 commit
- gr <gr@gr-laptop.(none)> — 1 commit
- illiliti <illiliti@dimension.sh> — 1 commit
- inostensibl <inostensibl@gmail.com> — 1 commit
- janus.lt <janus.lt> — 1 commit
- mob <mob> — 1 commit
- noneofyourbusiness <noneofyourbusiness@kolabnow.ch> — 1 commit
- orbea <ovariegata@yahoo.com> — 1 commit
- owenewans <owenewans@owenewans.org> — 1 commit
- root <root@mallow.celest.fr> — 1 commit
- tuxcrafting <tuxcrafting@proton.me> — 1 commit
- unknown <TimoL@.(none)> — 1 commit
- unknown <wctam@.itedn32a.localdomain> — 1 commit
- yan-bin2009 <yan20095642558@163.com> — 1 commit
- yuanbin <yuanbin0@gmail.com> — 1 commit
- Øyvind A. Holm <sunny@sunbase.org> — 1 commit

---

## Complexity

The measure is stated plainly, and no single score is made: lines are counted per file as total lines, blank lines, comment lines, and code lines (total minus blank minus comment). Functions are counted where the scanner reads them, and the largest functions are given by their span in lines.

- C: 537 files, 99266 lines total, 8525 blank, 14931 comment, 75810 code
- C header: 119 files, 49324 lines total, 5516 blank, 24677 comment, 19131 code
- Markdown: 19 files, 6747 lines total, 494 blank, 13 comment, 6240 code
- Python: 20 files, 7947 lines total, 650 blank, 439 comment, 6858 code
- Shell: 4 files, 611 lines total, 42 blank, 40 comment, 529 code

All languages: 699 files, 163895 lines total, 108568 lines of code. Functions read: 169.

Largest files:

- tccgen.c — 13490 lines
- win32/include/winapi/winnt.h — 5994 lines
- tccelf.c — 5976 lines
- win32/include/winapi/winuser.h — 5652 lines
- tccpp.c — 5039 lines

Largest functions, by span in lines:

- conformance_section in tools/phcc_gate.py — about 179 lines
- structured_file in tools/test_prune.py — about 140 lines
- add_root in tools/phcc_prune.py — about 136 lines
- main in tools/phcc_harvest.py — about 126 lines
- check in examples/verify_examples.sh — about 125 lines

---

## Code quality signals

Each signal is named and shown with its evidence. A signal is a fact the scanner read, not a grade.

- Tests: 0 test functions counted in the source, across 13 files whose names mark them as test files.
- CI: no CI configuration was found in the project.
- Lint and format configuration files: Makefile, examples/multifile/Makefile, lib/Makefile, phext/tests/Makefile, tests/Makefile, tests/pp/Makefile, tests/tests2/Makefile.
- TODO marks in the source: 35. FIXME marks: 7.
- Public items with a doc comment: 53 of 164 (32%).

---

## Declared RFCs

No rfcs.json was found in this project, so it declares no RFCs. This is a finding, not an error: the project has not made an RFC declaration.


---

## Upstream git repo

- Origin: https://github.com/wbic16/phcc
- Branch: exo
- Head commit: 0628c00360a22aafcaf4d0bf26a4991f9d29b900
- Working tree: clean

---

## Public methods and commands

Every public method and command the scanner found, each with a plain English note. Each note is labeled with where its words came from: direct (the code's own doc comment), inferred (read off the item's signature), agent-supplied (written by the user's agent), or awaiting a summary.

- function `check` (examples/verify_examples.sh:27) [example] — Function check takes no inputs beyond the value it is called on.
- function `sha` (tools/acceptance_local.py:21) — Function sha takes b.
- function `main` (tools/acceptance_local.py:25) — Function main takes no inputs beyond the value it is called on.
- function `sha` (tools/assemble_reissue.py:21) — Function sha takes b.
- function `main` (tools/assemble_reissue.py:25) — Function main takes no inputs beyond the value it is called on.
- function `sha` (tools/build_reissue.py:60) — Function sha takes b.
- function `splice_block` (tools/build_reissue.py:64) — Function splice_block takes lines, path, new_lines.
- function `normalize_listing` (tools/build_reissue.py:71) — Function normalize_listing takes body, objname.
- function `assembly_scroll` (tools/build_reissue.py:82) — Function assembly_scroll takes artifact, src_coord, src_sha, target, compiler, cmd, flags, renderer, render_cmd, obj_coord, obj_raw, listing_body, obj_name.
- function `canonical` (tools/build_reissue.py:113) — Function canonical takes raw.
- function `main` (tools/build_reissue.py:117) — Function main takes no inputs beyond the value it is called on.
- function `rd` (tools/build_reissue.py:195) — Function rd takes n.
- function `old_body` (tools/build_reissue.py:247) — Function old_body takes coord.
- function `parse_machine` (tools/check_linker.py:118) — Function parse_machine takes text.
- function `vol3_members` (tools/check_linker.py:162) — Function vol3_members takes scrolls, index_coord, kind.
- function `struct_fields` (tools/check_linker.py:172) — Function struct_fields takes text, struct_name.
- function `defines` (tools/check_linker.py:210) — Function defines takes text.
- function `main` (tools/check_linker.py:214) — Function main takes no inputs beyond the value it is called on.
- function `parse_machine` (tools/check_machine.py:52) — Split the scroll into sections; return dict of parsed parts.
- function `vol3_functions` (tools/check_machine.py:98) — Function vol3_functions takes scrolls, index_coord.
- function `vol3_objects` (tools/check_machine.py:108) — Function vol3_objects takes scrolls, index_coord.
- function `struct_fields` (tools/check_machine.py:118) — Extract field identifiers from a struct body in C source.
- function `defines` (tools/check_machine.py:166) — Function defines takes text.
- function `main` (tools/check_machine.py:170) — Function main takes no inputs beyond the value it is called on.
- function `parse_machine` (tools/check_regalloc.py:153) — Function parse_machine takes text.
- function `vol3_members` (tools/check_regalloc.py:198) — Function vol3_members takes scrolls, index_coord, kind.
- function `struct_fields` (tools/check_regalloc.py:208) — Function struct_fields takes text, struct_name.
- function `defines` (tools/check_regalloc.py:247) — Function defines takes text.
- function `check_one` (tools/check_regalloc.py:252) — Function check_one takes scrolls, mcoord, funcs, universe_base, label.
- function `main` (tools/check_regalloc.py:320) — Function main takes no inputs beyond the value it is called on.
- function `parse_machine` (tools/check_vstack.py:118) — Split the scroll into sections; return dict of parsed parts.
- function `vol3_members` (tools/check_vstack.py:164) — Function vol3_members takes scrolls, index_coord, kind.
- function `struct_fields` (tools/check_vstack.py:174) — Extract field identifiers from a struct body in C source.
- function `defines` (tools/check_vstack.py:216) — Function defines takes text.
- function `main` (tools/check_vstack.py:220) — Function main takes no inputs beyond the value it is called on.
- function `load_scrolls` (tools/phcc_analyze.py:75) — Function load_scrolls takes path.
- function `is_binary_scroll` (tools/phcc_analyze.py:79) — Function is_binary_scroll takes content.
- function `collect_segments` (tools/phcc_analyze.py:85) — Segment records for every manifest file with a Volume 3 index,
- function `normalize_code` (tools/phcc_analyze.py:140) — C-comment-stripped, whitespace-collapsed form of a member
- function `census` (tools/phcc_analyze.py:180) — All census groupings.
- function `cluster_stats` (tools/phcc_analyze.py:253) — Function cluster_stats takes clusters, size_of.
- function `first_line` (tools/phcc_analyze.py:259) — Function first_line takes content, limit=72.
- function `render_report` (tools/phcc_analyze.py:264) — Function render_report takes phext_path, raw, scrolls, entries, segs, problems, cz, apply_info=None.
- function `render_apply` (tools/phcc_analyze.py:384) — Function render_apply takes ai.
- function `apply_dedup` (tools/phcc_analyze.py:432) — Rewrite exact duplicates as dedup stubs.
- function `demote_scan` (tools/phcc_analyze.py:502) — Find every binding a rewrite invalidates.
- function `split_segments` (tools/phcc_analyze.py:587) — Split a C payload into statement-grain segments.
- function `cut` (tools/phcc_analyze.py:621) — Function cut takes end, term.
- function `stmt_group` (tools/phcc_analyze.py:742) — Coarse grouping for per-backend ratios.
- function `statement_census` (tools/phcc_analyze.py:764) — Segment every segment payload and tally the statement
- function `bucket` (tools/phcc_analyze.py:788) — Function bucket takes ln.
- function `render_stmt_report` (tools/phcc_analyze.py:870) — Function render_stmt_report takes phext_path, raw, segs, sc, build_info=None.
- function `render_stmt_build` (tools/phcc_analyze.py:967) — Function render_stmt_build takes bi.
- function `assign_stmt_coords` (tools/phcc_analyze.py:997) — Content-derived coordinates for the statement library.
- function `build_stmt` (tools/phcc_analyze.py:1045) — Rewrite every segment payload as a statement reference
- function `verify_stmt` (tools/phcc_analyze.py:1096) — Verify a statement-level phext against its own records:
- function `run` (tools/phcc_analyze.py:1176) — Function run takes a.
- function `cli` (tools/phcc_analyze.py:1289) — Function cli takes argv.
- function `sha256` (tools/phcc_builds.py:53) — Function sha256 takes b.
- function `is_region` (tools/phcc_builds.py:57) — Function is_region takes coord.
- function `is_canonical_binary_scroll` (tools/phcc_builds.py:61) — Function is_canonical_binary_scroll takes content.
- function `encode_canonical_binary_scroll` (tools/phcc_builds.py:65) — Function encode_canonical_binary_scroll takes raw.
- function `decode_canonical_binary_scroll` (tools/phcc_builds.py:69) — Raw bytes of a canonical binary scroll, or raise ValueError.
- function `parse_binary_scroll` (tools/phcc_builds.py:77) — (header dict, raw bytes) or raise ValueError.
- function `parse_region_manifest` (tools/phcc_builds.py:110) — {coord-string: entry dict} from the region root manifest.
- function `cmd_region_list` (tools/phcc_builds.py:128) — Function cmd_region_list takes a.
- function `cmd_region_wipe` (tools/phcc_builds.py:170) — Function cmd_region_wipe takes a.
- function `cmd_seal` (tools/phcc_builds.py:184) — Function cmd_seal takes a.
- function `cmd_unseal` (tools/phcc_builds.py:201) — Function cmd_unseal takes a.
- function `cli` (tools/phcc_builds.py:215) — Function cli takes argv.
- function `analyse` (tools/phcc_gate.py:99) — Function analyse takes scrolls.
- function `pct` (tools/phcc_gate.py:137) — Function pct takes n, d.
- function `tables_text` (tools/phcc_gate.py:141) — Function tables_text takes per_book, artifacts, gaps, t1_eligible, receipted=None, proof_notes=None, design=None…
- function `verdict_text` (tools/phcc_gate.py:215) — Function verdict_text takes scrolls, artifacts, per_book, gaps, t1_eligible, receipted=None, proof_notes=None…
- function `parse_proof_claims` (tools/phcc_gate.py:267) — Return {source_coord: claim-dict} for artifacts with Proofs scrolls.
- function `tiers_from_receipts` (tools/phcc_gate.py:289) — {source_coord: (tier, note)} — the receipted tier overlay.
- function `parse_design_views` (tools/phcc_gate.py:352) — Return {'checked': n, 'current': [path], 'stale': [(path,
- function `machine_section` (tools/phcc_gate.py:410) — Function machine_section takes scrolls, phext_path.
- function `machine2_section` (tools/phcc_gate.py:483) — Function machine2_section takes scrolls, phext_path.
- function `machine3_section` (tools/phcc_gate.py:564) — Function machine3_section takes scrolls, phext_path.
- function `machine4_section` (tools/phcc_gate.py:641) — Function machine4_section takes scrolls, phext_path.
- function `decode_binary_scroll` (tools/phcc_gate.py:748) — Raw bytes of a binary scroll, or None if undecodable.
- function `assembly_section` (tools/phcc_gate.py:774) — Function assembly_section takes scrolls.
- function `conformance_section` (tools/phcc_gate.py:885) — Function conformance_section takes scrolls.
- function `main` (tools/phcc_gate.py:1064) — Function main takes no inputs beyond the value it is called on.
- function `nm_symbols` (tools/phcc_harvest.py:20) — Return {object_name: (defined_global, defined_local, undefined)}.
- function `main` (tools/phcc_harvest.py:52) — Function main takes no inputs beyond the value it is called on.
- function `load_scrolls` (tools/phcc_migrate.py:78) — Function load_scrolls takes path.
- function `manifest_entries` (tools/phcc_migrate.py:82) — Function manifest_entries takes scrolls.
- function `write_file` (tools/phcc_migrate.py:96) — Function write_file takes out, path, data, mode.
- function `resolve_stub` (tools/phcc_migrate.py:107) — Expand a dedup stub (phext_io dedup refs, written by
- function `resolve_stmt_seq` (tools/phcc_migrate.py:147) — Expand a statement reference sequence (phext_io statement
- function `unpack_vol2` (tools/phcc_migrate.py:185) — Yield (path, data, mode, ok, why).
- function `unpack_vol3` (tools/phcc_migrate.py:208) — Reassemble from Volume 3 segment scrolls where a file index
- function `cmd_export` (tools/phcc_migrate.py:286) — Function cmd_export takes a.
- function `place` (tools/phcc_migrate.py:335) — Return (book, chapter) for a repo-relative path, or None.
- function `place_full` (tools/phcc_migrate.py:387) — Return (collection, volume, book, chapter) for a path, or None.
- function `phext_dir_exclusion` (tools/phcc_migrate.py:418) — Why a phext/** path is excluded from pack placement (the
- function `enumerate_tree` (tools/phcc_migrate.py:499) — [(mode, path)] for every packable file under tree, path-sorted.
- function `stage_a` (tools/phcc_migrate.py:528) — Build the Volume 1 + Volume 2 scroll set.
- function `find_tool` (tools/phcc_migrate.py:597) — Function find_tool takes name.
- function `find_ts_python` (tools/phcc_migrate.py:609) — Function find_ts_python takes explicit.
- function `cmd_import` (tools/phcc_migrate.py:627) — Function cmd_import takes a.
- function `cli` (tools/phcc_migrate.py:668) — Function cli takes argv.
- function `Member.__init__` (tools/phcc_prune.py:156) — Function Member.__init__ takes self, path, scr, kind, names, form, span, psha, payload, seg_coord.
- function `Member.mid` (tools/phcc_prune.py:163) — Function Member.mid takes no inputs beyond the value it is called on.
- function `CarriedFile.__init__` (tools/phcc_prune.py:171) — Function CarriedFile.__init__ takes self, entry.
- function `load_carried` (tools/phcc_prune.py:181) — Return (scrolls, [CarriedFile]) — main manifest + Sec-14 addendum.
- function `Harvest.__init__` (tools/phcc_prune.py:241) — Function Harvest.__init__ takes self, path.
- function `include_closure` (tools/phcc_prune.py:300) — #include closure over carried files, starting at seeds.
- function `resolve` (tools/phcc_prune.py:320) — Function resolve takes p, name.
- function `is_build_ref_file` (tools/phcc_prune.py:358) — Files whose text the build itself reads as instructions —
- function `classify_files` (tools/phcc_prune.py:366) — path -> 'graph' | 'keep' | 'drop' for manifest files.
- function `build_graph` (tools/phcc_prune.py:429) — Members of graph-class files; name index; raw payload store.
- function `build_suffix_index` (tools/phcc_prune.py:460) — name-suffix -> [member ids]: for every member name, every
- function `member_edges` (tools/phcc_prune.py:475) — Out-edges of one member: every member bearing a name tokenised
- function `build_providers` (tools/phcc_prune.py:504) — token -> [member ids] for referenced tokens no member is
- function `prune` (tools/phcc_prune.py:526) — Function prune takes phext_path, harvest_path, extra_roots, extra_keep, runtime_prefix=LIB_PREFIX, keep_prefixes=None, keep_files=None.
- function `add_root` (tools/phcc_prune.py:594) — Function add_root takes mid, why.
- function `emit` (tools/phcc_prune.py:730) — Function emit takes res, phext_path.
- function `prune_manifest_text` (tools/phcc_prune.py:822) — Function prune_manifest_text takes res, stats.
- function `cli` (tools/phcc_prune.py:873) — Function cli takes argv.
- function `source_bytes` (tools/phcc_prune.py:922) — Function source_bytes takes scrolls.
- function `fmt` (tools/phext_io.py:18) — Function fmt takes coord.
- function `parse_coord` (tools/phext_io.py:22) — Function parse_coord takes s.
- function `walk_bytes` (tools/phext_io.py:30) — Dead-reckoning byte walk — same state machine as parse_phext.py,
- function `implode` (tools/phext_io.py:52) — scrolls: dict coord_tuple -> bytes.
- function `sha256` (tools/phext_io.py:68) — Function sha256 takes data.
- function `load_phext` (tools/phext_io.py:72) — Function load_phext takes path.
- function `dedup_ref` (tools/phext_io.py:99) — The stub bytes standing for the payload at `coord`.
- function `parse_dedup_ref` (tools/phext_io.py:105) — (coord_tuple, sha256_hex) if `data` is exactly a dedup stub,
- function `stmt_ref` (tools/phext_io.py:149) — One reference line (bytes, newline-terminated).
- function `stmt_ref_empty` (tools/phext_io.py:156) — The reference line for a whitespace-only segment.
- function `parse_stmt_seq` (tools/phext_io.py:162) — Parse a statement reference sequence payload.
- function `parse_manifest` (tools/phext_io.py:200) — Return {path: coord_tuple} from the Slice 0 placement manifest
- function `segment_one` (tools/segment_libtcc1.py:33) — Function segment_one takes path, data, v2coord_s.
- function `main` (tools/segment_libtcc1.py:145) — Function main takes no inputs beyond the value it is called on.
- function `check` (tools/test_analyze.py:38) [test] — Function check takes cond, label.
- function `run` (tools/test_analyze.py:44) [test] — Function run takes args, cwd.
- function `file_bytes` (tools/test_analyze.py:89) [test] — Function file_bytes takes path.
- function `build_fixture` (tools/test_analyze.py:96) [test] — Function build_fixture takes corrupt_index=False.
- function `read_tree` (tools/test_analyze.py:149) [test] — Function read_tree takes d.
- function `main` (tools/test_analyze.py:153) [test] — Function main takes no inputs beyond the value it is called on.
- function `check` (tools/test_prune.py:32) [test] — Function check takes cond, label.
- function `seg_header` (tools/test_prune.py:38) [test] — Function seg_header takes kind, name, form, span, psha.
- function `structured_file` (tools/test_prune.py:43) [test] — rows: [(kind, name, form, payload)] -> (vol2 bytes, scrolls dict)
- function `build_fixture` (tools/test_prune.py:183) [test] — Function build_fixture takes td.
- function `main` (tools/test_prune.py:211) [test] — Function main takes no inputs beyond the value it is called on.
- function `check` (tools/test_stmt.py:26) [test] — Function check takes name, cond, detail="".
- function `cores` (tools/test_stmt.py:35) [test] — Function cores takes data.
- function `seg_scroll` (tools/test_stmt.py:151) [test] — Function seg_scroll takes kind, name, form, span, payload.
- function `fmt` (tools/upstream_carry.py:81) — Function fmt takes coord.
- function `parse_coord` (tools/upstream_carry.py:85) — Function parse_coord takes s.
- function `walk_bytes` (tools/upstream_carry.py:93) — Dead-reckoning byte walk — yields (coord_tuple, bytes) so
- function `implode` (tools/upstream_carry.py:112) — scrolls: dict coord_tuple -> bytes.
- function `sha256` (tools/upstream_carry.py:128) — Function sha256 takes data.
- function `encode_scroll` (tools/upstream_carry.py:132) — Return (scroll_bytes, encoding) for one file's bytes.
- function `decode_scroll` (tools/upstream_carry.py:150) — Function decode_scroll takes scroll, encoding.
- function `git` (tools/upstream_carry.py:196) — Function git takes repo, *args.
- function `placement` (tools/upstream_carry.py:201) — entries: list of paths (files only).
- function `cmd_build` (tools/upstream_carry.py:229) — Function cmd_build takes a.
- function `cmd_unpack` (tools/upstream_carry.py:335) — Function cmd_unpack takes a.
- function `main` (tools/upstream_carry.py:365) — Function main takes no inputs beyond the value it is called on.
- function `func` (tools/verify_gate_splice_81.py:23) — Function func takes text, name.

---

## function — check

Found in examples/verify_examples.sh at line 27.

Surface: example.

Signature: `check()`

What it does: Function check takes no inputs beyond the value it is called on. Its return type is the unit value, which the signature leaves unwritten.

(Where these words came from: inferred.)

---

## function — sha

Found in tools/acceptance_local.py at line 21.

Surface: public API.

Signature: `def sha(b)`

What it does: Function sha takes b. Its return type is unannotated; the code shows it returning hashlib.sha256(b).hexdigest().

(Where these words came from: inferred.)

---

## function — main

Found in tools/acceptance_local.py at line 25.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sha

Found in tools/assemble_reissue.py at line 21.

Surface: public API.

Signature: `def sha(b)`

What it does: Function sha takes b. Its return type is unannotated; the code shows it returning phext_io.sha256(b).

(Where these words came from: inferred.)

---

## function — main

Found in tools/assemble_reissue.py at line 25.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — sha

Found in tools/build_reissue.py at line 60.

Surface: public API.

Signature: `def sha(b)`

What it does: Function sha takes b. Its return type is unannotated; the code shows it returning hashlib.sha256(b).hexdigest().

(Where these words came from: inferred.)

---

## function — splice_block

Found in tools/build_reissue.py at line 64.

Surface: public API.

Signature: `def splice_block(lines, path, new_lines)`

What it does: Function splice_block takes lines, path, new_lines. Its return type is unannotated; the code shows it returning lines[:idx[0]] + new_lines + lines[idx[-1] + 1:].

(Where these words came from: inferred.)

---

## function — normalize_listing

Found in tools/build_reissue.py at line 71.

Surface: public API.

Signature: `def normalize_listing(body, objname)`

What it does: Function normalize_listing takes body, objname. Its return type is unannotated; the code shows it returning "\n".join(lines).encode("utf-8").

(Where these words came from: inferred.)

---

## function — assembly_scroll

Found in tools/build_reissue.py at line 82.

Surface: public API.

Signature: `def assembly_scroll(artifact, src_coord, src_sha, target, compiler, cmd, flags, renderer, render_cmd, obj_coord, obj_raw, listing_body, obj_name)`

What it does: Function assembly_scroll takes artifact, src_coord, src_sha, target, compiler, cmd, flags, renderer, render_cmd, obj_coord, obj_raw, listing_body, obj_name. Its return type is unannotated; the code shows it returning ("\n".join(hdr) + "\n\n").encode("utf-8") + body.

(Where these words came from: inferred.)

---

## function — canonical

Found in tools/build_reissue.py at line 113.

Surface: public API.

Signature: `def canonical(raw)`

What it does: Function canonical takes raw. Its return type is unannotated; the code shows it returning b"phext-binary:1:base64\n" + base64.b64encode(ra.

(Where these words came from: inferred.)

---

## function — main

Found in tools/build_reissue.py at line 117.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — rd

Found in tools/build_reissue.py at line 195.

Surface: public API.

Signature: `def rd(n)`

What it does: Function rd takes n. Its return type is unannotated; the code shows it returning open(os.path.join(outdir, n), "rb").read().

(Where these words came from: inferred.)

---

## function — old_body

Found in tools/build_reissue.py at line 247.

Surface: public API.

Signature: `def old_body(coord)`

What it does: Function old_body takes coord. Its return type is unannotated; the code shows it returning b.

(Where these words came from: inferred.)

---

## function — parse_machine

Found in tools/check_linker.py at line 118.

Surface: public API.

Signature: `def parse_machine(text)`

What it does: Function parse_machine takes text. Its return type is unannotated; the code shows it returning states, events, notation, transitions.

(Where these words came from: inferred.)

---

## function — vol3_members

Found in tools/check_linker.py at line 162.

Surface: public API.

Signature: `def vol3_members(scrolls, index_coord, kind)`

What it does: Function vol3_members takes scrolls, index_coord, kind. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — struct_fields

Found in tools/check_linker.py at line 172.

Surface: public API.

Signature: `def struct_fields(text, struct_name)`

What it does: Function struct_fields takes text, struct_name. Its return type is unannotated; the code shows it returning set(); fields.

(Where these words came from: inferred.)

---

## function — defines

Found in tools/check_linker.py at line 210.

Surface: public API.

Signature: `def defines(text)`

What it does: Function defines takes text. Its return type is unannotated; the code shows it returning set(re.findall(r"#\s*define\s+([A-Za-z_][A-Za-z0.

(Where these words came from: inferred.)

---

## function — main

Found in tools/check_linker.py at line 214.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — parse_machine

Found in tools/check_machine.py at line 52.

Surface: public API.

Signature: `def parse_machine(text)`

What it does: Split the scroll into sections; return dict of parsed parts.

(Where these words came from: direct.)

---

## function — vol3_functions

Found in tools/check_machine.py at line 98.

Surface: public API.

Signature: `def vol3_functions(scrolls, index_coord)`

What it does: Function vol3_functions takes scrolls, index_coord. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — vol3_objects

Found in tools/check_machine.py at line 108.

Surface: public API.

Signature: `def vol3_objects(scrolls, index_coord)`

What it does: Function vol3_objects takes scrolls, index_coord. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — struct_fields

Found in tools/check_machine.py at line 118.

Surface: public API.

Signature: `def struct_fields(text, struct_name)`

What it does: Extract field identifiers from a struct body in C source.

(Where these words came from: direct.)

---

## function — defines

Found in tools/check_machine.py at line 166.

Surface: public API.

Signature: `def defines(text)`

What it does: Function defines takes text. Its return type is unannotated; the code shows it returning set(re.findall(r"#\s*define\s+([A-Za-z_][A-Za-z0.

(Where these words came from: inferred.)

---

## function — main

Found in tools/check_machine.py at line 170.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — parse_machine

Found in tools/check_regalloc.py at line 153.

Surface: public API.

Signature: `def parse_machine(text)`

What it does: Function parse_machine takes text. Its return type is unannotated; the code shows it returning states, events, notation, transitions, invariant.

(Where these words came from: inferred.)

---

## function — vol3_members

Found in tools/check_regalloc.py at line 198.

Surface: public API.

Signature: `def vol3_members(scrolls, index_coord, kind)`

What it does: Function vol3_members takes scrolls, index_coord, kind. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — struct_fields

Found in tools/check_regalloc.py at line 208.

Surface: public API.

Signature: `def struct_fields(text, struct_name)`

What it does: Function struct_fields takes text, struct_name. Its return type is unannotated; the code shows it returning set(); fields.

(Where these words came from: inferred.)

---

## function — defines

Found in tools/check_regalloc.py at line 247.

Surface: public API.

Signature: `def defines(text)`

What it does: Function defines takes text. Its return type is unannotated; the code shows it returning set(re.findall(r"#\s*define\s+([A-Za-z_][A-Za-z0.

(Where these words came from: inferred.)

---

## function — check_one

Found in tools/check_regalloc.py at line 252.

Surface: public API.

Signature: `def check_one(scrolls, mcoord, funcs, universe_base, label)`

What it does: Function check_one takes scrolls, mcoord, funcs, universe_base, label. Its return type is unannotated; the code shows it returning [f"{label}: machine scroll absent at "; violations, counts.

(Where these words came from: inferred.)

---

## function — main

Found in tools/check_regalloc.py at line 320.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — parse_machine

Found in tools/check_vstack.py at line 118.

Surface: public API.

Signature: `def parse_machine(text)`

What it does: Split the scroll into sections; return dict of parsed parts.

(Where these words came from: direct.)

---

## function — vol3_members

Found in tools/check_vstack.py at line 164.

Surface: public API.

Signature: `def vol3_members(scrolls, index_coord, kind)`

What it does: Function vol3_members takes scrolls, index_coord, kind. Its return type is unannotated; the code shows it returning out.

(Where these words came from: inferred.)

---

## function — struct_fields

Found in tools/check_vstack.py at line 174.

Surface: public API.

Signature: `def struct_fields(text, struct_name)`

What it does: Extract field identifiers from a struct body in C source.

(Where these words came from: direct.)

---

## function — defines

Found in tools/check_vstack.py at line 216.

Surface: public API.

Signature: `def defines(text)`

What it does: Function defines takes text. Its return type is unannotated; the code shows it returning set(re.findall(r"#\s*define\s+([A-Za-z_][A-Za-z0.

(Where these words came from: inferred.)

---

## function — main

Found in tools/check_vstack.py at line 220.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — load_scrolls

Found in tools/phcc_analyze.py at line 75.

Surface: public API.

Signature: `def load_scrolls(path)`

What it does: Function load_scrolls takes path. Its return type is unannotated; the code shows it returning dict(walk_bytes(open(path, "rb").read())).

(Where these words came from: inferred.)

---

## function — is_binary_scroll

Found in tools/phcc_analyze.py at line 79.

Surface: public API.

Signature: `def is_binary_scroll(content)`

What it does: Function is_binary_scroll takes content. Its return type is unannotated; the code shows it returning content == BINARY_HEADER or \.

(Where these words came from: inferred.)

---

## function — collect_segments

Found in tools/phcc_analyze.py at line 85.

Surface: public API.

Signature: `def collect_segments(scrolls, entries)`

What it does: Segment records for every manifest file with a Volume 3 index,

(Where these words came from: direct.)

---

## function — normalize_code

Found in tools/phcc_analyze.py at line 140.

Surface: public API.

Signature: `def normalize_code(data)`

What it does: C-comment-stripped, whitespace-collapsed form of a member

(Where these words came from: direct.)

---

## function — census

Found in tools/phcc_analyze.py at line 180.

Surface: public API.

Signature: `def census(scrolls, entries, segs)`

What it does: All census groupings. Returns a dict of tallies + clusters.

(Where these words came from: direct.)

---

## function — cluster_stats

Found in tools/phcc_analyze.py at line 253.

Surface: public API.

Signature: `def cluster_stats(clusters, size_of)`

What it does: Function cluster_stats takes clusters, size_of. Its return type is unannotated; the code shows it returning n, dup.

(Where these words came from: inferred.)

---

## function — first_line

Found in tools/phcc_analyze.py at line 259.

Surface: public API.

Signature: `def first_line(content, limit=72)`

What it does: Function first_line takes content, limit=72. Its return type is unannotated; the code shows it returning line[:limit].

(Where these words came from: inferred.)

---

## function — render_report

Found in tools/phcc_analyze.py at line 264.

Surface: public API.

Signature: `def render_report(phext_path, raw, scrolls, entries, segs, problems, cz, apply_info=None)`

What it does: Function render_report takes phext_path, raw, scrolls, entries, segs, problems, cz, apply_info=None. Its return type is unannotated; the code shows it returning "\n".join(L) + "\n".

(Where these words came from: inferred.)

---

## function — render_apply

Found in tools/phcc_analyze.py at line 384.

Surface: public API.

Signature: `def render_apply(ai)`

What it does: Function render_apply takes ai. Its return type is unannotated; the code shows it returning L.

(Where these words came from: inferred.)

---

## function — apply_dedup

Found in tools/phcc_analyze.py at line 432.

Surface: public API.

Signature: `def apply_dedup(scrolls, entries, segs, cz)`

What it does: Rewrite exact duplicates as dedup stubs. Returns

(Where these words came from: direct.)

---

## function — demote_scan

Found in tools/phcc_analyze.py at line 502.

Surface: public API.

Signature: `def demote_scan(scrolls, changed, touched, rewritten, seg_stubs)`

What it does: Find every binding a rewrite invalidates. A scroll (other

(Where these words came from: direct.)

---

## function — split_segments

Found in tools/phcc_analyze.py at line 587.

Surface: public API.

Signature: `def split_segments(data)`

What it does: Split a C payload into statement-grain segments.

(Where these words came from: direct.)

---

## function — cut

Found in tools/phcc_analyze.py at line 621.

Surface: public API.

Signature: `def cut(end, term)`

What it does: Function cut takes end, term. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — stmt_group

Found in tools/phcc_analyze.py at line 742.

Surface: public API.

Signature: `def stmt_group(path)`

What it does: Coarse grouping for per-backend ratios.

(Where these words came from: direct.)

---

## function — statement_census

Found in tools/phcc_analyze.py at line 764.

Surface: public API.

Signature: `def statement_census(segs)`

What it does: Segment every segment payload and tally the statement

(Where these words came from: direct.)

---

## function — bucket

Found in tools/phcc_analyze.py at line 788.

Surface: public API.

Signature: `def bucket(ln)`

What it does: Function bucket takes ln. Its return type is unannotated; the code shows it returning f"≤{cap}"; ">1024".

(Where these words came from: inferred.)

---

## function — render_stmt_report

Found in tools/phcc_analyze.py at line 870.

Surface: public API.

Signature: `def render_stmt_report(phext_path, raw, segs, sc, build_info=None)`

What it does: Function render_stmt_report takes phext_path, raw, segs, sc, build_info=None. Its return type is unannotated; the code shows it returning "\n".join(L) + "\n".

(Where these words came from: inferred.)

---

## function — render_stmt_build

Found in tools/phcc_analyze.py at line 967.

Surface: public API.

Signature: `def render_stmt_build(bi)`

What it does: Function render_stmt_build takes bi. Its return type is unannotated; the code shows it returning L.

(Where these words came from: inferred.)

---

## function — assign_stmt_coords

Found in tools/phcc_analyze.py at line 997.

Surface: public API.

Signature: `def assign_stmt_coords(cores)`

What it does: Content-derived coordinates for the statement library.

(Where these words came from: direct.)

---

## function — build_stmt

Found in tools/phcc_analyze.py at line 1045.

Surface: public API.

Signature: `def build_stmt(scrolls, segs, sc, src_sha)`

What it does: Rewrite every segment payload as a statement reference

(Where these words came from: direct.)

---

## function — verify_stmt

Found in tools/phcc_analyze.py at line 1096.

Surface: public API.

Signature: `def verify_stmt(scrolls, entries)`

What it does: Verify a statement-level phext against its own records:

(Where these words came from: direct.)

---

## function — run

Found in tools/phcc_analyze.py at line 1176.

Surface: public API.

Signature: `def run(a)`

What it does: Function run takes a. Its return type is unannotated; the code shows it returning 2; 0 if ok else 1; 1; 0.

(Where these words came from: inferred.)

---

## function — cli

Found in tools/phcc_analyze.py at line 1289.

Surface: public API.

Signature: `def cli(argv)`

What it does: Function cli takes argv. Its return type is unannotated; the code shows it returning run(ap.parse_args(argv)).

(Where these words came from: inferred.)

---

## function — sha256

Found in tools/phcc_builds.py at line 53.

Surface: public API.

Signature: `def sha256(b)`

What it does: Function sha256 takes b. Its return type is unannotated; the code shows it returning hashlib.sha256(b).hexdigest().

(Where these words came from: inferred.)

---

## function — is_region

Found in tools/phcc_builds.py at line 57.

Surface: public API.

Signature: `def is_region(coord)`

What it does: Function is_region takes coord. Its return type is unannotated; the code shows it returning coord[0] == REGION_LIBRARY.

(Where these words came from: inferred.)

---

## function — is_canonical_binary_scroll

Found in tools/phcc_builds.py at line 61.

Surface: public API.

Signature: `def is_canonical_binary_scroll(content)`

What it does: Function is_canonical_binary_scroll takes content. Its return type is unannotated; the code shows it returning content.startswith(CANONICAL_HEADER + b"\n").

(Where these words came from: inferred.)

---

## function — encode_canonical_binary_scroll

Found in tools/phcc_builds.py at line 65.

Surface: public API.

Signature: `def encode_canonical_binary_scroll(raw)`

What it does: Function encode_canonical_binary_scroll takes raw. Its return type is unannotated; the code shows it returning CANONICAL_HEADER + b"\n" + base64.b64encode(raw).

(Where these words came from: inferred.)

---

## function — decode_canonical_binary_scroll

Found in tools/phcc_builds.py at line 69.

Surface: public API.

Signature: `def decode_canonical_binary_scroll(content)`

What it does: Raw bytes of a canonical binary scroll, or raise ValueError.

(Where these words came from: direct.)

---

## function — parse_binary_scroll

Found in tools/phcc_builds.py at line 77.

Surface: public API.

Signature: `def parse_binary_scroll(content)`

What it does: (header dict, raw bytes) or raise ValueError.

(Where these words came from: direct.)

---

## function — parse_region_manifest

Found in tools/phcc_builds.py at line 110.

Surface: public API.

Signature: `def parse_region_manifest(content)`

What it does: {coord-string: entry dict} from the region root manifest.

(Where these words came from: direct.)

---

## function — cmd_region_list

Found in tools/phcc_builds.py at line 128.

Surface: public API.

Signature: `def cmd_region_list(a)`

What it does: Function cmd_region_list takes a. Its return type is unannotated; the code shows it returning 0.

(Where these words came from: inferred.)

---

## function — cmd_region_wipe

Found in tools/phcc_builds.py at line 170.

Surface: public API.

Signature: `def cmd_region_wipe(a)`

What it does: Function cmd_region_wipe takes a. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — cmd_seal

Found in tools/phcc_builds.py at line 184.

Surface: public API.

Signature: `def cmd_seal(a)`

What it does: Function cmd_seal takes a. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — cmd_unseal

Found in tools/phcc_builds.py at line 201.

Surface: public API.

Signature: `def cmd_unseal(a)`

What it does: Function cmd_unseal takes a. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — cli

Found in tools/phcc_builds.py at line 215.

Surface: public API.

Signature: `def cli(argv)`

What it does: Function cli takes argv. Its return type is unannotated; the code shows it returning a.fn(a).

(Where these words came from: inferred.)

---

## function — analyse

Found in tools/phcc_gate.py at line 99.

Surface: public API.

Signature: `def analyse(scrolls)`

What it does: Function analyse takes scrolls. Its return type is unannotated; the code shows it returning artifacts, per_book, gaps, t1_eligible, populate.

(Where these words came from: inferred.)

---

## function — pct

Found in tools/phcc_gate.py at line 137.

Surface: public API.

Signature: `def pct(n, d)`

What it does: Function pct takes n, d. Its return type is unannotated; the code shows it returning (100.0 * n / d) if d else 0.0.

(Where these words came from: inferred.)

---

## function — tables_text

Found in tools/phcc_gate.py at line 141.

Surface: public API.

Signature: `def tables_text(per_book, artifacts, gaps, t1_eligible, receipted=None, proof_notes=None, design=None, machine=None, machine2=None, machine3=None, machine4=None, assembly=None, conformance=None)`

What it does: Function tables_text takes per_book, artifacts, gaps, t1_eligible, receipted=None, proof_notes=None, design=None, machine=None, machine2=None, machine3=None, machine4=None, assembly=None, conformance=None. Its return type is unannotated; the code shows it returning "\n".join(lines).

(Where these words came from: inferred.)

---

## function — verdict_text

Found in tools/phcc_gate.py at line 215.

Surface: public API.

Signature: `def verdict_text(scrolls, artifacts, per_book, gaps, t1_eligible, receipted=None, proof_notes=None, design=None, machine=None, machine2=None, machine3=None, machine4=None, assembly=None, conformance=None)`

What it does: Function verdict_text takes scrolls, artifacts, per_book, gaps, t1_eligible, receipted=None, proof_notes=None, design=None, machine=None, machine2=None, machine3=None, machine4=None, assembly=None, conformance=None. Its return type is unannotated; the code shows it returning ("\n".join(lines) + "\n").encode().

(Where these words came from: inferred.)

---

## function — parse_proof_claims

Found in tools/phcc_gate.py at line 267.

Surface: public API.

Signature: `def parse_proof_claims(scrolls)`

What it does: Return {source_coord: claim-dict} for artifacts with Proofs scrolls.

(Where these words came from: direct.)

---

## function — tiers_from_receipts

Found in tools/phcc_gate.py at line 289.

Surface: public API.

Signature: `def tiers_from_receipts(claims)`

What it does: {source_coord: (tier, note)} — the receipted tier overlay.

(Where these words came from: direct.)

---

## function — parse_design_views

Found in tools/phcc_gate.py at line 352.

Surface: public API.

Signature: `def parse_design_views(scrolls)`

What it does: Return {'checked': n, 'current': [path], 'stale': [(path,

(Where these words came from: direct.)

---

## function — machine_section

Found in tools/phcc_gate.py at line 410.

Surface: public API.

Signature: `def machine_section(scrolls, phext_path)`

What it does: Function machine_section takes scrolls, phext_path. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — machine2_section

Found in tools/phcc_gate.py at line 483.

Surface: public API.

Signature: `def machine2_section(scrolls, phext_path)`

What it does: Function machine2_section takes scrolls, phext_path. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — machine3_section

Found in tools/phcc_gate.py at line 564.

Surface: public API.

Signature: `def machine3_section(scrolls, phext_path)`

What it does: Function machine3_section takes scrolls, phext_path. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — machine4_section

Found in tools/phcc_gate.py at line 641.

Surface: public API.

Signature: `def machine4_section(scrolls, phext_path)`

What it does: Function machine4_section takes scrolls, phext_path. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — decode_binary_scroll

Found in tools/phcc_gate.py at line 748.

Surface: public API.

Signature: `def decode_binary_scroll(content)`

What it does: Raw bytes of a binary scroll, or None if undecodable.

(Where these words came from: direct.)

---

## function — assembly_section

Found in tools/phcc_gate.py at line 774.

Surface: public API.

Signature: `def assembly_section(scrolls)`

What it does: Function assembly_section takes scrolls. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — conformance_section

Found in tools/phcc_gate.py at line 885.

Surface: public API.

Signature: `def conformance_section(scrolls)`

What it does: Function conformance_section takes scrolls. Its return type is unannotated; the code shows it returning lines.

(Where these words came from: inferred.)

---

## function — main

Found in tools/phcc_gate.py at line 1064.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — nm_symbols

Found in tools/phcc_harvest.py at line 20.

Surface: public API.

Signature: `def nm_symbols(path)`

What it does: Return {object_name: (defined_global, defined_local, undefined)}.

(Where these words came from: direct.)

---

## function — main

Found in tools/phcc_harvest.py at line 52.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load_scrolls

Found in tools/phcc_migrate.py at line 78.

Surface: public API.

Signature: `def load_scrolls(path)`

What it does: Function load_scrolls takes path. Its return type is unannotated; the code shows it returning dict(walk_bytes(open(path, "rb").read())).

(Where these words came from: inferred.)

---

## function — manifest_entries

Found in tools/phcc_migrate.py at line 82.

Surface: public API.

Signature: `def manifest_entries(scrolls)`

What it does: Function manifest_entries takes scrolls. Its return type is unannotated; the code shows it returning entries.

(Where these words came from: inferred.)

---

## function — write_file

Found in tools/phcc_migrate.py at line 96.

Surface: public API.

Signature: `def write_file(out, path, data, mode)`

What it does: Function write_file takes out, path, data, mode. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — resolve_stub

Found in tools/phcc_migrate.py at line 107.

Surface: public API.

Signature: `def resolve_stub(scrolls, data, segment, stats)`

What it does: Expand a dedup stub (phext_io dedup refs, written by

(Where these words came from: direct.)

---

## function — resolve_stmt_seq

Found in tools/phcc_migrate.py at line 147.

Surface: public API.

Signature: `def resolve_stmt_seq(scrolls, data, stats=None)`

What it does: Expand a statement reference sequence (phext_io statement

(Where these words came from: direct.)

---

## function — unpack_vol2

Found in tools/phcc_migrate.py at line 185.

Surface: public API.

Signature: `def unpack_vol2(scrolls, entries, stats=None)`

What it does: Yield (path, data, mode, ok, why).

(Where these words came from: direct.)

---

## function — unpack_vol3

Found in tools/phcc_migrate.py at line 208.

Surface: public API.

Signature: `def unpack_vol3(scrolls, entries, stats=None)`

What it does: Reassemble from Volume 3 segment scrolls where a file index

(Where these words came from: direct.)

---

## function — cmd_export

Found in tools/phcc_migrate.py at line 286.

Surface: public API.

Signature: `def cmd_export(a)`

What it does: Function cmd_export takes a. Its return type is unannotated; the code shows it returning 2; 1; 0.

(Where these words came from: inferred.)

---

## function — place

Found in tools/phcc_migrate.py at line 335.

Surface: public API.

Signature: `def place(path)`

What it does: Return (book, chapter) for a repo-relative path, or None.

(Where these words came from: direct.)

---

## function — place_full

Found in tools/phcc_migrate.py at line 387.

Surface: public API.

Signature: `def place_full(path)`

What it does: Return (collection, volume, book, chapter) for a path, or None.

(Where these words came from: direct.)

---

## function — phext_dir_exclusion

Found in tools/phcc_migrate.py at line 418.

Surface: public API.

Signature: `def phext_dir_exclusion(path)`

What it does: Why a phext/** path is excluded from pack placement (the

(Where these words came from: direct.)

---

## function — enumerate_tree

Found in tools/phcc_migrate.py at line 499.

Surface: public API.

Signature: `def enumerate_tree(tree)`

What it does: [(mode, path)] for every packable file under tree, path-sorted.

(Where these words came from: direct.)

---

## function — stage_a

Found in tools/phcc_migrate.py at line 528.

Surface: public API.

Signature: `def stage_a(tree, base_phext)`

What it does: Build the Volume 1 + Volume 2 scroll set. Returns dict.

(Where these words came from: direct.)

---

## function — find_tool

Found in tools/phcc_migrate.py at line 597.

Surface: public API.

Signature: `def find_tool(name)`

What it does: Function find_tool takes name. Its return type is unannotated; the code shows it returning p; None.

(Where these words came from: inferred.)

---

## function — find_ts_python

Found in tools/phcc_migrate.py at line 609.

Surface: public API.

Signature: `def find_ts_python(explicit)`

What it does: Function find_ts_python takes explicit. Its return type is unannotated; the code shows it returning c.

(Where these words came from: inferred.)

---

## function — cmd_import

Found in tools/phcc_migrate.py at line 627.

Surface: public API.

Signature: `def cmd_import(a)`

What it does: Function cmd_import takes a. Its return type is unannotated; the code shows it returning 0; 1.

(Where these words came from: inferred.)

---

## function — cli

Found in tools/phcc_migrate.py at line 668.

Surface: public API.

Signature: `def cli(argv)`

What it does: Function cli takes argv. Its return type is unannotated; the code shows it returning a.fn(a).

(Where these words came from: inferred.)

---

## function — Member.__init__

Found in tools/phcc_prune.py at line 156.

Surface: public API.

Signature: `def __init__(self, path, scr, kind, names, form, span, psha, payload, seg_coord)`

What it does: Function Member.__init__ takes self, path, scr, kind, names, form, span, psha, payload, seg_coord. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — Member.mid

Found in tools/phcc_prune.py at line 163.

Surface: public API.

Signature: `def mid(self)`

What it does: Function Member.mid takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning (self.path, self.scr).

(Where these words came from: inferred.)

---

## function — CarriedFile.__init__

Found in tools/phcc_prune.py at line 171.

Surface: public API.

Signature: `def __init__(self, entry)`

What it does: Function CarriedFile.__init__ takes self, entry. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — load_carried

Found in tools/phcc_prune.py at line 181.

Surface: public API.

Signature: `def load_carried(phext_path)`

What it does: Return (scrolls, [CarriedFile]) — main manifest + Sec-14 addendum.

(Where these words came from: direct.)

---

## function — Harvest.__init__

Found in tools/phcc_prune.py at line 241.

Surface: public API.

Signature: `def __init__(self, path)`

What it does: Function Harvest.__init__ takes self, path. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — include_closure

Found in tools/phcc_prune.py at line 300.

Surface: public API.

Signature: `def include_closure(files_by_path, seeds, build_prereqs=frozenset())`

What it does: #include closure over carried files, starting at seeds.

(Where these words came from: direct.)

---

## function — resolve

Found in tools/phcc_prune.py at line 320.

Surface: public API.

Signature: `def resolve(p, name)`

What it does: Function resolve takes p, name. Its return type is unannotated; the code shows it returning hit.

(Where these words came from: inferred.)

---

## function — is_build_ref_file

Found in tools/phcc_prune.py at line 358.

Surface: public API.

Signature: `def is_build_ref_file(path)`

What it does: Files whose text the build itself reads as instructions —

(Where these words came from: direct.)

---

## function — classify_files

Found in tools/phcc_prune.py at line 366.

Surface: public API.

Signature: `def classify_files(files, graph_files, extra_keep, keep_prefixes=KEEP_WHOLE_PREFIXES, runtime_prefix=LIB_PREFIX, keep_files=KEEP_WHOLE_FILES, build_prereqs=frozenset())`

What it does: path -> 'graph' | 'keep' | 'drop' for manifest files.

(Where these words came from: direct.)

---

## function — build_graph

Found in tools/phcc_prune.py at line 429.

Surface: public API.

Signature: `def build_graph(files, classes)`

What it does: Members of graph-class files; name index; raw payload store.

(Where these words came from: direct.)

---

## function — build_suffix_index

Found in tools/phcc_prune.py at line 460.

Surface: public API.

Signature: `def build_suffix_index(name2members)`

What it does: name-suffix -> [member ids]: for every member name, every

(Where these words came from: direct.)

---

## function — member_edges

Found in tools/phcc_prune.py at line 475.

Surface: public API.

Signature: `def member_edges(m, name2members, toks, providers, suffix_index)`

What it does: Out-edges of one member: every member bearing a name tokenised

(Where these words came from: direct.)

---

## function — build_providers

Found in tools/phcc_prune.py at line 504.

Surface: public API.

Signature: `def build_providers(members, name2members, toks)`

What it does: token -> [member ids] for referenced tokens no member is

(Where these words came from: direct.)

---

## function — prune

Found in tools/phcc_prune.py at line 526.

Surface: public API.

Signature: `def prune(phext_path, harvest_path, extra_roots, extra_keep, runtime_prefix=LIB_PREFIX, keep_prefixes=None, keep_files=None)`

What it does: Function prune takes phext_path, harvest_path, extra_roots, extra_keep, runtime_prefix=LIB_PREFIX, keep_prefixes=None, keep_files=None. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — add_root

Found in tools/phcc_prune.py at line 594.

Surface: public API.

Signature: `def add_root(mid, why)`

What it does: Function add_root takes mid, why. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — emit

Found in tools/phcc_prune.py at line 730.

Surface: public API.

Signature: `def emit(res, phext_path)`

What it does: Function emit takes res, phext_path. Its return type is unannotated; the code shows it returning out, stats.

(Where these words came from: inferred.)

---

## function — prune_manifest_text

Found in tools/phcc_prune.py at line 822.

Surface: public API.

Signature: `def prune_manifest_text(res, stats)`

What it does: Function prune_manifest_text takes res, stats. Its return type is unannotated; the code shows it returning ("\n".join(lines) + "\n").encode().

(Where these words came from: inferred.)

---

## function — cli

Found in tools/phcc_prune.py at line 873.

Surface: public API.

Signature: `def cli(argv)`

What it does: Function cli takes argv. Its return type is unannotated; the code shows it returning 1.

(Where these words came from: inferred.)

---

## function — source_bytes

Found in tools/phcc_prune.py at line 922.

Surface: public API.

Signature: `def source_bytes(scrolls)`

What it does: Function source_bytes takes scrolls. Its return type is unannotated; the code shows it returning v2, v3.

(Where these words came from: inferred.)

---

## function — fmt

Found in tools/phext_io.py at line 18.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning f"{coord[0]}.{coord[1]}.{coord[2]}/{coord[3]}.{c.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in tools/phext_io.py at line 22.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning tuple(parts).

(Where these words came from: inferred.)

---

## function — walk_bytes

Found in tools/phext_io.py at line 30.

Surface: public API.

Signature: `def walk_bytes(data)`

What it does: Dead-reckoning byte walk — same state machine as parse_phext.py,

(Where these words came from: direct.)

---

## function — implode

Found in tools/phext_io.py at line 52.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord_tuple -> bytes. Same emitter as Slice 0.

(Where these words came from: direct.)

---

## function — sha256

Found in tools/phext_io.py at line 68.

Surface: public API.

Signature: `def sha256(data)`

What it does: Function sha256 takes data. Its return type is unannotated; the code shows it returning hashlib.sha256(data).hexdigest().

(Where these words came from: inferred.)

---

## function — load_phext

Found in tools/phext_io.py at line 72.

Surface: public API.

Signature: `def load_phext(path)`

What it does: Function load_phext takes path. Its return type is unannotated; the code shows it returning dict(walk_bytes(f.read())).

(Where these words came from: inferred.)

---

## function — dedup_ref

Found in tools/phext_io.py at line 99.

Surface: public API.

Signature: `def dedup_ref(coord, payload_sha256)`

What it does: The stub bytes standing for the payload at `coord`.

(Where these words came from: direct.)

---

## function — parse_dedup_ref

Found in tools/phext_io.py at line 105.

Surface: public API.

Signature: `def parse_dedup_ref(data)`

What it does: (coord_tuple, sha256_hex) if `data` is exactly a dedup stub,

(Where these words came from: direct.)

---

## function — stmt_ref

Found in tools/phext_io.py at line 149.

Surface: public API.

Signature: `def stmt_ref(coord, core_sha256, indent)`

What it does: One reference line (bytes, newline-terminated).

(Where these words came from: direct.)

---

## function — stmt_ref_empty

Found in tools/phext_io.py at line 156.

Surface: public API.

Signature: `def stmt_ref_empty(indent)`

What it does: The reference line for a whitespace-only segment.

(Where these words came from: direct.)

---

## function — parse_stmt_seq

Found in tools/phext_io.py at line 162.

Surface: public API.

Signature: `def parse_stmt_seq(data)`

What it does: Parse a statement reference sequence payload. Returns a list

(Where these words came from: direct.)

---

## function — parse_manifest

Found in tools/phext_io.py at line 200.

Surface: public API.

Signature: `def parse_manifest(scrolls)`

What it does: Return {path: coord_tuple} from the Slice 0 placement manifest

(Where these words came from: direct.)

---

## function — segment_one

Found in tools/segment_libtcc1.py at line 33.

Surface: public API.

Signature: `def segment_one(path, data, v2coord_s)`

What it does: Function segment_one takes path, data, v2coord_s. Its return type is unannotated; the code shows it returning {"scrolls": scrolls, "map_lines": map_lines, "re.

(Where these words came from: inferred.)

---

## function — main

Found in tools/segment_libtcc1.py at line 145.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — check

Found in tools/test_analyze.py at line 38.

Surface: test.

Signature: `def check(cond, label)`

What it does: Function check takes cond, label. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — run

Found in tools/test_analyze.py at line 44.

Surface: test.

Signature: `def run(args, cwd)`

What it does: Function run takes args, cwd. Its return type is unannotated; the code shows it returning r.returncode, r.stdout + r.stderr.

(Where these words came from: inferred.)

---

## function — file_bytes

Found in tools/test_analyze.py at line 89.

Surface: test.

Signature: `def file_bytes(path)`

What it does: Function file_bytes takes path. Its return type is unannotated; the code shows it returning file_bytes("a.c"); b"".join(p for _, _, p in segs).

(Where these words came from: inferred.)

---

## function — build_fixture

Found in tools/test_analyze.py at line 96.

Surface: test.

Signature: `def build_fixture(corrupt_index=False)`

What it does: Function build_fixture takes corrupt_index=False. Its return type is unannotated; the code shows it returning phext_io.implode(scrolls).

(Where these words came from: inferred.)

---

## function — read_tree

Found in tools/test_analyze.py at line 149.

Surface: test.

Signature: `def read_tree(d)`

What it does: Function read_tree takes d. Its return type is unannotated; the code shows it returning {p: open(os.path.join(d, p), "rb").read() for p .

(Where these words came from: inferred.)

---

## function — main

Found in tools/test_analyze.py at line 153.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1; 0.

(Where these words came from: inferred.)

---

## function — check

Found in tools/test_prune.py at line 32.

Surface: test.

Signature: `def check(cond, label)`

What it does: Function check takes cond, label. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — seg_header

Found in tools/test_prune.py at line 38.

Surface: test.

Signature: `def seg_header(kind, name, form, span, psha)`

What it does: Function seg_header takes kind, name, form, span, psha. Its return type is unannotated; the code shows it returning (f"#phcc-member\tkind={kind}\tname={name}\tform=.

(Where these words came from: inferred.)

---

## function — structured_file

Found in tools/test_prune.py at line 43.

Surface: test.

Signature: `def structured_file(path, coord, rows)`

What it does: rows: [(kind, name, form, payload)] -> (vol2 bytes, scrolls dict)

(Where these words came from: direct.)

---

## function — build_fixture

Found in tools/test_prune.py at line 183.

Surface: test.

Signature: `def build_fixture(td)`

What it does: Function build_fixture takes td. Its return type is unannotated; the code shows it returning phext_path, hv_path.

(Where these words came from: inferred.)

---

## function — main

Found in tools/test_prune.py at line 211.

Surface: test.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; the code shows it returning 1 if FAILS else 0.

(Where these words came from: inferred.)

---

## function — check

Found in tools/test_stmt.py at line 26.

Surface: test.

Signature: `def check(name, cond, detail="")`

What it does: Function check takes name, cond, detail="". Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — cores

Found in tools/test_stmt.py at line 35.

Surface: test.

Signature: `def cores(data)`

What it does: Function cores takes data. Its return type is unannotated; the code shows it returning [(core, term) for _ind, core, term in parts].

(Where these words came from: inferred.)

---

## function — seg_scroll

Found in tools/test_stmt.py at line 151.

Surface: test.

Signature: `def seg_scroll(kind, name, form, span, payload)`

What it does: Function seg_scroll takes kind, name, form, span, payload. Its return type is unannotated; the code shows it returning hdr.encode() + b"\n" + payload.

(Where these words came from: inferred.)

---

## function — fmt

Found in tools/upstream_carry.py at line 81.

Surface: public API.

Signature: `def fmt(coord)`

What it does: Function fmt takes coord. Its return type is unannotated; the code shows it returning f"{coord[0]}.{coord[1]}.{coord[2]}/{coord[3]}.{c.

(Where these words came from: inferred.)

---

## function — parse_coord

Found in tools/upstream_carry.py at line 85.

Surface: public API.

Signature: `def parse_coord(s)`

What it does: Function parse_coord takes s. Its return type is unannotated; the code shows it returning tuple(parts).

(Where these words came from: inferred.)

---

## function — walk_bytes

Found in tools/upstream_carry.py at line 93.

Surface: public API.

Signature: `def walk_bytes(data)`

What it does: Dead-reckoning byte walk — yields (coord_tuple, bytes) so

(Where these words came from: direct.)

---

## function — implode

Found in tools/upstream_carry.py at line 112.

Surface: public API.

Signature: `def implode(scrolls)`

What it does: scrolls: dict coord_tuple -> bytes. The lineage emitter.

(Where these words came from: direct.)

---

## function — sha256

Found in tools/upstream_carry.py at line 128.

Surface: public API.

Signature: `def sha256(data)`

What it does: Function sha256 takes data. Its return type is unannotated; the code shows it returning hashlib.sha256(data).hexdigest().

(Where these words came from: inferred.)

---

## function — encode_scroll

Found in tools/upstream_carry.py at line 132.

Surface: public API.

Signature: `def encode_scroll(data)`

What it does: Return (scroll_bytes, encoding) for one file's bytes.

(Where these words came from: direct.)

---

## function — decode_scroll

Found in tools/upstream_carry.py at line 150.

Surface: public API.

Signature: `def decode_scroll(scroll, encoding)`

What it does: Function decode_scroll takes scroll, encoding. Its return type is unannotated; the code shows it returning base64.b64decode(scroll[len(BINARY_HEADER) + 1:]; scroll.

(Where these words came from: inferred.)

---

## function — git

Found in tools/upstream_carry.py at line 196.

Surface: public API.

Signature: `def git(repo, *args)`

What it does: Function git takes repo, *args. Its return type is unannotated; the code shows it returning subprocess.run(["git", "-C", repo] + list(args),.

(Where these words came from: inferred.)

---

## function — placement

Found in tools/upstream_carry.py at line 201.

Surface: public API.

Signature: `def placement(entries)`

What it does: entries: list of paths (files only). Returns {path: coord}.

(Where these words came from: direct.)

---

## function — cmd_build

Found in tools/upstream_carry.py at line 229.

Surface: public API.

Signature: `def cmd_build(a)`

What it does: Function cmd_build takes a. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — cmd_unpack

Found in tools/upstream_carry.py at line 335.

Surface: public API.

Signature: `def cmd_unpack(a)`

What it does: Function cmd_unpack takes a. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — main

Found in tools/upstream_carry.py at line 365.

Surface: public API.

Signature: `def main()`

What it does: Function main takes no inputs beyond the value it is called on. Its return type is unannotated; no return statement was read in its body.

(Where these words came from: inferred.)

---

## function — func

Found in tools/verify_gate_splice_81.py at line 23.

Surface: public API.

Signature: `def func(text, name)`

What it does: Function func takes text, name. Its return type is unannotated; the code shows it returning text[i:end].rstrip() + "\n".

(Where these words came from: inferred.)

---

## Provenance

This documentation is produced for the export repo wbic16/phdoc (its chain of record lives at docs/phcc/chain.phext) and for the doc site https://docs.phext.io. Both targets are the project's own configuration, held in its .phdoc.json as "export-repo" and "doc-site"; when a project does not name them, the defaults are wbic16/phdoc and https://docs.phext.io.
