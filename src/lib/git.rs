// Github cli
pub enum CommitOptions {
     // This will be implemented but not recommended
     OnSave,
     // On a custom condition (for plugins)
     OnCondition(FnMut -> bool),
     // On the built in filter
     OnFilter(Filter)
}

pub struct Filter {
     
}