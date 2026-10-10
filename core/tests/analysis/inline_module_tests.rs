// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use grip::analysis::inline_module::InlineModule;
use syn::{Item, ItemMod, parse_str};

#[test]
fn items_of_a_module_declared_by_file_returns_no_items() {
    // Arrange
    let item_mod: ItemMod = parse_str("mod elsewhere;").expect("parse module");

    // Act
    let items = InlineModule::new(&item_mod).items();

    // Assert
    assert!(items.is_empty());
}

#[test]
fn items_of_an_empty_inline_module_returns_no_items() {
    // Arrange
    let item_mod: ItemMod = parse_str("mod inner {}").expect("parse module");

    // Act
    let items = InlineModule::new(&item_mod).items();

    // Assert
    assert!(items.is_empty());
}

#[test]
fn items_of_an_inline_module_leaves_nested_modules_unopened() {
    // Arrange
    let item_mod: ItemMod =
        parse_str("mod outer { mod nested { struct Deep; } }").expect("parse module");

    // Act
    let items = InlineModule::new(&item_mod).items();

    // Assert
    assert_eq!(items.len(), 1);
    assert!(matches!(&items[0], Item::Mod(m) if m.ident == "nested"));
}

#[test]
fn items_of_an_inline_module_returns_its_items_in_order() {
    // Arrange
    let item_mod: ItemMod =
        parse_str("mod inner { struct First; fn second() {} }").expect("parse module");

    // Act
    let items = InlineModule::new(&item_mod).items();

    // Assert
    assert_eq!(items.len(), 2);
    assert!(matches!(&items[0], Item::Struct(s) if s.ident == "First"));
    assert!(matches!(&items[1], Item::Fn(f) if f.sig.ident == "second"));
}
