use embed_it_utils::entry::EntryKind;
use syn::{parse_quote, Ident};

pub trait EntryKindExt {
    fn ident(&self) -> Ident;
}

impl EntryKindExt for EntryKind {
    fn ident(&self) -> Ident {
        match self {
            EntryKind::Dir => parse_quote!(Dir),
            EntryKind::File => parse_quote!(File),
        }
    }
}
