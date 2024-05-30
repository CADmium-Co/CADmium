use std::collections::HashMap;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
// use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Attribute, DeriveInput, Fields, Ident, MetaNameValue, Token};
use syn::spanned::Spanned;

const ATTR_NAME: &str = "step_data";

#[proc_macro_derive(StepDataActions, attributes(step_data))]
pub fn derive_step_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => panic!("StepData can only be derived for enums"),
    };
    let mut actions = vec![];

    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let mut workbench_field = None;
        let mut parent_type = None;
        let mut skip_history = false;
        let mut skip_add = false;
        let mut skip_update = false;
        let mut skip_delete = false;
        let mut skip_all = false;

        // Parse the attributes above each variant
        for (k, v) in get_meta_kv(&variant.attrs).iter() {
            match k.to_string().as_str() {
                "workbench_field" => {
                    if let syn::Lit::Str(value) = &v.lit {
                        workbench_field = Some(value.value());
                    } else {
                        panic!("workbench_field must be a string literal");
                    }
                },
                "type_name" => {
                    if let syn::Lit::Str(value) = &v.lit {
                        parent_type = Some(value.value());
                    } else {
                        panic!("type_name must be a string literal");
                    }
                },
                "skip_history" => {
                    if let syn::Lit::Bool(_value) = &v.lit {
                        skip_history = true;
                    } else {
                        panic!("skip_history must be a bool literal");
                    }
                },
                "skip_all" => {
                    if let syn::Lit::Bool(_value) = &v.lit {
                        skip_all = true;
                        skip_history = true;
                    } else {
                        panic!("skip_add must be a bool literal");
                    }
                },
                "skip_add" => {
                    if let syn::Lit::Bool(_value) = &v.lit {
                        skip_add = true;
                    } else {
                        panic!("skip_add must be a bool literal");
                    }
                },
                "skip_update" => {
                    if let syn::Lit::Bool(_value) = &v.lit {
                        skip_update = true;
                    } else {
                        panic!("skip_update must be a bool literal");
                    }
                },
                "skip_delete" => {
                    if let syn::Lit::Bool(_value) = &v.lit {
                        skip_delete = true;
                    } else {
                        panic!("skip_delete must be a bool literal");
                    }
                },
                &_ => {}
            }
        }

        let needs_workbench = variant.fields.iter().any(|field| field.ident.as_ref().unwrap().to_string() == "workbench_id");

        // Process not skipped workbench
        let mut wb_var = quote! {};
        if needs_workbench {
            wb_var = quote! {
                let mut wb_ = self.workbenches
                    .get_mut(workbench_id as usize)
                    .ok_or(anyhow::anyhow!("Could not find workbench ID {}", workbench_id))?;
            };
        }

        // Process type_name to expected id field (e.g. sketch_id for Sketch)
        let mut field_var = quote! {};
        let parent_var;
        let id_arg_name = if let Some(f) = parent_type.clone() {
            Ident::new(format!("{}_id", f.to_string().to_case(Case::Snake)).as_str(), f.span())
        } else {
            Ident::new("id", variant_name.span())
        };

        // Generate the parent variable of which the actual function will be called on
        if let Some(field_ident) = workbench_field.clone() {
            let field_name = Ident::new(field_ident.as_str(), field_ident.span());
            field_var = quote! {
                let parent_ref_ = wb_.#field_name
                    .get(& #id_arg_name)
                    .ok_or(anyhow::anyhow!(concat!("Could not find parent ", stringify!(#parent_type), " with ID {}"), #id_arg_name))?;
                let mut parent_ = parent_ref_.borrow_mut();
            };
            parent_var = quote! { parent_ };
        } else if needs_workbench {
            parent_var = quote! { wb_ };
        } else {
            parent_var = quote! { self };
        }

        // Generated function bodies
        // TODO: Make the return types useful
        let body = if skip_all {
            let func_name = Ident::new(format!("do_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());
            let gen = get_function_body(
                func_name,
                name,
                variant_name,
                &variant.fields,
                id_arg_name.clone(),
                wb_var.clone(),
                field_var.clone(),
                parent_var.clone(),
                quote! { crate::step::StepOperation::Add }.into(),
                skip_history,
                quote! { crate:::IDType }.into()
            );

            actions.push(gen.1);
            gen.0
        } else {
            let add_func = if !skip_add {
                let func_name = Ident::new(format!("add_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());
                let gen = get_function_body(
                    func_name,
                    name,
                    variant_name,
                    &variant.fields,
                    id_arg_name.clone(),
                    wb_var.clone(),
                    field_var.clone(),
                    parent_var.clone(),
                    quote! { crate::step::StepOperation::Add }.into(),
                    skip_history,
                    quote! { crate:::IDType }.into()
                );

                actions.push(gen.1);
                gen.0
            } else { quote! {} };

            let update_func = if !skip_update {
                let func_name = Ident::new(format!("update_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());
                let gen = get_function_body(
                    func_name,
                    name,
                    variant_name,
                    &variant.fields,
                    id_arg_name.clone(),
                    wb_var.clone(),
                    field_var.clone(),
                    parent_var.clone(),
                    quote! { crate::step::StepOperation::Add }.into(),
                    skip_history,
                    quote! { crate:::IDType }.into()
                );

                actions.push(gen.1);
                gen.0
            } else { quote! {} };

            let delete_func = if !skip_delete {
                let func_name = Ident::new(format!("delete_{}", variant_name.to_string().to_case(Case::Snake)).as_str(), variant_name.span());
                let gen = get_function_body(
                    func_name,
                    name,
                    variant_name,
                    &variant.fields,
                    id_arg_name,
                    wb_var,
                    field_var,
                    parent_var,
                    quote! { crate::step::StepOperation::Add }.into(),
                    skip_history,
                    quote! { crate:::IDType }.into()
                );

                actions.push(gen.1);
                gen.0
            } else { quote! {} };

            quote! {
                #add_func
                #update_func
                #delete_func
            }
        };

        quote! { #body }
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

    TokenStream::from(expanded).into()
}

fn get_meta_kv(attrs: &Vec<Attribute>) -> HashMap<Ident, MetaNameValue> {
    let mut result = HashMap::new();

    for attr in attrs {
        if !attr.path.is_ident(ATTR_NAME) {
            continue;
        }

        let Ok(name_values): Result<Punctuated<MetaNameValue, Token![,]>, _> = attr
            .parse_args_with(Punctuated::parse_terminated) else { continue; };

        for nv in name_values {
            let Some(ident) = nv.path.get_ident() else { continue; };
            result.insert(ident.clone(), nv);
        }
    }

    result
}

fn get_function_body(
    func_name: Ident,
    name: &Ident,
    variant_name: &Ident,
    fields: &Fields,
    id_ident: Ident,
    wb_var: TokenStream,
    self_field_code: TokenStream,
    self_field_var: TokenStream,
    operation: TokenStream,
    skip_history: bool,
    _return_type: TokenStream,
) -> (TokenStream, TokenStream) {
    // Function arguments - both on definition and call
    let function_defs = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;

        quote! { #field_name: #field_type }
    }).collect::<Vec<_>>();
    let function_args_full = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! { #field_name }
    }).collect::<Vec<_>>();

    let function_args2 = function_args_full.clone();
    let function_args_noauto = function_args2
        .iter()
        .filter(|field|
            field.to_string() != "workbench_id"
            && field.to_string() != id_ident.to_string()
        ).collect::<Vec<_>>();

    // Generate history entry
    let history_code = if skip_history {
        quote! {}
    } else {
        quote! {
            let step_ = crate::step::Step {
                name,
                id: result_id_,
                operation: operation_,
                suppressed: false,
                data: #name::#variant_name {
                    #( #function_args_full ),*
                },
            };

            wb_.history.push(step_);
        }
    };

    // Code to run during `do_action`
    let action = quote! {
        #name::#variant_name {
            #( #function_args_full ),*
        } => project.#func_name(
            name,
            #( #function_args_full.clone() ),*
        ),
    };

    // The actual function body
    let body = quote! {
        pub fn #func_name(&mut self, name: String, #( #function_defs ),*) -> anyhow::Result<crate::IDType> {
            let operation_ = #operation;
            #wb_var
            #self_field_code
            let result_id_ = #self_field_var.#func_name(#( #function_args_noauto.clone() ),*)?;

            #history_code

            Ok(result_id_)
        }
    };

    (body, action)
}
