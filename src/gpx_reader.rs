use crate::config::Config;
use geozero::geojson::GeoJsonWriter;
use serde_json::Value;
use std::io::Cursor;

pub fn read_polylines(config: &Config) -> Vec<Value> {
    config.routes.iter().map(|route_path| {
        let file_contents = std::fs::read(route_path)
            .expect(format!("Failed to read {}", route_path).as_str());
        
        let mut output = Vec::new();
        let mut writer = GeoJsonWriter::new(&mut output);
        geozero::gpx::read_gpx(&mut Cursor::new(file_contents), &mut writer).unwrap();
        
        let geo_json = String::from_utf8(output).unwrap();
        serde_json::from_str(&geo_json).unwrap()
    }).collect()
}
