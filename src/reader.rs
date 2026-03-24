use crate::common::BinaryHeader;
use crate::error::WrpError;
use numpy::{IntoPyArray, PyArrayMethods};
use pyo3::{exceptions, prelude::*};
use segyfile::{
    common::{self, TEXT_HEADER_SIZE},
    reader,
};

#[pyclass]
pub struct Reader {
    inner: reader::Reader,
}

#[pymethods]
impl Reader {
    #[new]
    fn new(name: String) -> Result<Self, WrpError> {
        let inner = reader::Reader::open(&name)?;
        Ok(Reader { inner })
    }
    fn text_header(&mut self) -> Result<String, WrpError> {
        let hdr = self.inner.read_text_header()?;
        Ok(str::from_utf8(&hdr)?.to_owned())
    }
    fn text_header_raw(&mut self) -> Result<[u8; TEXT_HEADER_SIZE], WrpError> {
        let hdr = self.inner.read_raw_text_header()?;
        Ok(hdr)
    }
    fn binary_header(&self) -> BinaryHeader {
        BinaryHeader::from(self.inner.get_binary_header())
    }
    #[pyo3(signature = (hdr_names, dtype = "int32"))]
    fn read_headers(
        &mut self,
        py: Python,
        hdr_names: Vec<i32>,
        dtype: &str,
    ) -> PyResult<Py<PyAny>> {
        let hdrs: Py<PyAny> = match dtype {
            "int8" => match self.inner.read_headers_1d::<i8>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "int16" => match self.inner.read_headers_1d::<i16>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "int32" => match self.inner.read_headers_1d::<i32>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "int64" => match self.inner.read_headers_1d::<i64>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "uint8" => match self.inner.read_headers_1d::<u8>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "uint16" => match self.inner.read_headers_1d::<u16>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "uint32" => match self.inner.read_headers_1d::<u32>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "uint64" => match self.inner.read_headers_1d::<u64>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "float32" => match self.inner.read_headers_1d::<f32>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            "float64" => match self.inner.read_headers_1d::<f64>(&hdr_names) {
                Ok(h) => make_pyarray(py, h, hdr_names.len())?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            _ => {
                return Err(PyErr::new::<exceptions::PyValueError, _>(
                    "Wrong dtype value. Possible values: int8..int64, uint8..uint64, float32, float64",
                ));
            }
        };
        Ok(hdrs)
    }
    fn read_samples(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        let bh = self.inner.get_binary_header();
        let samp_num = if bh.ext_samp_num > 0 {
            bh.ext_samp_num as usize
        } else {
            bh.samp_num as usize
        };
        let samp: Py<PyAny> = match bh.format_code {
            1 | 6 => match self.inner.read_samples_1d::<f64>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            2 | 7 => match self.inner.read_samples_1d::<i32>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            3 => match self.inner.read_samples_1d::<i16>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            5 => match self.inner.read_samples_1d::<f32>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            8 => match self.inner.read_samples_1d::<i8>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            9 => match self.inner.read_samples_1d::<i64>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            10 | 15 => match self.inner.read_samples_1d::<u32>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            11 => match self.inner.read_samples_1d::<u16>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            12 => match self.inner.read_samples_1d::<u64>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            16 => match self.inner.read_samples_1d::<u8>() {
                Ok(s) => make_pyarray(py, s, samp_num)?,
                Err(e) => return Err(PyErr::from(WrpError::from(e))),
            },
            other => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unsupported format code: '{other}'."
                )));
            }
        };
        Ok(samp)
    }
    #[pyo3(signature = (hdr_names, dtype = "int32"))]
    fn read_traces(
        &mut self,
        py: Python,
        hdr_names: Vec<i32>,
        dtype: &str,
    ) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let bh = self.inner.get_binary_header();
        let samp_num = if bh.ext_samp_num > 0 {
            bh.ext_samp_num as usize
        } else {
            bh.samp_num as usize
        };
        let hdrs_samp: (Py<PyAny>, Py<PyAny>) = match dtype {
            "int8" => make_two_pyarrays::<i8>(py, self, &hdr_names, samp_num)?,
            "int16" => make_two_pyarrays::<i16>(py, self, &hdr_names, samp_num)?,
            "int32" => make_two_pyarrays::<i32>(py, self, &hdr_names, samp_num)?,
            "int64" => make_two_pyarrays::<i64>(py, self, &hdr_names, samp_num)?,
            "uint8" => make_two_pyarrays::<u8>(py, self, &hdr_names, samp_num)?,
            "uint16" => make_two_pyarrays::<u16>(py, self, &hdr_names, samp_num)?,
            "uint32" => make_two_pyarrays::<u32>(py, self, &hdr_names, samp_num)?,
            "uint64" => make_two_pyarrays::<u64>(py, self, &hdr_names, samp_num)?,
            "float32" => make_two_pyarrays::<f32>(py, self, &hdr_names, samp_num)?,
            "float64" => make_two_pyarrays::<f64>(py, self, &hdr_names, samp_num)?,
            _ => {
                return Err(PyErr::new::<exceptions::PyValueError, _>(
                    "Wrong dtype value. Possible values: int8..int64, uint8..uint64, float32, float64",
                ));
            }
        };
        Ok(hdrs_samp)
    }
}

fn make_pyarray<T: numpy::Element>(py: Python, v: Vec<T>, first_dim: usize) -> PyResult<Py<PyAny>> {
    let trc_num = v.len() / first_dim;
    let res: Py<PyAny> = v
        .into_pyarray(py)
        .reshape([trc_num, first_dim])
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
        .unbind()
        .into();
    Ok(res)
}

fn make_two_pyarrays<T: numpy::Element + common::Primitive + Copy>(
    py: Python,
    reader: &mut Reader,
    hdr_names: &[i32],
    samp_dim: usize,
) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
    let hdrs_samp: (Py<PyAny>, Py<PyAny>) = match reader.inner.get_binary_header().format_code {
        1 | 6 => match reader.inner.read_traces_1d::<T, f64>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        2 | 7 => match reader.inner.read_traces_1d::<T, i32>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        3 => match reader.inner.read_traces_1d::<T, i16>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        5 => match reader.inner.read_traces_1d::<T, f32>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        8 => match reader.inner.read_traces_1d::<T, i8>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        9 => match reader.inner.read_traces_1d::<T, i64>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        10 | 15 => match reader.inner.read_traces_1d::<i8, u32>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        11 => match reader.inner.read_traces_1d::<T, u16>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        12 => match reader.inner.read_traces_1d::<T, u64>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        16 => match reader.inner.read_traces_1d::<T, u8>(&hdr_names) {
            Ok((h, s)) => (
                make_pyarray(py.clone(), h, hdr_names.len())?,
                make_pyarray(py, s, samp_dim)?,
            ),
            Err(e) => return Err(PyErr::from(WrpError::from(e))),
        },
        other => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Unsupported format code: '{other}'."
            )));
        }
    };
    Ok(hdrs_samp)
}
