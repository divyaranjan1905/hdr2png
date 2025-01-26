# hdr2png

`hdr2png` is a command-line tool that takes a HDR image file of Radiance (RGBE) format and encodes into a 8-bit RGBM/A PNG file. It is primarily for asset creation and management in graphical computing environments, where speed, bit precision and compression are of high priority.

It is written entirely in Rust, tries to be minimal in its dependencies, and is cross-platform across major operating systems.

# Building & Installation

The binary can be downloaded from the latest release and that would be all that you need. If for developmental or other purposes, one wishes to build from source, it is very simple:

```
git clone https://codeberg.org/divyaranjan/hdr2png
cd hdr2png/
cargo build --release
```
This would create a binary in the `target/release/` directory.

## Usage

The following help page details the arguments needed by the tool and its usage:

```
Commandline tool to encode Radiance HDR images into RGBM/RGBA PNG

Usage: hdr2png [OPTIONS] --input <HDR_FILE> --format <ENCODING_FORMAT>

Options:
      --input <HDR_FILE>          Path to the input HDR file
      --output <PNG_FILE>         Path to the output PNG file
      --format <ENCODING_FORMAT>  The encoding format to use for PNG [possible values: rgba, rgbm]
  -h, --help                      Print help
  -V, --version                   Print version
```

The three important arguments/flags are `format`, `input` and `output`, of them only the last one is optional. So, if you wish to convert a HDR file into RGBM-encoded PNG, all you need to do is:

```
hdr2png --format rgbm --input input.hdr --output output.png
```

Currently the tool supports encoding the PNG either in RGBM, or in the usual [RGBA](https://en.wikipedia.org/wiki/RGBA_color_model) format.

Since the output argument is _optional_, even if it is not provided in the command, by default the tool will produce a PNG file of the same name as the input file.

In the `img/hdr/` directory are some sample HDR files fetched from Greg Ward’s website, and some other sources. One can try the tool on them, their encoded PNGs are also available in `img/png/` directory.

For comparison purposes, a proper HDR viewer is recommended such as [HDRView](https://wkjarosz.github.io/hdrview/) or [LuminanceHDR](https://github.com/LuminanceHDR/LuminanceHDR).

## On Encodings of HDR & PNG

Since its inception, HDR (or, Higher Dynamic Range) has been attempted to be encoded and formatted in many different ways. Among them, the one that really got popular and in widespread use was Gregory Ward Larson’s [RGBE](https://en.wikipedia.org/wiki/RGBE_image_format) format that became the defacto encoding system for the Radiance rendering engine, that Greg himself was the primary developer of. Thus current tool for now, **only encodes RGBE HDR format** ála Radiance.

For PNGs, the default format is that of RGBA (Red, Green, Blue, Alpha) but since HDR does not limit itself to this but rather has a whole _range_ that it captures, without special encoding a RGBA PNG loses much of the precision and range that a HDR has. This is where an encoding such as RGBM (Red, Green, Blue, Multiplier) becomes really crucial, since in this case you sacrifice the 4th alpha channel to store a 8-bit multiplier which changes by every pixel, and as the name goes, it gets multiplied by the bits of RGB (alongwith another constant). This allows us to achieve much more bit precision and range than RGBA, while still being in the PNG format and thus getting it compressed.

Furthermore, it is to be noted that since in RGBM encoding you sacrifice the alpha channel and since universally PNGs are used in RGBA format, if you open a PNG of the former kind in a viewer that only supports the latter, **you would see a distorted picture** with some parts being transparent and others not. **This is because the viewer treats the fourth channel as alpha, when it is clearly not that.**

It is for the same reason, that if you encode a HDR with `hdr2png` in RGBM format it will be compressed to some extent, and opening it in a typical viewer wouldn’t show much. For this, one has to decode it into RGBA format using the `--format rgba` argument, which will provide a PNG that is _very_ much compressed but as we described, it is at the cost of bit precision and range.

## Compression and Speed

There hasn’t been conducted a proper benchmark of this tool, but from using it on the provided sample HDR files, it can encode a **1 megabyte** HDR into RGBM PNG in **0.0001328s**, and the final PNG is about **20-25%** smaller than the size of the original HDR. And in the case of RGBA, the final PNG is about **45-50%** smaller though it usually takes 0.002s longer in encoding.

## References

- http://www.anyhere.com/gward/hdrenc/pages/originals.html
- http://www.anyhere.com/gward/hdrenc/hdr_encodings.html
- https://www.pauldebevec.com/Research/HDR/Ward-HDRImaging-20010521.pdf
- Original source code of first RGBE implementaion by Ward: https://www.graphics.cornell.edu/%7Ebjw/rgbe/rgbe.c
- https://graphicrants.blogspot.com/2009/04/rgbm-color-encoding.html?m=1
- https://stackoverflow.com/questions/12253425/encoding-rgb-colors-in-16-bits
- https://lousodrome.net/blog/light/tag/rgbm/
- https://enkimute.github.io/hdrpng.js/
- https://www.cg.tuwien.ac.at/research/publications/2006/Holzer-06-HDR/Holzer-06-HDR-.pdf
- https://cbloomrants.blogspot.com/2020/06/widespread-error-in-radiance-hdr-rgbe.html?m=1
- https://cbloomrants.blogspot.com/2020/06/followup-tidbits-on-rgbe.html
- https://iwasbeingirony.blogspot.com/2010/06/difference-between-rgbm-and-rgbd.html
- https://docs.unity3d.com/Manual/Lightmaps-TechnicalInformation.html
- http://www.ww.hdrlabs.com/book/
- Reinhard, Ward, Pattanaik, Debevec, _High Dynamic Range Imaging: Acquisition, Display and Image-Based Lighting_, 2nd Ed. (2010). Chapter 3.
- Hughes, Dam et.al, _Computer Graphics: Principles and Practice_, 3rd Ed. (2014). Chapter 17.
