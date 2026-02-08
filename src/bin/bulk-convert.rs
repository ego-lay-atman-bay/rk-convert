use std::fs::File;
use std::io;
use std::env;
use std::io::BufReader;
use std::path::Path;

use rk_convert::glb::rk_to_gltf;
use xml::EventReader;
use xml::reader;
use xml::reader::XmlEvent;

pub struct GameObject {
    pub id: String,
    pub model_name: String,
    pub anim_name: String,
}

pub struct BulkConvertor<I: AsRef<Path>, O: AsRef<Path>> {
    game_folder: I,
    output_folder: O,
}

impl<I: AsRef<Path>, O: AsRef<Path>> BulkConvertor<I, O> {
    pub fn new(game_folder: I, output_folder: O) -> BulkConvertor<I,O> {
        BulkConvertor {
            game_folder,
            output_folder,
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        let game_object_data_file = self.game_folder.as_ref().join("gameobjectdata.xml");
        let game_object_data_reader = File::open(game_object_data_file)?;
        let game_object_data_buffer = BufReader::new(game_object_data_reader);
        let game_object_data_parser = EventReader::new(game_object_data_buffer);
        let mut game_object_data_iter = game_object_data_parser.into_iter();


        self.read_game_object_data(&mut game_object_data_iter);

        Ok(())
    }

    fn read_game_object_data(
        &mut self,
        iter: &mut impl Iterator<Item = reader::Result<XmlEvent>>,
    ) -> reader::Result<()> {
        loop {
            let evt = match iter.next() {
                Some(x) => x?,
                None => break,
            };
            match evt {
                XmlEvent::StartDocument { .. } => {},
                XmlEvent::EndDocument => break,
                XmlEvent::ProcessingInstruction { .. } => {},

                XmlEvent::StartElement { ref name, ref attributes, .. }
                if name.local_name == "Category" => {
                    let mut id = None;
                    for attr in attributes {
                        if attr.name.local_name == "ID" {
                            assert!(id.is_none(), "duplicate attribute ID");
                            id = Some(attr.value.clone());
                        }
                    }

                    if id.unwrap() != "Pony" {
                        continue;
                    }

                    self.read_objects(iter);
                },

                
                XmlEvent::StartElement { .. } => {},
                XmlEvent::EndElement { .. } => {},
                XmlEvent::CData(..) => {},
                XmlEvent::Comment(..) => {},
                XmlEvent::Characters(..) => {},
                XmlEvent::Whitespace(..) => {},
            }
        }

        Ok(())
    }

    fn read_objects(
        &mut self,
        iter: &mut impl Iterator<Item = reader::Result<XmlEvent>>,
    ) -> reader::Result<()> {
        let mut num_objects = 0;
        loop {
            let evt = match iter.next() {
                Some(x) => x?,
                None => break,
            };
            match evt {
                XmlEvent::StartDocument { .. } => {},
                XmlEvent::EndDocument => break,
                XmlEvent::ProcessingInstruction { .. } => {},

                XmlEvent::StartElement { ref name, ref attributes, .. }
                // if name.local_name == "GameObject"
                 => {
                    num_objects += 1;
                    let mut id = None;
                    for attr in attributes {
                        if attr.name.local_name == "ID" {
                            assert!(id.is_none(), "duplicate attribute ID");
                            id = Some(attr.value.clone());
                        }
                    }

                    if id.is_none() {
                        // println!("Couldn't find ID: {}", name);
                        continue;
                    }

                    self.read_object(iter, id.unwrap())?;
                },

                
                // XmlEvent::StartElement { .. } => {},
                XmlEvent::EndElement { ref name, .. } if name.local_name == "Category" => break,
                XmlEvent::EndElement { .. } => {},
                XmlEvent::CData(..) => {},
                XmlEvent::Comment(..) => {},
                XmlEvent::Characters(..) => {},
                XmlEvent::Whitespace(..) => {},
            }
        }

        println!("Total ponies: {num_objects}");

        Ok(())
    }
    
    fn read_object(
        &mut self,
        iter: &mut impl Iterator<Item = Result<XmlEvent, reader::Error>>,
        id: String,
    ) -> reader::Result<()> {
        let mut model_name = None;
        let mut anim_name = None;
        

        loop {
            let evt = match iter.next() {
                Some(x) => x?,
                None => break,
            };
            match evt {
                XmlEvent::StartDocument { .. } => {},
                XmlEvent::EndDocument => break,
                XmlEvent::ProcessingInstruction { .. } => {},

                XmlEvent::StartElement { ref name, ref attributes, .. }
                if name.local_name == "Model" => {
                    for attr in attributes {
                        if attr.name.local_name == "MediumLOD" {
                            assert!(model_name.is_none(), "duplicate attribute MediumLOD");
                            model_name = Some(attr.value.clone());
                        }
                    }
                },

                XmlEvent::StartElement { ref name, ref attributes, .. }
                if name.local_name == "Animation" => {
                    for attr in attributes {
                        if attr.name.local_name == "Rig_MediumLOD" {
                            assert!(anim_name.is_none(), "duplicate attribute Rig_MediumLOD");
                            anim_name = Some(attr.value.clone());
                        }
                    }
                },

                
                XmlEvent::StartElement { .. } => {},
                XmlEvent::EndElement { ref name, .. } if name.local_name == "GameObject" => break,
                XmlEvent::EndElement { .. } => {},
                XmlEvent::CData(..) => {},
                XmlEvent::Comment(..) => {},
                XmlEvent::Characters(..) => {},
                XmlEvent::Whitespace(..) => {},
            }
        }

        if model_name.is_none() || anim_name.is_none() {
            println!("Skipping: {}", id);
            return Ok(());
        }

        let game_object = GameObject {
            id,
            model_name: model_name.unwrap(),
            anim_name: anim_name.unwrap()
        };

        self.convert_model(game_object);

        Ok(())
    }

    fn convert_model(&mut self, game_object: GameObject) -> io::Result<()> {
        
        let model_path = self.game_folder.as_ref().join(&game_object.model_name).with_extension("rk");
        let anim_path = self.game_folder.as_ref().join(&game_object.anim_name).with_extension("csv");
        let output_path = self.output_folder.as_ref().join(&game_object.id).with_extension("glb");
        // println!("model_path: {}", model_path.display());
        // println!("anim_path: {}", anim_path.display());
        // println!("output_path: {}", output_path.display());
        
        if !output_path.exists() {
            rk_to_gltf(model_path, Some(anim_path), output_path)?;
            println!("Converting game object {}", game_object.id);
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    
    let args = env::args_os().collect::<Vec<_>>();
    assert!(
        args.len() == 3,
        "usage: {} <game folder> <output folder>",
        args[0].to_string_lossy(),
    );

    let game_folder = &args[1];
    let output_folder = &args[2];

    let mut bulk_convertor = BulkConvertor::new(game_folder, output_folder);
    let _ = bulk_convertor.read();

    Ok(())
}
