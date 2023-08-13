use std::error::Error;
use std::env;
use std::fs::File;
use std::io::BufReader;
use crate::scene::Scene;

pub fn load_scene() -> Result<Scene, Box<dyn Error + Send + Sync>> {
    let xml_path = env::args()
        .nth(1)
        .ok_or("No XML file specified")?;
    let xml_file = File::open(xml_path)?;
    let ref mut xml_reader = BufReader::new(xml_file);
    quick_xml::de::from_reader(xml_reader).map_err(|e| e.into())
}
