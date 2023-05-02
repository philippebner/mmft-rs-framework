// Not used at the moment, cf. macros crate
pub trait MMFTInterface {
    fn schema() -> String;
    fn from_json(str: &str) -> Self;
    fn to_json(&self) -> String;
}
