use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MessageEnum)]
pub fn message_handler_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("MessageEnum can only be derived for enums"),
    };

    let variants = data.variants.iter().map(|variant| {
        println!("Message Handler: {}", variant.ident);
        let variant_name = &variant.ident;

        quote! {
            #name::#variant_name(msg)
        }
    });
    let variant_names = data.variants.iter().map(|variant| &variant.ident);
    let variants_clone = variants.clone();
    let variants_clone2 = variants.clone();

    quote! {
        impl #name {
            pub fn handle(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<crate::IDType>> {
                match self {
                    #( #variants_clone => msg.handle_project_message(project), )*
                }
            }

            pub fn realize(&self, realization: crate::realization::Realization) -> anyhow::Result<crate::realization::Realization> {
                match self {
                    #( #variants_clone2 => msg.realize(realization), )*
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #( #variants => write!(f, stringify!(#variant_names)), )*
                }
            }
        }
    }.into()
}

#[proc_macro_derive(NoRealize)]
pub fn no_realize_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    match input.data {
        syn::Data::Enum(_) => {},
        syn::Data::Struct(_) => {},
        _ => panic!("NoRealize can only be derived for enums or structs"),
    };

    quote! {
        impl crate::realization::Realizable for #name {
            fn realize(&self, realization: crate::realization::Realization) -> anyhow::Result<crate::realization::Realization> {
                Ok(realization)
            }
        }
    }.into()
}
