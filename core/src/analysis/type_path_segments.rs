// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::punctuated::Punctuated;
use syn::token::PathSep;
use syn::{PathSegment, Type};

pub struct TypePathSegments<'ty> {
    segments: Option<&'ty Punctuated<PathSegment, PathSep>>,
}

impl<'ty> TypePathSegments<'ty> {
    #[must_use]
    pub fn new(ty: &'ty Type) -> Self {
        let segments = match ty {
            Type::Path(type_path) => Some(&type_path.path.segments),
            _ => None,
        };
        Self { segments }
    }

    #[must_use]
    pub fn first_ident(&self) -> String {
        Self::ident_of(self.segments.and_then(Punctuated::first))
    }

    #[must_use]
    pub fn last_ident(&self) -> String {
        Self::ident_of(self.segments.and_then(Punctuated::last))
    }

    fn ident_of(segment: Option<&PathSegment>) -> String {
        segment
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default()
    }
}
