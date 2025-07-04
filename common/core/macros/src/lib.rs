use cargo_metadata::{MetadataCommand, Package};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::path;
use walkdir::WalkDir;

fn get_dependencies_from_cargo_toml(toml: String) -> BTreeMap<String, Package> {
    let mut cmd = MetadataCommand::new();
    let metadata_command = cmd.manifest_path(toml);
    let metadata = metadata_command.exec().unwrap();

    metadata
        .packages
        .iter()
        .map(|p| (p.name.to_string(), p.clone()))
        .collect()
}

fn normalize(license_string: &str) -> String {
    let mut list: Vec<&str> = license_string
        .split('/')
        .flat_map(|e| e.split(" OR "))
        .map(str::trim)
        .collect();
    list.sort();
    list.dedup();
    list.join(" OR ")
}

#[proc_macro]
pub fn all_dependencies(_input: TokenStream) -> TokenStream {
    let dependencies = WalkDir::new(
        path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("..")
            .join("..")
            .join("Cargo.toml"),
    )
    .follow_links(false)
    .into_iter()
    .filter_map(|e| {
        e.ok()
            .map(|e| get_dependencies_from_cargo_toml(e.path().display().to_string()))
    })
    .flatten()
    .collect::<BTreeMap<_, _>>()
    .into_values()
    .map(|p| {
        let authors = if p.authors.is_empty() {
            String::default()
        } else {
            p.authors.join(", ")
        };
        let description = p
            .description
            .clone()
            .map(|s| s.trim().replace('\n', " "))
            .unwrap_or_default();
        let license = p.license.as_ref().map(|s| normalize(s)).unwrap_or_default();
        let repository = p.repository.clone().unwrap_or_default();
        let name = p.name.to_string();

        quote! {
            DependencyDetails::new(
                #authors,
                #name,
                #repository,
                #license,
                #description,
            )
        }
    })
    .collect::<Vec<_>>();

    let expanded = quote! {
        vec![#(#dependencies),*]
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
