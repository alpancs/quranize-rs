#![feature(test)]

extern crate test;

use xml::reader::XmlEvent::{EndElement, StartElement};
use xml::EventReader;

mod harf;
use harf::Harf;
mod quran_simple_clean;

pub fn build_quran_index() -> Harf {
    let mut root = Harf::new('*');
    let mut sura_number = 0;
    let mut aya_number = 0;

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
                    let aya_chars: Vec<_> = attributes
                        .iter()
                        .find(|a| a.name.to_string() == "text")
                        .unwrap()
                        .value
                        .chars()
                        .collect();
                    let mut word_number = 0;
                    for i in 0..aya_chars.len() {
                        if i == 0 || aya_chars[i - 1] == ' ' {
                            word_number += 1;
                            let mut node = &mut root;
                            for j in i..aya_chars.len() {
                                let c = aya_chars[j];
                                let pos = node.next_harfs.iter().position(|h| h.content == c);
                                node = match pos {
                                    Some(index) => node.next_harfs.get_mut(index).unwrap(),
                                    None => {
                                        node.next_harfs.push(Harf::new(c));
                                        node.next_harfs.last_mut().unwrap()
                                    }
                                };
                                if j == aya_chars.len() - 1 || aya_chars[j + 1] == ' ' {
                                    let location = (sura_number, aya_number, word_number);
                                    node.locations.push(location);
                                }
                            }
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
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_build_quran_index() {
        let quran_index = build_quran_index();
        assert_eq!(quran_index.content, '*');
        assert_eq!(quran_index.next_harfs.len(), 31);

        let ba = quran_index
            .next_harfs
            .iter()
            .find(|h| h.content == 'ب')
            .unwrap();
        assert!(ba.locations.is_empty());

        let nun = quran_index
            .next_harfs
            .iter()
            .find(|h| h.content == 'ن')
            .unwrap();
        assert_eq!(nun.locations.len(), 1);
    }

    #[bench]
    fn bench_build_quran_index(b: &mut Bencher) {
        b.iter(build_quran_index);
    }
}
