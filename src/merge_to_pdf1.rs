use image::GenericImageView;
//use image::io::Reader as ImageReader;
use image::ImageReader;
use lopdf::{Document, Object, Stream, dictionary};
use std::fs::File;
use std::io::Read;

//use image::ImageReader;
//use image::
//use image::image_reader::image_reader_type::ImageReader;

// use printpdf::{PdfDocument, Mm, Image};
// use lopdf::Document as LopdfDocument;
// use image::codecs::png::PngDecoder;

pub fn merge_image_to_pdf(image_path: &str, pdf_path: &str, pdf_output: &str) -> Result<(), Box<dyn std::error::Error>> {


    // Load the image
    let img = ImageReader::open(image_path)?.decode()?;
    let (width, height) = img.dimensions();
    let img = img.to_rgba8();

    // Load the existing PDF
    let mut doc = Document::load(pdf_path)?;

    // Create an XObject for the image
    let image_xobject = Stream::new(
        lopdf::dictionary! {
            "Type" => "XObject",
            "Subtype" => "Image",
            "Width" => width as i32,
            "Height" => height as i32,
            "ColorSpace" => "DeviceRGB",
            "BitsPerComponent" => 8,
            "Filter" => "DCTDecode",
        },
        img.into_raw(),
    );

    let image_id = doc.add_object(image_xobject);

    // Get the first page
    let page_id = *doc.get_pages().keys().next().ok_or("No pages found in PDF")?;
    
    // Add the image to the page content
    let content = format!(
        "q\n{} 0 0 {} 0 0 cm\n/{} Do\nQ\n",
        width, height, image_id.0
    );

    let content_stream = Stream::new(
        lopdf::dictionary! {
            "Length" => content.len() as i32,
        },
        content.into_bytes(),
    );

    let content_id = doc.add_object(content_stream);

    // Modify the page content
    {
        let page = doc.get_object_mut((page_id, 0))?;
        if let Object::Dictionary(ref mut page_dict) = *page {
            page_dict.set("Contents", content_id);
        }
    }


    // Save the modified PDF to the output path
    doc.save(pdf_output)?;



    /* 
   // Load the image
   let img = ImageReader::open(image_path)?.decode()?;
   let (width, height) = img.dimensions();
   let img = img.to_rgba8();

   // Convert the image to raw bytes
   let img_data = img.into_raw();

   // Load the existing PDF
   let mut doc = Document::load(pdf_path)?;

   // Create an XObject for the image
   let image_xobject = Stream::new(
       lopdf::dictionary! {
           "Type" => "XObject",
           "Subtype" => "Image",
           "Width" => width as i32,
           "Height" => height as i32,
           "ColorSpace" => "DeviceRGB",
           "BitsPerComponent" => 8,
           "Filter" => "FlateDecode",
       },
       img_data,
   );

   let image_id = doc.add_object(image_xobject);
   println!("Image XObject ID: {:?}", image_id);

   // Get the first page
//    let page_id = *doc.get_pages().keys().next().ok_or("No pages found in PDF")?;
    //let (page_number, page_id) = *doc.get_pages().iter().next().ok_or("No pages found in PDF")?;
    let (&page_number, &page_id) = doc.get_pages().iter().next().ok_or("No pages found in PDF")?;
    println!("Page ID: {:?}", page_id);
    
    // Add the image to the page content
    let content = format!(
        "q\n{} 0 0 {} 50 {} cm\n/Im0 Do\nQ\n",
        width, height, height - 50
    );
    println!("Content Stream: {}", content);
    
    let content_stream = Stream::new(
        lopdf::dictionary! {
            "Length" => content.len() as i32,
        },
        content.into_bytes(),
    );
    
    let content_id = doc.add_object(content_stream);
    println!("Content Stream ID: {:?}", content_id);
    
    // Modify the page content
    {
        let page = doc.get_object_mut(page_id)?; // Use page_id directly
        if let Object::Dictionary(ref mut page_dict) = *page {
            // Ensure the Resources dictionary exists
            if !page_dict.has(b"Resources") {
                page_dict.set("Resources", lopdf::dictionary! {});
            }
    
            let resources = page_dict.get_mut(b"Resources").unwrap();
            if let Object::Dictionary(ref mut resources_dict) = *resources {
                // Ensure the XObject dictionary exists
                if !resources_dict.has(b"XObject") {
                    resources_dict.set("XObject", lopdf::dictionary! {});
                }
    
                let xobjects = resources_dict.get_mut(b"XObject").unwrap();
                if let Object::Dictionary(ref mut xobjects_dict) = *xobjects {
                    xobjects_dict.set("Im0", image_id);
                    println!("XObject dictionary updated with image ID");
                }
            }
    
            // Update the Contents entry
            if let Ok(contents) = page_dict.get_mut(b"Contents") {
                match contents {
                    Object::Reference(content_ref) => {
                        let content_array = vec![Object::Reference(*content_ref), Object::Reference(content_id)];
                        *contents = Object::Array(content_array);
                        println!("Contents updated with new content stream");
                    }
                    Object::Array(content_array) => {
                        content_array.push(Object::Reference(content_id));
                        println!("Contents array updated with new content stream");
                    }
                    _ => {
                        *contents = Object::Array(vec![Object::Reference(content_id)]);
                        println!("Contents set to new content stream array");
                    }
                }
            } else {
                page_dict.set("Contents", Object::Reference(content_id));
                println!("Contents set to new content stream");
            }
        }
    }

    // Save the modified PDF
    doc.save(pdf_output)?;
    println!("PDF saved to {}", pdf_output);

    */

    Ok(())
}

