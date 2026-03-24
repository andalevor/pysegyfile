use crate::common::BinaryHeader;
use crate::error::WrpError;
use numpy::{PyArray2, PyArrayDescrMethods, PyArrayMethods, PyUntypedArray, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, prelude::*};
use segyfile::{common, writer};

macro_rules! match_dtype {
    ($self:expr, $hdr_names:expr, $hdrs_ro:expr, $samp:expr, $($dtype:expr => $ty:ty),* $(,)?) => {{
        let untyped: &Bound<'_, PyUntypedArray> = $samp.cast()?;
        let dt = untyped.dtype().num();
        match dt {
            $(
            $dtype => {
                let samp_typed: &Bound<'_, PyArray2<$ty>> = $samp.cast()?;
                let samp_ro = samp_typed.readonly();
                $self.inner
                    .write_traces_1d(&$hdr_names, $hdrs_ro.as_slice()?, samp_ro.as_slice()?)
                    .map_err(|e| return WrpError::from(e))?;
            }
            )*
            _ => {
                return Err(PyValueError::new_err(format!("Unsupported dtype: {}", dt)));
            }
        }
    }}
}

#[pyclass]
pub struct Writer {
    inner: writer::Writer,
}

#[pymethods]
impl Writer {
    #[new]
    fn new(name: &str, txt_hdr: &[u8], bin_hdr: BinaryHeader) -> Result<Self, WrpError> {
        let txt_hdr: &[u8; common::TEXT_HEADER_SIZE] = match txt_hdr.try_into() {
            Ok(h) => h,
            Err(e) => return Err(WrpError::from(e)),
        };
        let inner = writer::Writer::create(&name, txt_hdr, bin_hdr.into_binary_header())?;
        Ok(Writer { inner })
    }
    fn write_traces(
        &mut self,
        hdr_names: Vec<i32>,
        hdrs: &Bound<'_, PyAny>,
        samp: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let untyped: &Bound<'_, PyUntypedArray> = hdrs.cast()?;

        if untyped.ndim() != 2 {
            return Err(PyValueError::new_err(format!(
                "2D array expected. Got: {}D",
                untyped.ndim()
            )));
        }

        let dtype = untyped.dtype().num();

        match dtype {
            1 => {
                let hdr_typed: &Bound<'_, PyArray2<i8>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            2 => {
                let hdr_typed: &Bound<'_, PyArray2<u8>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            3 => {
                let hdr_typed: &Bound<'_, PyArray2<i16>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            4 => {
                let hdr_typed: &Bound<'_, PyArray2<u16>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            5 => {
                let hdr_typed: &Bound<'_, PyArray2<i32>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            6 => {
                let hdr_typed: &Bound<'_, PyArray2<u32>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            7 => {
                let hdr_typed: &Bound<'_, PyArray2<i64>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            8 => {
                let hdr_typed: &Bound<'_, PyArray2<u64>> = hdrs.cast()?;
                let hdrs_ro = hdr_typed.readonly();
                match_dtype!(self, hdr_names, hdrs_ro, samp,
                1 => i8,
                2 => u8,
                3 => i16,
                4 => u16,
                5 => i32,
                6 => u32,
                7 => i64,
                8 => u64,
                9 => i64,
                10 => u64,
                11 => f32,
                12 => f64,
                );
            }
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unsupported dtype: {}",
                    dtype
                )));
            }
        };

        Ok(())
    }
}
