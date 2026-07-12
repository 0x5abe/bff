use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use bff::bigfile::BigFile;
use bff::bigfile::platforms::Platform;
use bff::bigfile::resource::bff_resource::BffResourceRef;
use bff::bigfile::versions::Version;
use bff::names::NameContext;
use bff::source::asset::SourceAsset;
use bff::source::project::SourceProject;
use bff::traits::Export as _;
use clap::ValueEnum;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{ParallelBridge as _, ParallelIterator as _};

use crate::error::{BffCliError, BffCliResult};
use crate::shared::{
    probe_bigfile_name_context,
    read_bigfile,
    read_bigfile_names,
    read_in_names,
    resource_json_path,
    write_artifacts,
};

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ExportStrategy {
    #[value(alias("b"))]
    Binary,
    #[value(alias("r"))]
    Rich,
    #[value(alias("s"))]
    Source,
}

pub struct ExportOptions<'a> {
    pub strategy: ExportStrategy,
    pub rich_suffix: &'a str,
    pub debug_rich_errors: bool,
}

const INVALID_PATH_CHARS: [u8; 41] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 34, 42, 47, 58, 60, 62, 63, 92, 124,
];

fn clean_path(path: String) -> String {
    path.chars()
        .map(|v| {
            if INVALID_PATH_CHARS.contains(&(v as u8)) {
                '_'
            } else {
                v
            }
        })
        .collect()
}

fn strip_suffix_if_exists(s: String, suffix: &str) -> String {
    let mut s = s;
    if let Some(stripped) = s.strip_suffix(suffix) {
        s.truncate(stripped.len());
    }
    s
}

fn dump_bff_resource(
    resources_path: &Path,
    bff_resource: &BffResourceRef,
    name_context: &NameContext,
) -> BffCliResult<()> {
    let class_name = bff_resource
        .resource
        .class_name
        .with_context(name_context)
        .to_string();
    let name = clean_path(strip_suffix_if_exists(
        bff_resource
            .resource
            .name
            .with_context(name_context)
            .to_string(),
        &format!(".{}", class_name),
    ));
    let mut path = resources_path.join(format!("{}.{}", name, class_name));
    let mut i = 0;
    while path.exists() {
        path.set_file_name(format!("{}_{}.{}", name, i, class_name));
        i += 1;
    }
    let mut writer = BufWriter::new(File::create(path)?);
    bff_resource.write(&mut writer, name_context)?;
    Ok(())
}

fn export_bff_resource(
    resources_path: &Path,
    bff_resource: &BffResourceRef,
    name_context: &NameContext,
    rich_suffix: &str,
) -> BffCliResult<()> {
    let bff_class = bff_resource.bff_class(name_context)?;

    let class_name = bff_resource
        .resource
        .class_name
        .with_context(name_context)
        .to_string();
    let name = clean_path(strip_suffix_if_exists(
        bff_resource
            .resource
            .name
            .with_context(name_context)
            .to_string(),
        &format!(".{}", class_name),
    ));
    let mut directory = resources_path.join(format!("{}.{}{}", name, class_name, rich_suffix));
    let mut i = 0;
    while directory.exists() {
        directory.set_file_name(format!("{}_{}.{}{}", name, i, class_name, rich_suffix));
        i += 1;
    }

    std::fs::create_dir(&directory)?;

    let resource_serialized_path = resource_json_path(&directory);
    let resource_serialized_writer = BufWriter::new(File::create(resource_serialized_path)?);
    bff::names::json::to_writer_pretty(resource_serialized_writer, &bff_class, name_context)?;

    if let Ok(artifacts) = bff_class.class.export() {
        write_artifacts(&directory, artifacts)?;
    }

    Ok(())
}

fn export_source_asset(
    resources_path: &Path,
    bigfile: &BigFile,
    source_asset: &SourceAsset,
    name_context: &NameContext,
) -> BffCliResult<()> {
    let class_name = bigfile
        .bff_resource(source_asset.name)
        .map(|bff_resource| {
            bff_resource
                .resource
                .class_name
                .with_context(name_context)
                .to_string()
        })
        .unwrap_or_else(|| format!("{:?}", source_asset.class_type()));
    let name = clean_path(strip_suffix_if_exists(
        source_asset.name.with_context(name_context).to_string(),
        &format!(".{}", class_name),
    ));
    let mut directory = resources_path.join(format!("{}.{}.s", name, class_name));
    let mut i = 0;
    while directory.exists() {
        directory.set_file_name(format!("{}_{}.{}.s", name, i, class_name));
        i += 1;
    }

    std::fs::create_dir(&directory)?;

    let source_serialized_path = directory.join("source.json");
    let source_serialized_writer = BufWriter::new(File::create(source_serialized_path)?);
    bff::names::json::to_writer_pretty(source_serialized_writer, source_asset, name_context)?;

    Ok(())
}

pub fn extract(
    bigfile_path: &Path,
    directory: &Path,
    in_names: &[PathBuf],
    platform_override: Option<Platform>,
    version_override: Option<&Version>,
    export_options: ExportOptions<'_>,
) -> BffCliResult<()> {
    let mut name_context =
        probe_bigfile_name_context(bigfile_path, platform_override, version_override)?;
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message("Reading names");
    read_bigfile_names(bigfile_path, &mut name_context)?;
    read_in_names(in_names, &mut name_context)?;

    progress_bar.set_message("Reading BigFile");
    let bigfile = read_bigfile(
        bigfile_path,
        platform_override,
        version_override,
        &name_context,
    )?;

    let source_project = match export_options.strategy {
        ExportStrategy::Source => {
            progress_bar.set_message("Building source project");
            Some(SourceProject::from_bigfile(&bigfile, &name_context))
        }
        _ => None,
    };

    progress_bar.set_message("Writing manifest");
    std::fs::create_dir_all(directory)?;

    let manifest_path = directory.join("manifest.json");
    let manifest_writer = BufWriter::new(File::create(manifest_path)?);
    bff::names::json::to_writer_pretty(manifest_writer, bigfile.manifest(), &name_context)?;

    progress_bar.set_style(ProgressStyle::default_bar());
    let resources_path = directory.join("resources");
    std::fs::create_dir(&resources_path)?;

    progress_bar.set_length(
        (bigfile.bff_resources().len()
            + source_project
                .as_ref()
                .map(|source_project| source_project.assets.len())
                .unwrap_or_default()) as u64,
    );

    if let Some(source_project) = &source_project {
        progress_bar.set_message("Writing source assets");
        for source_asset in &source_project.assets {
            export_source_asset(&resources_path, &bigfile, source_asset, &name_context)?;
            progress_bar.inc(1);
        }
    }

    let represented_resources = source_project
        .as_ref()
        .map(|source_project| &source_project.represented_resources);

    progress_bar.set_message("Writing resources");
    bigfile
        .bff_resources()
        .par_bridge()
        .try_for_each(|bff_resource| {
            progress_bar.inc(1);
            if represented_resources
                .map(|represented_resources| {
                    represented_resources.contains(&bff_resource.resource.name)
                })
                .unwrap_or(false)
            {
                return Ok(());
            }

            let rich_export_failed = if matches!(export_options.strategy, ExportStrategy::Binary) {
                false
            } else {
                match export_bff_resource(
                    &resources_path,
                    &bff_resource,
                    &name_context,
                    export_options.rich_suffix,
                ) {
                    Ok(()) => false,
                    Err(error) => {
                        if export_options.debug_rich_errors {
                            let resource_name = bff_resource
                                .resource
                                .name
                                .with_context(&name_context)
                                .to_string();
                            let class_name = bff_resource
                                .resource
                                .class_name
                                .with_context(&name_context)
                                .to_string();
                            progress_bar.suspend(|| {
                                eprintln!(
                                    "Rich export failed for {resource_name}.{class_name}: {error:#?}"
                                );
                            });
                        }
                        true
                    }
                }
            };

            if matches!(export_options.strategy, ExportStrategy::Binary) || rich_export_failed {
                dump_bff_resource(&resources_path, &bff_resource, &name_context)?;
            }

            Ok::<(), BffCliError>(())
        })?;

    progress_bar.finish_and_clear();

    Ok(())
}
