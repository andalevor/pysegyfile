mod common;
mod error;
mod reader;
mod writer;
use pyo3::prelude::*;

#[pymodule]
mod pysegyfile {
    #[pymodule_export]
    use crate::reader::Reader;
    #[pymodule_export]
    use crate::writer::Writer;
    use pyo3::prelude::*;
    #[pymodule]
    mod trc_hdr_names {
        #[pymodule_export]
        const TRC_SEQ_LINE: i32 = 0;
        #[pymodule_export]
        const TRC_SEQ_SEGY: i32 = 1;
        #[pymodule_export]
        const FFID: i32 = 2;
        #[pymodule_export]
        const CHAN: i32 = 3;
        #[pymodule_export]
        const ESP: i32 = 4;
        #[pymodule_export]
        const ENS_NO: i32 = 5;
        #[pymodule_export]
        const SEQ_NO: i32 = 6;
        #[pymodule_export]
        const TRACE_ID: i32 = 7;
        #[pymodule_export]
        const VERT_SUM: i32 = 8;
        #[pymodule_export]
        const HOR_SUM: i32 = 9;
        #[pymodule_export]
        const DATA_USE: i32 = 10;
        #[pymodule_export]
        const OFFSET: i32 = 11;
        #[pymodule_export]
        const R_ELEV: i32 = 12;
        #[pymodule_export]
        const S_ELEV: i32 = 13;
        #[pymodule_export]
        const S_DEPTH: i32 = 14;
        #[pymodule_export]
        const R_DATUM: i32 = 15;
        #[pymodule_export]
        const S_DATUM: i32 = 16;
        #[pymodule_export]
        const S_WATER: i32 = 17;
        #[pymodule_export]
        const R_WATER: i32 = 18;
        #[pymodule_export]
        const ELEV_SCALAR: i32 = 19;
        #[pymodule_export]
        const COORD_SCALAR: i32 = 20;
        #[pymodule_export]
        const SOU_X: i32 = 21;
        #[pymodule_export]
        const SOU_Y: i32 = 22;
        #[pymodule_export]
        const REC_X: i32 = 23;
        #[pymodule_export]
        const REC_Y: i32 = 24;
        #[pymodule_export]
        const COORD_UNITS: i32 = 25;
        #[pymodule_export]
        const WEATH_VEL: i32 = 26;
        #[pymodule_export]
        const SUBWEATH_VEL: i32 = 27;
        #[pymodule_export]
        const S_UPHOLE: i32 = 28;
        #[pymodule_export]
        const R_UPHOLE: i32 = 29;
        #[pymodule_export]
        const S_STAT: i32 = 30;
        #[pymodule_export]
        const R_STAT: i32 = 31;
        #[pymodule_export]
        const TOT_STAT: i32 = 32;
        #[pymodule_export]
        const LAG_A: i32 = 33;
        #[pymodule_export]
        const LAG_B: i32 = 34;
        #[pymodule_export]
        const DELAY_TIME: i32 = 35;
        #[pymodule_export]
        const MUTE_START: i32 = 36;
        #[pymodule_export]
        const MUTE_END: i32 = 37;
        #[pymodule_export]
        const SAMP_NUM: i32 = 38;
        #[pymodule_export]
        const SAMP_INT: i32 = 39;
        #[pymodule_export]
        const GAIN_TYPE: i32 = 40;
        #[pymodule_export]
        const GAIN_CONST: i32 = 41;
        #[pymodule_export]
        const INIT_GAIN: i32 = 42;
        #[pymodule_export]
        const CORRELATED: i32 = 43;
        #[pymodule_export]
        const SW_START: i32 = 44;
        #[pymodule_export]
        const SW_END: i32 = 45;
        #[pymodule_export]
        const SW_LENGTH: i32 = 46;
        #[pymodule_export]
        const SW_TYPE: i32 = 47;
        #[pymodule_export]
        const SW_TAPER_START: i32 = 48;
        #[pymodule_export]
        const SW_TAPER_END: i32 = 49;
        #[pymodule_export]
        const TAPER_TYPE: i32 = 50;
        #[pymodule_export]
        const ALIAS_FILT_FREQ: i32 = 51;
        #[pymodule_export]
        const ALIAS_FILT_SLOPE: i32 = 52;
        #[pymodule_export]
        const NOTCH_FILT_FREQ: i32 = 53;
        #[pymodule_export]
        const NOTCH_FILT_SLOPE: i32 = 54;
        #[pymodule_export]
        const LOW_CUT_FREQ: i32 = 55;
        #[pymodule_export]
        const HIGH_CUT_FREQ: i32 = 56;
        #[pymodule_export]
        const LOW_CUT_SLOPE: i32 = 57;
        #[pymodule_export]
        const HIGH_CUT_SLOPE: i32 = 58;
        #[pymodule_export]
        const YEAR: i32 = 59;
        #[pymodule_export]
        const DAY: i32 = 60;
        #[pymodule_export]
        const HOUR: i32 = 61;
        #[pymodule_export]
        const MINUTE: i32 = 62;
        #[pymodule_export]
        const SECOND: i32 = 63;
        #[pymodule_export]
        const TIME_BASIS_CODE: i32 = 64;
        #[pymodule_export]
        const TRACE_WEIGHT: i32 = 65;
        #[pymodule_export]
        const GROUP_NUM_ROLL: i32 = 66;
        #[pymodule_export]
        const GROUP_NUM_FIRST: i32 = 67;
        #[pymodule_export]
        const GROUP_NUM_LAST: i32 = 68;
        #[pymodule_export]
        const GAP_SIZE: i32 = 69;
        #[pymodule_export]
        const OVER_TRAVEL: i32 = 70;
        #[pymodule_export]
        const CDP_X: i32 = 71;
        #[pymodule_export]
        const CDP_Y: i32 = 72;
        #[pymodule_export]
        const INLINE: i32 = 73;
        #[pymodule_export]
        const XLINE: i32 = 74;
        #[pymodule_export]
        const SP_NUM: i32 = 75;
        #[pymodule_export]
        const SP_NUM_SCALAR: i32 = 76;
        #[pymodule_export]
        const TR_VAL_UNIT: i32 = 77;
        #[pymodule_export]
        const TRANS_CONST_MANT: i32 = 78;
        #[pymodule_export]
        const TRANS_CONST_EXP: i32 = 79;
        #[pymodule_export]
        const TRANS_UNITS: i32 = 80;
        #[pymodule_export]
        const DEVICE_ID: i32 = 81;
        #[pymodule_export]
        const TIME_SCALAR: i32 = 82;
        #[pymodule_export]
        const SOURCE_TYPE: i32 = 83;
        #[pymodule_export]
        const SOU_V_DIR: i32 = 84;
        #[pymodule_export]
        const SOU_X_DIR: i32 = 85;
        #[pymodule_export]
        const SOU_I_DIR: i32 = 86;
        #[pymodule_export]
        const SOU_MEAS_MANT: i32 = 87;
        #[pymodule_export]
        const SOU_MEAS_EXP: i32 = 88;
        #[pymodule_export]
        const SOU_MEAS_UNIT: i32 = 89;
        #[pymodule_export]
        const EXT_TRC_SEQ_LINE: i32 = 90;
        #[pymodule_export]
        const EXT_TRC_SEQ_SEGY: i32 = 91;
        #[pymodule_export]
        const EXT_FFID: i32 = 92;
        #[pymodule_export]
        const EXT_ENS_NO: i32 = 93;
        #[pymodule_export]
        const EXT_R_ELEV: i32 = 94;
        #[pymodule_export]
        const R_DEPTH: i32 = 95;
        #[pymodule_export]
        const EXT_S_ELEV: i32 = 96;
        #[pymodule_export]
        const EXT_S_DEPTH: i32 = 97;
        #[pymodule_export]
        const EXT_R_DATUM: i32 = 98;
        #[pymodule_export]
        const EXT_S_DATUM: i32 = 99;
        #[pymodule_export]
        const EXT_S_WATER: i32 = 100;
        #[pymodule_export]
        const EXT_R_WATER: i32 = 101;
        #[pymodule_export]
        const EXT_SOU_X: i32 = 102;
        #[pymodule_export]
        const EXT_SOU_Y: i32 = 103;
        #[pymodule_export]
        const EXT_REC_X: i32 = 104;
        #[pymodule_export]
        const EXT_REC_Y: i32 = 105;
        #[pymodule_export]
        const EXT_OFFSET: i32 = 106;
        #[pymodule_export]
        const EXT_SAMP_NUM: i32 = 107;
        #[pymodule_export]
        const NANOSECOND: i32 = 108;
        #[pymodule_export]
        const EXT_SAMP_INT: i32 = 109;
        #[pymodule_export]
        const CABLE_NUM: i32 = 110;
        #[pymodule_export]
        const ADD_TRC_HDR_NUM: i32 = 111;
        #[pymodule_export]
        const LAST_TRC_FLAG: i32 = 112;
        #[pymodule_export]
        const EXT_CDP_X: i32 = 113;
        #[pymodule_export]
        const EXT_CDP_Y: i32 = 114;
    }
}
