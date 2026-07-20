//! Static lookup tables for the AArch64 encoder (AsmJit's
//! `shift_op_to_ld_st_opt_map`, `size_op_table` and `size_op_map` from
//! `a64assembler.cpp`).
//!
//! Derived from AsmJit (Zlib license) — this file is an altered version; see LICENSE notices.

#![allow(dead_code, unused)]
use crate::aarch64::encoder::{SizeOp, SizeOpMap, SizeOpTable};

macro_rules! B {
    ($e: expr) => {
        1 << $e
    };
}

pub(crate) const SHIFT_OP_TO_LD_ST_OP_MAP: [u8; 16] = [
    3, 255, 255, 255, 255, 255, 255, 255, 2, 255, 255, 255, 6, 7, 255, 255,
];

pub(crate) static SIZE_OP_TABLE: [SizeOpTable; 2] = [SizeOpTable::bin(), SizeOpTable::any()];

const TABLE_BIN: usize = 0;
const TABLE_ANY: usize = 1;

pub(crate) static SIZE_OP_MAP: [SizeOpMap; 23] = [
    // kVO_V_B
    SizeOpMap {
        table_id: TABLE_BIN as u8,
        size_op_mask: SizeOp::K_Q,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K00_Q)) as u16,
    },
    // kVO_V_BH
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K00_Q) | B!(SizeOp::K01) | B!(SizeOp::K01_Q))
            as u16,
    },
    // kVO_V_BH_4S
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_BHS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_BHS_D2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_V_HS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01) | B!(SizeOp::K01_Q) | B!(SizeOp::K10) | B!(SizeOp::K10_Q))
            as u16,
    },
    // kVO_V_S
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_Q,
        accept_mask: (B!(SizeOp::K10) | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_B8H4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K01)) as u16,
    },
    // kVO_V_B8H4S2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K01) | B!(SizeOp::K10)) as u16,
    },
    // kVO_V_B8D1
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_Q,
        accept_mask: (B!(SizeOp::K00) | B!(SizeOp::K11_S)) as u16,
    },
    // kVO_V_H4S2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01) | B!(SizeOp::K10)) as u16,
    },
    // kVO_V_B16
    SizeOpMap {
        table_id: TABLE_BIN as u8,
        size_op_mask: SizeOp::K_Q,
        accept_mask: (B!(SizeOp::K00_Q)) as u16,
    },
    // kVO_V_B16H8
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00_Q) | B!(SizeOp::K01_Q)) as u16,
    },
    // kVO_V_B16H8S4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00_Q) | B!(SizeOp::K01_Q) | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_B16D2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00_Q) | B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_V_H8S4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01_Q) | B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_S4
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: 0,
        accept_mask: (B!(SizeOp::K10_Q)) as u16,
    },
    // kVO_V_D2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: 0,
        accept_mask: (B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_SV_BHS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K00_S)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K10_S)) as u16,
    },
    // kVO_SV_B8H4S2
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_S)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_S)) as u16,
    },
    // kVO_SV_HS
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K10_S)) as u16,
    },
    // kVO_V_Any
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K11_S)
            | B!(SizeOp::K11_Q)) as u16,
    },
    // kVO_SV_Any
    SizeOpMap {
        table_id: TABLE_ANY as u8,
        size_op_mask: SizeOp::K_SZ_QS,
        accept_mask: (B!(SizeOp::K00)
            | B!(SizeOp::K00_Q)
            | B!(SizeOp::K00_S)
            | B!(SizeOp::K01)
            | B!(SizeOp::K01_Q)
            | B!(SizeOp::K01_S)
            | B!(SizeOp::K10)
            | B!(SizeOp::K10_Q)
            | B!(SizeOp::K10_S)
            | B!(SizeOp::K11)
            | B!(SizeOp::K11_Q)
            | B!(SizeOp::K11_S)) as u16,
    },
];
