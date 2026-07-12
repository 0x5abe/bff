use crate::BffResult;
use crate::names::Name;
use crate::source::preservation::PreservedFragment;
use crate::source::project::CookedProject;

pub struct Named<'a, T> {
    pub name: Name,
    pub value: &'a T,
}

pub struct SourcePartBuild<T> {
    pub part: T,
    pub represented_resources: Vec<Name>,
    pub preserved: Vec<PreservedFragment>,
}

impl<T> SourcePartBuild<T> {
    pub fn new(name: Name, part: T) -> Self {
        Self {
            part,
            represented_resources: vec![name],
            preserved: Vec::new(),
        }
    }
}

pub trait SourcePart: Sized {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self>;
}

pub trait ToSourcePart<P> {
    fn to_source_part(&self, name: Name) -> BffResult<P>;
}
