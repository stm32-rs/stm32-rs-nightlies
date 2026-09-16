#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x04],
    iasr: IASR,
    iacr: IACR,
    _reserved3: [u8; 0x10],
    iaesr: IAESR,
    iaddr: IADDR,
    _reserved5: [u8; 0x18],
    reg1_cfgr: REG1_CFGR,
    reg1_startr: REG1_STARTR,
    reg1_endr: REG1_ENDR,
    reg1_cidcfgr: REG1_CIDCFGR,
    reg1_acfgr: REG1_ACFGR,
    reg1_astartr: REG1_ASTARTR,
    reg1_aendr: REG1_AENDR,
    reg1_anestr: REG1_ANESTR,
    reg1_bcfgr: REG1_BCFGR,
    reg1_bstartr: REG1_BSTARTR,
    reg1_bendr: REG1_BENDR,
    reg1_bnestr: REG1_BNESTR,
    _reserved17: [u8; 0x10],
    reg2_cfgr: REG2_CFGR,
    reg2_startr: REG2_STARTR,
    reg2_endr: REG2_ENDR,
    reg2_cidcfgr: REG2_CIDCFGR,
    reg2_acfgr: REG2_ACFGR,
    reg2_astartr: REG2_ASTARTR,
    reg2_aendr: REG2_AENDR,
    reg2_anestr: REG2_ANESTR,
    reg2_bcfgr: REG2_BCFGR,
    reg2_bstartr: REG2_BSTARTR,
    reg2_bendr: REG2_BENDR,
    reg2_bnestr: REG2_BNESTR,
    _reserved29: [u8; 0x10],
    reg3_cfgr: REG3_CFGR,
    reg3_startr: REG3_STARTR,
    reg3_endr: REG3_ENDR,
    reg3_cidcfgr: REG3_CIDCFGR,
    reg3_acfgr: REG3_ACFGR,
    reg3_astartr: REG3_ASTARTR,
    reg3_aendr: REG3_AENDR,
    reg3_anestr: REG3_ANESTR,
    reg3_bcfgr: REG3_BCFGR,
    reg3_bstartr: REG3_BSTARTR,
    reg3_bendr: REG3_BENDR,
    reg3_bnestr: REG3_BNESTR,
    _reserved41: [u8; 0x10],
    reg4_cfgr: REG4_CFGR,
    reg4_startr: REG4_STARTR,
    reg4_endr: REG4_ENDR,
    reg4_cidcfgr: REG4_CIDCFGR,
    reg4_acfgr: REG4_ACFGR,
    reg4_astartr: REG4_ASTARTR,
    reg4_aendr: REG4_AENDR,
    reg4_anestr: REG4_ANESTR,
    reg4_bcfgr: REG4_BCFGR,
    reg4_bstartr: REG4_BSTARTR,
    reg4_bendr: REG4_BENDR,
    reg4_bnestr: REG4_BNESTR,
    _reserved53: [u8; 0x10],
    reg5_cfgr: REG5_CFGR,
    reg5_startr: REG5_STARTR,
    reg5_endr: REG5_ENDR,
    reg5_cidcfgr: REG5_CIDCFGR,
    reg5_acfgr: REG5_ACFGR,
    reg5_astartr: REG5_ASTARTR,
    reg5_aendr: REG5_AENDR,
    reg5_anestr: REG5_ANESTR,
    reg5_bcfgr: REG5_BCFGR,
    reg5_bstartr: REG5_BSTARTR,
    reg5_bendr: REG5_BENDR,
    reg5_bnestr: REG5_BNESTR,
    _reserved65: [u8; 0x10],
    reg6_cfgr: REG6_CFGR,
    reg6_startr: REG6_STARTR,
    reg6_endr: REG6_ENDR,
    reg6_cidcfgr: REG6_CIDCFGR,
    reg6_acfgr: REG6_ACFGR,
    reg6_astartr: REG6_ASTARTR,
    reg6_aendr: REG6_AENDR,
    reg6_anestr: REG6_ANESTR,
    reg6_bcfgr: REG6_BCFGR,
    reg6_bstartr: REG6_BSTARTR,
    reg6_bendr: REG6_BENDR,
    reg6_bnestr: REG6_BNESTR,
    _reserved77: [u8; 0x10],
    reg7_cfgr: REG7_CFGR,
    reg7_startr: REG7_STARTR,
    reg7_endr: REG7_ENDR,
    reg7_cidcfgr: REG7_CIDCFGR,
    reg7_acfgr: REG7_ACFGR,
    reg7_astartr: REG7_ASTARTR,
    reg7_aendr: REG7_AENDR,
    reg7_anestr: REG7_ANESTR,
    reg7_bcfgr: REG7_BCFGR,
    reg7_bstartr: REG7_BSTARTR,
    reg7_bendr: REG7_BENDR,
    reg7_bnestr: REG7_BNESTR,
    _reserved89: [u8; 0x10],
    reg8_cfgr: REG8_CFGR,
    reg8_startr: REG8_STARTR,
    reg8_endr: REG8_ENDR,
    reg8_cidcfgr: REG8_CIDCFGR,
    reg8_acfgr: REG8_ACFGR,
    reg8_astartr: REG8_ASTARTR,
    reg8_aendr: REG8_AENDR,
    reg8_anestr: REG8_ANESTR,
    reg8_bcfgr: REG8_BCFGR,
    reg8_bstartr: REG8_BSTARTR,
    reg8_bendr: REG8_BENDR,
    reg8_bnestr: REG8_BNESTR,
    _reserved101: [u8; 0x10],
    reg9_cfgr: REG9_CFGR,
    reg9_startr: REG9_STARTR,
    reg9_endr: REG9_ENDR,
    reg9_cidcfgr: REG9_CIDCFGR,
    reg9_acfgr: REG9_ACFGR,
    reg9_astartr: REG9_ASTARTR,
    reg9_aendr: REG9_AENDR,
    reg9_anestr: REG9_ANESTR,
    reg9_bcfgr: REG9_BCFGR,
    reg9_bstartr: REG9_BSTARTR,
    reg9_bendr: REG9_BENDR,
    reg9_bnestr: REG9_BNESTR,
    _reserved113: [u8; 0x10],
    reg10_cfgr: REG10_CFGR,
    reg10_startr: REG10_STARTR,
    reg10_endr: REG10_ENDR,
    reg10_cidcfgr: REG10_CIDCFGR,
    reg10_acfgr: REG10_ACFGR,
    reg10_astartr: REG10_ASTARTR,
    reg10_aendr: REG10_AENDR,
    reg10_anestr: REG10_ANESTR,
    reg10_bcfgr: REG10_BCFGR,
    reg10_bstartr: REG10_BSTARTR,
    reg10_bendr: REG10_BENDR,
    reg10_bnestr: REG10_BNESTR,
    _reserved125: [u8; 0x10],
    reg11_cfgr: REG11_CFGR,
    reg11_startr: REG11_STARTR,
    reg11_endr: REG11_ENDR,
    reg11_cidcfgr: REG11_CIDCFGR,
    reg11_acfgr: REG11_ACFGR,
    reg11_astartr: REG11_ASTARTR,
    reg11_aendr: REG11_AENDR,
    reg11_anestr: REG11_ANESTR,
    reg11_bcfgr: REG11_BCFGR,
    reg11_bstartr: REG11_BSTARTR,
    reg11_bendr: REG11_BENDR,
    reg11_bnestr: REG11_BNESTR,
    _reserved137: [u8; 0x10],
    reg12_cfgr: REG12_CFGR,
    reg12_startr: REG12_STARTR,
    reg12_endr: REG12_ENDR,
    reg12_cidcfgr: REG12_CIDCFGR,
    reg12_acfgr: REG12_ACFGR,
    reg12_astartr: REG12_ASTARTR,
    reg12_aendr: REG12_AENDR,
    reg12_anestr: REG12_ANESTR,
    reg12_bcfgr: REG12_BCFGR,
    reg12_bstartr: REG12_BSTARTR,
    reg12_bendr: REG12_BENDR,
    reg12_bnestr: REG12_BNESTR,
    _reserved149: [u8; 0x10],
    reg13_cfgr: REG13_CFGR,
    reg13_startr: REG13_STARTR,
    reg13_endr: REG13_ENDR,
    reg13_cidcfgr: REG13_CIDCFGR,
    reg13_acfgr: REG13_ACFGR,
    reg13_astartr: REG13_ASTARTR,
    reg13_aendr: REG13_AENDR,
    reg13_anestr: REG13_ANESTR,
    reg13_bcfgr: REG13_BCFGR,
    reg13_bstartr: REG13_BSTARTR,
    reg13_bendr: REG13_BENDR,
    reg13_bnestr: REG13_BNESTR,
    _reserved161: [u8; 0x10],
    reg14_cfgr: REG14_CFGR,
    reg14_startr: REG14_STARTR,
    reg14_endr: REG14_ENDR,
    reg14_cidcfgr: REG14_CIDCFGR,
    reg14_acfgr: REG14_ACFGR,
    reg14_astartr: REG14_ASTARTR,
    reg14_aendr: REG14_AENDR,
    reg14_anestr: REG14_ANESTR,
    reg14_bcfgr: REG14_BCFGR,
    reg14_bstartr: REG14_BSTARTR,
    reg14_bendr: REG14_BENDR,
    reg14_bnestr: REG14_BNESTR,
    _reserved173: [u8; 0x10],
    reg15_cfgr: REG15_CFGR,
    reg15_startr: REG15_STARTR,
    reg15_endr: REG15_ENDR,
    reg15_cidcfgr: REG15_CIDCFGR,
    reg15_acfgr: REG15_ACFGR,
    reg15_astartr: REG15_ASTARTR,
    reg15_aendr: REG15_AENDR,
    reg15_anestr: REG15_ANESTR,
    reg15_bcfgr: REG15_BCFGR,
    reg15_bstartr: REG15_BSTARTR,
    reg15_bendr: REG15_BENDR,
    reg15_bnestr: REG15_BNESTR,
}
impl RegisterBlock {
    ///0x00 - RISAF configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - RISAF illegal access status register
    #[inline(always)]
    pub const fn iasr(&self) -> &IASR {
        &self.iasr
    }
    ///0x0c - RISAF illegal access clear register
    #[inline(always)]
    pub const fn iacr(&self) -> &IACR {
        &self.iacr
    }
    ///0x20 - RISAF illegal access error status register
    #[inline(always)]
    pub const fn iaesr(&self) -> &IAESR {
        &self.iaesr
    }
    ///0x24 - RISAF illegal address register
    #[inline(always)]
    pub const fn iaddr(&self) -> &IADDR {
        &self.iaddr
    }
    ///0x40 - RISAF region 1 configuration register
    #[inline(always)]
    pub const fn reg1_cfgr(&self) -> &REG1_CFGR {
        &self.reg1_cfgr
    }
    ///0x44 - RISAF region 1 start-address register
    #[inline(always)]
    pub const fn reg1_startr(&self) -> &REG1_STARTR {
        &self.reg1_startr
    }
    ///0x48 - RISAF region 1 end-address register
    #[inline(always)]
    pub const fn reg1_endr(&self) -> &REG1_ENDR {
        &self.reg1_endr
    }
    ///0x4c - RISAF region 1 CID configuration register
    #[inline(always)]
    pub const fn reg1_cidcfgr(&self) -> &REG1_CIDCFGR {
        &self.reg1_cidcfgr
    }
    ///0x50 - RISAF region 1 subregion A configuration register
    #[inline(always)]
    pub const fn reg1_acfgr(&self) -> &REG1_ACFGR {
        &self.reg1_acfgr
    }
    ///0x54 - RISAF region 1 subregion A start-address register
    #[inline(always)]
    pub const fn reg1_astartr(&self) -> &REG1_ASTARTR {
        &self.reg1_astartr
    }
    ///0x58 - RISAF region 1 subregion A end-address register
    #[inline(always)]
    pub const fn reg1_aendr(&self) -> &REG1_AENDR {
        &self.reg1_aendr
    }
    ///0x5c - RISAF region 1 subregion A nested mode register
    #[inline(always)]
    pub const fn reg1_anestr(&self) -> &REG1_ANESTR {
        &self.reg1_anestr
    }
    ///0x60 - RISAF region 1 subregion B configuration register
    #[inline(always)]
    pub const fn reg1_bcfgr(&self) -> &REG1_BCFGR {
        &self.reg1_bcfgr
    }
    ///0x64 - RISAF region 1 subregion B start-address register
    #[inline(always)]
    pub const fn reg1_bstartr(&self) -> &REG1_BSTARTR {
        &self.reg1_bstartr
    }
    ///0x68 - RISAF region 1 subregion B end-address register
    #[inline(always)]
    pub const fn reg1_bendr(&self) -> &REG1_BENDR {
        &self.reg1_bendr
    }
    ///0x6c - RISAF region 1 subregion B nested mode register
    #[inline(always)]
    pub const fn reg1_bnestr(&self) -> &REG1_BNESTR {
        &self.reg1_bnestr
    }
    ///0x80 - RISAF region 2 configuration register
    #[inline(always)]
    pub const fn reg2_cfgr(&self) -> &REG2_CFGR {
        &self.reg2_cfgr
    }
    ///0x84 - RISAF region 2 start-address register
    #[inline(always)]
    pub const fn reg2_startr(&self) -> &REG2_STARTR {
        &self.reg2_startr
    }
    ///0x88 - RISAF region 2 end-address register
    #[inline(always)]
    pub const fn reg2_endr(&self) -> &REG2_ENDR {
        &self.reg2_endr
    }
    ///0x8c - RISAF region 2 CID configuration register
    #[inline(always)]
    pub const fn reg2_cidcfgr(&self) -> &REG2_CIDCFGR {
        &self.reg2_cidcfgr
    }
    ///0x90 - RISAF region 2 subregion A configuration register
    #[inline(always)]
    pub const fn reg2_acfgr(&self) -> &REG2_ACFGR {
        &self.reg2_acfgr
    }
    ///0x94 - RISAF region 2 subregion A start-address register
    #[inline(always)]
    pub const fn reg2_astartr(&self) -> &REG2_ASTARTR {
        &self.reg2_astartr
    }
    ///0x98 - RISAF region 2 subregion A end-address register
    #[inline(always)]
    pub const fn reg2_aendr(&self) -> &REG2_AENDR {
        &self.reg2_aendr
    }
    ///0x9c - RISAF region 2 subregion A nested mode register
    #[inline(always)]
    pub const fn reg2_anestr(&self) -> &REG2_ANESTR {
        &self.reg2_anestr
    }
    ///0xa0 - RISAF region 2 subregion B configuration register
    #[inline(always)]
    pub const fn reg2_bcfgr(&self) -> &REG2_BCFGR {
        &self.reg2_bcfgr
    }
    ///0xa4 - RISAF region 2 subregion B start-address register
    #[inline(always)]
    pub const fn reg2_bstartr(&self) -> &REG2_BSTARTR {
        &self.reg2_bstartr
    }
    ///0xa8 - RISAF region 2 subregion B end-address register
    #[inline(always)]
    pub const fn reg2_bendr(&self) -> &REG2_BENDR {
        &self.reg2_bendr
    }
    ///0xac - RISAF region 2 subregion B nested mode register
    #[inline(always)]
    pub const fn reg2_bnestr(&self) -> &REG2_BNESTR {
        &self.reg2_bnestr
    }
    ///0xc0 - RISAF region 3 configuration register
    #[inline(always)]
    pub const fn reg3_cfgr(&self) -> &REG3_CFGR {
        &self.reg3_cfgr
    }
    ///0xc4 - RISAF region 3 start-address register
    #[inline(always)]
    pub const fn reg3_startr(&self) -> &REG3_STARTR {
        &self.reg3_startr
    }
    ///0xc8 - RISAF region 3 end-address register
    #[inline(always)]
    pub const fn reg3_endr(&self) -> &REG3_ENDR {
        &self.reg3_endr
    }
    ///0xcc - RISAF region 3 CID configuration register
    #[inline(always)]
    pub const fn reg3_cidcfgr(&self) -> &REG3_CIDCFGR {
        &self.reg3_cidcfgr
    }
    ///0xd0 - RISAF region 3 subregion A configuration register
    #[inline(always)]
    pub const fn reg3_acfgr(&self) -> &REG3_ACFGR {
        &self.reg3_acfgr
    }
    ///0xd4 - RISAF region 3 subregion A start-address register
    #[inline(always)]
    pub const fn reg3_astartr(&self) -> &REG3_ASTARTR {
        &self.reg3_astartr
    }
    ///0xd8 - RISAF region 3 subregion A end-address register
    #[inline(always)]
    pub const fn reg3_aendr(&self) -> &REG3_AENDR {
        &self.reg3_aendr
    }
    ///0xdc - RISAF region 3 subregion A nested mode register
    #[inline(always)]
    pub const fn reg3_anestr(&self) -> &REG3_ANESTR {
        &self.reg3_anestr
    }
    ///0xe0 - RISAF region 3 subregion B configuration register
    #[inline(always)]
    pub const fn reg3_bcfgr(&self) -> &REG3_BCFGR {
        &self.reg3_bcfgr
    }
    ///0xe4 - RISAF region 3 subregion B start-address register
    #[inline(always)]
    pub const fn reg3_bstartr(&self) -> &REG3_BSTARTR {
        &self.reg3_bstartr
    }
    ///0xe8 - RISAF region 3 subregion B end-address register
    #[inline(always)]
    pub const fn reg3_bendr(&self) -> &REG3_BENDR {
        &self.reg3_bendr
    }
    ///0xec - RISAF region 3 subregion B nested mode register
    #[inline(always)]
    pub const fn reg3_bnestr(&self) -> &REG3_BNESTR {
        &self.reg3_bnestr
    }
    ///0x100 - RISAF region 4 configuration register
    #[inline(always)]
    pub const fn reg4_cfgr(&self) -> &REG4_CFGR {
        &self.reg4_cfgr
    }
    ///0x104 - RISAF region 4 start-address register
    #[inline(always)]
    pub const fn reg4_startr(&self) -> &REG4_STARTR {
        &self.reg4_startr
    }
    ///0x108 - RISAF region 4 end-address register
    #[inline(always)]
    pub const fn reg4_endr(&self) -> &REG4_ENDR {
        &self.reg4_endr
    }
    ///0x10c - RISAF region 4 CID configuration register
    #[inline(always)]
    pub const fn reg4_cidcfgr(&self) -> &REG4_CIDCFGR {
        &self.reg4_cidcfgr
    }
    ///0x110 - RISAF region 4 subregion A configuration register
    #[inline(always)]
    pub const fn reg4_acfgr(&self) -> &REG4_ACFGR {
        &self.reg4_acfgr
    }
    ///0x114 - RISAF region 4 subregion A start-address register
    #[inline(always)]
    pub const fn reg4_astartr(&self) -> &REG4_ASTARTR {
        &self.reg4_astartr
    }
    ///0x118 - RISAF region 4 subregion A end-address register
    #[inline(always)]
    pub const fn reg4_aendr(&self) -> &REG4_AENDR {
        &self.reg4_aendr
    }
    ///0x11c - RISAF region 4 subregion A nested mode register
    #[inline(always)]
    pub const fn reg4_anestr(&self) -> &REG4_ANESTR {
        &self.reg4_anestr
    }
    ///0x120 - RISAF region 4 subregion B configuration register
    #[inline(always)]
    pub const fn reg4_bcfgr(&self) -> &REG4_BCFGR {
        &self.reg4_bcfgr
    }
    ///0x124 - RISAF region 4 subregion B start-address register
    #[inline(always)]
    pub const fn reg4_bstartr(&self) -> &REG4_BSTARTR {
        &self.reg4_bstartr
    }
    ///0x128 - RISAF region 4 subregion B end-address register
    #[inline(always)]
    pub const fn reg4_bendr(&self) -> &REG4_BENDR {
        &self.reg4_bendr
    }
    ///0x12c - RISAF region 4 subregion B nested mode register
    #[inline(always)]
    pub const fn reg4_bnestr(&self) -> &REG4_BNESTR {
        &self.reg4_bnestr
    }
    ///0x140 - RISAF region 5 configuration register
    #[inline(always)]
    pub const fn reg5_cfgr(&self) -> &REG5_CFGR {
        &self.reg5_cfgr
    }
    ///0x144 - RISAF region 5 start-address register
    #[inline(always)]
    pub const fn reg5_startr(&self) -> &REG5_STARTR {
        &self.reg5_startr
    }
    ///0x148 - RISAF region 5 end-address register
    #[inline(always)]
    pub const fn reg5_endr(&self) -> &REG5_ENDR {
        &self.reg5_endr
    }
    ///0x14c - RISAF region 5 CID configuration register
    #[inline(always)]
    pub const fn reg5_cidcfgr(&self) -> &REG5_CIDCFGR {
        &self.reg5_cidcfgr
    }
    ///0x150 - RISAF region 5 subregion A configuration register
    #[inline(always)]
    pub const fn reg5_acfgr(&self) -> &REG5_ACFGR {
        &self.reg5_acfgr
    }
    ///0x154 - RISAF region 5 subregion A start-address register
    #[inline(always)]
    pub const fn reg5_astartr(&self) -> &REG5_ASTARTR {
        &self.reg5_astartr
    }
    ///0x158 - RISAF region 5 subregion A end-address register
    #[inline(always)]
    pub const fn reg5_aendr(&self) -> &REG5_AENDR {
        &self.reg5_aendr
    }
    ///0x15c - RISAF region 5 subregion A nested mode register
    #[inline(always)]
    pub const fn reg5_anestr(&self) -> &REG5_ANESTR {
        &self.reg5_anestr
    }
    ///0x160 - RISAF region 5 subregion B configuration register
    #[inline(always)]
    pub const fn reg5_bcfgr(&self) -> &REG5_BCFGR {
        &self.reg5_bcfgr
    }
    ///0x164 - RISAF region 5 subregion B start-address register
    #[inline(always)]
    pub const fn reg5_bstartr(&self) -> &REG5_BSTARTR {
        &self.reg5_bstartr
    }
    ///0x168 - RISAF region 5 subregion B end-address register
    #[inline(always)]
    pub const fn reg5_bendr(&self) -> &REG5_BENDR {
        &self.reg5_bendr
    }
    ///0x16c - RISAF region 5 subregion B nested mode register
    #[inline(always)]
    pub const fn reg5_bnestr(&self) -> &REG5_BNESTR {
        &self.reg5_bnestr
    }
    ///0x180 - RISAF region 6 configuration register
    #[inline(always)]
    pub const fn reg6_cfgr(&self) -> &REG6_CFGR {
        &self.reg6_cfgr
    }
    ///0x184 - RISAF region 6 start-address register
    #[inline(always)]
    pub const fn reg6_startr(&self) -> &REG6_STARTR {
        &self.reg6_startr
    }
    ///0x188 - RISAF region 6 end-address register
    #[inline(always)]
    pub const fn reg6_endr(&self) -> &REG6_ENDR {
        &self.reg6_endr
    }
    ///0x18c - RISAF region 6 CID configuration register
    #[inline(always)]
    pub const fn reg6_cidcfgr(&self) -> &REG6_CIDCFGR {
        &self.reg6_cidcfgr
    }
    ///0x190 - RISAF region 6 subregion A configuration register
    #[inline(always)]
    pub const fn reg6_acfgr(&self) -> &REG6_ACFGR {
        &self.reg6_acfgr
    }
    ///0x194 - RISAF region 6 subregion A start-address register
    #[inline(always)]
    pub const fn reg6_astartr(&self) -> &REG6_ASTARTR {
        &self.reg6_astartr
    }
    ///0x198 - RISAF region 6 subregion A end-address register
    #[inline(always)]
    pub const fn reg6_aendr(&self) -> &REG6_AENDR {
        &self.reg6_aendr
    }
    ///0x19c - RISAF region 6 subregion A nested mode register
    #[inline(always)]
    pub const fn reg6_anestr(&self) -> &REG6_ANESTR {
        &self.reg6_anestr
    }
    ///0x1a0 - RISAF region 6 subregion B configuration register
    #[inline(always)]
    pub const fn reg6_bcfgr(&self) -> &REG6_BCFGR {
        &self.reg6_bcfgr
    }
    ///0x1a4 - RISAF region 6 subregion B start-address register
    #[inline(always)]
    pub const fn reg6_bstartr(&self) -> &REG6_BSTARTR {
        &self.reg6_bstartr
    }
    ///0x1a8 - RISAF region 6 subregion B end-address register
    #[inline(always)]
    pub const fn reg6_bendr(&self) -> &REG6_BENDR {
        &self.reg6_bendr
    }
    ///0x1ac - RISAF region 6 subregion B nested mode register
    #[inline(always)]
    pub const fn reg6_bnestr(&self) -> &REG6_BNESTR {
        &self.reg6_bnestr
    }
    ///0x1c0 - RISAF region 7 configuration register
    #[inline(always)]
    pub const fn reg7_cfgr(&self) -> &REG7_CFGR {
        &self.reg7_cfgr
    }
    ///0x1c4 - RISAF region 7 start-address register
    #[inline(always)]
    pub const fn reg7_startr(&self) -> &REG7_STARTR {
        &self.reg7_startr
    }
    ///0x1c8 - RISAF region 7 end-address register
    #[inline(always)]
    pub const fn reg7_endr(&self) -> &REG7_ENDR {
        &self.reg7_endr
    }
    ///0x1cc - RISAF region 7 CID configuration register
    #[inline(always)]
    pub const fn reg7_cidcfgr(&self) -> &REG7_CIDCFGR {
        &self.reg7_cidcfgr
    }
    ///0x1d0 - RISAF region 7 subregion A configuration register
    #[inline(always)]
    pub const fn reg7_acfgr(&self) -> &REG7_ACFGR {
        &self.reg7_acfgr
    }
    ///0x1d4 - RISAF region 7 subregion A start-address register
    #[inline(always)]
    pub const fn reg7_astartr(&self) -> &REG7_ASTARTR {
        &self.reg7_astartr
    }
    ///0x1d8 - RISAF region 7 subregion A end-address register
    #[inline(always)]
    pub const fn reg7_aendr(&self) -> &REG7_AENDR {
        &self.reg7_aendr
    }
    ///0x1dc - RISAF region 7 subregion A nested mode register
    #[inline(always)]
    pub const fn reg7_anestr(&self) -> &REG7_ANESTR {
        &self.reg7_anestr
    }
    ///0x1e0 - RISAF region 7 subregion B configuration register
    #[inline(always)]
    pub const fn reg7_bcfgr(&self) -> &REG7_BCFGR {
        &self.reg7_bcfgr
    }
    ///0x1e4 - RISAF region 7 subregion B start-address register
    #[inline(always)]
    pub const fn reg7_bstartr(&self) -> &REG7_BSTARTR {
        &self.reg7_bstartr
    }
    ///0x1e8 - RISAF region 7 subregion B end-address register
    #[inline(always)]
    pub const fn reg7_bendr(&self) -> &REG7_BENDR {
        &self.reg7_bendr
    }
    ///0x1ec - RISAF region 7 subregion B nested mode register
    #[inline(always)]
    pub const fn reg7_bnestr(&self) -> &REG7_BNESTR {
        &self.reg7_bnestr
    }
    ///0x200 - RISAF region 8 configuration register
    #[inline(always)]
    pub const fn reg8_cfgr(&self) -> &REG8_CFGR {
        &self.reg8_cfgr
    }
    ///0x204 - RISAF region 8 start-address register
    #[inline(always)]
    pub const fn reg8_startr(&self) -> &REG8_STARTR {
        &self.reg8_startr
    }
    ///0x208 - RISAF region 8 end-address register
    #[inline(always)]
    pub const fn reg8_endr(&self) -> &REG8_ENDR {
        &self.reg8_endr
    }
    ///0x20c - RISAF region 8 CID configuration register
    #[inline(always)]
    pub const fn reg8_cidcfgr(&self) -> &REG8_CIDCFGR {
        &self.reg8_cidcfgr
    }
    ///0x210 - RISAF region 8 subregion A configuration register
    #[inline(always)]
    pub const fn reg8_acfgr(&self) -> &REG8_ACFGR {
        &self.reg8_acfgr
    }
    ///0x214 - RISAF region 8 subregion A start-address register
    #[inline(always)]
    pub const fn reg8_astartr(&self) -> &REG8_ASTARTR {
        &self.reg8_astartr
    }
    ///0x218 - RISAF region 8 subregion A end-address register
    #[inline(always)]
    pub const fn reg8_aendr(&self) -> &REG8_AENDR {
        &self.reg8_aendr
    }
    ///0x21c - RISAF region 8 subregion A nested mode register
    #[inline(always)]
    pub const fn reg8_anestr(&self) -> &REG8_ANESTR {
        &self.reg8_anestr
    }
    ///0x220 - RISAF region 8 subregion B configuration register
    #[inline(always)]
    pub const fn reg8_bcfgr(&self) -> &REG8_BCFGR {
        &self.reg8_bcfgr
    }
    ///0x224 - RISAF region 8 subregion B start-address register
    #[inline(always)]
    pub const fn reg8_bstartr(&self) -> &REG8_BSTARTR {
        &self.reg8_bstartr
    }
    ///0x228 - RISAF region 8 subregion B end-address register
    #[inline(always)]
    pub const fn reg8_bendr(&self) -> &REG8_BENDR {
        &self.reg8_bendr
    }
    ///0x22c - RISAF region 8 subregion B nested mode register
    #[inline(always)]
    pub const fn reg8_bnestr(&self) -> &REG8_BNESTR {
        &self.reg8_bnestr
    }
    ///0x240 - RISAF region 9 configuration register
    #[inline(always)]
    pub const fn reg9_cfgr(&self) -> &REG9_CFGR {
        &self.reg9_cfgr
    }
    ///0x244 - RISAF region 9 start-address register
    #[inline(always)]
    pub const fn reg9_startr(&self) -> &REG9_STARTR {
        &self.reg9_startr
    }
    ///0x248 - RISAF region 9 end-address register
    #[inline(always)]
    pub const fn reg9_endr(&self) -> &REG9_ENDR {
        &self.reg9_endr
    }
    ///0x24c - RISAF region 9 CID configuration register
    #[inline(always)]
    pub const fn reg9_cidcfgr(&self) -> &REG9_CIDCFGR {
        &self.reg9_cidcfgr
    }
    ///0x250 - RISAF region 9 subregion A configuration register
    #[inline(always)]
    pub const fn reg9_acfgr(&self) -> &REG9_ACFGR {
        &self.reg9_acfgr
    }
    ///0x254 - RISAF region 9 subregion A start-address register
    #[inline(always)]
    pub const fn reg9_astartr(&self) -> &REG9_ASTARTR {
        &self.reg9_astartr
    }
    ///0x258 - RISAF region 9 subregion A end-address register
    #[inline(always)]
    pub const fn reg9_aendr(&self) -> &REG9_AENDR {
        &self.reg9_aendr
    }
    ///0x25c - RISAF region 9 subregion A nested mode register
    #[inline(always)]
    pub const fn reg9_anestr(&self) -> &REG9_ANESTR {
        &self.reg9_anestr
    }
    ///0x260 - RISAF region 9 subregion B configuration register
    #[inline(always)]
    pub const fn reg9_bcfgr(&self) -> &REG9_BCFGR {
        &self.reg9_bcfgr
    }
    ///0x264 - RISAF region 9 subregion B start-address register
    #[inline(always)]
    pub const fn reg9_bstartr(&self) -> &REG9_BSTARTR {
        &self.reg9_bstartr
    }
    ///0x268 - RISAF region 9 subregion B end-address register
    #[inline(always)]
    pub const fn reg9_bendr(&self) -> &REG9_BENDR {
        &self.reg9_bendr
    }
    ///0x26c - RISAF region 9 subregion B nested mode register
    #[inline(always)]
    pub const fn reg9_bnestr(&self) -> &REG9_BNESTR {
        &self.reg9_bnestr
    }
    ///0x280 - RISAF region 10 configuration register
    #[inline(always)]
    pub const fn reg10_cfgr(&self) -> &REG10_CFGR {
        &self.reg10_cfgr
    }
    ///0x284 - RISAF region 10 start-address register
    #[inline(always)]
    pub const fn reg10_startr(&self) -> &REG10_STARTR {
        &self.reg10_startr
    }
    ///0x288 - RISAF region 10 end-address register
    #[inline(always)]
    pub const fn reg10_endr(&self) -> &REG10_ENDR {
        &self.reg10_endr
    }
    ///0x28c - RISAF region 10 CID configuration register
    #[inline(always)]
    pub const fn reg10_cidcfgr(&self) -> &REG10_CIDCFGR {
        &self.reg10_cidcfgr
    }
    ///0x290 - RISAF region 10 subregion A configuration register
    #[inline(always)]
    pub const fn reg10_acfgr(&self) -> &REG10_ACFGR {
        &self.reg10_acfgr
    }
    ///0x294 - RISAF region 10 subregion A start-address register
    #[inline(always)]
    pub const fn reg10_astartr(&self) -> &REG10_ASTARTR {
        &self.reg10_astartr
    }
    ///0x298 - RISAF region 10 subregion A end-address register
    #[inline(always)]
    pub const fn reg10_aendr(&self) -> &REG10_AENDR {
        &self.reg10_aendr
    }
    ///0x29c - RISAF region 10 subregion A nested mode register
    #[inline(always)]
    pub const fn reg10_anestr(&self) -> &REG10_ANESTR {
        &self.reg10_anestr
    }
    ///0x2a0 - RISAF region 10 subregion B configuration register
    #[inline(always)]
    pub const fn reg10_bcfgr(&self) -> &REG10_BCFGR {
        &self.reg10_bcfgr
    }
    ///0x2a4 - RISAF region 10 subregion B start-address register
    #[inline(always)]
    pub const fn reg10_bstartr(&self) -> &REG10_BSTARTR {
        &self.reg10_bstartr
    }
    ///0x2a8 - RISAF region 10 subregion B end-address register
    #[inline(always)]
    pub const fn reg10_bendr(&self) -> &REG10_BENDR {
        &self.reg10_bendr
    }
    ///0x2ac - RISAF region 10 subregion B nested mode register
    #[inline(always)]
    pub const fn reg10_bnestr(&self) -> &REG10_BNESTR {
        &self.reg10_bnestr
    }
    ///0x2c0 - RISAF region 11 configuration register
    #[inline(always)]
    pub const fn reg11_cfgr(&self) -> &REG11_CFGR {
        &self.reg11_cfgr
    }
    ///0x2c4 - RISAF region 11 start-address register
    #[inline(always)]
    pub const fn reg11_startr(&self) -> &REG11_STARTR {
        &self.reg11_startr
    }
    ///0x2c8 - RISAF region 11 end-address register
    #[inline(always)]
    pub const fn reg11_endr(&self) -> &REG11_ENDR {
        &self.reg11_endr
    }
    ///0x2cc - RISAF region 11 CID configuration register
    #[inline(always)]
    pub const fn reg11_cidcfgr(&self) -> &REG11_CIDCFGR {
        &self.reg11_cidcfgr
    }
    ///0x2d0 - RISAF region 11 subregion A configuration register
    #[inline(always)]
    pub const fn reg11_acfgr(&self) -> &REG11_ACFGR {
        &self.reg11_acfgr
    }
    ///0x2d4 - RISAF region 11 subregion A start-address register
    #[inline(always)]
    pub const fn reg11_astartr(&self) -> &REG11_ASTARTR {
        &self.reg11_astartr
    }
    ///0x2d8 - RISAF region 11 subregion A end-address register
    #[inline(always)]
    pub const fn reg11_aendr(&self) -> &REG11_AENDR {
        &self.reg11_aendr
    }
    ///0x2dc - RISAF region 11 subregion A nested mode register
    #[inline(always)]
    pub const fn reg11_anestr(&self) -> &REG11_ANESTR {
        &self.reg11_anestr
    }
    ///0x2e0 - RISAF region 11 subregion B configuration register
    #[inline(always)]
    pub const fn reg11_bcfgr(&self) -> &REG11_BCFGR {
        &self.reg11_bcfgr
    }
    ///0x2e4 - RISAF region 11 subregion B start-address register
    #[inline(always)]
    pub const fn reg11_bstartr(&self) -> &REG11_BSTARTR {
        &self.reg11_bstartr
    }
    ///0x2e8 - RISAF region 11 subregion B end-address register
    #[inline(always)]
    pub const fn reg11_bendr(&self) -> &REG11_BENDR {
        &self.reg11_bendr
    }
    ///0x2ec - RISAF region 11 subregion B nested mode register
    #[inline(always)]
    pub const fn reg11_bnestr(&self) -> &REG11_BNESTR {
        &self.reg11_bnestr
    }
    ///0x300 - RISAF region 12 configuration register
    #[inline(always)]
    pub const fn reg12_cfgr(&self) -> &REG12_CFGR {
        &self.reg12_cfgr
    }
    ///0x304 - RISAF region 12 start-address register
    #[inline(always)]
    pub const fn reg12_startr(&self) -> &REG12_STARTR {
        &self.reg12_startr
    }
    ///0x308 - RISAF region 12 end-address register
    #[inline(always)]
    pub const fn reg12_endr(&self) -> &REG12_ENDR {
        &self.reg12_endr
    }
    ///0x30c - RISAF region 12 CID configuration register
    #[inline(always)]
    pub const fn reg12_cidcfgr(&self) -> &REG12_CIDCFGR {
        &self.reg12_cidcfgr
    }
    ///0x310 - RISAF region 12 subregion A configuration register
    #[inline(always)]
    pub const fn reg12_acfgr(&self) -> &REG12_ACFGR {
        &self.reg12_acfgr
    }
    ///0x314 - RISAF region 12 subregion A start-address register
    #[inline(always)]
    pub const fn reg12_astartr(&self) -> &REG12_ASTARTR {
        &self.reg12_astartr
    }
    ///0x318 - RISAF region 12 subregion A end-address register
    #[inline(always)]
    pub const fn reg12_aendr(&self) -> &REG12_AENDR {
        &self.reg12_aendr
    }
    ///0x31c - RISAF region 12 subregion A nested mode register
    #[inline(always)]
    pub const fn reg12_anestr(&self) -> &REG12_ANESTR {
        &self.reg12_anestr
    }
    ///0x320 - RISAF region 12 subregion B configuration register
    #[inline(always)]
    pub const fn reg12_bcfgr(&self) -> &REG12_BCFGR {
        &self.reg12_bcfgr
    }
    ///0x324 - RISAF region 12 subregion B start-address register
    #[inline(always)]
    pub const fn reg12_bstartr(&self) -> &REG12_BSTARTR {
        &self.reg12_bstartr
    }
    ///0x328 - RISAF region 12 subregion B end-address register
    #[inline(always)]
    pub const fn reg12_bendr(&self) -> &REG12_BENDR {
        &self.reg12_bendr
    }
    ///0x32c - RISAF region 12 subregion B nested mode register
    #[inline(always)]
    pub const fn reg12_bnestr(&self) -> &REG12_BNESTR {
        &self.reg12_bnestr
    }
    ///0x340 - RISAF region 13 configuration register
    #[inline(always)]
    pub const fn reg13_cfgr(&self) -> &REG13_CFGR {
        &self.reg13_cfgr
    }
    ///0x344 - RISAF region 13 start-address register
    #[inline(always)]
    pub const fn reg13_startr(&self) -> &REG13_STARTR {
        &self.reg13_startr
    }
    ///0x348 - RISAF region 13 end-address register
    #[inline(always)]
    pub const fn reg13_endr(&self) -> &REG13_ENDR {
        &self.reg13_endr
    }
    ///0x34c - RISAF region 13 CID configuration register
    #[inline(always)]
    pub const fn reg13_cidcfgr(&self) -> &REG13_CIDCFGR {
        &self.reg13_cidcfgr
    }
    ///0x350 - RISAF region 13 subregion A configuration register
    #[inline(always)]
    pub const fn reg13_acfgr(&self) -> &REG13_ACFGR {
        &self.reg13_acfgr
    }
    ///0x354 - RISAF region 13 subregion A start-address register
    #[inline(always)]
    pub const fn reg13_astartr(&self) -> &REG13_ASTARTR {
        &self.reg13_astartr
    }
    ///0x358 - RISAF region 13 subregion A end-address register
    #[inline(always)]
    pub const fn reg13_aendr(&self) -> &REG13_AENDR {
        &self.reg13_aendr
    }
    ///0x35c - RISAF region 13 subregion A nested mode register
    #[inline(always)]
    pub const fn reg13_anestr(&self) -> &REG13_ANESTR {
        &self.reg13_anestr
    }
    ///0x360 - RISAF region 13 subregion B configuration register
    #[inline(always)]
    pub const fn reg13_bcfgr(&self) -> &REG13_BCFGR {
        &self.reg13_bcfgr
    }
    ///0x364 - RISAF region 13 subregion B start-address register
    #[inline(always)]
    pub const fn reg13_bstartr(&self) -> &REG13_BSTARTR {
        &self.reg13_bstartr
    }
    ///0x368 - RISAF region 13 subregion B end-address register
    #[inline(always)]
    pub const fn reg13_bendr(&self) -> &REG13_BENDR {
        &self.reg13_bendr
    }
    ///0x36c - RISAF region 13 subregion B nested mode register
    #[inline(always)]
    pub const fn reg13_bnestr(&self) -> &REG13_BNESTR {
        &self.reg13_bnestr
    }
    ///0x380 - RISAF region 14 configuration register
    #[inline(always)]
    pub const fn reg14_cfgr(&self) -> &REG14_CFGR {
        &self.reg14_cfgr
    }
    ///0x384 - RISAF region 14 start-address register
    #[inline(always)]
    pub const fn reg14_startr(&self) -> &REG14_STARTR {
        &self.reg14_startr
    }
    ///0x388 - RISAF region 14 end-address register
    #[inline(always)]
    pub const fn reg14_endr(&self) -> &REG14_ENDR {
        &self.reg14_endr
    }
    ///0x38c - RISAF region 14 CID configuration register
    #[inline(always)]
    pub const fn reg14_cidcfgr(&self) -> &REG14_CIDCFGR {
        &self.reg14_cidcfgr
    }
    ///0x390 - RISAF region 14 subregion A configuration register
    #[inline(always)]
    pub const fn reg14_acfgr(&self) -> &REG14_ACFGR {
        &self.reg14_acfgr
    }
    ///0x394 - RISAF region 14 subregion A start-address register
    #[inline(always)]
    pub const fn reg14_astartr(&self) -> &REG14_ASTARTR {
        &self.reg14_astartr
    }
    ///0x398 - RISAF region 14 subregion A end-address register
    #[inline(always)]
    pub const fn reg14_aendr(&self) -> &REG14_AENDR {
        &self.reg14_aendr
    }
    ///0x39c - RISAF region 14 subregion A nested mode register
    #[inline(always)]
    pub const fn reg14_anestr(&self) -> &REG14_ANESTR {
        &self.reg14_anestr
    }
    ///0x3a0 - RISAF region 14 subregion B configuration register
    #[inline(always)]
    pub const fn reg14_bcfgr(&self) -> &REG14_BCFGR {
        &self.reg14_bcfgr
    }
    ///0x3a4 - RISAF region 14 subregion B start-address register
    #[inline(always)]
    pub const fn reg14_bstartr(&self) -> &REG14_BSTARTR {
        &self.reg14_bstartr
    }
    ///0x3a8 - RISAF region 14 subregion B end-address register
    #[inline(always)]
    pub const fn reg14_bendr(&self) -> &REG14_BENDR {
        &self.reg14_bendr
    }
    ///0x3ac - RISAF region 14 subregion B nested mode register
    #[inline(always)]
    pub const fn reg14_bnestr(&self) -> &REG14_BNESTR {
        &self.reg14_bnestr
    }
    ///0x3c0 - RISAF region 15 configuration register
    #[inline(always)]
    pub const fn reg15_cfgr(&self) -> &REG15_CFGR {
        &self.reg15_cfgr
    }
    ///0x3c4 - RISAF region 15 start-address register
    #[inline(always)]
    pub const fn reg15_startr(&self) -> &REG15_STARTR {
        &self.reg15_startr
    }
    ///0x3c8 - RISAF region 15 end-address register
    #[inline(always)]
    pub const fn reg15_endr(&self) -> &REG15_ENDR {
        &self.reg15_endr
    }
    ///0x3cc - RISAF region 15 CID configuration register
    #[inline(always)]
    pub const fn reg15_cidcfgr(&self) -> &REG15_CIDCFGR {
        &self.reg15_cidcfgr
    }
    ///0x3d0 - RISAF region 15 subregion A configuration register
    #[inline(always)]
    pub const fn reg15_acfgr(&self) -> &REG15_ACFGR {
        &self.reg15_acfgr
    }
    ///0x3d4 - RISAF region 15 subregion A start-address register
    #[inline(always)]
    pub const fn reg15_astartr(&self) -> &REG15_ASTARTR {
        &self.reg15_astartr
    }
    ///0x3d8 - RISAF region 15 subregion A end-address register
    #[inline(always)]
    pub const fn reg15_aendr(&self) -> &REG15_AENDR {
        &self.reg15_aendr
    }
    ///0x3dc - RISAF region 15 subregion A nested mode register
    #[inline(always)]
    pub const fn reg15_anestr(&self) -> &REG15_ANESTR {
        &self.reg15_anestr
    }
    ///0x3e0 - RISAF region 15 subregion B configuration register
    #[inline(always)]
    pub const fn reg15_bcfgr(&self) -> &REG15_BCFGR {
        &self.reg15_bcfgr
    }
    ///0x3e4 - RISAF region 15 subregion B start-address register
    #[inline(always)]
    pub const fn reg15_bstartr(&self) -> &REG15_BSTARTR {
        &self.reg15_bstartr
    }
    ///0x3e8 - RISAF region 15 subregion B end-address register
    #[inline(always)]
    pub const fn reg15_bendr(&self) -> &REG15_BENDR {
        &self.reg15_bendr
    }
    ///0x3ec - RISAF region 15 subregion B nested mode register
    #[inline(always)]
    pub const fn reg15_bnestr(&self) -> &REG15_BNESTR {
        &self.reg15_bnestr
    }
}
/**CR (rw) register accessor: RISAF configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///RISAF configuration register
pub mod cr;
/**IASR (r) register accessor: RISAF illegal access status register

You can [`read`](crate::Reg::read) this register and get [`iasr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:IASR)

For information about available fields see [`mod@iasr`] module*/
pub type IASR = crate::Reg<iasr::IASRrs>;
///RISAF illegal access status register
pub mod iasr;
/**IACR (w) register accessor: RISAF illegal access clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iacr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:IACR)

For information about available fields see [`mod@iacr`] module*/
pub type IACR = crate::Reg<iacr::IACRrs>;
///RISAF illegal access clear register
pub mod iacr;
/**IAESR (r) register accessor: RISAF illegal access error status register

You can [`read`](crate::Reg::read) this register and get [`iaesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:IAESR)

For information about available fields see [`mod@iaesr`] module*/
pub type IAESR = crate::Reg<iaesr::IAESRrs>;
///RISAF illegal access error status register
pub mod iaesr;
/**IADDR (r) register accessor: RISAF illegal address register

You can [`read`](crate::Reg::read) this register and get [`iaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:IADDR)

For information about available fields see [`mod@iaddr`] module*/
pub type IADDR = crate::Reg<iaddr::IADDRrs>;
///RISAF illegal address register
pub mod iaddr;
/**REG1_CFGR (rw) register accessor: RISAF region 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg1_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_CFGR)

For information about available fields see [`mod@reg1_cfgr`] module*/
pub type REG1_CFGR = crate::Reg<reg1_cfgr::REG1_CFGRrs>;
///RISAF region 1 configuration register
pub mod reg1_cfgr;
/**REG1_STARTR (rw) register accessor: RISAF region 1 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_STARTR)

For information about available fields see [`mod@reg1_startr`] module*/
pub type REG1_STARTR = crate::Reg<reg1_startr::REG1_STARTRrs>;
///RISAF region 1 start-address register
pub mod reg1_startr;
/**REG1_ENDR (rw) register accessor: RISAF region 1 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_ENDR)

For information about available fields see [`mod@reg1_endr`] module*/
pub type REG1_ENDR = crate::Reg<reg1_endr::REG1_ENDRrs>;
///RISAF region 1 end-address register
pub mod reg1_endr;
/**REG1_CIDCFGR (rw) register accessor: RISAF region 1 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg1_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_CIDCFGR)

For information about available fields see [`mod@reg1_cidcfgr`] module*/
pub type REG1_CIDCFGR = crate::Reg<reg1_cidcfgr::REG1_CIDCFGRrs>;
///RISAF region 1 CID configuration register
pub mod reg1_cidcfgr;
/**REG2_CFGR (rw) register accessor: RISAF region 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg2_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_CFGR)

For information about available fields see [`mod@reg2_cfgr`] module*/
pub type REG2_CFGR = crate::Reg<reg2_cfgr::REG2_CFGRrs>;
///RISAF region 2 configuration register
pub mod reg2_cfgr;
/**REG2_STARTR (rw) register accessor: RISAF region 2 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg2_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_STARTR)

For information about available fields see [`mod@reg2_startr`] module*/
pub type REG2_STARTR = crate::Reg<reg2_startr::REG2_STARTRrs>;
///RISAF region 2 start-address register
pub mod reg2_startr;
/**REG2_ENDR (rw) register accessor: RISAF region 2 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg2_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_ENDR)

For information about available fields see [`mod@reg2_endr`] module*/
pub type REG2_ENDR = crate::Reg<reg2_endr::REG2_ENDRrs>;
///RISAF region 2 end-address register
pub mod reg2_endr;
/**REG2_CIDCFGR (rw) register accessor: RISAF region 2 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg2_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_CIDCFGR)

For information about available fields see [`mod@reg2_cidcfgr`] module*/
pub type REG2_CIDCFGR = crate::Reg<reg2_cidcfgr::REG2_CIDCFGRrs>;
///RISAF region 2 CID configuration register
pub mod reg2_cidcfgr;
/**REG3_CFGR (rw) register accessor: RISAF region 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg3_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_CFGR)

For information about available fields see [`mod@reg3_cfgr`] module*/
pub type REG3_CFGR = crate::Reg<reg3_cfgr::REG3_CFGRrs>;
///RISAF region 3 configuration register
pub mod reg3_cfgr;
/**REG3_STARTR (rw) register accessor: RISAF region 3 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg3_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_STARTR)

For information about available fields see [`mod@reg3_startr`] module*/
pub type REG3_STARTR = crate::Reg<reg3_startr::REG3_STARTRrs>;
///RISAF region 3 start-address register
pub mod reg3_startr;
/**REG3_ENDR (rw) register accessor: RISAF region 3 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg3_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_ENDR)

For information about available fields see [`mod@reg3_endr`] module*/
pub type REG3_ENDR = crate::Reg<reg3_endr::REG3_ENDRrs>;
///RISAF region 3 end-address register
pub mod reg3_endr;
/**REG3_CIDCFGR (rw) register accessor: RISAF region 3 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg3_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_CIDCFGR)

For information about available fields see [`mod@reg3_cidcfgr`] module*/
pub type REG3_CIDCFGR = crate::Reg<reg3_cidcfgr::REG3_CIDCFGRrs>;
///RISAF region 3 CID configuration register
pub mod reg3_cidcfgr;
/**REG4_CFGR (rw) register accessor: RISAF region 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg4_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_CFGR)

For information about available fields see [`mod@reg4_cfgr`] module*/
pub type REG4_CFGR = crate::Reg<reg4_cfgr::REG4_CFGRrs>;
///RISAF region 4 configuration register
pub mod reg4_cfgr;
/**REG4_STARTR (rw) register accessor: RISAF region 4 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_STARTR)

For information about available fields see [`mod@reg4_startr`] module*/
pub type REG4_STARTR = crate::Reg<reg4_startr::REG4_STARTRrs>;
///RISAF region 4 start-address register
pub mod reg4_startr;
/**REG4_ENDR (rw) register accessor: RISAF region 4 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_ENDR)

For information about available fields see [`mod@reg4_endr`] module*/
pub type REG4_ENDR = crate::Reg<reg4_endr::REG4_ENDRrs>;
///RISAF region 4 end-address register
pub mod reg4_endr;
/**REG4_CIDCFGR (rw) register accessor: RISAF region 4 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg4_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_CIDCFGR)

For information about available fields see [`mod@reg4_cidcfgr`] module*/
pub type REG4_CIDCFGR = crate::Reg<reg4_cidcfgr::REG4_CIDCFGRrs>;
///RISAF region 4 CID configuration register
pub mod reg4_cidcfgr;
/**REG5_CFGR (rw) register accessor: RISAF region 5 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg5_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_CFGR)

For information about available fields see [`mod@reg5_cfgr`] module*/
pub type REG5_CFGR = crate::Reg<reg5_cfgr::REG5_CFGRrs>;
///RISAF region 5 configuration register
pub mod reg5_cfgr;
/**REG5_STARTR (rw) register accessor: RISAF region 5 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_STARTR)

For information about available fields see [`mod@reg5_startr`] module*/
pub type REG5_STARTR = crate::Reg<reg5_startr::REG5_STARTRrs>;
///RISAF region 5 start-address register
pub mod reg5_startr;
/**REG5_ENDR (rw) register accessor: RISAF region 5 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_ENDR)

For information about available fields see [`mod@reg5_endr`] module*/
pub type REG5_ENDR = crate::Reg<reg5_endr::REG5_ENDRrs>;
///RISAF region 5 end-address register
pub mod reg5_endr;
/**REG5_CIDCFGR (rw) register accessor: RISAF region 5 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg5_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_CIDCFGR)

For information about available fields see [`mod@reg5_cidcfgr`] module*/
pub type REG5_CIDCFGR = crate::Reg<reg5_cidcfgr::REG5_CIDCFGRrs>;
///RISAF region 5 CID configuration register
pub mod reg5_cidcfgr;
/**REG6_CFGR (rw) register accessor: RISAF region 6 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg6_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_CFGR)

For information about available fields see [`mod@reg6_cfgr`] module*/
pub type REG6_CFGR = crate::Reg<reg6_cfgr::REG6_CFGRrs>;
///RISAF region 6 configuration register
pub mod reg6_cfgr;
/**REG6_STARTR (rw) register accessor: RISAF region 6 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_STARTR)

For information about available fields see [`mod@reg6_startr`] module*/
pub type REG6_STARTR = crate::Reg<reg6_startr::REG6_STARTRrs>;
///RISAF region 6 start-address register
pub mod reg6_startr;
/**REG6_ENDR (rw) register accessor: RISAF region 6 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_ENDR)

For information about available fields see [`mod@reg6_endr`] module*/
pub type REG6_ENDR = crate::Reg<reg6_endr::REG6_ENDRrs>;
///RISAF region 6 end-address register
pub mod reg6_endr;
/**REG6_CIDCFGR (rw) register accessor: RISAF region 6 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg6_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_CIDCFGR)

For information about available fields see [`mod@reg6_cidcfgr`] module*/
pub type REG6_CIDCFGR = crate::Reg<reg6_cidcfgr::REG6_CIDCFGRrs>;
///RISAF region 6 CID configuration register
pub mod reg6_cidcfgr;
/**REG7_CFGR (rw) register accessor: RISAF region 7 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg7_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_CFGR)

For information about available fields see [`mod@reg7_cfgr`] module*/
pub type REG7_CFGR = crate::Reg<reg7_cfgr::REG7_CFGRrs>;
///RISAF region 7 configuration register
pub mod reg7_cfgr;
/**REG7_STARTR (rw) register accessor: RISAF region 7 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg7_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_STARTR)

For information about available fields see [`mod@reg7_startr`] module*/
pub type REG7_STARTR = crate::Reg<reg7_startr::REG7_STARTRrs>;
///RISAF region 7 start-address register
pub mod reg7_startr;
/**REG7_ENDR (rw) register accessor: RISAF region 7 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg7_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_ENDR)

For information about available fields see [`mod@reg7_endr`] module*/
pub type REG7_ENDR = crate::Reg<reg7_endr::REG7_ENDRrs>;
///RISAF region 7 end-address register
pub mod reg7_endr;
/**REG7_CIDCFGR (rw) register accessor: RISAF region 7 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg7_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_CIDCFGR)

For information about available fields see [`mod@reg7_cidcfgr`] module*/
pub type REG7_CIDCFGR = crate::Reg<reg7_cidcfgr::REG7_CIDCFGRrs>;
///RISAF region 7 CID configuration register
pub mod reg7_cidcfgr;
/**REG8_CFGR (rw) register accessor: RISAF region 8 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg8_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_CFGR)

For information about available fields see [`mod@reg8_cfgr`] module*/
pub type REG8_CFGR = crate::Reg<reg8_cfgr::REG8_CFGRrs>;
///RISAF region 8 configuration register
pub mod reg8_cfgr;
/**REG8_STARTR (rw) register accessor: RISAF region 8 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_STARTR)

For information about available fields see [`mod@reg8_startr`] module*/
pub type REG8_STARTR = crate::Reg<reg8_startr::REG8_STARTRrs>;
///RISAF region 8 start-address register
pub mod reg8_startr;
/**REG8_ENDR (rw) register accessor: RISAF region 8 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_ENDR)

For information about available fields see [`mod@reg8_endr`] module*/
pub type REG8_ENDR = crate::Reg<reg8_endr::REG8_ENDRrs>;
///RISAF region 8 end-address register
pub mod reg8_endr;
/**REG8_CIDCFGR (rw) register accessor: RISAF region 8 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg8_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_CIDCFGR)

For information about available fields see [`mod@reg8_cidcfgr`] module*/
pub type REG8_CIDCFGR = crate::Reg<reg8_cidcfgr::REG8_CIDCFGRrs>;
///RISAF region 8 CID configuration register
pub mod reg8_cidcfgr;
/**REG9_CFGR (rw) register accessor: RISAF region 9 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg9_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_CFGR)

For information about available fields see [`mod@reg9_cfgr`] module*/
pub type REG9_CFGR = crate::Reg<reg9_cfgr::REG9_CFGRrs>;
///RISAF region 9 configuration register
pub mod reg9_cfgr;
/**REG9_STARTR (rw) register accessor: RISAF region 9 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_STARTR)

For information about available fields see [`mod@reg9_startr`] module*/
pub type REG9_STARTR = crate::Reg<reg9_startr::REG9_STARTRrs>;
///RISAF region 9 start-address register
pub mod reg9_startr;
/**REG9_ENDR (rw) register accessor: RISAF region 9 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_ENDR)

For information about available fields see [`mod@reg9_endr`] module*/
pub type REG9_ENDR = crate::Reg<reg9_endr::REG9_ENDRrs>;
///RISAF region 9 end-address register
pub mod reg9_endr;
/**REG9_CIDCFGR (rw) register accessor: RISAF region 9 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg9_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_CIDCFGR)

For information about available fields see [`mod@reg9_cidcfgr`] module*/
pub type REG9_CIDCFGR = crate::Reg<reg9_cidcfgr::REG9_CIDCFGRrs>;
///RISAF region 9 CID configuration register
pub mod reg9_cidcfgr;
/**REG10_CFGR (rw) register accessor: RISAF region 10 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg10_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_CFGR)

For information about available fields see [`mod@reg10_cfgr`] module*/
pub type REG10_CFGR = crate::Reg<reg10_cfgr::REG10_CFGRrs>;
///RISAF region 10 configuration register
pub mod reg10_cfgr;
/**REG10_STARTR (rw) register accessor: RISAF region 10 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_STARTR)

For information about available fields see [`mod@reg10_startr`] module*/
pub type REG10_STARTR = crate::Reg<reg10_startr::REG10_STARTRrs>;
///RISAF region 10 start-address register
pub mod reg10_startr;
/**REG10_ENDR (rw) register accessor: RISAF region 10 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_ENDR)

For information about available fields see [`mod@reg10_endr`] module*/
pub type REG10_ENDR = crate::Reg<reg10_endr::REG10_ENDRrs>;
///RISAF region 10 end-address register
pub mod reg10_endr;
/**REG10_CIDCFGR (rw) register accessor: RISAF region 10 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg10_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_CIDCFGR)

For information about available fields see [`mod@reg10_cidcfgr`] module*/
pub type REG10_CIDCFGR = crate::Reg<reg10_cidcfgr::REG10_CIDCFGRrs>;
///RISAF region 10 CID configuration register
pub mod reg10_cidcfgr;
/**REG11_CFGR (rw) register accessor: RISAF region 11 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg11_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_CFGR)

For information about available fields see [`mod@reg11_cfgr`] module*/
pub type REG11_CFGR = crate::Reg<reg11_cfgr::REG11_CFGRrs>;
///RISAF region 11 configuration register
pub mod reg11_cfgr;
/**REG11_STARTR (rw) register accessor: RISAF region 11 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_STARTR)

For information about available fields see [`mod@reg11_startr`] module*/
pub type REG11_STARTR = crate::Reg<reg11_startr::REG11_STARTRrs>;
///RISAF region 11 start-address register
pub mod reg11_startr;
/**REG11_ENDR (rw) register accessor: RISAF region 11 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_ENDR)

For information about available fields see [`mod@reg11_endr`] module*/
pub type REG11_ENDR = crate::Reg<reg11_endr::REG11_ENDRrs>;
///RISAF region 11 end-address register
pub mod reg11_endr;
/**REG11_CIDCFGR (rw) register accessor: RISAF region 11 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg11_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_CIDCFGR)

For information about available fields see [`mod@reg11_cidcfgr`] module*/
pub type REG11_CIDCFGR = crate::Reg<reg11_cidcfgr::REG11_CIDCFGRrs>;
///RISAF region 11 CID configuration register
pub mod reg11_cidcfgr;
/**REG12_CFGR (rw) register accessor: RISAF region 12 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg12_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_CFGR)

For information about available fields see [`mod@reg12_cfgr`] module*/
pub type REG12_CFGR = crate::Reg<reg12_cfgr::REG12_CFGRrs>;
///RISAF region 12 configuration register
pub mod reg12_cfgr;
/**REG12_STARTR (rw) register accessor: RISAF region 12 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_STARTR)

For information about available fields see [`mod@reg12_startr`] module*/
pub type REG12_STARTR = crate::Reg<reg12_startr::REG12_STARTRrs>;
///RISAF region 12 start-address register
pub mod reg12_startr;
/**REG12_ENDR (rw) register accessor: RISAF region 12 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_ENDR)

For information about available fields see [`mod@reg12_endr`] module*/
pub type REG12_ENDR = crate::Reg<reg12_endr::REG12_ENDRrs>;
///RISAF region 12 end-address register
pub mod reg12_endr;
/**REG12_CIDCFGR (rw) register accessor: RISAF region 12 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg12_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_CIDCFGR)

For information about available fields see [`mod@reg12_cidcfgr`] module*/
pub type REG12_CIDCFGR = crate::Reg<reg12_cidcfgr::REG12_CIDCFGRrs>;
///RISAF region 12 CID configuration register
pub mod reg12_cidcfgr;
/**REG13_CFGR (rw) register accessor: RISAF region 13 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg13_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_CFGR)

For information about available fields see [`mod@reg13_cfgr`] module*/
pub type REG13_CFGR = crate::Reg<reg13_cfgr::REG13_CFGRrs>;
///RISAF region 13 configuration register
pub mod reg13_cfgr;
/**REG13_STARTR (rw) register accessor: RISAF region 13 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg13_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_STARTR)

For information about available fields see [`mod@reg13_startr`] module*/
pub type REG13_STARTR = crate::Reg<reg13_startr::REG13_STARTRrs>;
///RISAF region 13 start-address register
pub mod reg13_startr;
/**REG13_ENDR (rw) register accessor: RISAF region 13 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg13_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_ENDR)

For information about available fields see [`mod@reg13_endr`] module*/
pub type REG13_ENDR = crate::Reg<reg13_endr::REG13_ENDRrs>;
///RISAF region 13 end-address register
pub mod reg13_endr;
/**REG13_CIDCFGR (rw) register accessor: RISAF region 13 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg13_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_CIDCFGR)

For information about available fields see [`mod@reg13_cidcfgr`] module*/
pub type REG13_CIDCFGR = crate::Reg<reg13_cidcfgr::REG13_CIDCFGRrs>;
///RISAF region 13 CID configuration register
pub mod reg13_cidcfgr;
/**REG14_CFGR (rw) register accessor: RISAF region 14 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg14_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_CFGR)

For information about available fields see [`mod@reg14_cfgr`] module*/
pub type REG14_CFGR = crate::Reg<reg14_cfgr::REG14_CFGRrs>;
///RISAF region 14 configuration register
pub mod reg14_cfgr;
/**REG14_STARTR (rw) register accessor: RISAF region 14 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_STARTR)

For information about available fields see [`mod@reg14_startr`] module*/
pub type REG14_STARTR = crate::Reg<reg14_startr::REG14_STARTRrs>;
///RISAF region 14 start-address register
pub mod reg14_startr;
/**REG14_ENDR (rw) register accessor: RISAF region 14 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_ENDR)

For information about available fields see [`mod@reg14_endr`] module*/
pub type REG14_ENDR = crate::Reg<reg14_endr::REG14_ENDRrs>;
///RISAF region 14 end-address register
pub mod reg14_endr;
/**REG14_CIDCFGR (rw) register accessor: RISAF region 14 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg14_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_CIDCFGR)

For information about available fields see [`mod@reg14_cidcfgr`] module*/
pub type REG14_CIDCFGR = crate::Reg<reg14_cidcfgr::REG14_CIDCFGRrs>;
///RISAF region 14 CID configuration register
pub mod reg14_cidcfgr;
/**REG15_CFGR (rw) register accessor: RISAF region 15 configuration register

You can [`read`](crate::Reg::read) this register and get [`reg15_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_CFGR)

For information about available fields see [`mod@reg15_cfgr`] module*/
pub type REG15_CFGR = crate::Reg<reg15_cfgr::REG15_CFGRrs>;
///RISAF region 15 configuration register
pub mod reg15_cfgr;
/**REG15_STARTR (rw) register accessor: RISAF region 15 start-address register

You can [`read`](crate::Reg::read) this register and get [`reg15_startr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_startr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_STARTR)

For information about available fields see [`mod@reg15_startr`] module*/
pub type REG15_STARTR = crate::Reg<reg15_startr::REG15_STARTRrs>;
///RISAF region 15 start-address register
pub mod reg15_startr;
/**REG15_ENDR (rw) register accessor: RISAF region 15 end-address register

You can [`read`](crate::Reg::read) this register and get [`reg15_endr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_endr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_ENDR)

For information about available fields see [`mod@reg15_endr`] module*/
pub type REG15_ENDR = crate::Reg<reg15_endr::REG15_ENDRrs>;
///RISAF region 15 end-address register
pub mod reg15_endr;
/**REG15_CIDCFGR (rw) register accessor: RISAF region 15 CID configuration register

You can [`read`](crate::Reg::read) this register and get [`reg15_cidcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_cidcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_CIDCFGR)

For information about available fields see [`mod@reg15_cidcfgr`] module*/
pub type REG15_CIDCFGR = crate::Reg<reg15_cidcfgr::REG15_CIDCFGRrs>;
///RISAF region 15 CID configuration register
pub mod reg15_cidcfgr;
/**REG1_ACFGR (rw) register accessor: RISAF region 1 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg1_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_ACFGR)

For information about available fields see [`mod@reg1_acfgr`] module*/
pub type REG1_ACFGR = crate::Reg<reg1_acfgr::REG1_ACFGRrs>;
///RISAF region 1 subregion A configuration register
pub mod reg1_acfgr;
/**REG1_ASTARTR (rw) register accessor: RISAF region 1 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_ASTARTR)

For information about available fields see [`mod@reg1_astartr`] module*/
pub type REG1_ASTARTR = crate::Reg<reg1_astartr::REG1_ASTARTRrs>;
///RISAF region 1 subregion A start-address register
pub mod reg1_astartr;
/**REG1_AENDR (rw) register accessor: RISAF region 1 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_AENDR)

For information about available fields see [`mod@reg1_aendr`] module*/
pub type REG1_AENDR = crate::Reg<reg1_aendr::REG1_AENDRrs>;
///RISAF region 1 subregion A end-address register
pub mod reg1_aendr;
/**REG1_ANESTR (rw) register accessor: RISAF region 1 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg1_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_ANESTR)

For information about available fields see [`mod@reg1_anestr`] module*/
pub type REG1_ANESTR = crate::Reg<reg1_anestr::REG1_ANESTRrs>;
///RISAF region 1 subregion A nested mode register
pub mod reg1_anestr;
/**REG2_ACFGR (rw) register accessor: RISAF region 2 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg2_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_ACFGR)

For information about available fields see [`mod@reg2_acfgr`] module*/
pub type REG2_ACFGR = crate::Reg<reg2_acfgr::REG2_ACFGRrs>;
///RISAF region 2 subregion A configuration register
pub mod reg2_acfgr;
/**REG2_ASTARTR (rw) register accessor: RISAF region 2 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg2_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_ASTARTR)

For information about available fields see [`mod@reg2_astartr`] module*/
pub type REG2_ASTARTR = crate::Reg<reg2_astartr::REG2_ASTARTRrs>;
///RISAF region 2 subregion A start-address register
pub mod reg2_astartr;
/**REG2_AENDR (rw) register accessor: RISAF region 2 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg2_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_AENDR)

For information about available fields see [`mod@reg2_aendr`] module*/
pub type REG2_AENDR = crate::Reg<reg2_aendr::REG2_AENDRrs>;
///RISAF region 2 subregion A end-address register
pub mod reg2_aendr;
/**REG2_ANESTR (rw) register accessor: RISAF region 2 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg2_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_ANESTR)

For information about available fields see [`mod@reg2_anestr`] module*/
pub type REG2_ANESTR = crate::Reg<reg2_anestr::REG2_ANESTRrs>;
///RISAF region 2 subregion A nested mode register
pub mod reg2_anestr;
/**REG3_ACFGR (rw) register accessor: RISAF region 3 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg3_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_ACFGR)

For information about available fields see [`mod@reg3_acfgr`] module*/
pub type REG3_ACFGR = crate::Reg<reg3_acfgr::REG3_ACFGRrs>;
///RISAF region 3 subregion A configuration register
pub mod reg3_acfgr;
/**REG3_ASTARTR (rw) register accessor: RISAF region 3 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg3_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_ASTARTR)

For information about available fields see [`mod@reg3_astartr`] module*/
pub type REG3_ASTARTR = crate::Reg<reg3_astartr::REG3_ASTARTRrs>;
///RISAF region 3 subregion A start-address register
pub mod reg3_astartr;
/**REG3_AENDR (rw) register accessor: RISAF region 3 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg3_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_AENDR)

For information about available fields see [`mod@reg3_aendr`] module*/
pub type REG3_AENDR = crate::Reg<reg3_aendr::REG3_AENDRrs>;
///RISAF region 3 subregion A end-address register
pub mod reg3_aendr;
/**REG3_ANESTR (rw) register accessor: RISAF region 3 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg3_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_ANESTR)

For information about available fields see [`mod@reg3_anestr`] module*/
pub type REG3_ANESTR = crate::Reg<reg3_anestr::REG3_ANESTRrs>;
///RISAF region 3 subregion A nested mode register
pub mod reg3_anestr;
/**REG4_ACFGR (rw) register accessor: RISAF region 4 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg4_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_ACFGR)

For information about available fields see [`mod@reg4_acfgr`] module*/
pub type REG4_ACFGR = crate::Reg<reg4_acfgr::REG4_ACFGRrs>;
///RISAF region 4 subregion A configuration register
pub mod reg4_acfgr;
/**REG4_ASTARTR (rw) register accessor: RISAF region 4 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_ASTARTR)

For information about available fields see [`mod@reg4_astartr`] module*/
pub type REG4_ASTARTR = crate::Reg<reg4_astartr::REG4_ASTARTRrs>;
///RISAF region 4 subregion A start-address register
pub mod reg4_astartr;
/**REG4_AENDR (rw) register accessor: RISAF region 4 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_AENDR)

For information about available fields see [`mod@reg4_aendr`] module*/
pub type REG4_AENDR = crate::Reg<reg4_aendr::REG4_AENDRrs>;
///RISAF region 4 subregion A end-address register
pub mod reg4_aendr;
/**REG4_ANESTR (rw) register accessor: RISAF region 4 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg4_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_ANESTR)

For information about available fields see [`mod@reg4_anestr`] module*/
pub type REG4_ANESTR = crate::Reg<reg4_anestr::REG4_ANESTRrs>;
///RISAF region 4 subregion A nested mode register
pub mod reg4_anestr;
/**REG5_ACFGR (rw) register accessor: RISAF region 5 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg5_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_ACFGR)

For information about available fields see [`mod@reg5_acfgr`] module*/
pub type REG5_ACFGR = crate::Reg<reg5_acfgr::REG5_ACFGRrs>;
///RISAF region 5 subregion A configuration register
pub mod reg5_acfgr;
/**REG5_ASTARTR (rw) register accessor: RISAF region 5 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_ASTARTR)

For information about available fields see [`mod@reg5_astartr`] module*/
pub type REG5_ASTARTR = crate::Reg<reg5_astartr::REG5_ASTARTRrs>;
///RISAF region 5 subregion A start-address register
pub mod reg5_astartr;
/**REG5_AENDR (rw) register accessor: RISAF region 5 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_AENDR)

For information about available fields see [`mod@reg5_aendr`] module*/
pub type REG5_AENDR = crate::Reg<reg5_aendr::REG5_AENDRrs>;
///RISAF region 5 subregion A end-address register
pub mod reg5_aendr;
/**REG5_ANESTR (rw) register accessor: RISAF region 5 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg5_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_ANESTR)

For information about available fields see [`mod@reg5_anestr`] module*/
pub type REG5_ANESTR = crate::Reg<reg5_anestr::REG5_ANESTRrs>;
///RISAF region 5 subregion A nested mode register
pub mod reg5_anestr;
/**REG6_ACFGR (rw) register accessor: RISAF region 6 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg6_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_ACFGR)

For information about available fields see [`mod@reg6_acfgr`] module*/
pub type REG6_ACFGR = crate::Reg<reg6_acfgr::REG6_ACFGRrs>;
///RISAF region 6 subregion A configuration register
pub mod reg6_acfgr;
/**REG6_ASTARTR (rw) register accessor: RISAF region 6 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_ASTARTR)

For information about available fields see [`mod@reg6_astartr`] module*/
pub type REG6_ASTARTR = crate::Reg<reg6_astartr::REG6_ASTARTRrs>;
///RISAF region 6 subregion A start-address register
pub mod reg6_astartr;
/**REG6_AENDR (rw) register accessor: RISAF region 6 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_AENDR)

For information about available fields see [`mod@reg6_aendr`] module*/
pub type REG6_AENDR = crate::Reg<reg6_aendr::REG6_AENDRrs>;
///RISAF region 6 subregion A end-address register
pub mod reg6_aendr;
/**REG6_ANESTR (rw) register accessor: RISAF region 6 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg6_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_ANESTR)

For information about available fields see [`mod@reg6_anestr`] module*/
pub type REG6_ANESTR = crate::Reg<reg6_anestr::REG6_ANESTRrs>;
///RISAF region 6 subregion A nested mode register
pub mod reg6_anestr;
/**REG7_ACFGR (rw) register accessor: RISAF region 7 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg7_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_ACFGR)

For information about available fields see [`mod@reg7_acfgr`] module*/
pub type REG7_ACFGR = crate::Reg<reg7_acfgr::REG7_ACFGRrs>;
///RISAF region 7 subregion A configuration register
pub mod reg7_acfgr;
/**REG7_ASTARTR (rw) register accessor: RISAF region 7 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg7_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_ASTARTR)

For information about available fields see [`mod@reg7_astartr`] module*/
pub type REG7_ASTARTR = crate::Reg<reg7_astartr::REG7_ASTARTRrs>;
///RISAF region 7 subregion A start-address register
pub mod reg7_astartr;
/**REG7_AENDR (rw) register accessor: RISAF region 7 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg7_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_AENDR)

For information about available fields see [`mod@reg7_aendr`] module*/
pub type REG7_AENDR = crate::Reg<reg7_aendr::REG7_AENDRrs>;
///RISAF region 7 subregion A end-address register
pub mod reg7_aendr;
/**REG7_ANESTR (rw) register accessor: RISAF region 7 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg7_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_ANESTR)

For information about available fields see [`mod@reg7_anestr`] module*/
pub type REG7_ANESTR = crate::Reg<reg7_anestr::REG7_ANESTRrs>;
///RISAF region 7 subregion A nested mode register
pub mod reg7_anestr;
/**REG8_ACFGR (rw) register accessor: RISAF region 8 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg8_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_ACFGR)

For information about available fields see [`mod@reg8_acfgr`] module*/
pub type REG8_ACFGR = crate::Reg<reg8_acfgr::REG8_ACFGRrs>;
///RISAF region 8 subregion A configuration register
pub mod reg8_acfgr;
/**REG8_ASTARTR (rw) register accessor: RISAF region 8 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_ASTARTR)

For information about available fields see [`mod@reg8_astartr`] module*/
pub type REG8_ASTARTR = crate::Reg<reg8_astartr::REG8_ASTARTRrs>;
///RISAF region 8 subregion A start-address register
pub mod reg8_astartr;
/**REG8_AENDR (rw) register accessor: RISAF region 8 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_AENDR)

For information about available fields see [`mod@reg8_aendr`] module*/
pub type REG8_AENDR = crate::Reg<reg8_aendr::REG8_AENDRrs>;
///RISAF region 8 subregion A end-address register
pub mod reg8_aendr;
/**REG8_ANESTR (rw) register accessor: RISAF region 8 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg8_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_ANESTR)

For information about available fields see [`mod@reg8_anestr`] module*/
pub type REG8_ANESTR = crate::Reg<reg8_anestr::REG8_ANESTRrs>;
///RISAF region 8 subregion A nested mode register
pub mod reg8_anestr;
/**REG9_ACFGR (rw) register accessor: RISAF region 9 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg9_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_ACFGR)

For information about available fields see [`mod@reg9_acfgr`] module*/
pub type REG9_ACFGR = crate::Reg<reg9_acfgr::REG9_ACFGRrs>;
///RISAF region 9 subregion A configuration register
pub mod reg9_acfgr;
/**REG9_ASTARTR (rw) register accessor: RISAF region 9 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_ASTARTR)

For information about available fields see [`mod@reg9_astartr`] module*/
pub type REG9_ASTARTR = crate::Reg<reg9_astartr::REG9_ASTARTRrs>;
///RISAF region 9 subregion A start-address register
pub mod reg9_astartr;
/**REG9_AENDR (rw) register accessor: RISAF region 9 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_AENDR)

For information about available fields see [`mod@reg9_aendr`] module*/
pub type REG9_AENDR = crate::Reg<reg9_aendr::REG9_AENDRrs>;
///RISAF region 9 subregion A end-address register
pub mod reg9_aendr;
/**REG9_ANESTR (rw) register accessor: RISAF region 9 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg9_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_ANESTR)

For information about available fields see [`mod@reg9_anestr`] module*/
pub type REG9_ANESTR = crate::Reg<reg9_anestr::REG9_ANESTRrs>;
///RISAF region 9 subregion A nested mode register
pub mod reg9_anestr;
/**REG10_ACFGR (rw) register accessor: RISAF region 10 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg10_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_ACFGR)

For information about available fields see [`mod@reg10_acfgr`] module*/
pub type REG10_ACFGR = crate::Reg<reg10_acfgr::REG10_ACFGRrs>;
///RISAF region 10 subregion A configuration register
pub mod reg10_acfgr;
/**REG10_ASTARTR (rw) register accessor: RISAF region 10 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_ASTARTR)

For information about available fields see [`mod@reg10_astartr`] module*/
pub type REG10_ASTARTR = crate::Reg<reg10_astartr::REG10_ASTARTRrs>;
///RISAF region 10 subregion A start-address register
pub mod reg10_astartr;
/**REG10_AENDR (rw) register accessor: RISAF region 10 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_AENDR)

For information about available fields see [`mod@reg10_aendr`] module*/
pub type REG10_AENDR = crate::Reg<reg10_aendr::REG10_AENDRrs>;
///RISAF region 10 subregion A end-address register
pub mod reg10_aendr;
/**REG10_ANESTR (rw) register accessor: RISAF region 10 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg10_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_ANESTR)

For information about available fields see [`mod@reg10_anestr`] module*/
pub type REG10_ANESTR = crate::Reg<reg10_anestr::REG10_ANESTRrs>;
///RISAF region 10 subregion A nested mode register
pub mod reg10_anestr;
/**REG11_ACFGR (rw) register accessor: RISAF region 11 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg11_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_ACFGR)

For information about available fields see [`mod@reg11_acfgr`] module*/
pub type REG11_ACFGR = crate::Reg<reg11_acfgr::REG11_ACFGRrs>;
///RISAF region 11 subregion A configuration register
pub mod reg11_acfgr;
/**REG11_ASTARTR (rw) register accessor: RISAF region 11 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_ASTARTR)

For information about available fields see [`mod@reg11_astartr`] module*/
pub type REG11_ASTARTR = crate::Reg<reg11_astartr::REG11_ASTARTRrs>;
///RISAF region 11 subregion A start-address register
pub mod reg11_astartr;
/**REG11_AENDR (rw) register accessor: RISAF region 11 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_AENDR)

For information about available fields see [`mod@reg11_aendr`] module*/
pub type REG11_AENDR = crate::Reg<reg11_aendr::REG11_AENDRrs>;
///RISAF region 11 subregion A end-address register
pub mod reg11_aendr;
/**REG11_ANESTR (rw) register accessor: RISAF region 11 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg11_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_ANESTR)

For information about available fields see [`mod@reg11_anestr`] module*/
pub type REG11_ANESTR = crate::Reg<reg11_anestr::REG11_ANESTRrs>;
///RISAF region 11 subregion A nested mode register
pub mod reg11_anestr;
/**REG12_ACFGR (rw) register accessor: RISAF region 12 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg12_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_ACFGR)

For information about available fields see [`mod@reg12_acfgr`] module*/
pub type REG12_ACFGR = crate::Reg<reg12_acfgr::REG12_ACFGRrs>;
///RISAF region 12 subregion A configuration register
pub mod reg12_acfgr;
/**REG12_ASTARTR (rw) register accessor: RISAF region 12 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_ASTARTR)

For information about available fields see [`mod@reg12_astartr`] module*/
pub type REG12_ASTARTR = crate::Reg<reg12_astartr::REG12_ASTARTRrs>;
///RISAF region 12 subregion A start-address register
pub mod reg12_astartr;
/**REG12_AENDR (rw) register accessor: RISAF region 12 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_AENDR)

For information about available fields see [`mod@reg12_aendr`] module*/
pub type REG12_AENDR = crate::Reg<reg12_aendr::REG12_AENDRrs>;
///RISAF region 12 subregion A end-address register
pub mod reg12_aendr;
/**REG12_ANESTR (rw) register accessor: RISAF region 12 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg12_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_ANESTR)

For information about available fields see [`mod@reg12_anestr`] module*/
pub type REG12_ANESTR = crate::Reg<reg12_anestr::REG12_ANESTRrs>;
///RISAF region 12 subregion A nested mode register
pub mod reg12_anestr;
/**REG13_ACFGR (rw) register accessor: RISAF region 13 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg13_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_ACFGR)

For information about available fields see [`mod@reg13_acfgr`] module*/
pub type REG13_ACFGR = crate::Reg<reg13_acfgr::REG13_ACFGRrs>;
///RISAF region 13 subregion A configuration register
pub mod reg13_acfgr;
/**REG13_ASTARTR (rw) register accessor: RISAF region 13 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg13_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_ASTARTR)

For information about available fields see [`mod@reg13_astartr`] module*/
pub type REG13_ASTARTR = crate::Reg<reg13_astartr::REG13_ASTARTRrs>;
///RISAF region 13 subregion A start-address register
pub mod reg13_astartr;
/**REG13_AENDR (rw) register accessor: RISAF region 13 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg13_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_AENDR)

For information about available fields see [`mod@reg13_aendr`] module*/
pub type REG13_AENDR = crate::Reg<reg13_aendr::REG13_AENDRrs>;
///RISAF region 13 subregion A end-address register
pub mod reg13_aendr;
/**REG13_ANESTR (rw) register accessor: RISAF region 13 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg13_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_ANESTR)

For information about available fields see [`mod@reg13_anestr`] module*/
pub type REG13_ANESTR = crate::Reg<reg13_anestr::REG13_ANESTRrs>;
///RISAF region 13 subregion A nested mode register
pub mod reg13_anestr;
/**REG14_ACFGR (rw) register accessor: RISAF region 14 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg14_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_ACFGR)

For information about available fields see [`mod@reg14_acfgr`] module*/
pub type REG14_ACFGR = crate::Reg<reg14_acfgr::REG14_ACFGRrs>;
///RISAF region 14 subregion A configuration register
pub mod reg14_acfgr;
/**REG14_ASTARTR (rw) register accessor: RISAF region 14 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_ASTARTR)

For information about available fields see [`mod@reg14_astartr`] module*/
pub type REG14_ASTARTR = crate::Reg<reg14_astartr::REG14_ASTARTRrs>;
///RISAF region 14 subregion A start-address register
pub mod reg14_astartr;
/**REG14_AENDR (rw) register accessor: RISAF region 14 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_AENDR)

For information about available fields see [`mod@reg14_aendr`] module*/
pub type REG14_AENDR = crate::Reg<reg14_aendr::REG14_AENDRrs>;
///RISAF region 14 subregion A end-address register
pub mod reg14_aendr;
/**REG14_ANESTR (rw) register accessor: RISAF region 14 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg14_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_ANESTR)

For information about available fields see [`mod@reg14_anestr`] module*/
pub type REG14_ANESTR = crate::Reg<reg14_anestr::REG14_ANESTRrs>;
///RISAF region 14 subregion A nested mode register
pub mod reg14_anestr;
/**REG15_ACFGR (rw) register accessor: RISAF region 15 subregion A configuration register

You can [`read`](crate::Reg::read) this register and get [`reg15_acfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_acfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_ACFGR)

For information about available fields see [`mod@reg15_acfgr`] module*/
pub type REG15_ACFGR = crate::Reg<reg15_acfgr::REG15_ACFGRrs>;
///RISAF region 15 subregion A configuration register
pub mod reg15_acfgr;
/**REG15_ASTARTR (rw) register accessor: RISAF region 15 subregion A start-address register

You can [`read`](crate::Reg::read) this register and get [`reg15_astartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_astartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_ASTARTR)

For information about available fields see [`mod@reg15_astartr`] module*/
pub type REG15_ASTARTR = crate::Reg<reg15_astartr::REG15_ASTARTRrs>;
///RISAF region 15 subregion A start-address register
pub mod reg15_astartr;
/**REG15_AENDR (rw) register accessor: RISAF region 15 subregion A end-address register

You can [`read`](crate::Reg::read) this register and get [`reg15_aendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_aendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_AENDR)

For information about available fields see [`mod@reg15_aendr`] module*/
pub type REG15_AENDR = crate::Reg<reg15_aendr::REG15_AENDRrs>;
///RISAF region 15 subregion A end-address register
pub mod reg15_aendr;
/**REG15_ANESTR (rw) register accessor: RISAF region 15 subregion A nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg15_anestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_anestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_ANESTR)

For information about available fields see [`mod@reg15_anestr`] module*/
pub type REG15_ANESTR = crate::Reg<reg15_anestr::REG15_ANESTRrs>;
///RISAF region 15 subregion A nested mode register
pub mod reg15_anestr;
/**REG1_BCFGR (rw) register accessor: RISAF region 1 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg1_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_BCFGR)

For information about available fields see [`mod@reg1_bcfgr`] module*/
pub type REG1_BCFGR = crate::Reg<reg1_bcfgr::REG1_BCFGRrs>;
///RISAF region 1 subregion B configuration register
pub mod reg1_bcfgr;
/**REG1_BSTARTR (rw) register accessor: RISAF region 1 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_BSTARTR)

For information about available fields see [`mod@reg1_bstartr`] module*/
pub type REG1_BSTARTR = crate::Reg<reg1_bstartr::REG1_BSTARTRrs>;
///RISAF region 1 subregion B start-address register
pub mod reg1_bstartr;
/**REG1_BENDR (rw) register accessor: RISAF region 1 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg1_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_BENDR)

For information about available fields see [`mod@reg1_bendr`] module*/
pub type REG1_BENDR = crate::Reg<reg1_bendr::REG1_BENDRrs>;
///RISAF region 1 subregion B end-address register
pub mod reg1_bendr;
/**REG1_BNESTR (rw) register accessor: RISAF region 1 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg1_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG1_BNESTR)

For information about available fields see [`mod@reg1_bnestr`] module*/
pub type REG1_BNESTR = crate::Reg<reg1_bnestr::REG1_BNESTRrs>;
///RISAF region 1 subregion B nested mode register
pub mod reg1_bnestr;
/**REG2_BCFGR (rw) register accessor: RISAF region 2 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg2_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_BCFGR)

For information about available fields see [`mod@reg2_bcfgr`] module*/
pub type REG2_BCFGR = crate::Reg<reg2_bcfgr::REG2_BCFGRrs>;
///RISAF region 2 subregion B configuration register
pub mod reg2_bcfgr;
/**REG2_BSTARTR (rw) register accessor: RISAF region 2 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg2_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_BSTARTR)

For information about available fields see [`mod@reg2_bstartr`] module*/
pub type REG2_BSTARTR = crate::Reg<reg2_bstartr::REG2_BSTARTRrs>;
///RISAF region 2 subregion B start-address register
pub mod reg2_bstartr;
/**REG2_BENDR (rw) register accessor: RISAF region 2 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg2_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_BENDR)

For information about available fields see [`mod@reg2_bendr`] module*/
pub type REG2_BENDR = crate::Reg<reg2_bendr::REG2_BENDRrs>;
///RISAF region 2 subregion B end-address register
pub mod reg2_bendr;
/**REG2_BNESTR (rw) register accessor: RISAF region 2 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg2_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG2_BNESTR)

For information about available fields see [`mod@reg2_bnestr`] module*/
pub type REG2_BNESTR = crate::Reg<reg2_bnestr::REG2_BNESTRrs>;
///RISAF region 2 subregion B nested mode register
pub mod reg2_bnestr;
/**REG3_BCFGR (rw) register accessor: RISAF region 3 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg3_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_BCFGR)

For information about available fields see [`mod@reg3_bcfgr`] module*/
pub type REG3_BCFGR = crate::Reg<reg3_bcfgr::REG3_BCFGRrs>;
///RISAF region 3 subregion B configuration register
pub mod reg3_bcfgr;
/**REG3_BSTARTR (rw) register accessor: RISAF region 3 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg3_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_BSTARTR)

For information about available fields see [`mod@reg3_bstartr`] module*/
pub type REG3_BSTARTR = crate::Reg<reg3_bstartr::REG3_BSTARTRrs>;
///RISAF region 3 subregion B start-address register
pub mod reg3_bstartr;
/**REG3_BENDR (rw) register accessor: RISAF region 3 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg3_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_BENDR)

For information about available fields see [`mod@reg3_bendr`] module*/
pub type REG3_BENDR = crate::Reg<reg3_bendr::REG3_BENDRrs>;
///RISAF region 3 subregion B end-address register
pub mod reg3_bendr;
/**REG3_BNESTR (rw) register accessor: RISAF region 3 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg3_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG3_BNESTR)

For information about available fields see [`mod@reg3_bnestr`] module*/
pub type REG3_BNESTR = crate::Reg<reg3_bnestr::REG3_BNESTRrs>;
///RISAF region 3 subregion B nested mode register
pub mod reg3_bnestr;
/**REG4_BCFGR (rw) register accessor: RISAF region 4 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg4_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_BCFGR)

For information about available fields see [`mod@reg4_bcfgr`] module*/
pub type REG4_BCFGR = crate::Reg<reg4_bcfgr::REG4_BCFGRrs>;
///RISAF region 4 subregion B configuration register
pub mod reg4_bcfgr;
/**REG4_BSTARTR (rw) register accessor: RISAF region 4 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_BSTARTR)

For information about available fields see [`mod@reg4_bstartr`] module*/
pub type REG4_BSTARTR = crate::Reg<reg4_bstartr::REG4_BSTARTRrs>;
///RISAF region 4 subregion B start-address register
pub mod reg4_bstartr;
/**REG4_BENDR (rw) register accessor: RISAF region 4 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg4_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_BENDR)

For information about available fields see [`mod@reg4_bendr`] module*/
pub type REG4_BENDR = crate::Reg<reg4_bendr::REG4_BENDRrs>;
///RISAF region 4 subregion B end-address register
pub mod reg4_bendr;
/**REG4_BNESTR (rw) register accessor: RISAF region 4 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg4_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg4_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG4_BNESTR)

For information about available fields see [`mod@reg4_bnestr`] module*/
pub type REG4_BNESTR = crate::Reg<reg4_bnestr::REG4_BNESTRrs>;
///RISAF region 4 subregion B nested mode register
pub mod reg4_bnestr;
/**REG5_BCFGR (rw) register accessor: RISAF region 5 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg5_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_BCFGR)

For information about available fields see [`mod@reg5_bcfgr`] module*/
pub type REG5_BCFGR = crate::Reg<reg5_bcfgr::REG5_BCFGRrs>;
///RISAF region 5 subregion B configuration register
pub mod reg5_bcfgr;
/**REG5_BSTARTR (rw) register accessor: RISAF region 5 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_BSTARTR)

For information about available fields see [`mod@reg5_bstartr`] module*/
pub type REG5_BSTARTR = crate::Reg<reg5_bstartr::REG5_BSTARTRrs>;
///RISAF region 5 subregion B start-address register
pub mod reg5_bstartr;
/**REG5_BENDR (rw) register accessor: RISAF region 5 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg5_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_BENDR)

For information about available fields see [`mod@reg5_bendr`] module*/
pub type REG5_BENDR = crate::Reg<reg5_bendr::REG5_BENDRrs>;
///RISAF region 5 subregion B end-address register
pub mod reg5_bendr;
/**REG5_BNESTR (rw) register accessor: RISAF region 5 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg5_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg5_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG5_BNESTR)

For information about available fields see [`mod@reg5_bnestr`] module*/
pub type REG5_BNESTR = crate::Reg<reg5_bnestr::REG5_BNESTRrs>;
///RISAF region 5 subregion B nested mode register
pub mod reg5_bnestr;
/**REG6_BCFGR (rw) register accessor: RISAF region 6 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg6_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_BCFGR)

For information about available fields see [`mod@reg6_bcfgr`] module*/
pub type REG6_BCFGR = crate::Reg<reg6_bcfgr::REG6_BCFGRrs>;
///RISAF region 6 subregion B configuration register
pub mod reg6_bcfgr;
/**REG6_BSTARTR (rw) register accessor: RISAF region 6 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_BSTARTR)

For information about available fields see [`mod@reg6_bstartr`] module*/
pub type REG6_BSTARTR = crate::Reg<reg6_bstartr::REG6_BSTARTRrs>;
///RISAF region 6 subregion B start-address register
pub mod reg6_bstartr;
/**REG6_BENDR (rw) register accessor: RISAF region 6 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg6_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_BENDR)

For information about available fields see [`mod@reg6_bendr`] module*/
pub type REG6_BENDR = crate::Reg<reg6_bendr::REG6_BENDRrs>;
///RISAF region 6 subregion B end-address register
pub mod reg6_bendr;
/**REG6_BNESTR (rw) register accessor: RISAF region 6 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg6_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg6_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG6_BNESTR)

For information about available fields see [`mod@reg6_bnestr`] module*/
pub type REG6_BNESTR = crate::Reg<reg6_bnestr::REG6_BNESTRrs>;
///RISAF region 6 subregion B nested mode register
pub mod reg6_bnestr;
/**REG7_BCFGR (rw) register accessor: RISAF region 7 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg7_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_BCFGR)

For information about available fields see [`mod@reg7_bcfgr`] module*/
pub type REG7_BCFGR = crate::Reg<reg7_bcfgr::REG7_BCFGRrs>;
///RISAF region 7 subregion B configuration register
pub mod reg7_bcfgr;
/**REG7_BSTARTR (rw) register accessor: RISAF region 7 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg7_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_BSTARTR)

For information about available fields see [`mod@reg7_bstartr`] module*/
pub type REG7_BSTARTR = crate::Reg<reg7_bstartr::REG7_BSTARTRrs>;
///RISAF region 7 subregion B start-address register
pub mod reg7_bstartr;
/**REG7_BENDR (rw) register accessor: RISAF region 7 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg7_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_BENDR)

For information about available fields see [`mod@reg7_bendr`] module*/
pub type REG7_BENDR = crate::Reg<reg7_bendr::REG7_BENDRrs>;
///RISAF region 7 subregion B end-address register
pub mod reg7_bendr;
/**REG7_BNESTR (rw) register accessor: RISAF region 7 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg7_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg7_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG7_BNESTR)

For information about available fields see [`mod@reg7_bnestr`] module*/
pub type REG7_BNESTR = crate::Reg<reg7_bnestr::REG7_BNESTRrs>;
///RISAF region 7 subregion B nested mode register
pub mod reg7_bnestr;
/**REG8_BCFGR (rw) register accessor: RISAF region 8 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg8_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_BCFGR)

For information about available fields see [`mod@reg8_bcfgr`] module*/
pub type REG8_BCFGR = crate::Reg<reg8_bcfgr::REG8_BCFGRrs>;
///RISAF region 8 subregion B configuration register
pub mod reg8_bcfgr;
/**REG8_BSTARTR (rw) register accessor: RISAF region 8 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_BSTARTR)

For information about available fields see [`mod@reg8_bstartr`] module*/
pub type REG8_BSTARTR = crate::Reg<reg8_bstartr::REG8_BSTARTRrs>;
///RISAF region 8 subregion B start-address register
pub mod reg8_bstartr;
/**REG8_BENDR (rw) register accessor: RISAF region 8 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg8_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_BENDR)

For information about available fields see [`mod@reg8_bendr`] module*/
pub type REG8_BENDR = crate::Reg<reg8_bendr::REG8_BENDRrs>;
///RISAF region 8 subregion B end-address register
pub mod reg8_bendr;
/**REG8_BNESTR (rw) register accessor: RISAF region 8 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg8_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg8_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG8_BNESTR)

For information about available fields see [`mod@reg8_bnestr`] module*/
pub type REG8_BNESTR = crate::Reg<reg8_bnestr::REG8_BNESTRrs>;
///RISAF region 8 subregion B nested mode register
pub mod reg8_bnestr;
/**REG9_BCFGR (rw) register accessor: RISAF region 9 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg9_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_BCFGR)

For information about available fields see [`mod@reg9_bcfgr`] module*/
pub type REG9_BCFGR = crate::Reg<reg9_bcfgr::REG9_BCFGRrs>;
///RISAF region 9 subregion B configuration register
pub mod reg9_bcfgr;
/**REG9_BSTARTR (rw) register accessor: RISAF region 9 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_BSTARTR)

For information about available fields see [`mod@reg9_bstartr`] module*/
pub type REG9_BSTARTR = crate::Reg<reg9_bstartr::REG9_BSTARTRrs>;
///RISAF region 9 subregion B start-address register
pub mod reg9_bstartr;
/**REG9_BENDR (rw) register accessor: RISAF region 9 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg9_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_BENDR)

For information about available fields see [`mod@reg9_bendr`] module*/
pub type REG9_BENDR = crate::Reg<reg9_bendr::REG9_BENDRrs>;
///RISAF region 9 subregion B end-address register
pub mod reg9_bendr;
/**REG9_BNESTR (rw) register accessor: RISAF region 9 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg9_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg9_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG9_BNESTR)

For information about available fields see [`mod@reg9_bnestr`] module*/
pub type REG9_BNESTR = crate::Reg<reg9_bnestr::REG9_BNESTRrs>;
///RISAF region 9 subregion B nested mode register
pub mod reg9_bnestr;
/**REG10_BCFGR (rw) register accessor: RISAF region 10 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg10_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_BCFGR)

For information about available fields see [`mod@reg10_bcfgr`] module*/
pub type REG10_BCFGR = crate::Reg<reg10_bcfgr::REG10_BCFGRrs>;
///RISAF region 10 subregion B configuration register
pub mod reg10_bcfgr;
/**REG10_BSTARTR (rw) register accessor: RISAF region 10 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_BSTARTR)

For information about available fields see [`mod@reg10_bstartr`] module*/
pub type REG10_BSTARTR = crate::Reg<reg10_bstartr::REG10_BSTARTRrs>;
///RISAF region 10 subregion B start-address register
pub mod reg10_bstartr;
/**REG10_BENDR (rw) register accessor: RISAF region 10 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg10_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_BENDR)

For information about available fields see [`mod@reg10_bendr`] module*/
pub type REG10_BENDR = crate::Reg<reg10_bendr::REG10_BENDRrs>;
///RISAF region 10 subregion B end-address register
pub mod reg10_bendr;
/**REG10_BNESTR (rw) register accessor: RISAF region 10 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg10_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg10_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG10_BNESTR)

For information about available fields see [`mod@reg10_bnestr`] module*/
pub type REG10_BNESTR = crate::Reg<reg10_bnestr::REG10_BNESTRrs>;
///RISAF region 10 subregion B nested mode register
pub mod reg10_bnestr;
/**REG11_BCFGR (rw) register accessor: RISAF region 11 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg11_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_BCFGR)

For information about available fields see [`mod@reg11_bcfgr`] module*/
pub type REG11_BCFGR = crate::Reg<reg11_bcfgr::REG11_BCFGRrs>;
///RISAF region 11 subregion B configuration register
pub mod reg11_bcfgr;
/**REG11_BSTARTR (rw) register accessor: RISAF region 11 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_BSTARTR)

For information about available fields see [`mod@reg11_bstartr`] module*/
pub type REG11_BSTARTR = crate::Reg<reg11_bstartr::REG11_BSTARTRrs>;
///RISAF region 11 subregion B start-address register
pub mod reg11_bstartr;
/**REG11_BENDR (rw) register accessor: RISAF region 11 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg11_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_BENDR)

For information about available fields see [`mod@reg11_bendr`] module*/
pub type REG11_BENDR = crate::Reg<reg11_bendr::REG11_BENDRrs>;
///RISAF region 11 subregion B end-address register
pub mod reg11_bendr;
/**REG11_BNESTR (rw) register accessor: RISAF region 11 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg11_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg11_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG11_BNESTR)

For information about available fields see [`mod@reg11_bnestr`] module*/
pub type REG11_BNESTR = crate::Reg<reg11_bnestr::REG11_BNESTRrs>;
///RISAF region 11 subregion B nested mode register
pub mod reg11_bnestr;
/**REG12_BCFGR (rw) register accessor: RISAF region 12 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg12_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_BCFGR)

For information about available fields see [`mod@reg12_bcfgr`] module*/
pub type REG12_BCFGR = crate::Reg<reg12_bcfgr::REG12_BCFGRrs>;
///RISAF region 12 subregion B configuration register
pub mod reg12_bcfgr;
/**REG12_BSTARTR (rw) register accessor: RISAF region 12 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_BSTARTR)

For information about available fields see [`mod@reg12_bstartr`] module*/
pub type REG12_BSTARTR = crate::Reg<reg12_bstartr::REG12_BSTARTRrs>;
///RISAF region 12 subregion B start-address register
pub mod reg12_bstartr;
/**REG12_BENDR (rw) register accessor: RISAF region 12 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg12_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_BENDR)

For information about available fields see [`mod@reg12_bendr`] module*/
pub type REG12_BENDR = crate::Reg<reg12_bendr::REG12_BENDRrs>;
///RISAF region 12 subregion B end-address register
pub mod reg12_bendr;
/**REG12_BNESTR (rw) register accessor: RISAF region 12 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg12_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg12_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG12_BNESTR)

For information about available fields see [`mod@reg12_bnestr`] module*/
pub type REG12_BNESTR = crate::Reg<reg12_bnestr::REG12_BNESTRrs>;
///RISAF region 12 subregion B nested mode register
pub mod reg12_bnestr;
/**REG13_BCFGR (rw) register accessor: RISAF region 13 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg13_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_BCFGR)

For information about available fields see [`mod@reg13_bcfgr`] module*/
pub type REG13_BCFGR = crate::Reg<reg13_bcfgr::REG13_BCFGRrs>;
///RISAF region 13 subregion B configuration register
pub mod reg13_bcfgr;
/**REG13_BSTARTR (rw) register accessor: RISAF region 13 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg13_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_BSTARTR)

For information about available fields see [`mod@reg13_bstartr`] module*/
pub type REG13_BSTARTR = crate::Reg<reg13_bstartr::REG13_BSTARTRrs>;
///RISAF region 13 subregion B start-address register
pub mod reg13_bstartr;
/**REG13_BENDR (rw) register accessor: RISAF region 13 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg13_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_BENDR)

For information about available fields see [`mod@reg13_bendr`] module*/
pub type REG13_BENDR = crate::Reg<reg13_bendr::REG13_BENDRrs>;
///RISAF region 13 subregion B end-address register
pub mod reg13_bendr;
/**REG13_BNESTR (rw) register accessor: RISAF region 13 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg13_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg13_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG13_BNESTR)

For information about available fields see [`mod@reg13_bnestr`] module*/
pub type REG13_BNESTR = crate::Reg<reg13_bnestr::REG13_BNESTRrs>;
///RISAF region 13 subregion B nested mode register
pub mod reg13_bnestr;
/**REG14_BCFGR (rw) register accessor: RISAF region 14 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg14_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_BCFGR)

For information about available fields see [`mod@reg14_bcfgr`] module*/
pub type REG14_BCFGR = crate::Reg<reg14_bcfgr::REG14_BCFGRrs>;
///RISAF region 14 subregion B configuration register
pub mod reg14_bcfgr;
/**REG14_BSTARTR (rw) register accessor: RISAF region 14 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_BSTARTR)

For information about available fields see [`mod@reg14_bstartr`] module*/
pub type REG14_BSTARTR = crate::Reg<reg14_bstartr::REG14_BSTARTRrs>;
///RISAF region 14 subregion B start-address register
pub mod reg14_bstartr;
/**REG14_BENDR (rw) register accessor: RISAF region 14 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg14_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_BENDR)

For information about available fields see [`mod@reg14_bendr`] module*/
pub type REG14_BENDR = crate::Reg<reg14_bendr::REG14_BENDRrs>;
///RISAF region 14 subregion B end-address register
pub mod reg14_bendr;
/**REG14_BNESTR (rw) register accessor: RISAF region 14 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg14_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG14_BNESTR)

For information about available fields see [`mod@reg14_bnestr`] module*/
pub type REG14_BNESTR = crate::Reg<reg14_bnestr::REG14_BNESTRrs>;
///RISAF region 14 subregion B nested mode register
pub mod reg14_bnestr;
/**REG15_BCFGR (rw) register accessor: RISAF region 15 subregion B configuration register

You can [`read`](crate::Reg::read) this register and get [`reg15_bcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_bcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_BCFGR)

For information about available fields see [`mod@reg15_bcfgr`] module*/
pub type REG15_BCFGR = crate::Reg<reg15_bcfgr::REG15_BCFGRrs>;
///RISAF region 15 subregion B configuration register
pub mod reg15_bcfgr;
/**REG15_BSTARTR (rw) register accessor: RISAF region 15 subregion B start-address register

You can [`read`](crate::Reg::read) this register and get [`reg15_bstartr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_bstartr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_BSTARTR)

For information about available fields see [`mod@reg15_bstartr`] module*/
pub type REG15_BSTARTR = crate::Reg<reg15_bstartr::REG15_BSTARTRrs>;
///RISAF region 15 subregion B start-address register
pub mod reg15_bstartr;
/**REG15_BENDR (rw) register accessor: RISAF region 15 subregion B end-address register

You can [`read`](crate::Reg::read) this register and get [`reg15_bendr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_bendr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_BENDR)

For information about available fields see [`mod@reg15_bendr`] module*/
pub type REG15_BENDR = crate::Reg<reg15_bendr::REG15_BENDRrs>;
///RISAF region 15 subregion B end-address register
pub mod reg15_bendr;
/**REG15_BNESTR (rw) register accessor: RISAF region 15 subregion B nested mode register

You can [`read`](crate::Reg::read) this register and get [`reg15_bnestr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15_bnestr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RISAF:REG15_BNESTR)

For information about available fields see [`mod@reg15_bnestr`] module*/
pub type REG15_BNESTR = crate::Reg<reg15_bnestr::REG15_BNESTRrs>;
///RISAF region 15 subregion B nested mode register
pub mod reg15_bnestr;
