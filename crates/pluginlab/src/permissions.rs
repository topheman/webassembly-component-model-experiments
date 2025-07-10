use crate::cli::Cli;

pub enum NetworkPermissions {
    None,
    All,
    Custom(Vec<String>),
}

impl From<&Cli> for NetworkPermissions {
    fn from(cli: &Cli) -> Self {
        if cli.allow_net.is_some() {
            if cli.allow_net.as_ref().unwrap() == "@" {
                return NetworkPermissions::All;
            } else {
                let domains: Vec<String> = cli
                    .allow_net
                    .as_ref()
                    .unwrap()
                    .split(",")
                    .map(|d| d.to_string())
                    .collect();
                return NetworkPermissions::Custom(domains);
            }
        }
        return NetworkPermissions::None;
    }
}

impl NetworkPermissions {
    pub fn is_allowed(&self, domain: &str) -> bool {
        match self {
            NetworkPermissions::None => false,
            NetworkPermissions::All => true,
            NetworkPermissions::Custom(domains) => domains.contains(&domain.to_string()),
        }
    }
}
