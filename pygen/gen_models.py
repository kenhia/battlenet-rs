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
    if flavor == 'Guild':
        file_handle.write('        let (realm_slug, name_slug) = match url_args {\n')
        file_handle.write('            UrlArgs::Guild { realm_slug, name_slug } => (realm_slug, name_slug),\n')
        file_handle.write('            _ => panic!("UrlArgs::Guild expected"),\n')
        file_handle.write('        };\n\n')
        return
    if flavor == 'TwoIds':
        file_handle.write('        let (id1, id2) = match url_args {\n')
        file_handle.write('            UrlArgs::TwoIds { id1, id2 } => (id1, id2),\n')
        file_handle.write('            _ => panic!("UrlArgs::TwoIds expected"),\n')
        file_handle.write('        };\n\n')
        return
    if flavor == 'ThreeIds':
        file_handle.write('        let (id1, id2, id3) = match url_args {\n')
        file_handle.write('            UrlArgs::ThreeIds { id1, id2, id3 } => (id1, id2, id3),\n')
        file_handle.write('            _ => panic!("UrlArgs::ThreeIds expected"),\n')
        file_handle.write('        };\n\n')
        return
    if flavor == 'PlayerExtra':
        file_handle.write('        let (realm_slug, name, extra) = match url_args {\n')
        file_handle.write('            UrlArgs::PlayerExtra { realm_slug, name, extra } => (realm_slug, name, extra),\n')
        file_handle.write('            _ => panic!("UrlArgs::PlayerExtra expected"),\n')
        file_handle.write('        };\n\n')
        return
    if flavor == 'TwoStrings':
        file_handle.write('        let (first, second) = match url_args {\n')
        file_handle.write('            UrlArgs::TwoStrings { first, second } => (first, second),\n')
        file_handle.write('            _ => panic!("UrlArgs::TwoStrings expected"),\n')
        file_handle.write('        };\n\n')
        return
    if flavor == 'Search':
        file_handle.write('        let params = match url_args {\n')
        file_handle.write('            UrlArgs::Search { params } => params,\n')
        file_handle.write('            _ => panic!("UrlArgs::Search expected"),\n')
        file_handle.write('        };\n\n')
        return
    raise ValueError(f"Unknown flavor in impl_url_args(): {flavor}")


def impl_generate_url(item:str, item_data:dict, file_handle):
    file_handle.write(f"impl GenerateUrl for {item} {{\n")
    is_search = item_data.get('url_args') == 'Search'
    if 'url_args' in item_data:
        file_handle.write("    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {\n")
        impl_url_args(item_data['url_args'], file_handle)
        file_handle.write(f'        let endpoint = format!("{item_data["endpoint"]}");\n')
    else:
        file_handle.write("    fn url(client: &BattleNetClient, _: &UrlArgs) -> String {\n")
        file_handle.write(f'        let endpoint = "{item_data["endpoint"]}";\n')
    file_handle.write(f"        let namespace = WowNamespace::{item_data['namespace']}.to_region_string(&client.region);\n")
    file_handle.write("        let base = client.region.base_url();\n")
    file_handle.write("        let locale = &client.locale;\n\n")
    if is_search:
        file_handle.write('        let mut url = format!("{base}/{endpoint}?namespace={namespace}&locale={locale}");\n')
        file_handle.write('        for (key, value) in params {\n')
        file_handle.write('            url.push_str(&format!("&{key}={value}"));\n')
        file_handle.write('        }\n')
        file_handle.write('        url\n')
    else:
        file_handle.write('        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")\n')
    file_handle.write("    }\n")
    file_handle.write("}\n\n")


def write_types(name:str, file_handle):
    file_handle.write(f"pub type {name}Result = Result<{name}, BattleNetClientError>;\n")
    file_handle.write(f"pub type {name}JsonResult = Result<String, BattleNetClientError>;\n")
    file_handle.write("\n")


def gen_group_models(group:str, config:dict, outdir:str):
    group_lower = group.lower()
    outpath = os.path.join(outdir, f"{group_lower}.rs")
    print(f"Generating model -> {outpath}")

    with open(outpath, 'w') as file:
        file.write("use serde::Deserialize;\n")
        file.write("\n")
        file.write("use crate::client::BattleNetClient;\n")
        file.write("use crate::errors::BattleNetClientError;\n")
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
