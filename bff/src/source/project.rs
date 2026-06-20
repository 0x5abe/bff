use std::collections::HashMap;

use crate::bigfile::BigFile;
use crate::bigfile::dependency::DependencyIndex;
use crate::bigfile::resource::{BffClass, BffResourceHeader, Resource};
use crate::class::{Class, ClassType};
use crate::names::Name;
use crate::traits::{ReferencedNames, TryIntoVersionPlatform};

pub struct CookedProject {
    classes: HashMap<Name, BffClass>,
    class_type_by_name: HashMap<Name, ClassType>,
    names_by_class_type: HashMap<ClassType, Vec<Name>>,
    dependencies: DependencyIndex,
}

impl CookedProject {
    pub fn from_bigfile(bigfile: &BigFile) -> Self {
        let mut classes = HashMap::new();
        let mut class_type_by_name = HashMap::new();
        let mut names_by_class_type: HashMap<ClassType, Vec<Name>> = HashMap::new();

        for (&name, resource) in &bigfile.resources {
            let header = BffResourceHeader {
                platform: bigfile.manifest.platform,
                version: bigfile.manifest.version.clone(),
            };

            let Ok(class) = <&Resource as TryIntoVersionPlatform<Class>>::try_into_version_platform(
                resource,
                bigfile.manifest.version.clone(),
                bigfile.manifest.platform,
            ) else {
                continue;
            };

            let class_type = class.class_type();

            class_type_by_name.insert(name, class_type);
            names_by_class_type
                .entry(class_type)
                .or_default()
                .push(name);
            classes.insert(name, BffClass { header, class });
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

    pub fn dependencies(&self) -> &DependencyIndex {
        &self.dependencies
    }

    pub fn outgoing_of_type(&self, name: &Name, class_type: ClassType) -> Vec<Name> {
        self.dependencies
            .outgoing(name)
            .into_iter()
            .flatten()
            .filter(|to| self.class_type(to) == Some(class_type))
            .cloned()
            .collect()
    }

    pub fn incoming_of_type(&self, name: &Name, class_type: ClassType) -> Vec<Name> {
        self.dependencies
            .incoming(name)
            .into_iter()
            .flatten()
            .filter(|to| self.class_type(to) == Some(class_type))
            .cloned()
            .collect()
    }
}
