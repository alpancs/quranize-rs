#![feature(test)]

extern crate test;

use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent::{EndElement, StartElement};
use xml::EventReader;

mod quran_simple_clean;

pub fn build_location_map() -> Vec<(u8, u16, u8)> {
    let mut sura_number = 0;
    let mut aya_number = 0;
    let mut location_map = vec![];
    for event in EventReader::new(quran_simple_clean::RAW_XML.as_bytes()) {
        match event {
            Ok(StartElement {
                name, attributes, ..
            }) => match name.to_string().as_str() {
                "sura" => {
                    sura_number += 1;
                }
                "aya" => {
                    aya_number += 1;
                    let mut word_number = 0;
                    location_map.push((sura_number, aya_number, word_number));

                    let aya_text = get_text_from_attributes(attributes);
                    for c in aya_text.chars() {
                        if c == ' ' {
                            word_number += 1;
                            location_map.push((sura_number, aya_number, word_number));
                        }
                    }
                }
                _ => {}
            },
            Ok(EndElement { name }) => {
                if name.to_string() == "sura" {
                    aya_number = 0;
                }
            }
            Err(error) => {
                println!("error: {}", error);
            }
            _ => {}
        }
    }
    location_map
}

fn get_text_from_attributes(attributes: Vec<OwnedAttribute>) -> String {
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
    use test::Bencher;

    #[test]
    fn test_build_location_map() {
        let location_map = build_location_map();
        assert_eq!(location_map.len(), 77800);
        assert_eq!(*location_map.first().unwrap(), (1, 1, 0));
        assert_eq!(*location_map.last().unwrap(), (114, 6, 2));
    }

    #[bench]
    fn bench_build_location_map(b: &mut Bencher) {
        b.iter(build_location_map);
    }
}
