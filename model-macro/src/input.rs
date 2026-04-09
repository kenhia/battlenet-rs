use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

pub(crate) mod kw {
    syn::custom_keyword!(endpoint);
    syn::custom_keyword!(namespace);
    syn::custom_keyword!(url_args);
}

#[derive(Debug)]
pub struct BEndpointInput {
    pub endpoint: Option<String>,
    pub namespace: Option<String>,
    pub url_args: Option<String>,
}

impl Parse for BEndpointInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut endpoint = None;
        let mut namespace = None;
        let mut url_args = None;

        while !input.is_empty() {
            if input.peek(kw::endpoint) {
                let _: kw::endpoint = input.parse().expect("checked that this exists");
                let _: Token!(=) = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected equals sign after path"))?;
                let value: LitStr = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected value after the equals sign"))?;

                endpoint = Some(value.value());

            } else if input.peek(kw::url_args) {
                let _: kw::url_args = input.parse().expect("checked that this exists");
                let _: Token!(=) = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected equals sign after path"))?;
                let value: LitStr = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected value after the equals sign"))?;

                    url_args = Some(value.value());

            } else if input.peek(kw::namespace) {
                let _: kw::namespace = input.parse().expect("checked that this exists");
                let _: Token!(=) = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected equals sign after path"))?;
                let value: LitStr = input.parse()
                    .map_err(|_| syn::Error::new(input.span(), "expected value after the equals sign"))?;

                    // TODO: validate that the namespace is a valid namespace (static, dynamic, profile)
                    namespace = Some(value.value());

            } else {
                return Err(
                    syn::Error::new(
                        input.span(),
                        "config macro only allows for 'path' input",
                    )
                );
            }
        }

        Ok(BEndpointInput {
            endpoint,
            namespace,
            url_args,
        })
    }
}
