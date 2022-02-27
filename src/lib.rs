use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent::StartElement;
use xml::EventReader;

mod quran_simple_clean;

pub fn build_location_map() -> Vec<(u8, u16, u8)> {
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
                    let aya_number = get_index_from_attributes(&attributes);
                    let mut word_index = 0;
                    location_map.push((sura, aya_number, word_index));

                    let aya_text = get_text_from_attributes(&attributes);
                    for c in aya_text.chars() {
                        if c == ' ' {
                            word_index += 1;
                            location_map.push((sura, aya_number, word_index));
                        }
                    }
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

fn get_index_from_attributes(attributes: &Vec<OwnedAttribute>) -> u16 {
    attributes
        .iter()
        .find(|a| a.name.to_string() == "index")
        .unwrap()
        .value
        .parse::<u16>()
        .unwrap()
}

fn get_text_from_attributes(attributes: &Vec<OwnedAttribute>) -> String {
    attributes
        .iter()
        .find(|a| a.name.to_string() == "text")
        .unwrap()
        .value
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_location_map() {
        let location_map = build_location_map();
        assert_eq!(location_map.len(), 77800);
        assert_eq!(*location_map.first().unwrap(), (1, 1, 0));
        assert_eq!(*location_map.last().unwrap(), (114, 6, 2));
    }
}
