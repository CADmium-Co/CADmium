use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Ident, MetaNameValue, Token};
use syn::spanned::Spanned;

#[proc_macro_derive(StepDataActions, attributes(step_data))]
pub fn derive_step_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("StepData can only be derived for enums"),
    };
    let mut actions = vec![];

    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let mut skip_workbench = false;
        let mut workbench_field = None;
        let mut parent_type = None;

        for attr in &variant.attrs {
            if !attr.path.is_ident("step_data") {
                continue;
            }

            let name_values: Punctuated<MetaNameValue, Token![,]> = attr.parse_args_with(Punctuated::parse_terminated).unwrap(); // handle error instead of unwrap
            for nv in name_values {
                let ident = nv.path.get_ident().unwrap();

                match ident.to_string().as_str() {
                    "skip_workbench" => {
                        skip_workbench = true;
                    },
                    "workbench_field" => {
                        if let syn::Lit::Str(value) = nv.lit {
                            workbench_field = Some(value.value());
                        } else {
                            panic!("workbench_field must be a string literal");
                        }
                    },
                    "type" => {
                        if let syn::Lit::Str(value) = nv.lit {
                            parent_type = Some(value.value());
                        } else {
                            panic!("type must be a string literal");
                        }
                    },
                    &_ => {}
                }
            }
        }

        // Process not skipped workbench
        let mut wb_var = quote! {};
        if !skip_workbench {
            wb_var = quote! {
                let mut wb_ = self.workbenches
                    .get_mut(workbench_id as usize)
                    .ok_or(anyhow::anyhow!("Could not find workbench"))?;
            };
        }

        // Process type and workbench_field
        let mut _field_var = quote! {};
        let mut _parent_var_ = quote! { wb_ };
        let id_arg_name = if let Some(f) = parent_type.clone() {
            Ident::new(format!("{}_id", f.to_string().to_case(Case::Snake)).as_str(), f.span())
        } else {
            Ident::new("id", variant_name.span())
        };
        if let Some(field_ident) = workbench_field.clone() {
            let field_name = Ident::new(field_ident.as_str(), field_ident.span());
            _field_var = quote! {
                let parent_ref_ = wb_.#field_name
                    .get(& #id_arg_name)
                    .ok_or(anyhow::anyhow!(concat!("Could not find parent ", stringify!(#parent_type), " with ID {}"), #id_arg_name))?;
                let mut parent_ = parent_ref_.borrow_mut();
            };
            _parent_var_ = quote! { parent_ };
        } else if !skip_workbench {
            _parent_var_ = quote! { wb_ };
        } else {
            _parent_var_ = quote! { self };
        }

        // Function declaration
        let add_func_name = Ident::new(format!("add_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());
        let function_defs = variant.fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            quote! { #field_name: #field_type }
        }).collect::<Vec<_>>();
        let function_args_full = variant.fields.iter().map(|field| {
            let field_name = &field.ident;

            quote! { #field_name }
        }).collect::<Vec<_>>();

        let function_args2 = function_args_full.clone();
        let function_args_noauto = function_args2
            .iter()
            .filter(|field|
                field.to_string() != "workbench_id"
                && field.to_string() != id_arg_name.to_string()
            ).collect::<Vec<_>>();

        actions.push(quote! {
            #name::#variant_name {
                #( #function_args_full ),*
            } => project.#add_func_name(name, #( #function_args_full.clone() ),* ) , });

        quote! {
            pub fn #add_func_name(&mut self, name: String, #( #function_defs ),*) -> Result<crate::IDType, anyhow::Error> {
                #wb_var
                #_field_var
                let result_id_ = #_parent_var_.#add_func_name(#( #function_args_noauto.clone() ),*)?;

                let step_ = Step {
                    name,
                    id: result_id_,
                    operation: StepOperation::Add,
                    unique_id: format!(concat!("Add:", stringify!(#variant_name), "-{}"), result_id_),
                    suppressed: false,
                    data: #name::#variant_name {
                        #( #function_args_full ),*
                    },
                };

                wb_.history.push(step_);

                Ok(result_id_)
            }
        }
    });

    let expanded = quote! {
        impl crate::project::Project {
            #( #variants )*
        }

        impl #name {
            pub fn do_action(&self, project: &mut crate::project::Project, name: String) -> Result<IDType, anyhow::Error> {
                match self {
                    #( #actions )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
