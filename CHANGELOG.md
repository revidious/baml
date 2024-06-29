# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## [0.44.0](https://github.com/boundaryml/baml/compare/0.43.0..0.44.0) - 2024-06-26

### Bug Fixes
- Fix typebuilder for random enums (#721)

## [0.43.0](https://github.com/boundaryml/baml/compare/0.42.0..0.43.0) - 2024-06-26

### Bug Fixes
- fix pnpm lockfile issue (#720)

## [0.42.0](https://github.com/boundaryml/baml/compare/0.41.0..0.42.0) - 2024-06-26

### Bug Fixes

- correctly propagate LICENSE to baml-py (#695) - ([3fda880](https://github.com/boundaryml/baml/commit/3fda880bf39b32191b425ae75e8b491d10884cf6)) - Samuel Lijin

### Miscellaneous Chores

- update jsonish readme (#685) - ([b19f04a](https://github.com/boundaryml/baml/commit/b19f04a059ba18d54544cb278b6990b95170d3f3)) - Samuel Lijin

### Vscode

- add link to tracing, show token counts (#703) - ([64aa18a](https://github.com/boundaryml/baml/commit/64aa18a9cc34071655141c8f6e2ad04ac90e7be1)) - Samuel Lijin

## [0.41.0] - 2024-06-20

### Bug Fixes

- rollback git lfs, images broken in docs rn (#534) - ([6945506](https://github.com/boundaryml/baml/commit/694550664fa45b5f76987e2663c9d7e7a9a6a2d2)) - Samuel Lijin
- search for markdown blocks correctly (#641) - ([6b8abf1](https://github.com/boundaryml/baml/commit/6b8abf1ccf55bbe7c3bc1046c78081126e01f134)) - Samuel Lijin
- restore one-workspace-per-folder (#656) - ([a464bde](https://github.com/boundaryml/baml/commit/a464bde566199ace45285a78a7f542cd7217fb65)) - Samuel Lijin
- ruby generator should be ruby/sorbet (#661) - ([0019f39](https://github.com/boundaryml/baml/commit/0019f3951b8fe2b49e62eb11d869516b8088e9cb)) - Samuel Lijin
- ruby compile error snuck in (#663) - ([0cb2583](https://github.com/boundaryml/baml/commit/0cb25831788eb8b3eb0a38383917f6d1ffb5633a)) - Samuel Lijin

### Documentation

- add typescript examples (#477) - ([532481c](https://github.com/boundaryml/baml/commit/532481c3df4063b37a8834a5fe2bbce3bb37d2f5)) - Samuel Lijin
- add titles to code blocks for all CodeGroup elems (#483) - ([76c6b68](https://github.com/boundaryml/baml/commit/76c6b68b27ee37972fa226be0b4dfe31f7b4b5ec)) - Samuel Lijin
- add docs for round-robin clients (#500) - ([221f902](https://github.com/boundaryml/baml/commit/221f9020d850e6d24fe2fd8a684081726a0659af)) - Samuel Lijin
- add ruby example (#689) - ([16e187f](https://github.com/boundaryml/baml/commit/16e187f6698a1cc86a37eedf2447648d810370ad)) - Samuel Lijin

### Features

- implement `baml version --check --output json` (#444) - ([5f076ac](https://github.com/boundaryml/baml/commit/5f076ace1f92dc2141b231c9e62f4dc23f7fef18)) - Samuel Lijin
- show update prompts in vscode (#451) - ([b66da3e](https://github.com/boundaryml/baml/commit/b66da3ee355fcd6a8677d834ecb05af44cbf8f20)) - Samuel Lijin
- add tests to check that baml version --check works (#454) - ([be1499d](https://github.com/boundaryml/baml/commit/be1499dfa82ff8ab923a16d45290758120d95015)) - Samuel Lijin
- parse typescript versions in version --check (#473) - ([b4b2250](https://github.com/boundaryml/baml/commit/b4b2250c37b900db899256159bbfc3aa2ec819cb)) - Samuel Lijin
- implement round robin client strategies (#494) - ([599fcdd](https://github.com/boundaryml/baml/commit/599fcdd2a45c5b1e935f36769784ca944566b88c)) - Samuel Lijin
- add integ-tests support to build (#542) - ([f59cf2e](https://github.com/boundaryml/baml/commit/f59cf2e1a9ec7edbe174f4bc7ff9391f2cff3208)) - Samuel Lijin
- make ruby work again (#650) - ([6472bec](https://github.com/boundaryml/baml/commit/6472bec231b581076ee7edefaab2e7979b2bf336)) - Samuel Lijin
- Add RB2B tracking script (#682) - ([54547a3](https://github.com/boundaryml/baml/commit/54547a34d40cd40a43767919dbc9faa68a82faea)) - hellovai

### Miscellaneous Chores

- add nodemon config to typescript/ (#435) - ([231b396](https://github.com/boundaryml/baml/commit/231b3967bc947c4651156bc55fd66552782824c9)) - Samuel Lijin
- finish gloo to BoundaryML renames (#452) - ([88a7fda](https://github.com/boundaryml/baml/commit/88a7fdacc826e78ef21c6b24745ee469d9d02e6a)) - Samuel Lijin
- set up lfs (#511) - ([3a43143](https://github.com/boundaryml/baml/commit/3a431431e8e38dfc68763f15ccdcd1d131f23984)) - Samuel Lijin
- add internal build tooling for sam (#512) - ([9ebacca](https://github.com/boundaryml/baml/commit/9ebaccaa542760cb96382ae2a91d780f1ade613b)) - Samuel Lijin
- delete clients dir, this is now dead code (#652) - ([ec2627f](https://github.com/boundaryml/baml/commit/ec2627f59c7fe9edfff46fcdb65f9b9f0e2e072c)) - Samuel Lijin
- consolidate vscode workspace, bump a bunch of deps (#654) - ([82bf6ab](https://github.com/boundaryml/baml/commit/82bf6ab1ad839f84782a7ef0441f21124c368757)) - Samuel Lijin
- Add RB2B tracking script to propmt fiddle (#681) - ([4cf806b](https://github.com/boundaryml/baml/commit/4cf806bba26563fd8b6ddbd68296ab8bdfac21c4)) - hellovai
- Adding better release script (#688) - ([5bec282](https://github.com/boundaryml/baml/commit/5bec282d39d2250b39ef4aba5d6bba9830a35988)) - hellovai

### [AUTO

- patch] Version bump for nightly release [NIGHTLY:cli] [NIGHTLY:vscode_ext] [NIGHTLY:client-python] - ([d05a22c](https://github.com/boundaryml/baml/commit/d05a22ca4135887738adbce638193d71abca42ec)) - GitHub Action

### Build

- fix baml-core-ffi script (#521) - ([b1b7f4a](https://github.com/boundaryml/baml/commit/b1b7f4af0991ef6453f888f27930f3faaae337f5)) - Samuel Lijin
- fix engine/ (#522) - ([154f646](https://github.com/boundaryml/baml/commit/154f6468ec0aa6de1b033ee1cbc76e60acc363ea)) - Samuel Lijin

### Integ-tests

- add ruby test - ([c0bc101](https://github.com/boundaryml/baml/commit/c0bc10126ea32d099f1398f2c5faa08b111554ba)) - Sam Lijin

### Readme

- add function calling, collapse the table (#505) - ([2f9024c](https://github.com/boundaryml/baml/commit/2f9024c28ba438267de37ac43c6570a2f0398b5a)) - Samuel Lijin

### Release

- bump versions for everything (#662) - ([c0254ae](https://github.com/boundaryml/baml/commit/c0254ae680365854c51c7a4e58ea68d1901ea033)) - Samuel Lijin

### Vscode

- check for updates on the hour (#434) - ([c70a3b3](https://github.com/boundaryml/baml/commit/c70a3b373cb2346a0df9a1eba0ebacb74d59b53e)) - Samuel Lijin

<!-- generated by git-cliff -->