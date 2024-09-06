#![warn(missing_docs)]
//! The unimarkup-rs crate is the official implementation of the [Unimarkup specification](https://github.com/Unimarkup/Specification/).

// TODO: set to private modules that don't have to be public
pub mod document;
pub mod elements;
pub mod log_id;
pub mod metadata;
mod parser;
// mod parser2;
pub mod security;

pub use parser::*;

/*

~~~
test
~~~
## heading // indentation 3 Token::Indentation(usize)
   laskfjlks
  lkajsdflk
> hello there

. list entry   // let indent = Indentation(2)
  > quote      // Indentation(4)
    multi-line // Indentation(4)
  laksdjf      // indent is still available


fn parse_list(&mut self) -> List {
    let parent_indent = self.parent_indent();


    let indent = ctx.indent();
    ctx.push_indent();

    loop {
        let list_entry = list_entry(ctx);

        // indent is still available here...
    }

    ctx.pop_indent();
}

fn parse_list_entry(ctx: &mut Context) -> ListEntry {
    let indent = ctx.indent();

    loop {
        let quote = parse_quote(ctx);

        // indent is still available here
    }

    return ListEntry { ... };
}

[[[<section>
[[[
inner
]]]
]]]

[[[
# Heading lvl 1

- list entry 1

- entry 2
  . nested numbered list entry 1
    # body for nested numbered

    [[[
    ]]] bla bla


  body for bullet entry 2, but not for numbered

  Cow<'a, str>

===|#| |_|
| r1 c1 | r1 c2 | r1 c3 |
+ merge | merge | merge |
| r3 c1 | r3 c2 | r3 c3 |
| r4 c1 |+ r4 c1 | r4 c3 |
! not merged | merged |! not merged |
# head r5 | bla | bla |
_ lksad   | jsl | jkd |
==={
  id: my_table;
}

===|#| |_|
|  r1 c1      |  r1 c2  |  r1 c3       |
|+ merge      |  merge  |  merge       |
|  r3 c1      |  r3 c2  |  r3 c3       |
|  r4 c1      |+ r4 c1  |  r4 c3       |
|! not merged |  merged |! not merged  |
|# head r5    |  bla    |  bla         |
|_ lksad      |  jsl    |  jkd         |
==={
  id: my_table;
}
]]]

```rs
let bal;
```

. nested numbered list entry 1
  # body for nested numbered

=> ol li "nested numbered list entry 1 # body for nested numbered"

. nested numbered list entry 1

  body for nested numbered

=> ol li entry-head("nested numbered list entry 1") entry-body(p(body for nested numbered))


- list 1
- list 1



 */
