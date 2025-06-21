use crate::bindings::repl::api::transport::ReplVar;
use std::collections::HashMap;

/// A more efficient representation of environment variables using a HashMap
#[derive(Debug, Clone)]
pub struct ReplLogicVar {
    inner: HashMap<String, String>,
}

impl ReplLogicVar {
    /// Create a new empty EnvVars instance
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    /// Set a key-value pair
    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.inner.contains_key(key)
    }

    /// Get all keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.inner.keys()
    }

    /// Get all key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.inner.iter()
    }

    /// Expand variables in a string (e.g., "$HOME" -> "/home/user")
    pub fn expand_variables(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                // Handle $VAR syntax
                let mut var_name = String::new();
                while let Some(ch) = chars.peek() {
                    if ch.is_alphanumeric() || *ch == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if !var_name.is_empty() {
                    if let Some(value) = self.get(&var_name) {
                        result.push_str(value);
                    } else {
                        result.push_str(&format!("{}", ""));
                    }
                } else {
                    result.push('$');
                }
            } else {
                result.push(ch);
            }
        }

        result
    }
}

impl Default for ReplLogicVar {
    fn default() -> Self {
        Self::new()
    }
}

// Convert from Vec<ReplVar> to EnvVars
impl From<Vec<ReplVar>> for ReplLogicVar {
    fn from(env_vars: Vec<ReplVar>) -> Self {
        let mut map = HashMap::new();
        for env_var in env_vars {
            map.insert(env_var.key, env_var.value);
        }
        Self { inner: map }
    }
}

// Convert from EnvVars to Vec<ReplVar>
impl From<ReplLogicVar> for Vec<ReplVar> {
    fn from(env_vars: ReplLogicVar) -> Self {
        env_vars
            .inner
            .into_iter()
            .map(|(key, value)| ReplVar { key, value })
            .collect()
    }
}

// Convert from &[ReplVar] to EnvVars
impl From<&[ReplVar]> for ReplLogicVar {
    fn from(env_vars: &[ReplVar]) -> Self {
        let mut map = HashMap::new();
        for env_var in env_vars {
            map.insert(env_var.key.clone(), env_var.value.clone());
        }
        Self { inner: map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_vars_creation() {
        let env_vars = ReplLogicVar::new();
        assert!(env_vars.inner.is_empty());
    }

    #[test]
    fn test_env_vars_set_and_get() {
        let mut env_vars = ReplLogicVar::new();
        env_vars.set("HOME".to_string(), "/home/user".to_string());
        assert_eq!(env_vars.get("HOME"), Some(&"/home/user".to_string()));
        assert_eq!(env_vars.get("PATH"), None);
    }

    #[test]
    fn test_from_vec() {
        let vec_env_vars = vec![
            ReplVar {
                key: "HOME".to_string(),
                value: "/home/user".to_string(),
            },
            ReplVar {
                key: "PATH".to_string(),
                value: "/usr/bin".to_string(),
            },
        ];

        let env_vars: ReplLogicVar = vec_env_vars.into();
        assert_eq!(env_vars.get("HOME"), Some(&"/home/user".to_string()));
        assert_eq!(env_vars.get("PATH"), Some(&"/usr/bin".to_string()));
    }

    #[test]
    fn test_to_vec() {
        let mut env_vars = ReplLogicVar::new();
        env_vars.set("HOME".to_string(), "/home/user".to_string());
        env_vars.set("PATH".to_string(), "/usr/bin".to_string());

        let vec_env_vars: Vec<ReplVar> = env_vars.into();
        assert_eq!(vec_env_vars.len(), 2);

        let home_var = vec_env_vars.iter().find(|v| v.key == "HOME").unwrap();
        assert_eq!(home_var.value, "/home/user");

        let path_var = vec_env_vars.iter().find(|v| v.key == "PATH").unwrap();
        assert_eq!(path_var.value, "/usr/bin");
    }

    #[test]
    fn test_expand_variables() {
        let mut env_vars = ReplLogicVar::new();
        env_vars.set("HOME".to_string(), "/home/user".to_string());
        env_vars.set("USER".to_string(), "john".to_string());

        assert_eq!(env_vars.expand_variables("echo $HOME"), "echo /home/user");
        assert_eq!(
            env_vars.expand_variables("echo $HOME/$USER"),
            "echo /home/user/john"
        );
        assert_eq!(env_vars.expand_variables("echo $UNKNOWN"), "echo ");
        assert_eq!(env_vars.expand_variables("echo $"), "echo $");
    }
}
