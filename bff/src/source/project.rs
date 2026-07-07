use std::collections::{HashMap, HashSet};

use crate::bigfile::BigFile;
use crate::bigfile::dependency::DependencyIndex;
use crate::class::ClassType;
use crate::class::bff_class::BffClass;
use crate::names::{Name, NameContext};
use crate::source::asset::{SourceAsset, SourceAssetBuildFailure, build_source_assets};
use crate::source::context::UncookContext;
use crate::traits::ReferencedNames as _;

pub struct SourceProject {
    pub assets: Vec<SourceAsset>,
    pub failures: Vec<SourceAssetBuildFailure>,
    pub represented_resources: HashSet<Name>,
}

impl SourceProject {
    pub fn from_bigfile(bigfile: &BigFile, name_context: &NameContext) -> Self {
        name_context.scope(|| {
            let project = CookedProject::from_bigfile(bigfile, name_context);
            Self::from_cooked_project(&project)
        })
    }

    pub fn from_cooked_project(project: &CookedProject) -> Self {
        let mut ctx = UncookContext::new(project);
        let (assets, failures) = build_source_assets(project, &mut ctx);

        Self {
            assets,
            failures,
            represented_resources: ctx.represented_resources().clone(),
        }
    }
}

pub struct CookedProject {
    classes: HashMap<Name, BffClass>,
    class_type_by_name: HashMap<Name, ClassType>,
    names_by_class_type: HashMap<ClassType, Vec<Name>>,
    dependencies: DependencyIndex,
}

impl CookedProject {
    pub fn from_bigfile(bigfile: &BigFile, name_context: &NameContext) -> Self {
        let mut classes = HashMap::new();
        let mut class_type_by_name = HashMap::new();
        let mut names_by_class_type: HashMap<ClassType, Vec<Name>> = HashMap::new();

        for bff_resource in bigfile.bff_resources() {
            let name = bff_resource.resource.name;
            let Ok(bff_class) = bff_resource.bff_class(name_context) else {
                continue;
            };

            let class_type = bff_class.class.class_type();

            class_type_by_name.insert(name, class_type);
            names_by_class_type
                .entry(class_type)
                .or_default()
                .push(name);
            classes.insert(name, bff_class);
        }

        let dependencies = DependencyIndex::from_references(
            classes
                .iter()
                .map(|(&name, bff_class)| (name, bff_class.class.referenced_names())),
        );

        Self {
            classes,
            class_type_by_name,
            names_by_class_type,
            dependencies,
        }
    }

    pub fn class(&self, name: &Name) -> Option<&BffClass> {
        self.classes.get(name)
    }

    pub fn class_type(&self, name: &Name) -> Option<ClassType> {
        self.class_type_by_name.get(name).copied()
    }

    pub fn names_of_type(&self, class_type: ClassType) -> Option<&Vec<Name>> {
        self.names_by_class_type.get(&class_type)
    }

    pub const fn dependencies(&self) -> &DependencyIndex {
        &self.dependencies
    }

    pub fn outgoing_of_type(&self, name: &Name, class_type: ClassType) -> Vec<Name> {
        let Some(names) = self.dependencies.outgoing(name) else {
            return Vec::new();
        };

        names
            .iter()
            .filter(|to| self.class_type(to) == Some(class_type))
            .copied()
            .collect()
    }

    pub fn incoming_of_type(&self, name: &Name, class_type: ClassType) -> Vec<Name> {
        let Some(names) = self.dependencies.incoming(name) else {
            return Vec::new();
        };

        names
            .iter()
            .filter(|to| self.class_type(to) == Some(class_type))
            .copied()
            .collect()
    }
}
