# Changelog

### [0.2.1](https://www.github.com/Unimarkup/unimarkup-rs/compare/v0.2.0...v0.2.1) (2021-12-26)


### Bug Fixes

* fix logger when using installed unimarkup-rs ([cea675d](https://www.github.com/Unimarkup/unimarkup-rs/commit/cea675d3dd5506e38ad1d06bc33bbdcefec92b7e))
* fix simple_logger causing panic when using installed version of unimarkup-rs ([efdd2dd](https://www.github.com/Unimarkup/unimarkup-rs/commit/efdd2dd437383f42bcc4c829bfddcadab9098f00))
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
