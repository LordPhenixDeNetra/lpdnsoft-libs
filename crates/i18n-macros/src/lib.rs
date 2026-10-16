//! Macro `t!` : appelle `Localizer::tr` après avoir vérifié, à la compilation,
//! que l'identifiant existe dans toutes les locales du dossier `.ftl` de l'app.
//!
//! Dossier lu : `LPDNSOFT_I18N_LOCALES_DIR` si défini, sinon `$CARGO_MANIFEST_DIR/locales`.

use proc_macro::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token, parse_macro_input};

struct NamedArg {
    name: Ident,
    value: Expr,
}

impl Parse for NamedArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![=]>()?;
        Ok(Self {
            name,
            value: input.parse()?,
        })
    }
}

struct Input {
    loc: Expr,
    id: LitStr,
    args: Punctuated<NamedArg, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let loc = input.parse()?;
        input.parse::<Token![,]>()?;
        let id = input.parse()?;
        let args = if input.is_empty() {
            Punctuated::new()
        } else {
            input.parse::<Token![,]>()?;
            Punctuated::parse_terminated(input)?
        };
        Ok(Self { loc, id, args })
    }
}

/// `t!(loc, "id")` ou `t!(loc, "id", nom = expr, ...)` → `String`.
#[proc_macro]
pub fn t(input: TokenStream) -> TokenStream {
    let Input { loc, id, args } = parse_macro_input!(input as Input);
    if let Err(msg) = check_id(&id.value()) {
        return syn::Error::new(id.span(), msg).to_compile_error().into();
    }
    let expanded = if args.is_empty() {
        quote! { ::lpdnsoft_i18n::Localizer::tr(#loc, #id, ::core::option::Option::None) }
    } else {
        let sets = args.iter().map(|NamedArg { name, value }| {
            let key = name.to_string();
            quote! { __lpdnsoft_args.set(#key, #value); }
        });
        quote! {{
            let mut __lpdnsoft_args = ::lpdnsoft_i18n::FluentArgs::new();
            #(#sets)*
            ::lpdnsoft_i18n::Localizer::tr(#loc, #id, ::core::option::Option::Some(&__lpdnsoft_args))
        }}
    };
    expanded.into()
}

fn check_id(id: &str) -> Result<(), String> {
    let dir = match std::env::var_os("LPDNSOFT_I18N_LOCALES_DIR") {
        Some(d) => PathBuf::from(d),
        None => PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default())
            .join("locales"),
    };
    let catalog = lpdnsoft_i18n_catalog::load_catalog(&dir)
        .map_err(|e| format!("catalogue Fluent illisible ({}) : {e}", dir.display()))?;
    let missing = lpdnsoft_i18n_catalog::missing_locales_for(&catalog, id);
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "message `{id}` manquant pour : {}",
            missing.join(", ")
        ))
    }
}
