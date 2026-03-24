# pysegyfile

## Build
```
git clone git@github.com:andalevor/segyfile.git
git clone git@github.com:andalevor/pysegyfile.git
cd pysegyfile
python -m venv .env
source .env/bin/activate
pip install numpy maturin
maturin develop --release #to build and install to environment
#or
maturin build --release #to build wheel
```

## Usage
```
import pysegyfile as psf
isgy = psf.Reader("../segyfile/samples/ieee_single.sgy")
txt_hdr = isgy.text_header() #will try to guess encoding
#or
txt_hdr_raw = isgy.text_header_raw()
bin_hdr = isgy.binary_header()
hdr_names = list(range(psf.trc_hdr_names.TRC_SEQ_LINE, psf.trc_hdr_names.SOU_MEAS_UNIT)) #to read all standard headers
#or
hdr_names = [psf.trc_hdr_names.FFID, psf.trc_hdr_names.CHAN] #to read some of standard headers
hdrs, samps = isgy.read_traces(hdr_names) #to read all standard headers
osgy = psf.Writer("test.sgy", txt_hdr, bin_hdr)
osgy.write_traces(hdr_names, hdrs, samps)
```
