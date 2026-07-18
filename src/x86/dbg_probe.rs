#[test]
fn dbg_db_values() {
    use crate::x86::instdb::*;
    use crate::x86::opcode::Opcode;
    for inst in [InstId::Vaddps, InstId::Vaddpd, InstId::Vcmppd, InstId::Vgatherdpd, InstId::Vzeroupper] {
        let info = INST_INFO_TABLE[inst as usize];
        let opc = MAIN_OPCODE_TABLE[info.main_opcode_index as usize] | info.main_opcode_value as u32;
        let common = INST_COMMON_INFO_TABLE[info.common_info_index as usize];
        eprintln!(
            "{:?}: opcode={:#010x} cdshl={} cdtt={} W={} LL={} MM={:#x} PP={:#x} flags={:#x} avx512={:#x}",
            inst, opc,
            (opc & Opcode::CDSHL_MASK) >> Opcode::CDSHL_SHIFT,
            (opc & Opcode::CDTT_MASK) >> Opcode::CDTT_SHIFT,
            (opc & Opcode::W) != 0,
            (opc & Opcode::LL_MASK) >> Opcode::LL_SHIFT,
            (opc & Opcode::MM_MASK) >> Opcode::MM_SHIFT,
            (opc & Opcode::PP_VEX_MASK) >> Opcode::PP_SHIFT,
            common.flags, common.avx512_flags,
        );
    }
    panic!("dbg");
}
