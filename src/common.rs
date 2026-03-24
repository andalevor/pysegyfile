use pyo3::prelude::*;
use segyfile::common;

#[pyclass(from_py_object)]
#[derive(Default, Clone)]
pub struct BinaryHeader {
    #[pyo3(get, set)]
    job_id: i32,
    #[pyo3(get, set)]
    line_num: i32,
    #[pyo3(get, set)]
    reel_num: i32,
    #[pyo3(get, set)]
    trc_num: i16,
    #[pyo3(get, set)]
    aux_trc_num: i16,
    #[pyo3(get, set)]
    samp_int: i16,
    #[pyo3(get, set)]
    samp_int_orig: i16,
    #[pyo3(get, set)]
    samp_num: i16,
    #[pyo3(get, set)]
    samp_num_orig: i16,
    #[pyo3(get, set)]
    format_code: i16,
    #[pyo3(get, set)]
    ensemble_fold: i16,
    #[pyo3(get, set)]
    trc_sort_code: i16,
    #[pyo3(get, set)]
    vert_sum_code: i16,
    #[pyo3(get, set)]
    sw_freq_at_start: i16,
    #[pyo3(get, set)]
    sw_freq_at_end: i16,
    #[pyo3(get, set)]
    sw_length: i16,
    #[pyo3(get, set)]
    sw_type_code: i16,
    #[pyo3(get, set)]
    trc_num_of_sw_ch: i16,
    #[pyo3(get, set)]
    sw_trc_taper_length_start: i16,
    #[pyo3(get, set)]
    sw_trc_taper_length_end: i16,
    #[pyo3(get, set)]
    taper_type: i16,
    #[pyo3(get, set)]
    corr_data_trc: i16,
    #[pyo3(get, set)]
    bin_gain_recov: i16,
    #[pyo3(get, set)]
    amp_recov_meth: i16,
    #[pyo3(get, set)]
    measure_sys: i16,
    #[pyo3(get, set)]
    impulse_sig_pol: i16,
    #[pyo3(get, set)]
    vib_pol_code: i16,
    #[pyo3(get, set)]
    ext_trc_num: i32,
    #[pyo3(get, set)]
    ext_aux_trc_num: i32,
    #[pyo3(get, set)]
    ext_samp_num: i32,
    #[pyo3(get, set)]
    ext_samp_int: f64,
    #[pyo3(get, set)]
    ext_samp_int_orig: f64,
    #[pyo3(get, set)]
    ext_samp_num_orig: i32,
    #[pyo3(get, set)]
    ext_ensemble_fold: i32,
    #[pyo3(get, set)]
    endianness: i32,
    #[pyo3(get, set)]
    segy_maj_ver: u8,
    #[pyo3(get, set)]
    segy_min_ver: u8,
    #[pyo3(get, set)]
    fix_length_trc_flag: i16,
    #[pyo3(get, set)]
    ext_text_hdrs_num: i16,
    #[pyo3(get, set)]
    max_add_trc_hdrs: i32,
    #[pyo3(get, set)]
    time_basis_code: i16,
    #[pyo3(get, set)]
    num_of_trcs_in_file: u64,
    #[pyo3(get, set)]
    byte_off_of_first_trc: u64,
    #[pyo3(get, set)]
    trailer_stanza_num: i32,
}

#[pymethods]
impl BinaryHeader {
    fn __repr__(&self) -> String {
        format!(
            "BinaryHeader(
            Job identification number: {}
            Line number: {}
            Reel number: {}
            Number of data trace per ensemble: {}
            Number of auxiliary trace per ensemble: {}
            Sample interval: {}
            Sample interval of original field recording: {}
            Number of samples per data trace: {}
            Number of samples per data trace for original field recording: {}
            Data sample format code: {}
            Ensemble fold: {}
            Trace sorting code: {}
            Vertical sum code: {}
            Sweep frequency at start: {}
            Sweep frequency at end: {}
            Sweep length: {}
            Sweep type code: {}
            Trace number of sweep channel: {}
            Sweep trace taper length in ms at start: {}
            Sweep trace taper length in ms at end: {}
            Taper type: {}
            Correlated data traces: {}
            Binary gain recovered: {}
            Amplitude recovery method: {}
            Measurement system: {}
            Impulse signal polarity: {}
            Vibratory polarity code: {}
            Extended number of data traces per ensemble: {}
            Extended number of auxiliary traces per ensample: {}
            Extended number of samples per data trace: {}
            Extended sample interval: {}
            Extended sample interval of original field recording: {}
            Extended number of samples per data trace in original recording: {}
            Extended ensemble fold: {}
            Endianness constant: {:#x}
            Major SEG-Y Format Revision Number: {}
            Minor SEG-Y Format Revision Number: {}
            Fixed length trace flag: {}
            Number of Extended Textual File Headers: {}
            Maximum number of additional trace headers: {}
            Time basis code: {}
            Number of traces in this file: {}
            Byte offset of first trace: {}
            Number of data trailer staza records: {}\n)",
            self.job_id,
            self.line_num,
            self.reel_num,
            self.trc_num,
            self.aux_trc_num,
            self.samp_int,
            self.samp_int_orig,
            self.samp_num,
            self.samp_num_orig,
            self.format_code,
            self.ensemble_fold,
            self.trc_sort_code,
            self.vert_sum_code,
            self.sw_freq_at_start,
            self.sw_freq_at_end,
            self.sw_length,
            self.sw_type_code,
            self.trc_num_of_sw_ch,
            self.sw_trc_taper_length_start,
            self.sw_trc_taper_length_end,
            self.taper_type,
            self.corr_data_trc,
            self.bin_gain_recov,
            self.amp_recov_meth,
            self.measure_sys,
            self.impulse_sig_pol,
            self.vib_pol_code,
            self.ext_trc_num,
            self.aux_trc_num,
            self.ext_samp_num,
            self.ext_samp_int,
            self.ext_samp_int_orig,
            self.ext_samp_num_orig,
            self.ext_ensemble_fold,
            self.endianness,
            self.segy_maj_ver,
            self.segy_min_ver,
            self.fix_length_trc_flag,
            self.ext_text_hdrs_num,
            self.max_add_trc_hdrs,
            self.time_basis_code,
            self.num_of_trcs_in_file,
            self.byte_off_of_first_trc,
            self.trailer_stanza_num,
        )
    }
}

impl From<&common::BinaryHeader> for BinaryHeader {
    fn from(other: &common::BinaryHeader) -> Self {
        BinaryHeader {
            job_id: other.job_id,
            line_num: other.line_num,
            reel_num: other.reel_num,
            trc_num: other.trc_num,
            aux_trc_num: other.aux_trc_num,
            samp_int: other.samp_int,
            samp_int_orig: other.samp_int_orig,
            samp_num: other.samp_num,
            samp_num_orig: other.samp_num_orig,
            format_code: other.format_code,
            ensemble_fold: other.ensemble_fold,
            trc_sort_code: other.trc_sort_code,
            vert_sum_code: other.vert_sum_code,
            sw_freq_at_start: other.sw_freq_at_start,
            sw_freq_at_end: other.sw_freq_at_end,
            sw_length: other.sw_length,
            sw_type_code: other.sw_type_code,
            trc_num_of_sw_ch: other.trc_num_of_sw_ch,
            sw_trc_taper_length_start: other.sw_trc_taper_length_start,
            sw_trc_taper_length_end: other.sw_trc_taper_length_end,
            taper_type: other.taper_type,
            corr_data_trc: other.corr_data_trc,
            bin_gain_recov: other.bin_gain_recov,
            amp_recov_meth: other.amp_recov_meth,
            measure_sys: other.measure_sys,
            impulse_sig_pol: other.impulse_sig_pol,
            vib_pol_code: other.vib_pol_code,
            ext_trc_num: other.ext_trc_num,
            ext_aux_trc_num: other.ext_aux_trc_num,
            ext_samp_num: other.ext_samp_num,
            ext_samp_int: other.ext_samp_int,
            ext_samp_int_orig: other.ext_samp_int_orig,
            ext_samp_num_orig: other.ext_samp_num_orig,
            ext_ensemble_fold: other.ext_ensemble_fold,
            endianness: other.endianness,
            segy_maj_ver: other.segy_maj_ver,
            segy_min_ver: other.segy_min_ver,
            fix_length_trc_flag: other.fix_length_trc_flag,
            ext_text_hdrs_num: other.ext_text_hdrs_num,
            max_add_trc_hdrs: other.max_add_trc_hdrs,
            time_basis_code: other.time_basis_code,
            num_of_trcs_in_file: other.num_of_trcs_in_file,
            byte_off_of_first_trc: other.byte_off_of_first_trc,
            trailer_stanza_num: other.trailer_stanza_num,
        }
    }
}

impl BinaryHeader {
    pub fn into_binary_header(&self) -> common::BinaryHeader {
        common::BinaryHeader {
            job_id: self.job_id,
            line_num: self.line_num,
            reel_num: self.reel_num,
            trc_num: self.trc_num,
            aux_trc_num: self.aux_trc_num,
            samp_int: self.samp_int,
            samp_int_orig: self.samp_int_orig,
            samp_num: self.samp_num,
            samp_num_orig: self.samp_num_orig,
            format_code: self.format_code,
            ensemble_fold: self.ensemble_fold,
            trc_sort_code: self.trc_sort_code,
            vert_sum_code: self.vert_sum_code,
            sw_freq_at_start: self.sw_freq_at_start,
            sw_freq_at_end: self.sw_freq_at_end,
            sw_length: self.sw_length,
            sw_type_code: self.sw_type_code,
            trc_num_of_sw_ch: self.trc_num_of_sw_ch,
            sw_trc_taper_length_start: self.sw_trc_taper_length_start,
            sw_trc_taper_length_end: self.sw_trc_taper_length_end,
            taper_type: self.taper_type,
            corr_data_trc: self.corr_data_trc,
            bin_gain_recov: self.bin_gain_recov,
            amp_recov_meth: self.amp_recov_meth,
            measure_sys: self.measure_sys,
            impulse_sig_pol: self.impulse_sig_pol,
            vib_pol_code: self.vib_pol_code,
            ext_trc_num: self.ext_trc_num,
            ext_aux_trc_num: self.ext_aux_trc_num,
            ext_samp_num: self.ext_samp_num,
            ext_samp_int: self.ext_samp_int,
            ext_samp_int_orig: self.ext_samp_int_orig,
            ext_samp_num_orig: self.ext_samp_num_orig,
            ext_ensemble_fold: self.ext_ensemble_fold,
            endianness: self.endianness,
            segy_maj_ver: self.segy_maj_ver,
            segy_min_ver: self.segy_min_ver,
            fix_length_trc_flag: self.fix_length_trc_flag,
            ext_text_hdrs_num: self.ext_text_hdrs_num,
            max_add_trc_hdrs: self.max_add_trc_hdrs,
            time_basis_code: self.time_basis_code,
            num_of_trcs_in_file: self.num_of_trcs_in_file,
            byte_off_of_first_trc: self.byte_off_of_first_trc,
            trailer_stanza_num: self.trailer_stanza_num,
        }
    }
}
