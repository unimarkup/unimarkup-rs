# Changelog

## [0.3.0](https://www.github.com/Unimarkup/unimarkup-rs/compare/v0.2.0...v0.3.0) (2022-01-14)


### Features

* add api for creating pest error with span in UmError ([47b55e6](https://www.github.com/Unimarkup/unimarkup-rs/commit/47b55e6967254c5f8f3170883ac93244476e1ee1))
* add attributes to blocks ([91ec175](https://www.github.com/Unimarkup/unimarkup-rs/commit/91ec1750a3a11dd9c55589f7c419fa25d1e2b661))
* add basic attributes support to existing blocks ([7c6c248](https://www.github.com/Unimarkup/unimarkup-rs/commit/7c6c248be5fb79831f94f232c4049e07173d6587))
* add function for id generation ([259e9dd](https://www.github.com/Unimarkup/unimarkup-rs/commit/259e9dd26acbee09d37a32e2e0a4fff83bf46d15))
* add parsing, validation and merge with CLI for preamble ([08bcd6c](https://www.github.com/Unimarkup/unimarkup-rs/commit/08bcd6c956d58112c7b4d87f729047c5254f8195))
* add validation of configs ([7dea7c5](https://www.github.com/Unimarkup/unimarkup-rs/commit/7dea7c53de707df50169035d4ffcd390a72d8f64))
* add validation of configs ([ade82b0](https://www.github.com/Unimarkup/unimarkup-rs/commit/ade82b01209fcbb8d61821087a6bd827e7926479))
* added testing to preamble ([883f61b](https://www.github.com/Unimarkup/unimarkup-rs/commit/883f61bc5e6c5a56876bd70e65298442048b5201))
* **backend:** implement ParseFromIr for VerbatimBlock ([a100ae4](https://www.github.com/Unimarkup/unimarkup-rs/commit/a100ae4b8436bb1218832fb1ba6fd9015bbd6bbf))
* **backend:** implement Render for VerbatimBlock ([fac7605](https://www.github.com/Unimarkup/unimarkup-rs/commit/fac7605551571edbc17acb8eaaec31ace0c46038))
* **backend:** load Verbatim blocks from IR ([2852533](https://www.github.com/Unimarkup/unimarkup-rs/commit/2852533877d2fef1e7f5a8113432f89cbbd00182))
* checks paragraph content for inline-formatting ([9207fd1](https://www.github.com/Unimarkup/unimarkup-rs/commit/9207fd15259db48e3a547916c3b3fa1e140a09ca))
* **frontend:** add attributes grammar for headings ([f680542](https://www.github.com/Unimarkup/unimarkup-rs/commit/f680542326f48ca2971136aff51084b579a5e5de))
* **frontend:** add attributes grammar for paragraph ([c49d96f](https://www.github.com/Unimarkup/unimarkup-rs/commit/c49d96f73e031ce324c9a4e00b4ddfad845d1098))
* **frontend:** add attributes to verbatim in grammar ([d7dd8d1](https://www.github.com/Unimarkup/unimarkup-rs/commit/d7dd8d1c8f65711aec240a4be3d9a3325d1091db))
* **frontend:** add function for id generation ([1f4d7fb](https://www.github.com/Unimarkup/unimarkup-rs/commit/1f4d7fb5dfd879071b2b882372da21cf07e380d7))
* **frontend:** impl attributes parsing for paragraph ([d2dd1cd](https://www.github.com/Unimarkup/unimarkup-rs/commit/d2dd1cd976bd9cb9fee71482bdf1d6179ac90340))
* **frontend:** implement UmParse for verbatim block ([d19664d](https://www.github.com/Unimarkup/unimarkup-rs/commit/d19664d071f2e9a972c5e9f9e3972b9bf9288859))
* **frontend:** introduce attributes rule in grammar ([5f94226](https://www.github.com/Unimarkup/unimarkup-rs/commit/5f9422609350bfcb7874208aec3bd48b8deda169))
* **frontend:** parse attributes for verbatim block ([c8bf790](https://www.github.com/Unimarkup/unimarkup-rs/commit/c8bf79028b33518d97a958aeaa712fcf9f800a00))
* **frontend:** parse attributes on heading blocks ([7579502](https://www.github.com/Unimarkup/unimarkup-rs/commit/7579502db304c1b11687fd3ed174705228126aaf))
* **frontend:** setup parser for enclosed blocks ([9a7afe9](https://www.github.com/Unimarkup/unimarkup-rs/commit/9a7afe9f8271ccf199d361719da6d085ee4b19aa))
* implement AsIrLines for VerbatimBlock ([f274017](https://www.github.com/Unimarkup/unimarkup-rs/commit/f274017535f1a826ccdcad629064a2a9578c0901))
* implement render for html of unimarkup blocks ([144defb](https://www.github.com/Unimarkup/unimarkup-rs/commit/144defb20647c1bc311ff8bddaaefe60183eef5b))
* inline formatting in grammar ([8fac206](https://www.github.com/Unimarkup/unimarkup-rs/commit/8fac2069336b2364cb4c1c4dceeb2239c9b49d4a))
* introduce grammar for verbatim block ([5c516a8](https://www.github.com/Unimarkup/unimarkup-rs/commit/5c516a88f60304fdf09e7137312e4f0cf454f691))
* introduce VerbatimBlock ([4100f15](https://www.github.com/Unimarkup/unimarkup-rs/commit/4100f15a6bb18c5f92c2ca69cc8bd47bb4745996))
* parses inline formatting and saves in a vector ([58b2123](https://www.github.com/Unimarkup/unimarkup-rs/commit/58b2123a8d0731e052220125b6deba1a515238a5))
* parsing preamble at the beginning of file and add to cli configs ([a0d51f5](https://www.github.com/Unimarkup/unimarkup-rs/commit/a0d51f5f51f80c4e4f54dc09b04400197b505b5c))
* split umblock_tests into more files ([5c0404f](https://www.github.com/Unimarkup/unimarkup-rs/commit/5c0404ff03ae59132f568771c6196caa1259e375))
* write root file into metadata table ([ad73663](https://www.github.com/Unimarkup/unimarkup-rs/commit/ad73663b60f9cc5ac2465741be5bf91d8f657fd9))


### Bug Fixes

* add content to all_syntax.um ([7724081](https://www.github.com/Unimarkup/unimarkup-rs/commit/7724081b5209fa810c579a42b8ceac227a98cb59))
* add documentation to public function ([398895f](https://www.github.com/Unimarkup/unimarkup-rs/commit/398895fbc73375887bc04808e4dacc085d93ad98))
* add suggestion from comment ([68450de](https://www.github.com/Unimarkup/unimarkup-rs/commit/68450de84af11d26af5256cafa2cc39a7ce1aed7))
* added suggestions form PR, ([f53f933](https://www.github.com/Unimarkup/unimarkup-rs/commit/f53f93306119d4d9f5606048f044e6b32e384509))
* added suggestions from PR ([c31ecee](https://www.github.com/Unimarkup/unimarkup-rs/commit/c31eceeb97f20c4261b13e2e3f63697868dbd8a5))
* cargo fmt ([3d0a379](https://www.github.com/Unimarkup/unimarkup-rs/commit/3d0a37965e363d14a38e4acc0d4b8b559325ed41))
* cargo format ([5a7e168](https://www.github.com/Unimarkup/unimarkup-rs/commit/5a7e168fc74c203388f8bc5bd7f10ca27d93d84e))
* change config of test_valid_config() ([fd05740](https://www.github.com/Unimarkup/unimarkup-rs/commit/fd05740e58739b403b5b80515e1099c5c686dca5))
* change if statement to let Some, ([5efc48e](https://www.github.com/Unimarkup/unimarkup-rs/commit/5efc48ec310ac8e87d4035fc9ebf84c6772e8edd))
* change ownership of config because of merge conflict ([faaa228](https://www.github.com/Unimarkup/unimarkup-rs/commit/faaa2285f591693358543dd9a9fae72f229e7eb1))
* changing umblock_tests because of changes in parser ([4eaf3b3](https://www.github.com/Unimarkup/unimarkup-rs/commit/4eaf3b371d8a0ae084380ac934dff02b92170080))
* correct verbatim block rendering ([37757dd](https://www.github.com/Unimarkup/unimarkup-rs/commit/37757ddf670e37edfe07a3a87df344f9a483bae6))
* fix logger when using installed unimarkup-rs ([cea675d](https://www.github.com/Unimarkup/unimarkup-rs/commit/cea675d3dd5506e38ad1d06bc33bbdcefec92b7e))
* fix simple_logger causing panic when using installed version of unimarkup-rs ([efdd2dd](https://www.github.com/Unimarkup/unimarkup-rs/commit/efdd2dd437383f42bcc4c829bfddcadab9098f00))
* **frontend:** change verbatim grammar to reduce false-positives ([af47f00](https://www.github.com/Unimarkup/unimarkup-rs/commit/af47f00f7713ce55ae263d254f13e41718f0ad09))
* **frontend:** convert implicit heading id to lowercase ([c191c62](https://www.github.com/Unimarkup/unimarkup-rs/commit/c191c62d30db6db51469a691c4e1df21616bd544))
* **frontend:** fix headings grammar to get correct line numbers ([669462a](https://www.github.com/Unimarkup/unimarkup-rs/commit/669462a3206cb11e858d2b9591eac1202510354d))
* **frontend:** handle blank_line rule when parsing ([3c0ac08](https://www.github.com/Unimarkup/unimarkup-rs/commit/3c0ac084cba485c719633f2f26ab2b6724e91268))
* **frontend:** handle verbatim_end rule in verbatim block ([64169e1](https://www.github.com/Unimarkup/unimarkup-rs/commit/64169e1d94f06534aae57bc37030c048b1ef9920))
* **frontend:** improve clarity of error message ([9c64da6](https://www.github.com/Unimarkup/unimarkup-rs/commit/9c64da624cb667ed37ca3c403123f627693212e5))
* **frontend:** return error on unknown rule in verbatim block ([6befb00](https://www.github.com/Unimarkup/unimarkup-rs/commit/6befb008679eb4bf24b8cbfca46659d327a84927))
* **frontend:** skip rule when `Rule::EOI encountered` ([418fc60](https://www.github.com/Unimarkup/unimarkup-rs/commit/418fc60c4971852b7049387d0a92c45abb975d32))
* **frontend:** use custom whitespace rules in grammar ([d7eee86](https://www.github.com/Unimarkup/unimarkup-rs/commit/d7eee862f805720f0c37395e68047dcb25ed7e13))
* **frontend:** warn on fallback use when parsing enclosed blocks ([1118202](https://www.github.com/Unimarkup/unimarkup-rs/commit/11182020969626c0a2644a1ce69c8648331266c0))
* handle expect messages ([55fce22](https://www.github.com/Unimarkup/unimarkup-rs/commit/55fce22e06d4305eff65c20a41268e5d3d898643))
* make code consistent ([f38c2e8](https://www.github.com/Unimarkup/unimarkup-rs/commit/f38c2e8a9df9773d79f602b5bdf0cd8bc28f0622))
* merge branch 'main' into preamble ([cd85eb5](https://www.github.com/Unimarkup/unimarkup-rs/commit/cd85eb56d0b4e701312a6869e14ab5365f19be4a))
* Merge branch 'main' into umblock-tests ([a4860b8](https://www.github.com/Unimarkup/unimarkup-rs/commit/a4860b87796ae8ba24df676f73859aa88031ce51))
* Merge branch 'main' of https://github.com/Unimarkup/unimarkup-rs into umblock-tests ([e2e6bd7](https://www.github.com/Unimarkup/unimarkup-rs/commit/e2e6bd7e1bf078cc612185164127eb55b7304594))
* merge main into inline and clean up merge conflicts ([8d66522](https://www.github.com/Unimarkup/unimarkup-rs/commit/8d66522df1637dd5e289abf5ad7093e77c4641d4))
* more organised split up of tests ([d7b1490](https://www.github.com/Unimarkup/unimarkup-rs/commit/d7b1490ad97a8698425a1335a0e1467ff7e5c3c1))
* pass error message from caller to custom pest error ([df2f28c](https://www.github.com/Unimarkup/unimarkup-rs/commit/df2f28c2bd8c899a26d897e85942dfb76b9ead45))
* remove unnecessary println ([7ed91b3](https://www.github.com/Unimarkup/unimarkup-rs/commit/7ed91b344d2fea70edde85d73896d0ccdd05ee7b))
* rename variable um_blocks to um_file ([098d0d5](https://www.github.com/Unimarkup/unimarkup-rs/commit/098d0d58c7d323537d2ae4d6282fb86731525032))
* rework AsIrLines and add Clone trait ([7034e2b](https://www.github.com/Unimarkup/unimarkup-rs/commit/7034e2bffe3298b0e21e8d7db269da9fc44fe2be))
* shorten filepath to string conversion ([9de190b](https://www.github.com/Unimarkup/unimarkup-rs/commit/9de190b330fbd9c95470add832cb0bc1082b3b17))
* split up tests into corresponding file ([8bbe839](https://www.github.com/Unimarkup/unimarkup-rs/commit/8bbe839b0060f889fce72fb9f69b4f2b14670761))
* update verbatim grammar to capture language attribute ([ad81085](https://www.github.com/Unimarkup/unimarkup-rs/commit/ad810851244b6af6512ebfb02939085c87c6a98c))
* use env_logger instead of simple_logger ([fa67333](https://www.github.com/Unimarkup/unimarkup-rs/commit/fa67333c84be78c7b664daf6c1afee4ced5c404f))


### Documentation

* add documentation to the crate ([57326b8](https://www.github.com/Unimarkup/unimarkup-rs/commit/57326b86bea617dc0f8f2c1e7f775d5437c816cb))
* add overall documentation for backend module ([b7e642e](https://www.github.com/Unimarkup/unimarkup-rs/commit/b7e642e7958972b473e15b72636b3a326ae7d426))
* **backend:** document backend module ([ed88b6a](https://www.github.com/Unimarkup/unimarkup-rs/commit/ed88b6adbcc90f031bdafbc9e4d11bc62f2b1333))
* **backend:** update wording ([2108800](https://www.github.com/Unimarkup/unimarkup-rs/commit/21088009eed28446492314956bc21ed834046604))
* document config module ([b120b8a](https://www.github.com/Unimarkup/unimarkup-rs/commit/b120b8ad791f22d28f79c4e270d622e1b7c74ec4))
* document the (current) unimarkup elements ([10dd452](https://www.github.com/Unimarkup/unimarkup-rs/commit/10dd4521f96f38f538a6775f3c8707622a3e0c56))
* document the `BackendError` ([f6d949b](https://www.github.com/Unimarkup/unimarkup-rs/commit/f6d949bdcf68bd75fa972bbc1b8529228ea15260))
* document the `config` module ([9d3e4b2](https://www.github.com/Unimarkup/unimarkup-rs/commit/9d3e4b244dfd130d9de07fe31392ec00845b6681))
* document the `frontend` module ([eb1ed07](https://www.github.com/Unimarkup/unimarkup-rs/commit/eb1ed0752bb5c097d22a4bb0cdc74dd0dbd86144))
* document the renderer module ([4cba48f](https://www.github.com/Unimarkup/unimarkup-rs/commit/4cba48f195ced053843e7ff69bd146a9bdef18e5))
* document the UmError ([defb7d6](https://www.github.com/Unimarkup/unimarkup-rs/commit/defb7d64b7c0311e1ea63b42197820d7afd7887d))
* document the unimarkup-rs compiler ([2126fb7](https://www.github.com/Unimarkup/unimarkup-rs/commit/2126fb70575d72e3936c42eaadc4e8f5fb328cfd))
* enforce documenting code ([1cfdba7](https://www.github.com/Unimarkup/unimarkup-rs/commit/1cfdba7e856fc779457ba27404afa38f9084b576))
* **fronted:** document the frontend module ([4f82e96](https://www.github.com/Unimarkup/unimarkup-rs/commit/4f82e96a32375e59d17c17c1dbc0d0c2f3e83ab1))
* **frontend:** update wording ([f88cac8](https://www.github.com/Unimarkup/unimarkup-rs/commit/f88cac88a3bf1dcb04be6a97815ae94783f90ca4))
* **middleend:** document the middleend module ([9330451](https://www.github.com/Unimarkup/unimarkup-rs/commit/9330451b7d8c620edc902350d76829f5b6eaeea0))
* **middleend:** update wording ([162881c](https://www.github.com/Unimarkup/unimarkup-rs/commit/162881c173bf2f8ffcedcb23668a271a9953a2ff))
* **um_elements:** update wording ([9030ad8](https://www.github.com/Unimarkup/unimarkup-rs/commit/9030ad8d0b9b77c711b4e721e9a631b8bcf5661b))
* **um_error:** update wording ([10e6068](https://www.github.com/Unimarkup/unimarkup-rs/commit/10e60686ef2fd578b455aed713c9d4398e00e753))
* update the README.md ([700a48f](https://www.github.com/Unimarkup/unimarkup-rs/commit/700a48f1d01c6baedcc4e7476f757624861a6d1f))
* updated wording in general files ([eb79e4f](https://www.github.com/Unimarkup/unimarkup-rs/commit/eb79e4f4f171d30048c8f0ccf60ebda7b244756b))


### Testing

* add render test for verbatim block ([bd9dba5](https://www.github.com/Unimarkup/unimarkup-rs/commit/bd9dba51c34298e2c1ff4c96423286e88418fb37))
* add test file for attributes ([76e4844](https://www.github.com/Unimarkup/unimarkup-rs/commit/76e484418d7ea88f56266b0a4add4ec34ce497dc))
* add testcase validating metadata IR entry ([f2aa566](https://www.github.com/Unimarkup/unimarkup-rs/commit/f2aa566d6195dd806512d637cf05cdc9c4d3679a))
* add unit tests for VerbatimBlock ([b9d0f03](https://www.github.com/Unimarkup/unimarkup-rs/commit/b9d0f0330a470d865afed1ab793568035475ef80))
* add verbatim blocks to attr.um test file ([6364e36](https://www.github.com/Unimarkup/unimarkup-rs/commit/6364e36bf524c4152dbf7ec1dc917d0789853939))
* **frontend:** add test case with bad verbatim syntax ([184acdf](https://www.github.com/Unimarkup/unimarkup-rs/commit/184acdfd44963186b5982af4d52cf89cb9b4467c))
* **frontend:** update `generate_id` tests according to new api ([653e733](https://www.github.com/Unimarkup/unimarkup-rs/commit/653e733bbbbbca4aec687a71749c8747e8815593))
* split up UmBlock tests into individual test cases ([133fc1b](https://www.github.com/Unimarkup/unimarkup-rs/commit/133fc1b7d213235909e5e38444aa37c0c919731f))
* use new parser api in tests ([2bcf8fa](https://www.github.com/Unimarkup/unimarkup-rs/commit/2bcf8fa7837699e9877fb3f92ef5e896650f6315))

## [0.2.0](https://www.github.com/Unimarkup/unimarkup-rs/compare/v0.1.0...v0.2.0) (2021-12-17)


### Features

* add cli -> config conversion ([020b472](https://www.github.com/Unimarkup/unimarkup-rs/commit/020b4723d3986c03a2738f0208cea59df90e1067))
* add cli draft ([590fa6f](https://www.github.com/Unimarkup/unimarkup-rs/commit/590fa6fa778527cfcfe0cfcfaef243dcfbc0220b))
* add macro From<UM_BLOCK> for UnimarkupType ([af9200a](https://www.github.com/Unimarkup/unimarkup-rs/commit/af9200a3826b8b0aa1eb6657a26dc2df9af2d907))
* **backend:** impl parse from IR for Heading ([f7415d4](https://www.github.com/Unimarkup/unimarkup-rs/commit/f7415d401ebaa1238169ecb5a9577336681c4b78))
* **backend:** implement Render for HeadingBlock ([2a7e20e](https://www.github.com/Unimarkup/unimarkup-rs/commit/2a7e20ee01bb84329cb738abe1b7f6ecb97b6e0f))
* **backend:** implement um_type parser in loader ([989ac3d](https://www.github.com/Unimarkup/unimarkup-rs/commit/989ac3d3c1ecae480b51181a49c9500996171729))
* **backend:** implement write html to file ([939efa3](https://www.github.com/Unimarkup/unimarkup-rs/commit/939efa3db1e10db0a8c0ad89920030af3e427df8))
* **backend:** include backend module in lib.rs ([0a7cbe5](https://www.github.com/Unimarkup/unimarkup-rs/commit/0a7cbe50ada22fd006a86b3cd412f59fefb12191))
* **backend:** introduce loader - parser from IR ([ee82615](https://www.github.com/Unimarkup/unimarkup-rs/commit/ee8261534fd138a0e81f86ea7fb89832d0bf1df2))
* **backend:** introduce renderer ([8e578db](https://www.github.com/Unimarkup/unimarkup-rs/commit/8e578dbd7580b77c76b17fe3f150540775f262bc))
* **backend:** load, parse & render ParagraphBlocks ([4078463](https://www.github.com/Unimarkup/unimarkup-rs/commit/40784633f4153dad60f7539aa5d40344d5c1c7e9))
* **backend:** prepare parsing of UMBlocks from IR ([2bbf54d](https://www.github.com/Unimarkup/unimarkup-rs/commit/2bbf54d2bc5a0909b60a2dfe0c113e2b5e81ddac))
* extend UmError with general error variant ([4292c78](https://www.github.com/Unimarkup/unimarkup-rs/commit/4292c78b74467b63488cf35c4abbcf5c2d76d864))
* **frontend:** introduce headings grammar ([73f3978](https://www.github.com/Unimarkup/unimarkup-rs/commit/73f397874d4c7cfa542e3c69499f88c222927176))
* **frontent:** introduce paragraph grammar ([036e5cc](https://www.github.com/Unimarkup/unimarkup-rs/commit/036e5cc1d117f71e9ab927360b8c58faf6d32e1b))
* implement ParseFromIr for ParagraphBlock ([995e1c6](https://www.github.com/Unimarkup/unimarkup-rs/commit/995e1c6b095a149ae2f305344e53506a723db0de))
* implement parsing of paragraphs ([97b50c1](https://www.github.com/Unimarkup/unimarkup-rs/commit/97b50c1c88fd2eaa99967c0e85cd669138f956db))
* implement pipeline for heading blocks ([661ac31](https://www.github.com/Unimarkup/unimarkup-rs/commit/661ac31bc5a3fcd86784757deb19b8b97444bd40))
* implement write_to_ir in parser_pest ([5ffaac2](https://www.github.com/Unimarkup/unimarkup-rs/commit/5ffaac27d41f3e487159f0f3a6dc8851c185ea92))
* introduce backend error variant of UmError ([71f2f98](https://www.github.com/Unimarkup/unimarkup-rs/commit/71f2f9899399d7b39de0ac59401b8ddc4dedbad1))
* introduce backend module ([568a42c](https://www.github.com/Unimarkup/unimarkup-rs/commit/568a42cd1c15bee88c61dfc8a399c26c535ffead))
* make UnimarkupType variants comparable ([1586f18](https://www.github.com/Unimarkup/unimarkup-rs/commit/1586f185f9caf18b82d79c18b10a455a6659ce09))


### Bug Fixes

* adapt cli and add matching config struct ([48ca9c8](https://www.github.com/Unimarkup/unimarkup-rs/commit/48ca9c87c023e0968b26c9412ecfd4fef56372db))
* add clean, rebuild and dot-unimarkup args ([7dd26e0](https://www.github.com/Unimarkup/unimarkup-rs/commit/7dd26e07e0be0a8760c1a5292b721c6afc86f4e9))
* add features not enabled by default in clap ([c2a3111](https://www.github.com/Unimarkup/unimarkup-rs/commit/c2a3111d5d580b0f98b1efdd588eed04865fe751))
* add html-standalone flag ([b85a859](https://www.github.com/Unimarkup/unimarkup-rs/commit/b85a859dbdd5b240cfd09f5eed12635895e7b31c))
* add ParseFromIr trait bound to UnimarkupBlock ([799519c](https://www.github.com/Unimarkup/unimarkup-rs/commit/799519c068307dff66ec565a6d0c69f02c63d015))
* add template for unimarkup.rs ([c61b097](https://www.github.com/Unimarkup/unimarkup-rs/commit/c61b097aa036f668f28e495ea90f71d3fe14e191))
* **backend:** handle fallback fields for heading ([3e3b388](https://www.github.com/Unimarkup/unimarkup-rs/commit/3e3b388560f3f6ac46d9220e50b15a464de5aecc))
* change UmParse to only parse() ([75695b0](https://www.github.com/Unimarkup/unimarkup-rs/commit/75695b03b32ea215b6ab9290d78e31568ad2fa3f))
* check text line per line to avoid different behavior on different OS ([baf67f5](https://www.github.com/Unimarkup/unimarkup-rs/commit/baf67f54e9374d83da2aeb4e71648076097ab5d6))
* distribute all options to general and outputs ([1068440](https://www.github.com/Unimarkup/unimarkup-rs/commit/1068440a368c6faf0316688467693f14daaf3b51))
* move to clap_derive ([8781d77](https://www.github.com/Unimarkup/unimarkup-rs/commit/8781d77276b04379432f929294ab924e405ec4a1))
* move to option<> for optional config fields ([67d3c49](https://www.github.com/Unimarkup/unimarkup-rs/commit/67d3c49d2bfee580a1bf00be34d5f74a047f6cba))
* parse multiple blank lines correctly ([2e5588c](https://www.github.com/Unimarkup/unimarkup-rs/commit/2e5588c8b85792ee8668a17f2d0f9cc4d1c67a8f))
* prepare compile method ([3f45a47](https://www.github.com/Unimarkup/unimarkup-rs/commit/3f45a47e35f4c7781c220ad81afdc5454a7a3608))
* remove multiple config output options ([aac6726](https://www.github.com/Unimarkup/unimarkup-rs/commit/aac6726994778ecb2cb7fc73f13919afb616cf00))
* remove Option for flag arguments ([7d98fb6](https://www.github.com/Unimarkup/unimarkup-rs/commit/7d98fb61645199cefc825605a36ce7b283f55468))
* return error when paragraph type not valid ([4018ef5](https://www.github.com/Unimarkup/unimarkup-rs/commit/4018ef527b348906a01e6224d3f05d84d5eb6038))
* set config as mutable for frontend ([b4f16b4](https://www.github.com/Unimarkup/unimarkup-rs/commit/b4f16b4c4570d23d250322a0c615dcb19ef257dd))
* set macro/var definition in UmBlockElements ([1588b4d](https://www.github.com/Unimarkup/unimarkup-rs/commit/1588b4d29ec98dd08142147b072a6e3af290a694))
* set output_formats as optional arguments ([0abb076](https://www.github.com/Unimarkup/unimarkup-rs/commit/0abb076d30d812f2dfd80210d07a3abb423df686))
* specify macro/variable definition as options ([d2ff4b4](https://www.github.com/Unimarkup/unimarkup-rs/commit/d2ff4b445ff2e48136fc926d5577cc1128269083))
* type of ContentIrLine in AsIrLines fixed ([e72ed38](https://www.github.com/Unimarkup/unimarkup-rs/commit/e72ed38c3ad7c06cce7b6db632650a606c91b18f))


### Testing

* add cli test cases for all config fields ([7d98fb6](https://www.github.com/Unimarkup/unimarkup-rs/commit/7d98fb61645199cefc825605a36ce7b283f55468))
* add frontend parsing tests for heading and paragraph ([557cee1](https://www.github.com/Unimarkup/unimarkup-rs/commit/557cee15cb2bf0735526e48330d48ec9e668a30e))
* add nested multiline heading test ([011923a](https://www.github.com/Unimarkup/unimarkup-rs/commit/011923a5806336044d6f17e62a480816420a545f))
* add test for line number headings ([2e5588c](https://www.github.com/Unimarkup/unimarkup-rs/commit/2e5588c8b85792ee8668a17f2d0f9cc4d1c67a8f))
* add test um files for parser testing ([99a5a29](https://www.github.com/Unimarkup/unimarkup-rs/commit/99a5a293a562a7af24f7854428fdbbef928b89cd))
* **backend:** add basic test for backend::run() ([8654ab7](https://www.github.com/Unimarkup/unimarkup-rs/commit/8654ab735aa2837998252b9dbbfb2c6bb0fdaebc))
* **backend:** add unit test for Heading rendering ([85c6cc5](https://www.github.com/Unimarkup/unimarkup-rs/commit/85c6cc5be584db89585b132e1b68ec44f761737d))
* **backend:** add unit test for Paragraph ([fc74d54](https://www.github.com/Unimarkup/unimarkup-rs/commit/fc74d5406b03a0f6df87d51911ba49cc827d5e44))
* **backend:** check if html file is created ([82bd344](https://www.github.com/Unimarkup/unimarkup-rs/commit/82bd34401e5be2eb80432e3cd18c02846f551d95))
* **backend:** replace hard-coded values with variables ([da1881e](https://www.github.com/Unimarkup/unimarkup-rs/commit/da1881ea8c560983398a9a0fcd078607b390b486))
* **backend:** test render_html of ParagraphBlock ([5b1154d](https://www.github.com/Unimarkup/unimarkup-rs/commit/5b1154db76184ba618d227c4fde975225d8f4b7d))

## 0.1.0 (2021-11-18)


### Features

* Add ContentIrLine retrieval from IR ([7aab3dd](https://www.github.com/Unimarkup/unimarkup-rs/commit/7aab3dd882d8b48bb1dd3f9a6b8f49ce8bdaecf1))
* Add IR setup ([8e19e16](https://www.github.com/Unimarkup/unimarkup-rs/commit/8e19e16c11fcb32e852048978958ecb8b00e076a))
* Add MacroIrLine retrieval from IR ([882094c](https://www.github.com/Unimarkup/unimarkup-rs/commit/882094c480ada705423664e69f0d37d3547707b4))
* Add MetadataIrLine retrieval from IR ([946f3b2](https://www.github.com/Unimarkup/unimarkup-rs/commit/946f3b2042a2b435327711e326a07504779e7d57))
* Add ResourceIrLine retrieval from IR ([b8455fe](https://www.github.com/Unimarkup/unimarkup-rs/commit/b8455fe5e7e481b0747fb73f401d686cba68fc65))
* add simple logging functionality ([1155018](https://www.github.com/Unimarkup/unimarkup-rs/commit/115501886e9922a650b6d36615e70835bb8c5ffb))
* add statement to query all content rows ([37fe4d2](https://www.github.com/Unimarkup/unimarkup-rs/commit/37fe4d249160ca1b357b3afdb915c71be1080909))
* Add VariablesIrLine retrieval from Ir ([4324bf0](https://www.github.com/Unimarkup/unimarkup-rs/commit/4324bf0d0c945e92afa663bc56a748eca3bd2f29))
* implement Display + Debug for UmError ([bd3fd09](https://www.github.com/Unimarkup/unimarkup-rs/commit/bd3fd093e583aed05ad76743284cc7cc375d4f97))
* implement Display trait for IrError ([3ff2c77](https://www.github.com/Unimarkup/unimarkup-rs/commit/3ff2c77ea493915c50e83cf4048fdcf693f850f5))
* implement UmError wrapper for error types ([f1a79ed](https://www.github.com/Unimarkup/unimarkup-rs/commit/f1a79ed4d40a5d57f7fc9971182146df9aa9d16f))
* Implement write_to_ir for IrBlock, IrLines ([f425efc](https://www.github.com/Unimarkup/unimarkup-rs/commit/f425efcf2cea51419451eda7e1187c99b1a841f4))
* introduce function `new` for IrError ([b002d51](https://www.github.com/Unimarkup/unimarkup-rs/commit/b002d519e8f426e125338e7b99bb64e96a5f92bf))
* Set table name as trait per IrLine struct ([c776235](https://www.github.com/Unimarkup/unimarkup-rs/commit/c7762355684329c87a63a1e0760b3c857f6f0b2e))


### Bug Fixes

* add rusqlite error info in ir setup ([93cab60](https://www.github.com/Unimarkup/unimarkup-rs/commit/93cab600a474f4a8127f6f8c3136114490a4b0a8))
* Change return of `entry exists` query to i64 ([3956a07](https://www.github.com/Unimarkup/unimarkup-rs/commit/3956a07fe6de7859c6ddb45b243bf81bf41a4aa9))
* Check if values are already in table ([b8c1f7d](https://www.github.com/Unimarkup/unimarkup-rs/commit/b8c1f7d08ead354ca65ee0a2170aa5288c551d0e))
* commit transactions for all middle end tests ([d2fc728](https://www.github.com/Unimarkup/unimarkup-rs/commit/d2fc728d6a1e381e360179a7b8a970a10247db22))
* Commit transactions for content test ([f7e7eae](https://www.github.com/Unimarkup/unimarkup-rs/commit/f7e7eae9ccdce1f0973cc42c856bebaddc47e9d2))
* handle errors for vec<u8> to utf8 conversion ([bba8729](https://www.github.com/Unimarkup/unimarkup-rs/commit/bba87296033114fdcba756ce43c657cf12b59936))
* handle line_nr conversion for OutOfRange ([f359be1](https://www.github.com/Unimarkup/unimarkup-rs/commit/f359be143da9e460ebbc6c1a51ee7c665b62772d))
* remove clippy warnings ([2c47494](https://www.github.com/Unimarkup/unimarkup-rs/commit/2c47494cedb82eb3d86602d75be0dd3e8d96414e))
* Update IR entry if already existing ([48c7d1e](https://www.github.com/Unimarkup/unimarkup-rs/commit/48c7d1e588a2498ba64df9d626c918c71dfb5a68))
* write_to_ir for IrLines ([00aeeca](https://www.github.com/Unimarkup/unimarkup-rs/commit/00aeeca2d8340320df98fc1912abc72be8551986))


### CI

* add workflow for auto-generated changelog ([167aed4](https://www.github.com/Unimarkup/unimarkup-rs/commit/167aed4453b7ff0cc3899f1894b6e898a6deb44a))


### Testing

* add update and exist tests for all IrLines ([1dd584f](https://www.github.com/Unimarkup/unimarkup-rs/commit/1dd584f68fb3f79295a34146b7d2fa83a9b2a68f))
* Test IR setup ([8e19e16](https://www.github.com/Unimarkup/unimarkup-rs/commit/8e19e16c11fcb32e852048978958ecb8b00e076a))


### Documentation

* add '!' syntax support to commit-msg hook ([ddff236](https://www.github.com/Unimarkup/unimarkup-rs/commit/ddff236612b4e69a14439b610545ed001d97fdd8))
* add color to commit-msg output ([09e2d48](https://www.github.com/Unimarkup/unimarkup-rs/commit/09e2d487144337c763576e71b0ad21245eda9c92))
* add commit types from CONTRIBUTING guideline ([9c1bab7](https://www.github.com/Unimarkup/unimarkup-rs/commit/9c1bab78d05ec1ebfbc30c42754b957be2afb1b5))
* add git hook for commit message styles ([288a2ae](https://www.github.com/Unimarkup/unimarkup-rs/commit/288a2aec7ef0c7a609ccb3cfd84e9c5ee7f9d62d))
* add help for contributing to unimarkup-rs ([e8490f3](https://www.github.com/Unimarkup/unimarkup-rs/commit/e8490f3598b39483aa5302f1ce23e5179a335a69))
* add help on usage of provided git hooks ([811fc50](https://www.github.com/Unimarkup/unimarkup-rs/commit/811fc50b7cdd82461370e4b985d18f52372532d7))
* change auto-changelog category selection ([5d0b109](https://www.github.com/Unimarkup/unimarkup-rs/commit/5d0b109d6d64e90d4d7378243f17c1588bbac217))
* echo warning when commit is too long ([8bd797d](https://www.github.com/Unimarkup/unimarkup-rs/commit/8bd797db86f262599626f15c1dbd31bae9bc2167))
* use unimarkup syntax in CONTRIBUTING.md ([40e92c5](https://www.github.com/Unimarkup/unimarkup-rs/commit/40e92c5e5e293fa35a4771fd53019a9a8cb1a99d))


### Build

* migrate to Rust 2021 edition ([4a23fea](https://www.github.com/Unimarkup/unimarkup-rs/commit/4a23feab90cb6742b7f7792d46566c476662e692))
* migrate to Rust 2021 edition ([019ab07](https://www.github.com/Unimarkup/unimarkup-rs/commit/019ab07b25bfab52dc7b8433199e614e72e2ca98))
