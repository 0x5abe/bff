use std::collections::HashSet;

use crate::BffResult;
use crate::class::ClassType;
use crate::names::Name;
use crate::source::part::SourcePart;
use crate::source::project::CookedProject;

pub struct UncookContext<'a> {
    project: &'a CookedProject,
    // Source assets and cooked resources are tracked separately because several
    // source assets may read from the same cooked resource.
    built_source_assets: HashSet<(ClassType, Name)>,
    represented_resources: HashSet<Name>,
}

impl<'a> UncookContext<'a> {
    pub fn new(project: &'a CookedProject) -> Self {
        Self {
            project,
            built_source_assets: HashSet::new(),
            represented_resources: HashSet::new(),
        }
    }

    pub const fn project(&self) -> &'a CookedProject {
        self.project
    }

    pub fn mark_source_asset_built(&mut self, class_type: ClassType, name: Name) {
        self.built_source_assets.insert((class_type, name));
    }

    pub fn is_source_asset_built(&self, class_type: ClassType, name: &Name) -> bool {
        self.built_source_assets.contains(&(class_type, *name))
    }

    pub fn mark_represented_resource(&mut self, name: Name) {
        self.represented_resources.insert(name);
    }

    pub const fn represented_resources(&self) -> &HashSet<Name> {
        &self.represented_resources
    }

    pub fn require_part<P: SourcePart>(&self, name: &Name) -> BffResult<P> {
        P::from_project(name, self.project)
    }
}
