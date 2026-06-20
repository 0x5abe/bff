use std::collections::HashSet;

use crate::names::Name;
use crate::source::project::CookedProject;

pub struct UncookContext<'a> {
    project: &'a CookedProject,
    consumed_resources: HashSet<Name>,
}

impl<'a> UncookContext<'a> {
    pub fn new(project: &'a CookedProject) -> Self {
        Self {
            project,
            consumed_resources: HashSet::new(),
        }
    }

    pub fn project(&self) -> &'a CookedProject {
        self.project
    }

    pub fn mark_consumed(&mut self, name: Name) {
        self.consumed_resources.insert(name);
    }

    pub fn is_consumed(&self, name: &Name) -> bool {
        self.consumed_resources.contains(name)
    }

    pub fn consumed_resources(&self) -> &HashSet<Name> {
        &self.consumed_resources
    }
}
