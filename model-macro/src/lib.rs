use quote::ToTokens;
use syn::punctuated::Punctuated;

use proc_macro::TokenStream;
use quote::quote;
use syn::token::Colon;
use syn::{parse_macro_input, DeriveInput, Visibility};

use syn::Data::Struct;
use syn::Fields::Named;
use syn::{DataStruct, FieldsNamed, Ident};

use syn::parse::{Parse, ParseStream};
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

struct StructField {
    name: syn::Ident,
    ty: syn::Ident,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!( pub #n: #t ).to_tokens(tokens);
    }
}

impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input)?;

        Ok(StructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}


#[proc_macro_attribute]
pub fn bendpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: BEndpointInput = parse_macro_input!(attr);
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    // eprintln!("input: {:#?}", &input);

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only structs with named fields are supported"),
    };

    let builder_fields = fields.iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

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
    //
    // let id = match url_args {
    //     UrlArgs::Id { id } => id,
    //     _ => panic!("UrlArgs::Id expected"),
    // };
    //
    // let (realm_slug, name) = match url_args {
    //     UrlArgs::Player { realm_slug, name } => (realm_slug, name),
    //     _ => panic!("UrlArgs::Player expected"),
    // };
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
            _ => panic!("unsupported url arg"),
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
