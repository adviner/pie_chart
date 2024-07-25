
use image::{GenericImageView, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
use lopdf::{Document, Object, Stream, dictionary};
use std::fs::File;
use std::io::{Cursor, Read};


pub fn merge_image_to_pdf(image_path: &str, pdf_path: &str, pdf_output: &str) -> Result<(), Box<dyn std::error::Error>> {


   // Load the PDF document
   let mut doc = Document::load(pdf_path)?;
   println!("PDF loaded successfully.");

   // Load the image
   let mut image_file = File::open(image_path)?;
   let mut image_data = Vec::new();
   image_file.read_to_end(&mut image_data)?;
   let image = image::load_from_memory(&image_data)?;
   let (width, height) = image.dimensions();
   println!("Image loaded successfully with dimensions: {}x{}", width, height);

   // Convert the image to JPEG format if it's not already
   let mut jpeg_data = Vec::new();
   {
       let mut cursor = Cursor::new(&mut jpeg_data);
       let mut encoder = JpegEncoder::new(&mut cursor);
       encoder.encode_image(&image)?;
   }
   println!("Image converted to JPEG format.");

   // Create an image XObject
   let image_xobject = Stream::new(
       dictionary! {
           "Type" => "XObject",
           "Subtype" => "Image",
           "Width" => width as i32,
           "Height" => height as i32,
           "ColorSpace" => "DeviceRGB",
           "BitsPerComponent" => 8,
           "Filter" => "DCTDecode",
           "Length" => jpeg_data.len() as i32,
       },
       jpeg_data,
   );

   // Add the image to the document
   let image_id = doc.add_object(image_xobject);
   println!("Image XObject added to the document with ID: {:?}", image_id);

   // Get the first page
   let page_id = *doc.get_pages().keys().next().ok_or("No pages found in PDF")?;
   println!("First page ID: {:?}", page_id);

   // Add the image to the page resources
   {
       let page = doc.get_object_mut((page_id, 0))?;
       if let Object::Dictionary(ref mut page_dict) = *page {
           // Ensure the Resources dictionary exists
           if !page_dict.has(b"Resources") {
               page_dict.set("Resources", dictionary! {});
           }

           let resources = page_dict.get_mut(b"Resources").unwrap();
           if let Object::Dictionary(ref mut resources_dict) = *resources {
               // Ensure the XObject dictionary exists
               if !resources_dict.has(b"XObject") {
                   resources_dict.set("XObject", dictionary! {});
               }

               let xobjects = resources_dict.get_mut(b"XObject").unwrap();
               if let Object::Dictionary(ref mut xobjects_dict) = *xobjects {
                   xobjects_dict.set("Im0", image_id);
               }
           }
       }
   }
   println!("Image added to the page resources.");

   // Add the image to the page content at a specific location
   let x_position = 100.0; // X position where the image will be placed
   let y_position = 100.0; // Y position where the image will be placed

    // Check the Image Coordinates and Transformation Matrix
    let content = format!(
        "q\n{} 0 0 {} {} {} cm\n/Im0 Do\nQ\n",
        width, height, x_position, y_position
    );
   println!("Content stream created: {}", content);

   let content_stream = Stream::new(
       dictionary! {
           "Length" => content.len() as i32,
       },
       content.into_bytes(),
   );

   let content_id = doc.add_object(content_stream);
   println!("Content stream added to the document with ID: {:?}", content_id);

   // Ensure the Content Stream is Correctly Added
   {
       let page = doc.get_object_mut((page_id, 0))?;
       if let Object::Dictionary(ref mut page_dict) = *page {
           // Update the Contents entry
           if let Ok(contents) = page_dict.get_mut(b"Contents") {
               match contents {
                   Object::Reference(content_ref) => {
                       let content_array = vec![Object::Reference(*content_ref), Object::Reference(content_id)];
                       *contents = Object::Array(content_array);
                   }
                   Object::Array(content_array) => {
                       content_array.push(Object::Reference(content_id));
                   }
                   _ => {
                       *contents = Object::Array(vec![Object::Reference(content_id)]);
                   }
               }
           } else {
               println!("Contents does not exist");
               page_dict.set("Contents", Object::Reference(content_id));
           }
       }
   }
   println!("Content stream appended to the page.");

   // Save the modified PDF to the output path
   doc.save(&pdf_output)?;

    Ok(())
}

