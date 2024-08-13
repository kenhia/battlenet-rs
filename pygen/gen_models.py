#!/usr/bin/env python

import click
import os
import yaml

def write_structs(name:str, struct:dict, file_handle):
    file_handle.write('#[derive(Debug, Deserialize)]\n')
    file_handle.write(f"pub struct {name} {{\n")
    for field in struct:
        f = field
        parts = field.split('.')
        if len(parts) == 2:
            f = parts[0]
            file_handle.write(f'    #[serde(alias = "{parts[1]}")]\n')
        file_handle.write(f"    pub {f}: {struct[field]},\n")
    file_handle.write("}\n")
    file_handle.write("\n")


def gen_core_structs(config:dict, outdir:str):
    outpath = os.path.join(outdir, 'core_structs.rs')
    print(f"Generating core structs -> {outpath}")

    with open(outpath, 'w') as file:
        file.write("use serde::Deserialize;\n\n")
        for struct in config['CoreStructs']:
            write_structs(struct, config['CoreStructs'][struct], file)


def impl_url_args(flavor: str, file_handle):
    if flavor == 'None':
        return
    if flavor == 'Player':
        file_handle.write('        let (realm_slug, name) = match url_args {\n')
        file_handle.write('            UrlArgs::Player { realm_slug, name } => (realm_slug, name),\n')
        file_handle.write('            _ => panic!("UrlArgs::Player expected"),\n')
        file_handle.write('        };\n\n')
        return
    if flavor == 'Id':
        file_handle.write('        let id = match url_args {\n')
        file_handle.write('            UrlArgs::Id { id } => id,\n')
        file_handle.write('            _ => panic!("UrlArgs::Id expected"),\n')
        file_handle.write('        };\n\n')
        return
    raise "Unknown flavor in impl_url_args()"


def impl_generate_url(item:str, item_data:dict, file_handle):
    file_handle.write(f"impl GenerateUrl for {item} {{\n")
    if 'url_args' in item_data:
        file_handle.write("    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {\n")
        impl_url_args(item_data['url_args'], file_handle)
        file_handle.write(f'        let endpoint = format!("{item_data['endpoint']}");\n')
    else:
        file_handle.write("    fn url(client: &BattleNetClient, _: &UrlArgs) -> String {\n")
        file_handle.write(f'        let endpoint = "{item_data['endpoint']}";\n')
    file_handle.write(f"        let namespace = WowNamespace::{item_data['namespace']}.to_region_string(&client.region);\n")
    file_handle.write("        let base = client.region.base_url();\n")
    file_handle.write("        let locale = &client.locale;\n\n")
    file_handle.write('        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")\n')
    file_handle.write("    }\n")
    file_handle.write("}\n\n")


def write_types(name:str, file_handle):
    file_handle.write(f"pub type {name}Result = Result<{name}, BattlenetClientError>;\n")
    file_handle.write(f"pub type {name}JsonResult = Result<String, BattlenetClientError>;\n")
    file_handle.write("\n")


def gen_group_models(group:str, config:dict, outdir:str):
    group_lower = group.lower()
    outpath = os.path.join(outdir, f"{group_lower}.rs")
    print(f"Generating model -> {outpath}")

    with open(outpath, 'w') as file:
        file.write("use serde::Deserialize;\n")
        file.write("\n")
        file.write("use crate::client::BattleNetClient;\n")
        file.write("use crate::errors::BattlenetClientError;\n")
        file.write("use crate::namespace::WowNamespace;\n")
        if '_NO_CORE' in config[group] and config[group]['_NO_CORE']:
            file.write("use crate::wow_models::{ GenerateUrl, UrlArgs };\n")
        else:
            file.write("use crate::wow_models::{ GenerateUrl, UrlArgs, core_structs::* };\n")
        file.write("\n")

        for item in config[group]:
            if item in OPTION_FLAGS:
                continue
            write_structs(item, config[group][item]['struct'], file)
            write_types(item, file)
            impl_generate_url(item, config[group][item], file)

OPTION_FLAGS = (
    '_NO_CORE',
)

@click.command()
@click.option('--model', '-m', help='Path to model YAML file')
@click.option('--outdir', '-o', help='directory to output generated files')
def main(model, outdir):
    print(f"Generating model from YAML file: {model}")
    with open(model, 'r') as f:
        config = yaml.safe_load(f)
    # print(config)
    # print(config['_core_structs'])
    if 'CoreStructs' in config:
        gen_core_structs(config, outdir)
    for struct in config:
        if struct == 'CoreStructs':
            continue
        gen_group_models(struct, config, outdir)
    # print(config.__class__)

if __name__ == '__main__':
    main()
