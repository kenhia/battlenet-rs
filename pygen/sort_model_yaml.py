#!/usr/bin/env python

import click
import os
import shutil
import yaml


# def gen_core_structs(config:dict, outdir:str):
#     outpath = os.path.join(outdir, 'core_structs.rs')
#     print(f"Generating core structs -> {outpath}")

#     with open(outpath, 'w') as file:
#         file.write("use serde::Deserialize;\n\n")
#         for struct in config['CoreStructs']:
#             write_structs(struct, config['CoreStructs'][struct], file)



# @click.command()
# @click.option('--model', '-m', help='Path to model YAML file')
# @click.option('--outdir', '-o', help='directory to output generated files')
# def main(model, outdir):
#     print(f"Generating model from YAML file: {model}")
#     with open(model, 'r') as f:
#         config = yaml.safe_load(f)
#     # print(config)
#     # print(config['_core_structs'])
#     if 'CoreStructs' in config:
#         gen_core_structs(config, outdir)
#     for struct in config:
#         if struct == 'CoreStructs':
#             continue
#         gen_group_models(struct, config, outdir)
#     # print(config.__class__)

def sort_yaml(path:str):
    print(f"Sorting YAML file: {path}")
    with open(path, 'r') as f:
        data = yaml.safe_load(f)
    sorted_keys = sorted(data.keys())

    with open(path, 'w') as f:
        # output _helpers first
        if '_helpers' in sorted_keys:
            # output _helpers node as YAML
            helpers_parent = {'_helpers': data['_helpers']}
            helpers = yaml.dump(helpers_parent)
            f.write(helpers)
            sorted_keys.remove('_helpers')
        for key in sorted_keys:
            # output key node as YAML
            node = {key: data[key]}
            f.write('\n')
            f.write(yaml.dump(node))


@click.command()
@click.option('--models-dir', '-d', help='Path to model YAML files')
def main(models_dir):
    # Get paths to all YAML files in the models_dir
    yaml_files = [f for f in os.listdir(models_dir) if f.endswith('.yaml')]
    # print(yaml_files)
    
    # create BACK directory if it doesn't exist, BACK should be subdirectory of models_dir
    back_dir = os.path.join(models_dir, 'BACK')
    if not os.path.exists(back_dir):
        os.makedirs(back_dir)
    # copy all YAML files to BACK directory
    for file in yaml_files:
        shutil.copy(os.path.join(models_dir, file), back_dir)

    for file in yaml_files:
        sort_yaml(os.path.join(models_dir, file))

if __name__ == '__main__':
    main()
