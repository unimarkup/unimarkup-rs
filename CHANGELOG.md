# Changelog

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
