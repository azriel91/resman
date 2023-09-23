use std::fmt;

/// Indicates a resource that did not exist in `Resources`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceFetchError {
    /// Short type name of the `CmdBlock::Input` type.
    pub resource_name_short: String,
    /// Full type name of the `CmdBlock::Input` type.
    pub resource_name_full: String,
}

impl ResourceFetchError {
    /// Returns a new `ResourceFetchError` with the given type's name.
    pub fn new<R>() -> Self {
        let resource_name_short = tynm::type_name::<R>();
        let resource_name_full = std::any::type_name::<R>().to_string();

        Self {
            resource_name_short,
            resource_name_full,
        }
    }

    /// Returns the short type name, e.g. `String`
    ///
    /// For generic types, this returns the short type names for the type and
    /// the parameter, e.g. `Option<String>`.
    pub fn resource_name_short(&self) -> &str {
        self.resource_name_short.as_ref()
    }

    /// Returns the full type name, e.g. `std::string::String`
    ///
    /// For generic types, this returns the short type names for the type and
    /// the parameter, e.g. `std::option::Option<std::string::String>`.
    pub fn resource_name_full(&self) -> &str {
        self.resource_name_full.as_ref()
    }
}

impl fmt::Display for ResourceFetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            resource_name_short,
            resource_name_full: _,
        } = self;

        write!(
            f,
            "Failed to fetch `{resource_name_short}` from `resources`."
        )
    }
}

impl std::error::Error for ResourceFetchError {}
