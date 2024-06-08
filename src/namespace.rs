use crate::region::BattleNetRegion;

pub enum WowNamespace {
    Static,
    Dynamic,
    Profile,
}

impl WowNamespace {
    pub fn to_region_string(&self, region: &BattleNetRegion) -> String {
        let r = region.to_str();
        let pre = match self {
            Self::Static => "static",
            Self::Dynamic => "dynamic",
            Self::Profile => "profile",
        };
        format!("{pre}-{r}")
    }
}
