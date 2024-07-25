use plotters::{prelude::*, style::{full_palette::ORANGE}};
use printpdf::ColorSpace;
use std::collections::HashMap;

use printpdf::{ColorBits, Image, ImageTransform, ImageXObject, Mm, PdfDocument, PdfLayerReference, Pt, Px};
//use printpdf::*;
//use lopdf::{dictionary, Dictionary, Document, Object, ObjectId, Stream};
//use lopdf::content::{Content, Operation};
use std::fs::File;
use std::io::BufWriter;
//use image::GenericImageView;
use image::{codecs::png, GenericImageView};
use image::codecs::png::PngDecoder;

mod png_to_pdf;


// use pdf::{file::File, object::Dictionary, primitive::Primitive, writer::Writer};

// use pdf::file::FileOptions;
// use pdf::object::{Dictionary, Object, Stream};
// use pdf::primitive::Primitive;
// use pdf::writer::Writer;
//use image::GenericImageView;
//use std::fs::File;
//use std::io::{BufWriter, Write};

const OUT_FILE_NAME: &str = "C:\\Projects\\pie_chart\\pie-chart.png";
const PDF_FILE_NAME: &str = "C:\\Projects\\pie_chart\\tax_statement.pdf";
//const PDF_FILE_NAME_PIE: &str = "C:\\Projects\\pie_chart\\tax_statement_pie.pdf";
const PDF_FILE_NAME_PIE: &str = "tax_statement_pie_1.pdf";

fn hex_to_rgb(hex: &str) -> RGBColor {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    RGBColor(r, g, b)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new(&OUT_FILE_NAME, (300, 250)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let title_style = TextStyle::from(("sans-serif", 30).into_font()).color(&(BLACK));
    root_area.titled("", title_style).unwrap();

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 100.0;  // Adjusted radius to make the pie chart smaller
    
    let tax_map = vec![
        ("ED", 36.31),
        ("CC", 9.77),
        ("TV", 0.48),
        ("FA", 0.85),
        ("SD", 5.95),
        ("MC", 14.85),
        ("FD", 29.63),
        ("LI", 2.16),
    ];

    let color_lookup: HashMap<&str, &str> = [
        ("MC", "#F58282"),
        ("SE", "#F5B082"),
        ("ED", "#FDFA7E"),
        ("LI", "#DEF582"),
        ("LH", "#B0F582"),
        ("CC", "#82F582"),
        ("LD", "#82F5B0"),
        ("FA", "#82F5DE"),
        ("SD", "#82DEF5"),
        ("FD", "#82B0F5"),
        ("FL", "#8282F5"),
        ("RI", "#B082F5"),
        ("TV", "#DE82F5"),
        ("MR", "#F582DE"),
        ("WV", "#F582B0"),
    ].iter().cloned().collect();
    
    let mut sorted_tax_map = tax_map.clone();
    sorted_tax_map.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mid = sorted_tax_map.len() / 2;
    let (large, small) = sorted_tax_map.split_at(mid);
    
    let mut interleaved_tax_map = Vec::new();
    for i in 0..mid {
        interleaved_tax_map.push(large[i].clone());
        if i < small.len() {
            interleaved_tax_map.push(small[i].clone());
        }
    }

    let sizes: Vec<f64> = interleaved_tax_map.iter().map(|&(_, v)| v).collect();
    let labels: Vec<&str> = interleaved_tax_map.iter().map(|&(k, _)| k).collect();
    let colors: Vec<RGBColor> = interleaved_tax_map.iter().map(|&(k, _)| hex_to_rgb(color_lookup.get(k).unwrap())).collect();
    
    let mut pie = Pie::new(&center, &radius, &sizes, &colors, &labels);
    pie.start_angle(66.0);
    pie.label_style((("sans-serif", 15).into_font()).color(&(BLACK)));
    pie.label_offset(15.0);
    root_area.draw(&pie)?;


    /*     Add image to first page of the PDF Code */
   // add_image_to_pdf()?;
   //create_pdf_from_image()?;

   png_to_pdf::convert_png_to_pdf(&OUT_FILE_NAME, &PDF_FILE_NAME_PIE)?;



    Ok(())
}

// pub fn create_pdf_from_image() -> Result<(), Box<dyn std::error::Error>> {
//     // Load the PNG image
//     // Load the PNG image

//     // let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
//     // let (page2, layer1) = doc.add_page(Mm(10.0), Mm(250.0),"Page 2, Layer 1");
    
//     // doc.save(&mut BufWriter::new(File::create("test_working.pdf").unwrap())).unwrap();



//     // Create a new PDF document
//     let (doc, page1, layer1) = PdfDocument::new("PDF with PNG", Mm(210.0), Mm(297.0), "Layer 1");
//     let current_layer = doc.get_page(page1).get_layer(layer1);

//     // url https://docs.rs/printpdf/latest/printpdf/
//     // currently, the only reliable file formats are bmp/jpeg/png
//     // this is an issue of the image library, not a fault of printpdf
//     // let mut image_file = File::open(OUT_FILE_NAME).unwrap();
//     // let image = Image::try_from(image::codecs::png::PngDecoder::new(&mut image_file).unwrap()).unwrap();
    
    










//     let image = image::open(OUT_FILE_NAME).expect("Failed to open image");
//     let (width, height) = image.dimensions();
//     let image_data = image.to_rgba8().into_raw();

//     let image_xobject = ImageXObject {
//         width: Px(width as i64),
//         height: Px(height as i64),
//         color_space: ColorSpace::Rgb,
//         bits_per_component: ColorBits::Bit8,
//         interpolate: true,
//         image_data,
//         image_filter: None,
//         clipping_bbox: None,
//         smask: None,
//     };

//     let pdf_image = printpdf::Image::from(image_xobject);

//     image.add_to_layer(current_layer.clone(), ImageTransform::default());

//    // you can also construct images manually from your data:
//    let mut image_file_2 = ImageXObject {
//         width: Px(200),
//         height: Px(200),
//         color_space: ColorSpace::Greyscale,
//         bits_per_component: ColorBits::Bit8,
//         interpolate: true,
//         /* put your bytes here. Make sure the total number of bytes =
//         width * height * (bytes per component * number of components)
//         (e.g. 2 (bytes) x 3 (colors) for RGB 16bit) */
//         image_data: Vec::new(),
//         image_filter: None, /* does not work yet */
//         clipping_bbox: None, /* doesn't work either, untested */
//         smask: None,
//     };
        

//     // Convert image to a PDF-compatible format
//     // let image = ImageXObject {
//     //     width: Px(width as usize),
//     //     height: Px(height as usize),
//     //     color_space: ColorSpace::Rgba,
//     //     bits_per_component: ColorBits::Bit8,
//     //     interpolate: true,
//     //     image_data: img.to_rgba8().to_vec(),
//     //     image_filter: None,
//     //     clipping_bbox: None,
//     //     smask: None,
//     // };

//     // let img_stream = Stream::new(dictionary! {
//     //     "Type" => "XObject",
//     //     "Subtype" => "Image",
//     //     "Width" => width as i32,
//     //     "Height" => height as i32,
//     //     "ColorSpace" => "DeviceRGB",
//     //     "BitsPerComponent" => 8,
//     //     "Filter" => "DCTDecode",
//     //     "Length" => img_data.len() as i32,
//     // }, img_data);      

//     // Add the image to the PDF
//     // let current_layer = doc.get_page(page1).get_layer(layer1);
//     // image.add_to_layer(current_layer.clone(), ImageTransform {
//     //     translate_x: Some(Mm(50.0)),  // X coordinate in mm
//     //     translate_y: Some(Mm(200.0 - (height as f64 * 0.264583))), // Y coordinate in mm, adjusted for image height
//     //     rotate: None,
//     //     scale_x: Some(Mm(width as f64 * 0.264583)),  // Scale to original image width
//     //     scale_y: Some(Mm(height as f64 * 0.264583)), // Scale to original image height
//     //     ..Default::default()
//     // });    
    
//     // Convert image dimensions from pixels to points
//     let width_pts = width as f32;
//     let height_pts = height as f32;

//     // Create an image object
//     let rgba_image = img.to_rgba8();
//     let image_data = rgba_image.into_raw();
//     let image = ImageXObject {
//         width: Px(width as usize),
//         height: Px(height as usize),
//         color_space: ColorSpace::Rgba,
//         bits_per_component: ColorBits::Bit8,
//         interpolate: true,
//         image_data,
//         image_filter: None,
//         clipping_bbox: None,
//         smask: None,
//     };
//     let image = Image::from(image);

//     // Add the image to the PDF
//     let current_layer = doc.get_page(page1).get_layer(layer1);
//     image.add_to_layer(current_layer.clone(), ImageTransform {
//         translate_x: Some(Mm(50.0)),  // X coordinate in mm
//         translate_y: Some(Mm(200.0)), // Y coordinate in mm
//         scale_x: Some(width_pts),
//         scale_y: Some(height_pts),
//         ..Default::default()
//     });

//     // Save the PDF file
//     let output = File::create(PDF_FILE_NAME_PIE).expect("Failed to create output file");
//     doc.save(&mut BufWriter::new(output)).expect("Failed to save PDF");

//     Ok(())
// }

/* 
pub fn add_image_to_pdf() -> Result<(), Box<dyn std::error::Error>> {
    // Load the existing PDF
    let mut doc = Document::load(PDF_FILE_NAME).expect("Cannot load PDF");

    // Load the PNG image
    let img = image::open(OUT_FILE_NAME).expect("Cannot open image");
    let (width, height) = img.dimensions();

    // Convert the image to a format suitable for PDF embedding
    let img_data = match img {
        image::DynamicImage::ImageRgb8(ref img) => img.clone().into_raw(),
        _ => panic!("Image format not supported"),
    };

    // Create an XObject for the image
    let img_id = (doc.max_id + 1, 0);
    doc.max_id += 1;
    // doc.objects.insert(img_id, dictionary! {
    //     "Type" => "XObject",
    //     "Subtype" => "Image",
    //     "Width" => width as i32,
    //     "Height" => height as i32,
    //     "ColorSpace" => "DeviceRGB",
    //     "BitsPerComponent" => 8,
    //     "Filter" => "DCTDecode",
    //     "Length" => img_data.len() as i32,
    // }.into());    
    let img_stream = Stream::new(dictionary! {
        "Type" => "XObject",
        "Subtype" => "Image",
        "Width" => width as i32,
        "Height" => height as i32,
        "ColorSpace" => "DeviceRGB",
        "BitsPerComponent" => 8,
        "Filter" => "DCTDecode",
        "Length" => img_data.len() as i32,
    }, img_data);    

    // Insert the image stream into the document
    doc.objects.insert(img_id, Object::Stream(img_stream));    

    // Add the image data
    //doc.objects.insert((img_id.0 + 1, 0), lopdf::Object::Stream(img_data.into()));

    // Add the image to the first page of the PDF
    let page_id = (*doc.get_pages().keys().next().unwrap(), 0);
    let content_stream = doc.get_page_content(page_id).expect("Cannot get page content");
    let mut content = Content::decode(&content_stream).expect("Cannot decode content");

    // Calculate the position for the image to be on the lower left-hand side
    let x_position = 0; // Adjust as needed
    let y_position = 0; // Adjust as needed

    // Insert the image at the calculated position
    content.operations.push(Operation::new("q", vec![]));
    content.operations.push(Operation::new("cm", vec![
        width.into(), 0.into(), 0.into(), height.into(), x_position.into(), y_position.into() // Image transform matrix
    ]));
    content.operations.push(Operation::new("Do", vec![img_id.into()]));
    content.operations.push(Operation::new("Q", vec![]));

 
    // Create a new content stream for the page
    let new_content = Content::encode(&content).expect("Cannot encode content");
    let new_content_stream = Stream::new(dictionary!{}, new_content);
    let max_id = doc.max_id;
    let new_content_id = (max_id + 1, 0);
    doc.max_id += 1;
    doc.objects.insert(new_content_id, Object::Stream(new_content_stream)); 

    // Update the page to reference the new content stream
    let max_id = doc.max_id;
    let page = doc.get_object_mut(page_id).expect("Cannot get page object").as_dict_mut().expect("Cannot convert page object to dict");
    let new_content_id = (max_id + 1, 0);
    
    if let Ok(contents) = page.get_mut(b"Contents") {
        match contents {
            Object::Reference(id) => {
                let new_array = vec![Object::Reference(*id), Object::Reference(new_content_id)];
                page.set("Contents", Object::Array(new_array));
            },
            Object::Array(arr) => {
                arr.push(Object::Reference(new_content_id));
            },
            _ => panic!("Unexpected Contents type"),
        }
    } else {
        page.set("Contents", Object::Reference(new_content_id));
    }
        

    // Save the modified PDF
    doc.save(PDF_FILE_NAME_PIE).expect("Cannot save output PDF");


    Ok(())
}
*/
/* 
pub fn add_image_to_pdf1() -> Result<(), Box<dyn std::error::Error>> {
    // Load the existing PDF
    let mut doc = Document::load("tax_statement.pdf").expect("Cannot load PDF");

    // Load the PNG image
    let img = image::open("pie-chart.png").expect("Cannot open image");
    let (width, height) = img.dimensions();

    // Convert the image to a format suitable for PDF embedding
    let img_data = match img {
        image::DynamicImage::ImageRgb8(ref img) => img.clone().into_raw(),
        _ => panic!("Image format not supported"),
    };

    // Create an XObject for the image
    let img_id = (doc.max_id + 1, 0);
    doc.max_id += 1;
    let img_stream = Stream::new(dictionary! {
        "Type" => "XObject",
        "Subtype" => "Image",
        "Width" => width as i32,
        "Height" => height as i32,
        "ColorSpace" => "DeviceRGB",
        "BitsPerComponent" => 8,
        "Filter" => "DCTDecode",
        "Length" => img_data.len() as i32,
    }, img_data);

    // Insert the image stream into the document
    doc.objects.insert(img_id, Object::Stream(img_stream));

    // Add the image resource to the first page's resource dictionary
    let page_id = (*doc.get_pages().keys().next().unwrap(), 0);
    let page = doc.get_object_mut(page_id).expect("Cannot get page object").as_dict_mut().expect("Cannot convert page object to dict");

    let resources = page.get_mut(b"Resources").unwrap().as_dict_mut().unwrap();
    resources.set("XObject", dictionary! { "Im1" => Object::Reference(img_id) });

    // Create the content stream to include the image
    let content = Content {
        operations: vec![
            Operation::new("q", vec![]),
            Operation::new("cm", vec![
                width.into(), 0.into(), 0.into(), height.into(), 0.into(), 0.into() // Adjust position as needed
            ]),
            Operation::new("Do", vec!["Im1".into()]),
            Operation::new("Q", vec![])
        ],
    };

    // Encode the new content stream
    let new_content = Content::encode(&content).expect("Cannot encode content");
    let new_content_stream = Stream::new(dictionary!{}, new_content);
    let new_content_id = img_id; //(doc.max_id + 1, 0);
    //doc.max_id += 1;

    // Update the page to reference the new content stream
    if let Ok(contents) = page.get_mut(b"Contents") {
        match contents {
            Object::Reference(id) => {
                let new_array = vec![Object::Reference(*id), Object::Reference(new_content_id)];
                page.set("Contents", Object::Array(new_array));
            },
            Object::Array(arr) => {
                arr.push(Object::Reference(new_content_id));
            },
            _ => panic!("Unexpected Contents type"),
        }
    } else {
        page.set("Contents", Object::Reference(new_content_id));
    }

    // Encode the new content stream
    let new_content = Content::encode(&content).expect("Cannot encode content");
    let new_content_stream = Stream::new(dictionary!{}, new_content);
    let new_content_id = img_id; //(doc.max_id + 1, 0);
    //doc.max_id += 1;
    doc.objects.insert(new_content_id, Object::Stream(new_content_stream));

    // Save the modified PDF
    doc.save(PDF_FILE_NAME_PIE).expect("Cannot save output PDF");

    Ok(())
}
*/