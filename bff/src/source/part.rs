use crate::BffResult;
use crate::names::Name;
use crate::source::project::CookedProject;

pub struct Named<'a, T> {
    pub name: Name,
    pub value: &'a T,
}

pub trait SourcePart: Sized {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self>;
}

pub trait ToSourcePart<P> {
    fn to_source_part(&self, name: Name) -> BffResult<P>;
}
