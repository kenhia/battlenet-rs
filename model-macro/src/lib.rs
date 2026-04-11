use proc_macro::TokenStream;
use quote::quote;

use syn::parse_macro_input;
use syn::DeriveInput;

use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, FieldsNamed, Ident};

use syn::__private::Span;

use crate::input::BEndpointInput;

mod input;

#[proc_macro_attribute]
pub fn dumpast(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = item.clone();
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("ast: {:#?}", &ast);
    output // return the input unchanged
}


#[proc_macro_attribute]
pub fn bendpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: BEndpointInput = parse_macro_input!(attr);
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only structs with named fields are supported"),
    };

    // Preserve original field tokens (attributes, generics, paths, etc.)
    // and ensure all fields are pub
    let builder_fields = fields.iter().map(|f| {
        let attrs = &f.attrs;
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #(#attrs)* pub #name: #ty }
    });

    let pub_struct = quote! {
        pub struct #name {
            #(#builder_fields),*
        }
    };

    let result_name = Ident::new(&format!("{name}Result"), Span::call_site());
    let json_result_name = Ident::new(&format!("{name}JsonResult"), Span::call_site());

    let result_types = if input.endpoint.is_some() {
        quote! {
            pub type #result_name = Result<#name, BattleNetClientError>;
            pub type #json_result_name = Result<String, BattleNetClientError>;
        }
    } else {
        quote! {}
    };

    // piece together the UrlArgs bit if needed
    let is_search = input.url_args.as_deref() == Some("Search");

    let url_args_snip = if input.url_args.is_some() {
        let url_args = input.url_args.unwrap();
        match url_args.as_str() {
            "Id" => quote! { 
                let id = match url_args {
                    UrlArgs::Id { id } => id,
                    _ => panic!("UrlArgs::Id expected"),
                };
             },
            "Player" => quote! {
                let (realm_slug, name) = match url_args {
                    UrlArgs::Player { realm_slug, name } => (realm_slug, name),
                    _ => panic!("UrlArgs::Player expected"),
                };
            },
            "Guild" => quote! {
                let (realm_slug, name_slug) = match url_args {
                    UrlArgs::Guild { realm_slug, name_slug } => (realm_slug, name_slug),
                    _ => panic!("UrlArgs::Guild expected"),
                };
            },
            "TwoIds" => quote! {
                let (id1, id2) = match url_args {
                    UrlArgs::TwoIds { id1, id2 } => (id1, id2),
                    _ => panic!("UrlArgs::TwoIds expected"),
                };
            },
            "ThreeIds" => quote! {
                let (id1, id2, id3) = match url_args {
                    UrlArgs::ThreeIds { id1, id2, id3 } => (id1, id2, id3),
                    _ => panic!("UrlArgs::ThreeIds expected"),
                };
            },
            "PlayerExtra" => quote! {
                let (realm_slug, name, extra) = match url_args {
                    UrlArgs::PlayerExtra { realm_slug, name, extra } => (realm_slug, name, extra),
                    _ => panic!("UrlArgs::PlayerExtra expected"),
                };
            },
            "TwoStrings" => quote! {
                let (first, second) = match url_args {
                    UrlArgs::TwoStrings { first, second } => (first, second),
                    _ => panic!("UrlArgs::TwoStrings expected"),
                };
            },
            "Search" => quote! {
                let params = match url_args {
                    UrlArgs::Search { params } => params,
                    _ => panic!("UrlArgs::Search expected"),
                };
            },
            _ => panic!("unsupported url arg: {url_args}"),
        }
    } else {
        quote! {}
    };

    let namespace_snip = if input.namespace.is_some() {
        let namespace = input.namespace.unwrap();
        match namespace.as_str() {
            "static" => quote! {
                let namespace = WowNamespace::Static.to_region_string(&client.region);
            },
            "dynamic" => quote! {
                let namespace = WowNamespace::Dynamic.to_region_string(&client.region);
            },
            "profile" => quote! {
                let namespace = WowNamespace::Profile.to_region_string(&client.region);
            },
            _ => panic!("unsupported namespace: {namespace}"),
        }
    } else {
        quote! {}
    };

    let gen_url = if input.endpoint.is_some() {
        let endpoint = input.endpoint.unwrap();
        if is_search {
            quote! {
                impl GenerateUrl for #name {
                    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
                        #url_args_snip
                        let endpoint = format!(#endpoint);
                        #namespace_snip
                        let base = client.region.base_url();
                        let locale = &client.locale;
                        let mut url = format!("{base}/{endpoint}?namespace={namespace}&locale={locale}");
                        for (key, value) in params {
                            url.push_str(&format!("&{key}={value}"));
                        }
                        url
                    }
                }
            }
        } else {
            quote! {
                impl GenerateUrl for #name {
                    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
                        #url_args_snip
                        let endpoint = format!(#endpoint);
                        #namespace_snip
                        let base = client.region.base_url();
                        let locale = &client.locale;
                
                        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let output = quote! {
        #[derive(Debug, Deserialize)]
        #pub_struct

        #result_types

        #gen_url
    };

    output.into()
}
