use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent::StartElement;
use xml::EventReader;

mod quran_simple_clean;

pub fn build_location_map() -> Vec<(u8, u16)> {
    let mut sura = 0;
    let mut location_map = vec![];
    for event in EventReader::new(quran_simple_clean::RAW_XML.as_bytes()) {
        match event {
            Ok(StartElement {
                name, attributes, ..
            }) => match name.to_string().as_str() {
                "sura" => {
                    sura += 1;
                }
                "aya" => {
                    let aya = get_index_from_attributes(attributes);
                    location_map.push((sura, aya));
                }
                _ => {}
            },
            Err(error) => {
                println!("error: {}", error);
            }
            _ => {}
        }
    }
    location_map
}

fn get_index_from_attributes(attributes: Vec<OwnedAttribute>) -> u16 {
    attributes
        .iter()
        .find(|a| a.name.to_string() == "index")
        .unwrap()
        .value
        .parse::<u16>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_location_map() {
        let location_map = build_location_map();
        assert_eq!(location_map.len(), 6236);
        assert_eq!(*location_map.first().unwrap(), (1, 1));
        assert_eq!(*location_map.last().unwrap(), (114, 6));
    }
}
