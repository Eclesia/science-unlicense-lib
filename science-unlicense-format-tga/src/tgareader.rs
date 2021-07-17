//
// Public Domain - unlicense.science
//

use science_unlicense_image::api::Image;
use science_unlicense_common::api::MessageError;
use science_unlicense_encoding::api::io::DataRead;
use std::io::{Read, ErrorKind};
use std::io::Error;

pub struct TGAReader {
    id_length: u8,
    color_map_type: u8,
    image_type : u8,
    //colormap spec : 5 bytes
    color_map_spec_first_entry_index: u16,
    color_map_spec_color_map_length: u16,
    color_map_spec_color_map_entry_size: u8,
    //image spec : 10 bytes
    image_spec_x_origin: u16,
    image_spec_y_origin: u16,
    image_spec_width: u16,
    image_spec_height: u16,
    image_spec_pixel_depth: u8,
    image_spec_descriptor: i8,
    id : String
}

impl TGAReader {

    pub fn read(&mut self, stream : Box<dyn Read>) -> Result<Box<dyn Image>, Error> {
        let mut ds = DataRead::new(stream);

        self.id_length = ds.read_u8()?;
        self.color_map_type = ds.read_u8()?;
        self.image_type = ds.read_u8()?;

        //colormap spec : 5 bytes
        self.color_map_spec_first_entry_index = ds.read_u16()?;
        self.color_map_spec_color_map_length = ds.read_u16()?;
        self.color_map_spec_color_map_entry_size = ds.read_u8()?;

        //image spec : 10 bytes
        self.image_spec_x_origin = ds.read_u16()?;
        self.image_spec_y_origin = ds.read_u16()?;
        self.image_spec_width = ds.read_u16()?;
        self.image_spec_height = ds.read_u16()?;
        self.image_spec_pixel_depth = ds.read_u8()?;
        self.image_spec_descriptor = ds.read_i8()?;
        //read ID
        self.id = ds.read_string(self.id_length as usize)?;

        //read image datas
        let flip_vertical = (self.image_spec_descriptor & 0x20) == 0;
        let flip_horizontal = (self.image_spec_descriptor & 0x10) != 0;

        let mut image_spec_pixel_depth = self.image_spec_pixel_depth;
        if image_spec_pixel_depth % 8 != 0 {
            return Result::Err(Error::new(ErrorKind::InvalidInput, "pixel depth unsupported, must be a module of 8."));
        }
        image_spec_pixel_depth = image_spec_pixel_depth / 8;

        //read colormap
        // let colorIndex = null; //ColorIndex
        // if self.color_map_type != 0 {
        //     if self.color_map_spec_color_map_entry_size % 8 != 0 {
        //
        //         throw new IOException(ds, "colormap of size not a module of 8 not supported.");
        //     }
        //     final byte[][] colorMap = new byte[self.color_map_spec_color_map_length][color_map_spec_color_map_entry_size/8];
        //     for (int i=0;i<color_map_spec_color_map_length;i++) {
        //         ds.readFully(colorMap[i]);
        //     }
        //     //colors are in BVR order
        //     final Color[] palette = new Color[color_map_spec_color_map_length];
        //     for (int i=0;i<palette.length;i++){
        //         if (colorMap[0].length==3){
        //             palette[i] = new ColorRGB(colorMap[i][2]&0xFF, colorMap[i][1]&0xFF, colorMap[i][0]&0xFF);
        //         } else if (colorMap[0].length==4){
        //             palette[i] = new ColorRGB(colorMap[i][2]&0xFF, colorMap[i][1]&0xFF, colorMap[i][0]&0xFF, colorMap[i][3]&0xFF);
        //         } else {
        //             throw new IOException(ds, "unexpected colormap tuple size"+colorMap[0].length);
        //         }
        //     }
        //     colorIndex = new ColorIndex(palette);
        // }

        return Result::Err(Error::new(ErrorKind::Other, "Not implemented yet"));
    }

    // //read image data
    // final int lineSize = image_spec_width*image_spec_pixeldepth;
    // final int n = image_spec_height*lineSize;
    // final byte[] data = new byte[n];
    // final DataInputStream uncompressed;
    // if (    imageType == IMAGE_TYPE_RLE_TRUECOLOR
    // || imageType == IMAGE_TYPE_RLE_COLORMAP
    // || imageType == IMAGE_TYPE_RLE_BW){
    // //decompress rle stream
    // final RLEInputStream rle = new RLEInputStream(stream,image_spec_pixeldepth);
    // uncompressed = new DataInputStream(rle);
    // } else {
    // uncompressed = ds;
    // }
    //
    // //read and reorder bits
    // for (int y=0;y<image_spec_height;y++){
    // final int offset;
    // if (flipVertical){
    // offset = (image_spec_height-y-1)*lineSize;
    // } else {
    // offset = lineSize*y;
    // }
    // uncompressed.readFully(data,offset,lineSize);
    //
    // if (flipHorizontal){
    // Arrays.reverse(data, offset, lineSize, image_spec_pixeldepth);
    // }
    // }
    //
    // final Buffer bank = DefaultBufferFactory.wrap(data);
    //
    // if (imageType == IMAGE_TYPE_RLE_TRUECOLOR || imageType == IMAGE_TYPE_UNCOMPRESSED_TRUECOLOR){
    //
    // final ImageModel sm;
    // final ImageModel cm;
    // final int[] mapping;
    // if (image_spec_pixeldepth == 3){
    // mapping = new int[]{2,1,0};
    // sm = new InterleavedModel(new UndefinedSystem(image_spec_pixeldepth), UInt8.TYPE);
    // cm = DerivateModel.create(sm, mapping, null, null, ColorSystem.RGB_8BITS);
    // } else if (image_spec_pixeldepth == 4){
    // mapping = new int[]{2,1,0,3};
    // sm = new InterleavedModel(new UndefinedSystem(image_spec_pixeldepth), UInt8.TYPE);
    // cm = DerivateModel.create(sm, mapping, null, null, ColorSystem.RGBA_8BITS);
    // } else {
    // throw new IOException(ds, "Only handle 3 and 4 byte tga so far.");
    // }
    //
    // //colors are in BVR
    // return new DefaultImage(bank, new Extent.Long(image_spec_width, image_spec_height), sm,cm);
    // } else if (imageType == IMAGE_TYPE_RLE_COLORMAP || imageType == IMAGE_TYPE_UNCOMPRESSED_COLORMAP){
    // final ImageModel sm = new InterleavedModel(new UndefinedSystem(1), UInt8.TYPE);
    // final ImageModel cm = new IndexedColorModel(sm, UInt8.TYPE, colorIndex);
    // return new DefaultImage(bank, new Extent.Long(image_spec_width, image_spec_height), sm,cm);
    //
    // } else if (imageType == IMAGE_TYPE_RLE_BW || imageType == IMAGE_TYPE_UNCOMPRESSED_BW){
    // final int[] mapping = new int[]{0,0,0};
    // final ImageModel sm = new InterleavedModel(new UndefinedSystem(1), UInt8.TYPE);
    // final ImageModel cm = DerivateModel.create(sm, mapping, null, null, ColorSystem.RGB_8BITS);
    // return new DefaultImage(bank, new Extent.Long(image_spec_width, image_spec_height), sm,cm);
    // } else {
    // throw new IOException(ds, "Unsupported TGA image type "+imageType);
    // }
    // }


}