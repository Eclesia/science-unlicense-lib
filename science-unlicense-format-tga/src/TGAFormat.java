
package science.unlicense.format.tga;

import science.unlicense.common.api.Arrays;
import science.unlicense.common.api.character.Chars;
import science.unlicense.common.api.number.Endianness;
import science.unlicense.encoding.api.io.ByteInputStream;
import science.unlicense.encoding.api.io.DataInputStream;
import science.unlicense.encoding.api.io.IOException;
import science.unlicense.encoding.api.store.Store;
import science.unlicense.image.api.AbstractImageFormat;

/**
 *
 * @author Johann Sorel
 */
public class TGAFormat extends AbstractImageFormat {

    public static final TGAFormat INSTANCE = new TGAFormat();

    private TGAFormat() {
        super(new Chars("tga"));
        shortName = new Chars("TGA");
        longName = new Chars("Truevision Targa");
        mimeTypes.add(new Chars("image/tga"));
        mimeTypes.add(new Chars("image/targa"));
        mimeTypes.add(new Chars("application/targa"));
        extensions.add(new Chars("tga"));
        extensions.add(new Chars("tpic"));
    }

//    public boolean canDecode(Object input) throws IOException {
//        if (super.canDecode(input)){
//            return true;
//        } else {
//            final boolean[] closeStream = new boolean[1];
//            final ByteInputStream stream = IOUtilities.toInputStream(input, closeStream);
//            try{
//                return canDecode(stream);
//            }finally{
//                if (closeStream[0]){
//                    stream.close();
//                }
//            }
//        }
//    }

    private boolean canDecode(ByteInputStream stream) throws IOException {
        final DataInputStream ds = new DataInputStream(stream, Endianness.BIG_ENDIAN);
        final byte[] buffer = new byte[3];
        ds.readFully(buffer);
        //not exact but we don't want to read the file footer to know if it's a tga
        if (Arrays.contains(new int[]{
            TGAMetaModel.IMAGE_TYPE_NODATA,
            TGAMetaModel.IMAGE_TYPE_RLE_BW,
            TGAMetaModel.IMAGE_TYPE_RLE_COLORMAP,
            TGAMetaModel.IMAGE_TYPE_RLE_TRUECOLOR,
            TGAMetaModel.IMAGE_TYPE_UNCOMPRESSED_BW,
            TGAMetaModel.IMAGE_TYPE_UNCOMPRESSED_COLORMAP,
            TGAMetaModel.IMAGE_TYPE_UNCOMPRESSED_TRUECOLOR
            }, buffer[2])){
            //TODO we should do more checks
            return true;
        }
        return false;
    }

    @Override
    public boolean supportReading() {
        return true;
    }

    @Override
    public boolean supportWriting() {
        return false;
    }

    @Override
    public Store open(Object source) throws IOException {
        return new TGAStore(this, source);
    }

}
