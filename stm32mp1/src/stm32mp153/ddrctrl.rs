#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ddrctrl_mstr: DDRCTRL_MSTR,
    ddrctrl_stat: DDRCTRL_STAT,
    _reserved2: [u8; 0x08],
    ddrctrl_mrctrl0: DDRCTRL_MRCTRL0,
    ddrctrl_mrctrl1: DDRCTRL_MRCTRL1,
    ddrctrl_mrstat: DDRCTRL_MRSTAT,
    _reserved5: [u8; 0x04],
    ddrctrl_derateen: DDRCTRL_DERATEEN,
    ddrctrl_derateint: DDRCTRL_DERATEINT,
    _reserved7: [u8; 0x08],
    ddrctrl_pwrctl: DDRCTRL_PWRCTL,
    ddrctrl_pwrtmg: DDRCTRL_PWRTMG,
    ddrctrl_hwlpctl: DDRCTRL_HWLPCTL,
    _reserved10: [u8; 0x14],
    ddrctrl_rfshctl0: DDRCTRL_RFSHCTL0,
    _reserved11: [u8; 0x0c],
    ddrctrl_rfshctl3: DDRCTRL_RFSHCTL3,
    ddrctrl_rfshtmg: DDRCTRL_RFSHTMG,
    _reserved13: [u8; 0x58],
    ddrctrl_crcparctl0: DDRCTRL_CRCPARCTL0,
    _reserved14: [u8; 0x08],
    ddrctrl_crcparstat: DDRCTRL_CRCPARSTAT,
    ddrctrl_init0: DDRCTRL_INIT0,
    ddrctrl_init1: DDRCTRL_INIT1,
    ddrctrl_init2: DDRCTRL_INIT2,
    ddrctrl_init3: DDRCTRL_INIT3,
    ddrctrl_init4: DDRCTRL_INIT4,
    ddrctrl_init5: DDRCTRL_INIT5,
    _reserved21: [u8; 0x08],
    ddrctrl_dimmctl: DDRCTRL_DIMMCTL,
    _reserved22: [u8; 0x0c],
    ddrctrl_dramtmg0: DDRCTRL_DRAMTMG0,
    ddrctrl_dramtmg1: DDRCTRL_DRAMTMG1,
    ddrctrl_dramtmg2: DDRCTRL_DRAMTMG2,
    ddrctrl_dramtmg3: DDRCTRL_DRAMTMG3,
    ddrctrl_dramtmg4: DDRCTRL_DRAMTMG4,
    ddrctrl_dramtmg5: DDRCTRL_DRAMTMG5,
    ddrctrl_dramtmg6: DDRCTRL_DRAMTMG6,
    ddrctrl_dramtmg7: DDRCTRL_DRAMTMG7,
    ddrctrl_dramtmg8: DDRCTRL_DRAMTMG8,
    _reserved31: [u8; 0x14],
    ddrctrl_dramtmg14: DDRCTRL_DRAMTMG14,
    ddrctrl_dramtmg15: DDRCTRL_DRAMTMG15,
    _reserved33: [u8; 0x40],
    ddrctrl_zqctl0: DDRCTRL_ZQCTL0,
    ddrctrl_zqctl1: DDRCTRL_ZQCTL1,
    ddrctrl_zqctl2: DDRCTRL_ZQCTL2,
    ddrctrl_zqstat: DDRCTRL_ZQSTAT,
    ddrctrl_dfitmg0: DDRCTRL_DFITMG0,
    ddrctrl_dfitmg1: DDRCTRL_DFITMG1,
    ddrctrl_dfilpcfg0: DDRCTRL_DFILPCFG0,
    _reserved40: [u8; 0x04],
    ddrctrl_dfiupd0: DDRCTRL_DFIUPD0,
    ddrctrl_dfiupd1: DDRCTRL_DFIUPD1,
    ddrctrl_dfiupd2: DDRCTRL_DFIUPD2,
    _reserved43: [u8; 0x04],
    ddrctrl_dfimisc: DDRCTRL_DFIMISC,
    _reserved44: [u8; 0x08],
    ddrctrl_dfistat: DDRCTRL_DFISTAT,
    _reserved45: [u8; 0x04],
    ddrctrl_dfiphymstr: DDRCTRL_DFIPHYMSTR,
    _reserved46: [u8; 0x3c],
    ddrctrl_addrmap1: DDRCTRL_ADDRMAP1,
    ddrctrl_addrmap2: DDRCTRL_ADDRMAP2,
    ddrctrl_addrmap3: DDRCTRL_ADDRMAP3,
    ddrctrl_addrmap4: DDRCTRL_ADDRMAP4,
    ddrctrl_addrmap5: DDRCTRL_ADDRMAP5,
    ddrctrl_addrmap6: DDRCTRL_ADDRMAP6,
    _reserved52: [u8; 0x08],
    ddrctrl_addrmap9: DDRCTRL_ADDRMAP9,
    ddrctrl_addrmap10: DDRCTRL_ADDRMAP10,
    ddrctrl_addrmap11: DDRCTRL_ADDRMAP11,
    _reserved55: [u8; 0x10],
    ddrctrl_odtcfg: DDRCTRL_ODTCFG,
    ddrctrl_odtmap: DDRCTRL_ODTMAP,
    _reserved57: [u8; 0x08],
    ddrctrl_sched: DDRCTRL_SCHED,
    ddrctrl_sched1: DDRCTRL_SCHED1,
    _reserved59: [u8; 0x04],
    ddrctrl_perfhpr1: DDRCTRL_PERFHPR1,
    _reserved60: [u8; 0x04],
    ddrctrl_perflpr1: DDRCTRL_PERFLPR1,
    _reserved61: [u8; 0x04],
    ddrctrl_perfwr1: DDRCTRL_PERFWR1,
    _reserved62: [u8; 0x90],
    ddrctrl_dbg0: DDRCTRL_DBG0,
    ddrctrl_dbg1: DDRCTRL_DBG1,
    ddrctrl_dbgcam: DDRCTRL_DBGCAM,
    ddrctrl_dbgcmd: DDRCTRL_DBGCMD,
    ddrctrl_dbgstat: DDRCTRL_DBGSTAT,
    _reserved67: [u8; 0x0c],
    ddrctrl_swctl: DDRCTRL_SWCTL,
    ddrctrl_swstat: DDRCTRL_SWSTAT,
    _reserved69: [u8; 0x44],
    ddrctrl_poisoncfg: DDRCTRL_POISONCFG,
    ddrctrl_poisonstat: DDRCTRL_POISONSTAT,
    _reserved71: [u8; 0x88],
    ddrctrl_pstat: DDRCTRL_PSTAT,
    ddrctrl_pccfg: DDRCTRL_PCCFG,
    ddrctrl_pcfgr_0: DDRCTRL_PCFGR_0,
    ddrctrl_pcfgw_0: DDRCTRL_PCFGW_0,
    _reserved75: [u8; 0x84],
    ddrctrl_pctrl_0: DDRCTRL_PCTRL_0,
    ddrctrl_pcfgqos0_0: DDRCTRL_PCFGQOS0_0,
    ddrctrl_pcfgqos1_0: DDRCTRL_PCFGQOS1_0,
    ddrctrl_pcfgwqos0_0: DDRCTRL_PCFGWQOS0_0,
    ddrctrl_pcfgwqos1_0: DDRCTRL_PCFGWQOS1_0,
    _reserved80: [u8; 0x10],
    ddrctrl_pcfgr_1: DDRCTRL_PCFGR_1,
    ddrctrl_pcfgw_1: DDRCTRL_PCFGW_1,
    _reserved82: [u8; 0x84],
    ddrctrl_pctrl_1: DDRCTRL_PCTRL_1,
    ddrctrl_pcfgqos0_1: DDRCTRL_PCFGQOS0_1,
    ddrctrl_pcfgqos1_1: DDRCTRL_PCFGQOS1_1,
    ddrctrl_pcfgwqos0_1: DDRCTRL_PCFGWQOS0_1,
    ddrctrl_pcfgwqos1_1: DDRCTRL_PCFGWQOS1_1,
}
impl RegisterBlock {
    #[doc = "0x00 - DDRCTRL master register 0"]
    #[inline(always)]
    pub const fn ddrctrl_mstr(&self) -> &DDRCTRL_MSTR {
        &self.ddrctrl_mstr
    }
    #[doc = "0x04 - DDRCTRL operating mode status register"]
    #[inline(always)]
    pub const fn ddrctrl_stat(&self) -> &DDRCTRL_STAT {
        &self.ddrctrl_stat
    }
    #[doc = "0x10 - Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en"]
    #[inline(always)]
    pub const fn ddrctrl_mrctrl0(&self) -> &DDRCTRL_MRCTRL0 {
        &self.ddrctrl_mrctrl0
    }
    #[doc = "0x14 - DDRCTRL mode register read/write control register 1"]
    #[inline(always)]
    pub const fn ddrctrl_mrctrl1(&self) -> &DDRCTRL_MRCTRL1 {
        &self.ddrctrl_mrctrl1
    }
    #[doc = "0x18 - DDRCTRL mode register read/write status register"]
    #[inline(always)]
    pub const fn ddrctrl_mrstat(&self) -> &DDRCTRL_MRSTAT {
        &self.ddrctrl_mrstat
    }
    #[doc = "0x20 - DDRCTRL temperature derate enable register"]
    #[inline(always)]
    pub const fn ddrctrl_derateen(&self) -> &DDRCTRL_DERATEEN {
        &self.ddrctrl_derateen
    }
    #[doc = "0x24 - DDRCTRL temperature derate interval register"]
    #[inline(always)]
    pub const fn ddrctrl_derateint(&self) -> &DDRCTRL_DERATEINT {
        &self.ddrctrl_derateint
    }
    #[doc = "0x30 - DDRCTRL low power control register"]
    #[inline(always)]
    pub const fn ddrctrl_pwrctl(&self) -> &DDRCTRL_PWRCTL {
        &self.ddrctrl_pwrctl
    }
    #[doc = "0x34 - DDRCTRL low power timing register"]
    #[inline(always)]
    pub const fn ddrctrl_pwrtmg(&self) -> &DDRCTRL_PWRTMG {
        &self.ddrctrl_pwrtmg
    }
    #[doc = "0x38 - DDRCTRL hardware low power control register"]
    #[inline(always)]
    pub const fn ddrctrl_hwlpctl(&self) -> &DDRCTRL_HWLPCTL {
        &self.ddrctrl_hwlpctl
    }
    #[doc = "0x50 - DDRCTRL refresh control register 0"]
    #[inline(always)]
    pub const fn ddrctrl_rfshctl0(&self) -> &DDRCTRL_RFSHCTL0 {
        &self.ddrctrl_rfshctl0
    }
    #[doc = "0x60 - DDRCTRL refresh control register 3"]
    #[inline(always)]
    pub const fn ddrctrl_rfshctl3(&self) -> &DDRCTRL_RFSHCTL3 {
        &self.ddrctrl_rfshctl3
    }
    #[doc = "0x64 - DDRCTRL refresh timing register"]
    #[inline(always)]
    pub const fn ddrctrl_rfshtmg(&self) -> &DDRCTRL_RFSHTMG {
        &self.ddrctrl_rfshtmg
    }
    #[doc = "0xc0 - DDRCTRL CRC parity control register 0"]
    #[inline(always)]
    pub const fn ddrctrl_crcparctl0(&self) -> &DDRCTRL_CRCPARCTL0 {
        &self.ddrctrl_crcparctl0
    }
    #[doc = "0xcc - DDRCTRL CRC parity status register"]
    #[inline(always)]
    pub const fn ddrctrl_crcparstat(&self) -> &DDRCTRL_CRCPARSTAT {
        &self.ddrctrl_crcparstat
    }
    #[doc = "0xd0 - DDRCTRL SDRAM initialization register 0"]
    #[inline(always)]
    pub const fn ddrctrl_init0(&self) -> &DDRCTRL_INIT0 {
        &self.ddrctrl_init0
    }
    #[doc = "0xd4 - DDRCTRL SDRAM initialization register 1"]
    #[inline(always)]
    pub const fn ddrctrl_init1(&self) -> &DDRCTRL_INIT1 {
        &self.ddrctrl_init1
    }
    #[doc = "0xd8 - DDRCTRL SDRAM initialization register 2"]
    #[inline(always)]
    pub const fn ddrctrl_init2(&self) -> &DDRCTRL_INIT2 {
        &self.ddrctrl_init2
    }
    #[doc = "0xdc - DDRCTRL SDRAM initialization register 3"]
    #[inline(always)]
    pub const fn ddrctrl_init3(&self) -> &DDRCTRL_INIT3 {
        &self.ddrctrl_init3
    }
    #[doc = "0xe0 - DDRCTRL SDRAM initialization register 4"]
    #[inline(always)]
    pub const fn ddrctrl_init4(&self) -> &DDRCTRL_INIT4 {
        &self.ddrctrl_init4
    }
    #[doc = "0xe4 - DDRCTRL SDRAM initialization register 5"]
    #[inline(always)]
    pub const fn ddrctrl_init5(&self) -> &DDRCTRL_INIT5 {
        &self.ddrctrl_init5
    }
    #[doc = "0xf0 - DDRCTRL DIMM control register"]
    #[inline(always)]
    pub const fn ddrctrl_dimmctl(&self) -> &DDRCTRL_DIMMCTL {
        &self.ddrctrl_dimmctl
    }
    #[doc = "0x100 - DDRCTRL SDRAM timing register 0"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg0(&self) -> &DDRCTRL_DRAMTMG0 {
        &self.ddrctrl_dramtmg0
    }
    #[doc = "0x104 - DDRCTRL SDRAM timing register 1"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg1(&self) -> &DDRCTRL_DRAMTMG1 {
        &self.ddrctrl_dramtmg1
    }
    #[doc = "0x108 - DDRCTRL SDRAM timing register 2"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg2(&self) -> &DDRCTRL_DRAMTMG2 {
        &self.ddrctrl_dramtmg2
    }
    #[doc = "0x10c - DDRCTRL SDRAM timing register 3"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg3(&self) -> &DDRCTRL_DRAMTMG3 {
        &self.ddrctrl_dramtmg3
    }
    #[doc = "0x110 - DDRCTRL SDRAM timing register 4"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg4(&self) -> &DDRCTRL_DRAMTMG4 {
        &self.ddrctrl_dramtmg4
    }
    #[doc = "0x114 - DDRCTRL SDRAM timing register 5"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg5(&self) -> &DDRCTRL_DRAMTMG5 {
        &self.ddrctrl_dramtmg5
    }
    #[doc = "0x118 - DDRCTRL SDRAM timing register 6"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg6(&self) -> &DDRCTRL_DRAMTMG6 {
        &self.ddrctrl_dramtmg6
    }
    #[doc = "0x11c - DDRCTRL SDRAM timing register 7"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg7(&self) -> &DDRCTRL_DRAMTMG7 {
        &self.ddrctrl_dramtmg7
    }
    #[doc = "0x120 - DDRCTRL SDRAM timing register 8"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg8(&self) -> &DDRCTRL_DRAMTMG8 {
        &self.ddrctrl_dramtmg8
    }
    #[doc = "0x138 - DDRCTRL SDRAM timing register 14"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg14(&self) -> &DDRCTRL_DRAMTMG14 {
        &self.ddrctrl_dramtmg14
    }
    #[doc = "0x13c - DDRCTRL SDRAM timing register 15"]
    #[inline(always)]
    pub const fn ddrctrl_dramtmg15(&self) -> &DDRCTRL_DRAMTMG15 {
        &self.ddrctrl_dramtmg15
    }
    #[doc = "0x180 - DDRCTRL ZQ control register 0"]
    #[inline(always)]
    pub const fn ddrctrl_zqctl0(&self) -> &DDRCTRL_ZQCTL0 {
        &self.ddrctrl_zqctl0
    }
    #[doc = "0x184 - DDRCTRL ZQ control register 1"]
    #[inline(always)]
    pub const fn ddrctrl_zqctl1(&self) -> &DDRCTRL_ZQCTL1 {
        &self.ddrctrl_zqctl1
    }
    #[doc = "0x188 - DDRCTRL ZQ control register 2"]
    #[inline(always)]
    pub const fn ddrctrl_zqctl2(&self) -> &DDRCTRL_ZQCTL2 {
        &self.ddrctrl_zqctl2
    }
    #[doc = "0x18c - DDRCTRL ZQ status register"]
    #[inline(always)]
    pub const fn ddrctrl_zqstat(&self) -> &DDRCTRL_ZQSTAT {
        &self.ddrctrl_zqstat
    }
    #[doc = "0x190 - DDRCTRL DFI timing register 0"]
    #[inline(always)]
    pub const fn ddrctrl_dfitmg0(&self) -> &DDRCTRL_DFITMG0 {
        &self.ddrctrl_dfitmg0
    }
    #[doc = "0x194 - DDRCTRL DFI timing register 1"]
    #[inline(always)]
    pub const fn ddrctrl_dfitmg1(&self) -> &DDRCTRL_DFITMG1 {
        &self.ddrctrl_dfitmg1
    }
    #[doc = "0x198 - DDRCTRL low power configuration register 0"]
    #[inline(always)]
    pub const fn ddrctrl_dfilpcfg0(&self) -> &DDRCTRL_DFILPCFG0 {
        &self.ddrctrl_dfilpcfg0
    }
    #[doc = "0x1a0 - DDRCTRL DFI update register 0"]
    #[inline(always)]
    pub const fn ddrctrl_dfiupd0(&self) -> &DDRCTRL_DFIUPD0 {
        &self.ddrctrl_dfiupd0
    }
    #[doc = "0x1a4 - DDRCTRL DFI update register 1"]
    #[inline(always)]
    pub const fn ddrctrl_dfiupd1(&self) -> &DDRCTRL_DFIUPD1 {
        &self.ddrctrl_dfiupd1
    }
    #[doc = "0x1a8 - DDRCTRL DFI update register 2"]
    #[inline(always)]
    pub const fn ddrctrl_dfiupd2(&self) -> &DDRCTRL_DFIUPD2 {
        &self.ddrctrl_dfiupd2
    }
    #[doc = "0x1b0 - DDRCTRL DFI miscellaneous control register"]
    #[inline(always)]
    pub const fn ddrctrl_dfimisc(&self) -> &DDRCTRL_DFIMISC {
        &self.ddrctrl_dfimisc
    }
    #[doc = "0x1bc - DDRCTRL DFI status register"]
    #[inline(always)]
    pub const fn ddrctrl_dfistat(&self) -> &DDRCTRL_DFISTAT {
        &self.ddrctrl_dfistat
    }
    #[doc = "0x1c4 - DDRCTRL DFI PHY master register"]
    #[inline(always)]
    pub const fn ddrctrl_dfiphymstr(&self) -> &DDRCTRL_DFIPHYMSTR {
        &self.ddrctrl_dfiphymstr
    }
    #[doc = "0x204 - DDRCTRL address map register 1"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap1(&self) -> &DDRCTRL_ADDRMAP1 {
        &self.ddrctrl_addrmap1
    }
    #[doc = "0x208 - DDRCTRL address map register 2"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap2(&self) -> &DDRCTRL_ADDRMAP2 {
        &self.ddrctrl_addrmap2
    }
    #[doc = "0x20c - DDRCTRL address map register 3"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap3(&self) -> &DDRCTRL_ADDRMAP3 {
        &self.ddrctrl_addrmap3
    }
    #[doc = "0x210 - DDRCTRL address map register 4"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap4(&self) -> &DDRCTRL_ADDRMAP4 {
        &self.ddrctrl_addrmap4
    }
    #[doc = "0x214 - DDRCTRL address map register 5"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap5(&self) -> &DDRCTRL_ADDRMAP5 {
        &self.ddrctrl_addrmap5
    }
    #[doc = "0x218 - DDRCTRL address register 6"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap6(&self) -> &DDRCTRL_ADDRMAP6 {
        &self.ddrctrl_addrmap6
    }
    #[doc = "0x224 - DDRCTRL address map register 9"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap9(&self) -> &DDRCTRL_ADDRMAP9 {
        &self.ddrctrl_addrmap9
    }
    #[doc = "0x228 - DDRCTRL address map register 10"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap10(&self) -> &DDRCTRL_ADDRMAP10 {
        &self.ddrctrl_addrmap10
    }
    #[doc = "0x22c - DDRCTRL address map register 11"]
    #[inline(always)]
    pub const fn ddrctrl_addrmap11(&self) -> &DDRCTRL_ADDRMAP11 {
        &self.ddrctrl_addrmap11
    }
    #[doc = "0x240 - DDRCTRL ODT configuration register"]
    #[inline(always)]
    pub const fn ddrctrl_odtcfg(&self) -> &DDRCTRL_ODTCFG {
        &self.ddrctrl_odtcfg
    }
    #[doc = "0x244 - DDRCTRL ODT/Rank map register"]
    #[inline(always)]
    pub const fn ddrctrl_odtmap(&self) -> &DDRCTRL_ODTMAP {
        &self.ddrctrl_odtmap
    }
    #[doc = "0x250 - DDRCTRL scheduler control register"]
    #[inline(always)]
    pub const fn ddrctrl_sched(&self) -> &DDRCTRL_SCHED {
        &self.ddrctrl_sched
    }
    #[doc = "0x254 - DDRCTRL scheduler control register 1"]
    #[inline(always)]
    pub const fn ddrctrl_sched1(&self) -> &DDRCTRL_SCHED1 {
        &self.ddrctrl_sched1
    }
    #[doc = "0x25c - DDRCTRL high priority read CAM register 1"]
    #[inline(always)]
    pub const fn ddrctrl_perfhpr1(&self) -> &DDRCTRL_PERFHPR1 {
        &self.ddrctrl_perfhpr1
    }
    #[doc = "0x264 - DDRCTRL low priority read CAM register 1"]
    #[inline(always)]
    pub const fn ddrctrl_perflpr1(&self) -> &DDRCTRL_PERFLPR1 {
        &self.ddrctrl_perflpr1
    }
    #[doc = "0x26c - DDRCTRL write CAM register 1"]
    #[inline(always)]
    pub const fn ddrctrl_perfwr1(&self) -> &DDRCTRL_PERFWR1 {
        &self.ddrctrl_perfwr1
    }
    #[doc = "0x300 - DDRCTRL debug register 0"]
    #[inline(always)]
    pub const fn ddrctrl_dbg0(&self) -> &DDRCTRL_DBG0 {
        &self.ddrctrl_dbg0
    }
    #[doc = "0x304 - DDRCTRL debug register 1"]
    #[inline(always)]
    pub const fn ddrctrl_dbg1(&self) -> &DDRCTRL_DBG1 {
        &self.ddrctrl_dbg1
    }
    #[doc = "0x308 - DDRCTRL CAM debug register"]
    #[inline(always)]
    pub const fn ddrctrl_dbgcam(&self) -> &DDRCTRL_DBGCAM {
        &self.ddrctrl_dbgcam
    }
    #[doc = "0x30c - DDRCTRL command debug register"]
    #[inline(always)]
    pub const fn ddrctrl_dbgcmd(&self) -> &DDRCTRL_DBGCMD {
        &self.ddrctrl_dbgcmd
    }
    #[doc = "0x310 - DDRCTRL status debug register"]
    #[inline(always)]
    pub const fn ddrctrl_dbgstat(&self) -> &DDRCTRL_DBGSTAT {
        &self.ddrctrl_dbgstat
    }
    #[doc = "0x320 - DDRCTRL software register programming control enable"]
    #[inline(always)]
    pub const fn ddrctrl_swctl(&self) -> &DDRCTRL_SWCTL {
        &self.ddrctrl_swctl
    }
    #[doc = "0x324 - DDRCTRL software register programming control status"]
    #[inline(always)]
    pub const fn ddrctrl_swstat(&self) -> &DDRCTRL_SWSTAT {
        &self.ddrctrl_swstat
    }
    #[doc = "0x36c - AXI Poison configuration register common for all AXI ports."]
    #[inline(always)]
    pub const fn ddrctrl_poisoncfg(&self) -> &DDRCTRL_POISONCFG {
        &self.ddrctrl_poisoncfg
    }
    #[doc = "0x370 - DDRCTRL AXI Poison status register"]
    #[inline(always)]
    pub const fn ddrctrl_poisonstat(&self) -> &DDRCTRL_POISONSTAT {
        &self.ddrctrl_poisonstat
    }
    #[doc = "0x3fc - DDRCTRL port status register"]
    #[inline(always)]
    pub const fn ddrctrl_pstat(&self) -> &DDRCTRL_PSTAT {
        &self.ddrctrl_pstat
    }
    #[doc = "0x400 - DDRCTRL port common configuration register"]
    #[inline(always)]
    pub const fn ddrctrl_pccfg(&self) -> &DDRCTRL_PCCFG {
        &self.ddrctrl_pccfg
    }
    #[doc = "0x404 - DDRCTRL port 0 configuration read register"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgr_0(&self) -> &DDRCTRL_PCFGR_0 {
        &self.ddrctrl_pcfgr_0
    }
    #[doc = "0x408 - DDRCTRL port 0 configuration write register"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgw_0(&self) -> &DDRCTRL_PCFGW_0 {
        &self.ddrctrl_pcfgw_0
    }
    #[doc = "0x490 - DDRCTRL port 0 control register"]
    #[inline(always)]
    pub const fn ddrctrl_pctrl_0(&self) -> &DDRCTRL_PCTRL_0 {
        &self.ddrctrl_pctrl_0
    }
    #[doc = "0x494 - DDRCTRL port 0 read Q0S configuration register 0"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgqos0_0(&self) -> &DDRCTRL_PCFGQOS0_0 {
        &self.ddrctrl_pcfgqos0_0
    }
    #[doc = "0x498 - DDRCTRL port 0 read Q0S configuration register 1"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgqos1_0(&self) -> &DDRCTRL_PCFGQOS1_0 {
        &self.ddrctrl_pcfgqos1_0
    }
    #[doc = "0x49c - DDRCTRL port 0 write Q0S configuration register 0"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgwqos0_0(&self) -> &DDRCTRL_PCFGWQOS0_0 {
        &self.ddrctrl_pcfgwqos0_0
    }
    #[doc = "0x4a0 - DDRCTRL port 0 write Q0S configuration register 1"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgwqos1_0(&self) -> &DDRCTRL_PCFGWQOS1_0 {
        &self.ddrctrl_pcfgwqos1_0
    }
    #[doc = "0x4b4 - DDRCTRL port 1 configuration read register"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgr_1(&self) -> &DDRCTRL_PCFGR_1 {
        &self.ddrctrl_pcfgr_1
    }
    #[doc = "0x4b8 - DDRCTRL port 1 configuration write register"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgw_1(&self) -> &DDRCTRL_PCFGW_1 {
        &self.ddrctrl_pcfgw_1
    }
    #[doc = "0x540 - DDRCTRL port 1 control register"]
    #[inline(always)]
    pub const fn ddrctrl_pctrl_1(&self) -> &DDRCTRL_PCTRL_1 {
        &self.ddrctrl_pctrl_1
    }
    #[doc = "0x544 - DDRCTRL port 1 read Q0S configuration register 0"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgqos0_1(&self) -> &DDRCTRL_PCFGQOS0_1 {
        &self.ddrctrl_pcfgqos0_1
    }
    #[doc = "0x548 - DDRCTRL port 1 read Q0S configuration register 1"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgqos1_1(&self) -> &DDRCTRL_PCFGQOS1_1 {
        &self.ddrctrl_pcfgqos1_1
    }
    #[doc = "0x54c - DDRCTRL port 1 write Q0S configuration register 0"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgwqos0_1(&self) -> &DDRCTRL_PCFGWQOS0_1 {
        &self.ddrctrl_pcfgwqos0_1
    }
    #[doc = "0x550 - DDRCTRL port 1 write Q0S configuration register 1"]
    #[inline(always)]
    pub const fn ddrctrl_pcfgwqos1_1(&self) -> &DDRCTRL_PCFGWQOS1_1 {
        &self.ddrctrl_pcfgwqos1_1
    }
}
#[doc = "DDRCTRL_MSTR (rw) register accessor: DDRCTRL master register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_mstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_mstr`]
module"]
pub type DDRCTRL_MSTR = crate::Reg<ddrctrl_mstr::DDRCTRL_MSTRrs>;
#[doc = "DDRCTRL master register 0"]
pub mod ddrctrl_mstr;
#[doc = "DDRCTRL_STAT (r) register accessor: DDRCTRL operating mode status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_stat`]
module"]
pub type DDRCTRL_STAT = crate::Reg<ddrctrl_stat::DDRCTRL_STATrs>;
#[doc = "DDRCTRL operating mode status register"]
pub mod ddrctrl_stat;
#[doc = "DDRCTRL_MRCTRL0 (rw) register accessor: Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mrctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_mrctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_mrctrl0`]
module"]
pub type DDRCTRL_MRCTRL0 = crate::Reg<ddrctrl_mrctrl0::DDRCTRL_MRCTRL0rs>;
#[doc = "Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en"]
pub mod ddrctrl_mrctrl0;
#[doc = "DDRCTRL_MRCTRL1 (rw) register accessor: DDRCTRL mode register read/write control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mrctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_mrctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_mrctrl1`]
module"]
pub type DDRCTRL_MRCTRL1 = crate::Reg<ddrctrl_mrctrl1::DDRCTRL_MRCTRL1rs>;
#[doc = "DDRCTRL mode register read/write control register 1"]
pub mod ddrctrl_mrctrl1;
#[doc = "DDRCTRL_MRSTAT (r) register accessor: DDRCTRL mode register read/write status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_mrstat`]
module"]
pub type DDRCTRL_MRSTAT = crate::Reg<ddrctrl_mrstat::DDRCTRL_MRSTATrs>;
#[doc = "DDRCTRL mode register read/write status register"]
pub mod ddrctrl_mrstat;
#[doc = "DDRCTRL_DERATEEN (rw) register accessor: DDRCTRL temperature derate enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_derateen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_derateen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_derateen`]
module"]
pub type DDRCTRL_DERATEEN = crate::Reg<ddrctrl_derateen::DDRCTRL_DERATEENrs>;
#[doc = "DDRCTRL temperature derate enable register"]
pub mod ddrctrl_derateen;
#[doc = "DDRCTRL_DERATEINT (rw) register accessor: DDRCTRL temperature derate interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_derateint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_derateint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_derateint`]
module"]
pub type DDRCTRL_DERATEINT = crate::Reg<ddrctrl_derateint::DDRCTRL_DERATEINTrs>;
#[doc = "DDRCTRL temperature derate interval register"]
pub mod ddrctrl_derateint;
#[doc = "DDRCTRL_PWRCTL (rw) register accessor: DDRCTRL low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pwrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pwrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pwrctl`]
module"]
pub type DDRCTRL_PWRCTL = crate::Reg<ddrctrl_pwrctl::DDRCTRL_PWRCTLrs>;
#[doc = "DDRCTRL low power control register"]
pub mod ddrctrl_pwrctl;
#[doc = "DDRCTRL_PWRTMG (rw) register accessor: DDRCTRL low power timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pwrtmg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pwrtmg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pwrtmg`]
module"]
pub type DDRCTRL_PWRTMG = crate::Reg<ddrctrl_pwrtmg::DDRCTRL_PWRTMGrs>;
#[doc = "DDRCTRL low power timing register"]
pub mod ddrctrl_pwrtmg;
#[doc = "DDRCTRL_HWLPCTL (rw) register accessor: DDRCTRL hardware low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_hwlpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_hwlpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_hwlpctl`]
module"]
pub type DDRCTRL_HWLPCTL = crate::Reg<ddrctrl_hwlpctl::DDRCTRL_HWLPCTLrs>;
#[doc = "DDRCTRL hardware low power control register"]
pub mod ddrctrl_hwlpctl;
#[doc = "DDRCTRL_RFSHCTL0 (rw) register accessor: DDRCTRL refresh control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_rfshctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_rfshctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_rfshctl0`]
module"]
pub type DDRCTRL_RFSHCTL0 = crate::Reg<ddrctrl_rfshctl0::DDRCTRL_RFSHCTL0rs>;
#[doc = "DDRCTRL refresh control register 0"]
pub mod ddrctrl_rfshctl0;
#[doc = "DDRCTRL_RFSHCTL3 (rw) register accessor: DDRCTRL refresh control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_rfshctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_rfshctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_rfshctl3`]
module"]
pub type DDRCTRL_RFSHCTL3 = crate::Reg<ddrctrl_rfshctl3::DDRCTRL_RFSHCTL3rs>;
#[doc = "DDRCTRL refresh control register 3"]
pub mod ddrctrl_rfshctl3;
#[doc = "DDRCTRL_RFSHTMG (rw) register accessor: DDRCTRL refresh timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_rfshtmg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_rfshtmg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_rfshtmg`]
module"]
pub type DDRCTRL_RFSHTMG = crate::Reg<ddrctrl_rfshtmg::DDRCTRL_RFSHTMGrs>;
#[doc = "DDRCTRL refresh timing register"]
pub mod ddrctrl_rfshtmg;
#[doc = "DDRCTRL_CRCPARCTL0 (rw) register accessor: DDRCTRL CRC parity control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_crcparctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_crcparctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_crcparctl0`]
module"]
pub type DDRCTRL_CRCPARCTL0 = crate::Reg<ddrctrl_crcparctl0::DDRCTRL_CRCPARCTL0rs>;
#[doc = "DDRCTRL CRC parity control register 0"]
pub mod ddrctrl_crcparctl0;
#[doc = "DDRCTRL_CRCPARSTAT (r) register accessor: DDRCTRL CRC parity status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_crcparstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_crcparstat`]
module"]
pub type DDRCTRL_CRCPARSTAT = crate::Reg<ddrctrl_crcparstat::DDRCTRL_CRCPARSTATrs>;
#[doc = "DDRCTRL CRC parity status register"]
pub mod ddrctrl_crcparstat;
#[doc = "DDRCTRL_INIT0 (rw) register accessor: DDRCTRL SDRAM initialization register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_init0`]
module"]
pub type DDRCTRL_INIT0 = crate::Reg<ddrctrl_init0::DDRCTRL_INIT0rs>;
#[doc = "DDRCTRL SDRAM initialization register 0"]
pub mod ddrctrl_init0;
#[doc = "DDRCTRL_INIT1 (rw) register accessor: DDRCTRL SDRAM initialization register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_init1`]
module"]
pub type DDRCTRL_INIT1 = crate::Reg<ddrctrl_init1::DDRCTRL_INIT1rs>;
#[doc = "DDRCTRL SDRAM initialization register 1"]
pub mod ddrctrl_init1;
#[doc = "DDRCTRL_INIT2 (rw) register accessor: DDRCTRL SDRAM initialization register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_init2`]
module"]
pub type DDRCTRL_INIT2 = crate::Reg<ddrctrl_init2::DDRCTRL_INIT2rs>;
#[doc = "DDRCTRL SDRAM initialization register 2"]
pub mod ddrctrl_init2;
#[doc = "DDRCTRL_INIT3 (rw) register accessor: DDRCTRL SDRAM initialization register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_init3`]
module"]
pub type DDRCTRL_INIT3 = crate::Reg<ddrctrl_init3::DDRCTRL_INIT3rs>;
#[doc = "DDRCTRL SDRAM initialization register 3"]
pub mod ddrctrl_init3;
#[doc = "DDRCTRL_INIT4 (rw) register accessor: DDRCTRL SDRAM initialization register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_init4`]
module"]
pub type DDRCTRL_INIT4 = crate::Reg<ddrctrl_init4::DDRCTRL_INIT4rs>;
#[doc = "DDRCTRL SDRAM initialization register 4"]
pub mod ddrctrl_init4;
#[doc = "DDRCTRL_INIT5 (rw) register accessor: DDRCTRL SDRAM initialization register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_init5`]
module"]
pub type DDRCTRL_INIT5 = crate::Reg<ddrctrl_init5::DDRCTRL_INIT5rs>;
#[doc = "DDRCTRL SDRAM initialization register 5"]
pub mod ddrctrl_init5;
#[doc = "DDRCTRL_DIMMCTL (rw) register accessor: DDRCTRL DIMM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dimmctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dimmctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dimmctl`]
module"]
pub type DDRCTRL_DIMMCTL = crate::Reg<ddrctrl_dimmctl::DDRCTRL_DIMMCTLrs>;
#[doc = "DDRCTRL DIMM control register"]
pub mod ddrctrl_dimmctl;
#[doc = "DDRCTRL_DRAMTMG0 (rw) register accessor: DDRCTRL SDRAM timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg0`]
module"]
pub type DDRCTRL_DRAMTMG0 = crate::Reg<ddrctrl_dramtmg0::DDRCTRL_DRAMTMG0rs>;
#[doc = "DDRCTRL SDRAM timing register 0"]
pub mod ddrctrl_dramtmg0;
#[doc = "DDRCTRL_DRAMTMG1 (rw) register accessor: DDRCTRL SDRAM timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg1`]
module"]
pub type DDRCTRL_DRAMTMG1 = crate::Reg<ddrctrl_dramtmg1::DDRCTRL_DRAMTMG1rs>;
#[doc = "DDRCTRL SDRAM timing register 1"]
pub mod ddrctrl_dramtmg1;
#[doc = "DDRCTRL_DRAMTMG2 (rw) register accessor: DDRCTRL SDRAM timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg2`]
module"]
pub type DDRCTRL_DRAMTMG2 = crate::Reg<ddrctrl_dramtmg2::DDRCTRL_DRAMTMG2rs>;
#[doc = "DDRCTRL SDRAM timing register 2"]
pub mod ddrctrl_dramtmg2;
#[doc = "DDRCTRL_DRAMTMG3 (rw) register accessor: DDRCTRL SDRAM timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg3`]
module"]
pub type DDRCTRL_DRAMTMG3 = crate::Reg<ddrctrl_dramtmg3::DDRCTRL_DRAMTMG3rs>;
#[doc = "DDRCTRL SDRAM timing register 3"]
pub mod ddrctrl_dramtmg3;
#[doc = "DDRCTRL_DRAMTMG4 (rw) register accessor: DDRCTRL SDRAM timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg4`]
module"]
pub type DDRCTRL_DRAMTMG4 = crate::Reg<ddrctrl_dramtmg4::DDRCTRL_DRAMTMG4rs>;
#[doc = "DDRCTRL SDRAM timing register 4"]
pub mod ddrctrl_dramtmg4;
#[doc = "DDRCTRL_DRAMTMG5 (rw) register accessor: DDRCTRL SDRAM timing register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg5`]
module"]
pub type DDRCTRL_DRAMTMG5 = crate::Reg<ddrctrl_dramtmg5::DDRCTRL_DRAMTMG5rs>;
#[doc = "DDRCTRL SDRAM timing register 5"]
pub mod ddrctrl_dramtmg5;
#[doc = "DDRCTRL_DRAMTMG6 (rw) register accessor: DDRCTRL SDRAM timing register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg6`]
module"]
pub type DDRCTRL_DRAMTMG6 = crate::Reg<ddrctrl_dramtmg6::DDRCTRL_DRAMTMG6rs>;
#[doc = "DDRCTRL SDRAM timing register 6"]
pub mod ddrctrl_dramtmg6;
#[doc = "DDRCTRL_DRAMTMG7 (rw) register accessor: DDRCTRL SDRAM timing register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg7`]
module"]
pub type DDRCTRL_DRAMTMG7 = crate::Reg<ddrctrl_dramtmg7::DDRCTRL_DRAMTMG7rs>;
#[doc = "DDRCTRL SDRAM timing register 7"]
pub mod ddrctrl_dramtmg7;
#[doc = "DDRCTRL_DRAMTMG8 (rw) register accessor: DDRCTRL SDRAM timing register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg8`]
module"]
pub type DDRCTRL_DRAMTMG8 = crate::Reg<ddrctrl_dramtmg8::DDRCTRL_DRAMTMG8rs>;
#[doc = "DDRCTRL SDRAM timing register 8"]
pub mod ddrctrl_dramtmg8;
#[doc = "DDRCTRL_DRAMTMG14 (rw) register accessor: DDRCTRL SDRAM timing register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg14`]
module"]
pub type DDRCTRL_DRAMTMG14 = crate::Reg<ddrctrl_dramtmg14::DDRCTRL_DRAMTMG14rs>;
#[doc = "DDRCTRL SDRAM timing register 14"]
pub mod ddrctrl_dramtmg14;
#[doc = "DDRCTRL_DRAMTMG15 (rw) register accessor: DDRCTRL SDRAM timing register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dramtmg15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dramtmg15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dramtmg15`]
module"]
pub type DDRCTRL_DRAMTMG15 = crate::Reg<ddrctrl_dramtmg15::DDRCTRL_DRAMTMG15rs>;
#[doc = "DDRCTRL SDRAM timing register 15"]
pub mod ddrctrl_dramtmg15;
#[doc = "DDRCTRL_ZQCTL0 (rw) register accessor: DDRCTRL ZQ control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_zqctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_zqctl0`]
module"]
pub type DDRCTRL_ZQCTL0 = crate::Reg<ddrctrl_zqctl0::DDRCTRL_ZQCTL0rs>;
#[doc = "DDRCTRL ZQ control register 0"]
pub mod ddrctrl_zqctl0;
#[doc = "DDRCTRL_ZQCTL1 (rw) register accessor: DDRCTRL ZQ control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_zqctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_zqctl1`]
module"]
pub type DDRCTRL_ZQCTL1 = crate::Reg<ddrctrl_zqctl1::DDRCTRL_ZQCTL1rs>;
#[doc = "DDRCTRL ZQ control register 1"]
pub mod ddrctrl_zqctl1;
#[doc = "DDRCTRL_ZQCTL2 (rw) register accessor: DDRCTRL ZQ control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_zqctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_zqctl2`]
module"]
pub type DDRCTRL_ZQCTL2 = crate::Reg<ddrctrl_zqctl2::DDRCTRL_ZQCTL2rs>;
#[doc = "DDRCTRL ZQ control register 2"]
pub mod ddrctrl_zqctl2;
#[doc = "DDRCTRL_ZQSTAT (r) register accessor: DDRCTRL ZQ status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_zqstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_zqstat`]
module"]
pub type DDRCTRL_ZQSTAT = crate::Reg<ddrctrl_zqstat::DDRCTRL_ZQSTATrs>;
#[doc = "DDRCTRL ZQ status register"]
pub mod ddrctrl_zqstat;
#[doc = "DDRCTRL_DFITMG0 (rw) register accessor: DDRCTRL DFI timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfitmg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfitmg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfitmg0`]
module"]
pub type DDRCTRL_DFITMG0 = crate::Reg<ddrctrl_dfitmg0::DDRCTRL_DFITMG0rs>;
#[doc = "DDRCTRL DFI timing register 0"]
pub mod ddrctrl_dfitmg0;
#[doc = "DDRCTRL_DFITMG1 (rw) register accessor: DDRCTRL DFI timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfitmg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfitmg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfitmg1`]
module"]
pub type DDRCTRL_DFITMG1 = crate::Reg<ddrctrl_dfitmg1::DDRCTRL_DFITMG1rs>;
#[doc = "DDRCTRL DFI timing register 1"]
pub mod ddrctrl_dfitmg1;
#[doc = "DDRCTRL_DFILPCFG0 (rw) register accessor: DDRCTRL low power configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfilpcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfilpcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfilpcfg0`]
module"]
pub type DDRCTRL_DFILPCFG0 = crate::Reg<ddrctrl_dfilpcfg0::DDRCTRL_DFILPCFG0rs>;
#[doc = "DDRCTRL low power configuration register 0"]
pub mod ddrctrl_dfilpcfg0;
#[doc = "DDRCTRL_DFIUPD0 (rw) register accessor: DDRCTRL DFI update register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiupd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiupd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfiupd0`]
module"]
pub type DDRCTRL_DFIUPD0 = crate::Reg<ddrctrl_dfiupd0::DDRCTRL_DFIUPD0rs>;
#[doc = "DDRCTRL DFI update register 0"]
pub mod ddrctrl_dfiupd0;
#[doc = "DDRCTRL_DFIUPD1 (rw) register accessor: DDRCTRL DFI update register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiupd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiupd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfiupd1`]
module"]
pub type DDRCTRL_DFIUPD1 = crate::Reg<ddrctrl_dfiupd1::DDRCTRL_DFIUPD1rs>;
#[doc = "DDRCTRL DFI update register 1"]
pub mod ddrctrl_dfiupd1;
#[doc = "DDRCTRL_DFIUPD2 (rw) register accessor: DDRCTRL DFI update register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiupd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiupd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfiupd2`]
module"]
pub type DDRCTRL_DFIUPD2 = crate::Reg<ddrctrl_dfiupd2::DDRCTRL_DFIUPD2rs>;
#[doc = "DDRCTRL DFI update register 2"]
pub mod ddrctrl_dfiupd2;
#[doc = "DDRCTRL_DFIMISC (rw) register accessor: DDRCTRL DFI miscellaneous control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfimisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfimisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfimisc`]
module"]
pub type DDRCTRL_DFIMISC = crate::Reg<ddrctrl_dfimisc::DDRCTRL_DFIMISCrs>;
#[doc = "DDRCTRL DFI miscellaneous control register"]
pub mod ddrctrl_dfimisc;
#[doc = "DDRCTRL_DFISTAT (r) register accessor: DDRCTRL DFI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfistat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfistat`]
module"]
pub type DDRCTRL_DFISTAT = crate::Reg<ddrctrl_dfistat::DDRCTRL_DFISTATrs>;
#[doc = "DDRCTRL DFI status register"]
pub mod ddrctrl_dfistat;
#[doc = "DDRCTRL_DFIPHYMSTR (rw) register accessor: DDRCTRL DFI PHY master register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiphymstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiphymstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dfiphymstr`]
module"]
pub type DDRCTRL_DFIPHYMSTR = crate::Reg<ddrctrl_dfiphymstr::DDRCTRL_DFIPHYMSTRrs>;
#[doc = "DDRCTRL DFI PHY master register"]
pub mod ddrctrl_dfiphymstr;
#[doc = "DDRCTRL_ADDRMAP1 (rw) register accessor: DDRCTRL address map register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap1`]
module"]
pub type DDRCTRL_ADDRMAP1 = crate::Reg<ddrctrl_addrmap1::DDRCTRL_ADDRMAP1rs>;
#[doc = "DDRCTRL address map register 1"]
pub mod ddrctrl_addrmap1;
#[doc = "DDRCTRL_ADDRMAP2 (rw) register accessor: DDRCTRL address map register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap2`]
module"]
pub type DDRCTRL_ADDRMAP2 = crate::Reg<ddrctrl_addrmap2::DDRCTRL_ADDRMAP2rs>;
#[doc = "DDRCTRL address map register 2"]
pub mod ddrctrl_addrmap2;
#[doc = "DDRCTRL_ADDRMAP3 (rw) register accessor: DDRCTRL address map register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap3`]
module"]
pub type DDRCTRL_ADDRMAP3 = crate::Reg<ddrctrl_addrmap3::DDRCTRL_ADDRMAP3rs>;
#[doc = "DDRCTRL address map register 3"]
pub mod ddrctrl_addrmap3;
#[doc = "DDRCTRL_ADDRMAP4 (rw) register accessor: DDRCTRL address map register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap4`]
module"]
pub type DDRCTRL_ADDRMAP4 = crate::Reg<ddrctrl_addrmap4::DDRCTRL_ADDRMAP4rs>;
#[doc = "DDRCTRL address map register 4"]
pub mod ddrctrl_addrmap4;
#[doc = "DDRCTRL_ADDRMAP5 (rw) register accessor: DDRCTRL address map register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap5`]
module"]
pub type DDRCTRL_ADDRMAP5 = crate::Reg<ddrctrl_addrmap5::DDRCTRL_ADDRMAP5rs>;
#[doc = "DDRCTRL address map register 5"]
pub mod ddrctrl_addrmap5;
#[doc = "DDRCTRL_ADDRMAP6 (rw) register accessor: DDRCTRL address register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap6`]
module"]
pub type DDRCTRL_ADDRMAP6 = crate::Reg<ddrctrl_addrmap6::DDRCTRL_ADDRMAP6rs>;
#[doc = "DDRCTRL address register 6"]
pub mod ddrctrl_addrmap6;
#[doc = "DDRCTRL_ADDRMAP9 (rw) register accessor: DDRCTRL address map register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap9`]
module"]
pub type DDRCTRL_ADDRMAP9 = crate::Reg<ddrctrl_addrmap9::DDRCTRL_ADDRMAP9rs>;
#[doc = "DDRCTRL address map register 9"]
pub mod ddrctrl_addrmap9;
#[doc = "DDRCTRL_ADDRMAP10 (rw) register accessor: DDRCTRL address map register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap10`]
module"]
pub type DDRCTRL_ADDRMAP10 = crate::Reg<ddrctrl_addrmap10::DDRCTRL_ADDRMAP10rs>;
#[doc = "DDRCTRL address map register 10"]
pub mod ddrctrl_addrmap10;
#[doc = "DDRCTRL_ADDRMAP11 (rw) register accessor: DDRCTRL address map register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_addrmap11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_addrmap11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_addrmap11`]
module"]
pub type DDRCTRL_ADDRMAP11 = crate::Reg<ddrctrl_addrmap11::DDRCTRL_ADDRMAP11rs>;
#[doc = "DDRCTRL address map register 11"]
pub mod ddrctrl_addrmap11;
#[doc = "DDRCTRL_ODTCFG (rw) register accessor: DDRCTRL ODT configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_odtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_odtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_odtcfg`]
module"]
pub type DDRCTRL_ODTCFG = crate::Reg<ddrctrl_odtcfg::DDRCTRL_ODTCFGrs>;
#[doc = "DDRCTRL ODT configuration register"]
pub mod ddrctrl_odtcfg;
#[doc = "DDRCTRL_ODTMAP (rw) register accessor: DDRCTRL ODT/Rank map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_odtmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_odtmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_odtmap`]
module"]
pub type DDRCTRL_ODTMAP = crate::Reg<ddrctrl_odtmap::DDRCTRL_ODTMAPrs>;
#[doc = "DDRCTRL ODT/Rank map register"]
pub mod ddrctrl_odtmap;
#[doc = "DDRCTRL_SCHED (rw) register accessor: DDRCTRL scheduler control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_sched::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_sched::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_sched`]
module"]
pub type DDRCTRL_SCHED = crate::Reg<ddrctrl_sched::DDRCTRL_SCHEDrs>;
#[doc = "DDRCTRL scheduler control register"]
pub mod ddrctrl_sched;
#[doc = "DDRCTRL_SCHED1 (rw) register accessor: DDRCTRL scheduler control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_sched1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_sched1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_sched1`]
module"]
pub type DDRCTRL_SCHED1 = crate::Reg<ddrctrl_sched1::DDRCTRL_SCHED1rs>;
#[doc = "DDRCTRL scheduler control register 1"]
pub mod ddrctrl_sched1;
#[doc = "DDRCTRL_PERFHPR1 (rw) register accessor: DDRCTRL high priority read CAM register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_perfhpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_perfhpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_perfhpr1`]
module"]
pub type DDRCTRL_PERFHPR1 = crate::Reg<ddrctrl_perfhpr1::DDRCTRL_PERFHPR1rs>;
#[doc = "DDRCTRL high priority read CAM register 1"]
pub mod ddrctrl_perfhpr1;
#[doc = "DDRCTRL_PERFLPR1 (rw) register accessor: DDRCTRL low priority read CAM register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_perflpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_perflpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_perflpr1`]
module"]
pub type DDRCTRL_PERFLPR1 = crate::Reg<ddrctrl_perflpr1::DDRCTRL_PERFLPR1rs>;
#[doc = "DDRCTRL low priority read CAM register 1"]
pub mod ddrctrl_perflpr1;
#[doc = "DDRCTRL_PERFWR1 (rw) register accessor: DDRCTRL write CAM register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_perfwr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_perfwr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_perfwr1`]
module"]
pub type DDRCTRL_PERFWR1 = crate::Reg<ddrctrl_perfwr1::DDRCTRL_PERFWR1rs>;
#[doc = "DDRCTRL write CAM register 1"]
pub mod ddrctrl_perfwr1;
#[doc = "DDRCTRL_DBG0 (rw) register accessor: DDRCTRL debug register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dbg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dbg0`]
module"]
pub type DDRCTRL_DBG0 = crate::Reg<ddrctrl_dbg0::DDRCTRL_DBG0rs>;
#[doc = "DDRCTRL debug register 0"]
pub mod ddrctrl_dbg0;
#[doc = "DDRCTRL_DBG1 (rw) register accessor: DDRCTRL debug register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dbg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dbg1`]
module"]
pub type DDRCTRL_DBG1 = crate::Reg<ddrctrl_dbg1::DDRCTRL_DBG1rs>;
#[doc = "DDRCTRL debug register 1"]
pub mod ddrctrl_dbg1;
#[doc = "DDRCTRL_DBGCAM (r) register accessor: DDRCTRL CAM debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbgcam::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dbgcam`]
module"]
pub type DDRCTRL_DBGCAM = crate::Reg<ddrctrl_dbgcam::DDRCTRL_DBGCAMrs>;
#[doc = "DDRCTRL CAM debug register"]
pub mod ddrctrl_dbgcam;
#[doc = "DDRCTRL_DBGCMD (rw) register accessor: DDRCTRL command debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbgcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dbgcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dbgcmd`]
module"]
pub type DDRCTRL_DBGCMD = crate::Reg<ddrctrl_dbgcmd::DDRCTRL_DBGCMDrs>;
#[doc = "DDRCTRL command debug register"]
pub mod ddrctrl_dbgcmd;
#[doc = "DDRCTRL_DBGSTAT (r) register accessor: DDRCTRL status debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbgstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_dbgstat`]
module"]
pub type DDRCTRL_DBGSTAT = crate::Reg<ddrctrl_dbgstat::DDRCTRL_DBGSTATrs>;
#[doc = "DDRCTRL status debug register"]
pub mod ddrctrl_dbgstat;
#[doc = "DDRCTRL_SWCTL (rw) register accessor: DDRCTRL software register programming control enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_swctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_swctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_swctl`]
module"]
pub type DDRCTRL_SWCTL = crate::Reg<ddrctrl_swctl::DDRCTRL_SWCTLrs>;
#[doc = "DDRCTRL software register programming control enable"]
pub mod ddrctrl_swctl;
#[doc = "DDRCTRL_SWSTAT (r) register accessor: DDRCTRL software register programming control status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_swstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_swstat`]
module"]
pub type DDRCTRL_SWSTAT = crate::Reg<ddrctrl_swstat::DDRCTRL_SWSTATrs>;
#[doc = "DDRCTRL software register programming control status"]
pub mod ddrctrl_swstat;
#[doc = "DDRCTRL_POISONCFG (rw) register accessor: AXI Poison configuration register common for all AXI ports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_poisoncfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_poisoncfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_poisoncfg`]
module"]
pub type DDRCTRL_POISONCFG = crate::Reg<ddrctrl_poisoncfg::DDRCTRL_POISONCFGrs>;
#[doc = "AXI Poison configuration register common for all AXI ports."]
pub mod ddrctrl_poisoncfg;
#[doc = "DDRCTRL_POISONSTAT (r) register accessor: DDRCTRL AXI Poison status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_poisonstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_poisonstat`]
module"]
pub type DDRCTRL_POISONSTAT = crate::Reg<ddrctrl_poisonstat::DDRCTRL_POISONSTATrs>;
#[doc = "DDRCTRL AXI Poison status register"]
pub mod ddrctrl_poisonstat;
#[doc = "DDRCTRL_PSTAT (r) register accessor: DDRCTRL port status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pstat`]
module"]
pub type DDRCTRL_PSTAT = crate::Reg<ddrctrl_pstat::DDRCTRL_PSTATrs>;
#[doc = "DDRCTRL port status register"]
pub mod ddrctrl_pstat;
#[doc = "DDRCTRL_PCCFG (rw) register accessor: DDRCTRL port common configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pccfg`]
module"]
pub type DDRCTRL_PCCFG = crate::Reg<ddrctrl_pccfg::DDRCTRL_PCCFGrs>;
#[doc = "DDRCTRL port common configuration register"]
pub mod ddrctrl_pccfg;
#[doc = "DDRCTRL_PCFGR_0 (rw) register accessor: DDRCTRL port 0 configuration read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgr_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgr_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgr_0`]
module"]
pub type DDRCTRL_PCFGR_0 = crate::Reg<ddrctrl_pcfgr_0::DDRCTRL_PCFGR_0rs>;
#[doc = "DDRCTRL port 0 configuration read register"]
pub mod ddrctrl_pcfgr_0;
#[doc = "DDRCTRL_PCFGW_0 (rw) register accessor: DDRCTRL port 0 configuration write register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgw_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgw_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgw_0`]
module"]
pub type DDRCTRL_PCFGW_0 = crate::Reg<ddrctrl_pcfgw_0::DDRCTRL_PCFGW_0rs>;
#[doc = "DDRCTRL port 0 configuration write register"]
pub mod ddrctrl_pcfgw_0;
#[doc = "DDRCTRL_PCTRL_0 (rw) register accessor: DDRCTRL port 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pctrl_0`]
module"]
pub type DDRCTRL_PCTRL_0 = crate::Reg<ddrctrl_pctrl_0::DDRCTRL_PCTRL_0rs>;
#[doc = "DDRCTRL port 0 control register"]
pub mod ddrctrl_pctrl_0;
#[doc = "DDRCTRL_PCFGQOS0_0 (rw) register accessor: DDRCTRL port 0 read Q0S configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgqos0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgqos0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgqos0_0`]
module"]
pub type DDRCTRL_PCFGQOS0_0 = crate::Reg<ddrctrl_pcfgqos0_0::DDRCTRL_PCFGQOS0_0rs>;
#[doc = "DDRCTRL port 0 read Q0S configuration register 0"]
pub mod ddrctrl_pcfgqos0_0;
#[doc = "DDRCTRL_PCFGQOS1_0 (rw) register accessor: DDRCTRL port 0 read Q0S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgqos1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgqos1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgqos1_0`]
module"]
pub type DDRCTRL_PCFGQOS1_0 = crate::Reg<ddrctrl_pcfgqos1_0::DDRCTRL_PCFGQOS1_0rs>;
#[doc = "DDRCTRL port 0 read Q0S configuration register 1"]
pub mod ddrctrl_pcfgqos1_0;
#[doc = "DDRCTRL_PCFGWQOS0_0 (rw) register accessor: DDRCTRL port 0 write Q0S configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgwqos0_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgwqos0_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgwqos0_0`]
module"]
pub type DDRCTRL_PCFGWQOS0_0 = crate::Reg<ddrctrl_pcfgwqos0_0::DDRCTRL_PCFGWQOS0_0rs>;
#[doc = "DDRCTRL port 0 write Q0S configuration register 0"]
pub mod ddrctrl_pcfgwqos0_0;
#[doc = "DDRCTRL_PCFGWQOS1_0 (rw) register accessor: DDRCTRL port 0 write Q0S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgwqos1_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgwqos1_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgwqos1_0`]
module"]
pub type DDRCTRL_PCFGWQOS1_0 = crate::Reg<ddrctrl_pcfgwqos1_0::DDRCTRL_PCFGWQOS1_0rs>;
#[doc = "DDRCTRL port 0 write Q0S configuration register 1"]
pub mod ddrctrl_pcfgwqos1_0;
#[doc = "DDRCTRL_PCFGR_1 (rw) register accessor: DDRCTRL port 1 configuration read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgr_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgr_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgr_1`]
module"]
pub type DDRCTRL_PCFGR_1 = crate::Reg<ddrctrl_pcfgr_1::DDRCTRL_PCFGR_1rs>;
#[doc = "DDRCTRL port 1 configuration read register"]
pub mod ddrctrl_pcfgr_1;
#[doc = "DDRCTRL_PCFGW_1 (rw) register accessor: DDRCTRL port 1 configuration write register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgw_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgw_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgw_1`]
module"]
pub type DDRCTRL_PCFGW_1 = crate::Reg<ddrctrl_pcfgw_1::DDRCTRL_PCFGW_1rs>;
#[doc = "DDRCTRL port 1 configuration write register"]
pub mod ddrctrl_pcfgw_1;
#[doc = "DDRCTRL_PCTRL_1 (rw) register accessor: DDRCTRL port 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pctrl_1`]
module"]
pub type DDRCTRL_PCTRL_1 = crate::Reg<ddrctrl_pctrl_1::DDRCTRL_PCTRL_1rs>;
#[doc = "DDRCTRL port 1 control register"]
pub mod ddrctrl_pctrl_1;
#[doc = "DDRCTRL_PCFGQOS0_1 (rw) register accessor: DDRCTRL port 1 read Q0S configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgqos0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgqos0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgqos0_1`]
module"]
pub type DDRCTRL_PCFGQOS0_1 = crate::Reg<ddrctrl_pcfgqos0_1::DDRCTRL_PCFGQOS0_1rs>;
#[doc = "DDRCTRL port 1 read Q0S configuration register 0"]
pub mod ddrctrl_pcfgqos0_1;
#[doc = "DDRCTRL_PCFGQOS1_1 (rw) register accessor: DDRCTRL port 1 read Q0S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgqos1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgqos1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgqos1_1`]
module"]
pub type DDRCTRL_PCFGQOS1_1 = crate::Reg<ddrctrl_pcfgqos1_1::DDRCTRL_PCFGQOS1_1rs>;
#[doc = "DDRCTRL port 1 read Q0S configuration register 1"]
pub mod ddrctrl_pcfgqos1_1;
#[doc = "DDRCTRL_PCFGWQOS0_1 (rw) register accessor: DDRCTRL port 1 write Q0S configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgwqos0_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgwqos0_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgwqos0_1`]
module"]
pub type DDRCTRL_PCFGWQOS0_1 = crate::Reg<ddrctrl_pcfgwqos0_1::DDRCTRL_PCFGWQOS0_1rs>;
#[doc = "DDRCTRL port 1 write Q0S configuration register 0"]
pub mod ddrctrl_pcfgwqos0_1;
#[doc = "DDRCTRL_PCFGWQOS1_1 (rw) register accessor: DDRCTRL port 1 write Q0S configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_pcfgwqos1_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_pcfgwqos1_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddrctrl_pcfgwqos1_1`]
module"]
pub type DDRCTRL_PCFGWQOS1_1 = crate::Reg<ddrctrl_pcfgwqos1_1::DDRCTRL_PCFGWQOS1_1rs>;
#[doc = "DDRCTRL port 1 write Q0S configuration register 1"]
pub mod ddrctrl_pcfgwqos1_1;
