use std::{fmt::Display, str::FromStr};

pub struct Manifest(toml_edit::DocumentMut);

impl Manifest {
    /// Gets the dependency from the manifest if it exists
    pub fn get_dependency(&self, name: &str) -> Option<Dependency> {
        self.0
            .get("dependencies")
            .and_then(|dep| dep.get(name))
            .map(Dependency)
    }

    /// Gets the dependency mutably from the manifest if it exists
    pub fn get_dependency_mut(&mut self, name: &str) -> Option<DependencyMut> {
        self.0
            .get_mut("dependencies")
            .and_then(|dep| dep.get_mut(name))
            .map(DependencyMut)
    }

    /// Gets the names of all the dependencies from the manifest
    pub fn get_dependency_names(&self) -> Option<impl Iterator<Item = &'_ str>> {
        self.0
            .get("dependencies")
            .and_then(|item| item.as_table())
            .map(|table| table.iter().map(|(name, _)| name))
    }
}

pub struct Dependency<'a>(&'a toml_edit::Item);
pub struct DependencyMut<'a>(&'a mut toml_edit::Item);

impl<'a> Dependency<'a> {
    /// Gets the version of the dependency by searching first
    /// the value of the entry and then as a inline table
    pub fn get_version(&self) -> Option<&'a str> {
        if let Some(version) = self.0.get("version").and_then(|i| i.as_str()) {
            Some(version)
        } else if let Some(table) = self.0.as_inline_table() {
            table.get("version").and_then(|i| i.as_str())
        } else {
            None
        }
    }
}

impl<'a> DependencyMut<'a> {
    /// Sets the version of the dependency by searching first
    /// the value of the entry and then as a inline table.
    /// Returns whether it was successful
    pub fn set_version(&mut self, version: &str) -> bool {
        if let Some(value) = self
            .0
            .as_inline_table_mut()
            .and_then(|table| table.get_mut("version"))
        {
            *value = version.into();
            true
        } else if let Some(value) = self.0.as_value_mut() {
            *value = version.into();
            true
        } else {
            false
        }
    }
}

impl FromStr for Manifest {
    type Err = toml_edit::TomlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<toml_edit::DocumentMut>().map(Manifest)
    }
}

impl Display for Manifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
