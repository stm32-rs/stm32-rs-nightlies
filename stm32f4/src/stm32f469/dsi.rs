#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vr: VR,
    cr: CR,
    ccr: CCR,
    lvcidr: LVCIDR,
    lcolcr: LCOLCR,
    lpcr: LPCR,
    lpmcr: LPMCR,
    _reserved7: [u8; 0x10],
    pcr: PCR,
    gvcidr: GVCIDR,
    mcr: MCR,
    vmcr: VMCR,
    vpcr: VPCR,
    vccr: VCCR,
    vnpcr: VNPCR,
    vhsacr: VHSACR,
    vhbpcr: VHBPCR,
    vlcr: VLCR,
    vvsacr: VVSACR,
    vvbpcr: VVBPCR,
    vvfpcr: VVFPCR,
    vvacr: VVACR,
    lccr: LCCR,
    cmcr: CMCR,
    ghcr: GHCR,
    gpdr: GPDR,
    gpsr: GPSR,
    tccr1: TCCR1,
    tccr2: TCCR2,
    tccr3: TCCR3,
    tccr4: TCCR4,
    tccr5: TCCR5,
    tccr6: TCCR6,
    _reserved32: [u8; 0x04],
    clcr: CLCR,
    cltcr: CLTCR,
    dltcr: DLTCR,
    pctlr: PCTLR,
    pcconfr: PCCONFR,
    pucr: PUCR,
    pttcr: PTTCR,
    psr: PSR,
    _reserved40: [u8; 0x08],
    isr0: ISR0,
    isr1: ISR1,
    ier0: IER0,
    ier1: IER1,
    _reserved44: [u8; 0x0c],
    fir0: FIR0,
    fir1: FIR1,
    _reserved46: [u8; 0x20],
    vscr: VSCR,
    _reserved47: [u8; 0x08],
    lcvcidr: LCVCIDR,
    lcccr: LCCCR,
    _reserved49: [u8; 0x04],
    lpmccr: LPMCCR,
    _reserved50: [u8; 0x1c],
    vmccr: VMCCR,
    vpccr: VPCCR,
    vcccr: VCCCR,
    vnpccr: VNPCCR,
    vhsaccr: VHSACCR,
    vhbpccr: VHBPCCR,
    vlccr: VLCCR,
    vvsaccr: VVSACCR,
    vvbpccr: VVBPCCR,
    vvfpccr: VVFPCCR,
    vvaccr: VVACCR,
    _reserved61: [u8; 0x029c],
    wcfgr: WCFGR,
    wcr: WCR,
    wier: WIER,
    wisr: WISR,
    wifcr: WIFCR,
    _reserved66: [u8; 0x04],
    wpcr1: WPCR1,
    wpcr2: WPCR2,
    wpcr3: WPCR3,
    wpcr4: WPCR4,
    wpcr5: WPCR5,
    _reserved71: [u8; 0x04],
    wrpcr: WRPCR,
}
impl RegisterBlock {
    #[doc = "0x00 - DSI Host Version Register"]
    #[inline(always)]
    pub const fn vr(&self) -> &VR {
        &self.vr
    }
    #[doc = "0x04 - DSI Host Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - DSI HOST Clock Control Register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x0c - DSI Host LTDC VCID Register"]
    #[inline(always)]
    pub const fn lvcidr(&self) -> &LVCIDR {
        &self.lvcidr
    }
    #[doc = "0x10 - DSI Host LTDC Color Coding Register"]
    #[inline(always)]
    pub const fn lcolcr(&self) -> &LCOLCR {
        &self.lcolcr
    }
    #[doc = "0x14 - DSI Host LTDC Polarity Configuration Register"]
    #[inline(always)]
    pub const fn lpcr(&self) -> &LPCR {
        &self.lpcr
    }
    #[doc = "0x18 - DSI Host Low-Power Mode Configuration Register"]
    #[inline(always)]
    pub const fn lpmcr(&self) -> &LPMCR {
        &self.lpmcr
    }
    #[doc = "0x2c - DSI Host Protocol Configuration Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    #[doc = "0x30 - DSI Host Generic VCID Register"]
    #[inline(always)]
    pub const fn gvcidr(&self) -> &GVCIDR {
        &self.gvcidr
    }
    #[doc = "0x34 - DSI Host Mode Configuration Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x38 - DSI Host Video mode Configuration Register"]
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    #[doc = "0x3c - DSI Host Video Packet Configuration Register"]
    #[inline(always)]
    pub const fn vpcr(&self) -> &VPCR {
        &self.vpcr
    }
    #[doc = "0x40 - DSI Host Video Chunks Configuration Register"]
    #[inline(always)]
    pub const fn vccr(&self) -> &VCCR {
        &self.vccr
    }
    #[doc = "0x44 - DSI Host Video Null Packet Configuration Register"]
    #[inline(always)]
    pub const fn vnpcr(&self) -> &VNPCR {
        &self.vnpcr
    }
    #[doc = "0x48 - DSI Host Video HSA Configuration Register"]
    #[inline(always)]
    pub const fn vhsacr(&self) -> &VHSACR {
        &self.vhsacr
    }
    #[doc = "0x4c - DSI Host Video HBP Configuration Register"]
    #[inline(always)]
    pub const fn vhbpcr(&self) -> &VHBPCR {
        &self.vhbpcr
    }
    #[doc = "0x50 - DSI Host Video Line Configuration Register"]
    #[inline(always)]
    pub const fn vlcr(&self) -> &VLCR {
        &self.vlcr
    }
    #[doc = "0x54 - DSI Host Video VSA Configuration Register"]
    #[inline(always)]
    pub const fn vvsacr(&self) -> &VVSACR {
        &self.vvsacr
    }
    #[doc = "0x58 - DSI Host Video VBP Configuration Register"]
    #[inline(always)]
    pub const fn vvbpcr(&self) -> &VVBPCR {
        &self.vvbpcr
    }
    #[doc = "0x5c - DSI Host Video VFP Configuration Register"]
    #[inline(always)]
    pub const fn vvfpcr(&self) -> &VVFPCR {
        &self.vvfpcr
    }
    #[doc = "0x60 - DSI Host Video VA Configuration Register"]
    #[inline(always)]
    pub const fn vvacr(&self) -> &VVACR {
        &self.vvacr
    }
    #[doc = "0x64 - DSI Host LTDC Command Configuration Register"]
    #[inline(always)]
    pub const fn lccr(&self) -> &LCCR {
        &self.lccr
    }
    #[doc = "0x68 - DSI Host Command mode Configuration Register"]
    #[inline(always)]
    pub const fn cmcr(&self) -> &CMCR {
        &self.cmcr
    }
    #[doc = "0x6c - DSI Host Generic Header Configuration Register"]
    #[inline(always)]
    pub const fn ghcr(&self) -> &GHCR {
        &self.ghcr
    }
    #[doc = "0x70 - DSI Host Generic Payload Data Register"]
    #[inline(always)]
    pub const fn gpdr(&self) -> &GPDR {
        &self.gpdr
    }
    #[doc = "0x74 - DSI Host Generic Packet Status Register"]
    #[inline(always)]
    pub const fn gpsr(&self) -> &GPSR {
        &self.gpsr
    }
    #[doc = "0x78 - DSI Host Timeout Counter Configuration Register1"]
    #[inline(always)]
    pub const fn tccr1(&self) -> &TCCR1 {
        &self.tccr1
    }
    #[doc = "0x7c - DSI Host Timeout Counter Configuration Register2"]
    #[inline(always)]
    pub const fn tccr2(&self) -> &TCCR2 {
        &self.tccr2
    }
    #[doc = "0x80 - DSI Host Timeout Counter Configuration Register3"]
    #[inline(always)]
    pub const fn tccr3(&self) -> &TCCR3 {
        &self.tccr3
    }
    #[doc = "0x84 - DSI Host Timeout Counter Configuration Register4"]
    #[inline(always)]
    pub const fn tccr4(&self) -> &TCCR4 {
        &self.tccr4
    }
    #[doc = "0x88 - DSI Host Timeout Counter Configuration Register5"]
    #[inline(always)]
    pub const fn tccr5(&self) -> &TCCR5 {
        &self.tccr5
    }
    #[doc = "0x8c - DSI Host Timeout Counter Configuration Register6"]
    #[inline(always)]
    pub const fn tccr6(&self) -> &TCCR6 {
        &self.tccr6
    }
    #[doc = "0x94 - DSI Host Clock Lane Configuration Register"]
    #[inline(always)]
    pub const fn clcr(&self) -> &CLCR {
        &self.clcr
    }
    #[doc = "0x98 - DSI Host Clock Lane Timer Configuration Register"]
    #[inline(always)]
    pub const fn cltcr(&self) -> &CLTCR {
        &self.cltcr
    }
    #[doc = "0x9c - DSI Host Data Lane Timer Configuration Register"]
    #[inline(always)]
    pub const fn dltcr(&self) -> &DLTCR {
        &self.dltcr
    }
    #[doc = "0xa0 - DSI Host PHY Control Register"]
    #[inline(always)]
    pub const fn pctlr(&self) -> &PCTLR {
        &self.pctlr
    }
    #[doc = "0xa4 - DSI Host PHY Configuration Register"]
    #[inline(always)]
    pub const fn pcconfr(&self) -> &PCCONFR {
        &self.pcconfr
    }
    #[doc = "0xa8 - DSI Host PHY ULPS Control Register"]
    #[inline(always)]
    pub const fn pucr(&self) -> &PUCR {
        &self.pucr
    }
    #[doc = "0xac - DSI Host PHY TX Triggers Configuration Register"]
    #[inline(always)]
    pub const fn pttcr(&self) -> &PTTCR {
        &self.pttcr
    }
    #[doc = "0xb0 - DSI Host PHY Status Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0xbc - DSI Host Interrupt &amp; Status Register 0"]
    #[inline(always)]
    pub const fn isr0(&self) -> &ISR0 {
        &self.isr0
    }
    #[doc = "0xc0 - DSI Host Interrupt &amp; Status Register 1"]
    #[inline(always)]
    pub const fn isr1(&self) -> &ISR1 {
        &self.isr1
    }
    #[doc = "0xc4 - DSI Host Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    #[doc = "0xc8 - DSI Host Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    #[doc = "0xd8 - DSI Host Force Interrupt Register 0"]
    #[inline(always)]
    pub const fn fir0(&self) -> &FIR0 {
        &self.fir0
    }
    #[doc = "0xdc - DSI Host Force Interrupt Register 1"]
    #[inline(always)]
    pub const fn fir1(&self) -> &FIR1 {
        &self.fir1
    }
    #[doc = "0x100 - DSI Host Video Shadow Control Register"]
    #[inline(always)]
    pub const fn vscr(&self) -> &VSCR {
        &self.vscr
    }
    #[doc = "0x10c - DSI Host LTDC Current VCID Register"]
    #[inline(always)]
    pub const fn lcvcidr(&self) -> &LCVCIDR {
        &self.lcvcidr
    }
    #[doc = "0x110 - DSI Host LTDC Current Color Coding Register"]
    #[inline(always)]
    pub const fn lcccr(&self) -> &LCCCR {
        &self.lcccr
    }
    #[doc = "0x118 - DSI Host Low-power Mode Current Configuration Register"]
    #[inline(always)]
    pub const fn lpmccr(&self) -> &LPMCCR {
        &self.lpmccr
    }
    #[doc = "0x138 - DSI Host Video mode Current Configuration Register"]
    #[inline(always)]
    pub const fn vmccr(&self) -> &VMCCR {
        &self.vmccr
    }
    #[doc = "0x13c - DSI Host Video Packet Current Configuration Register"]
    #[inline(always)]
    pub const fn vpccr(&self) -> &VPCCR {
        &self.vpccr
    }
    #[doc = "0x140 - DSI Host Video Chunks Current Configuration Register"]
    #[inline(always)]
    pub const fn vcccr(&self) -> &VCCCR {
        &self.vcccr
    }
    #[doc = "0x144 - DSI Host Video Null Packet Current Configuration Register"]
    #[inline(always)]
    pub const fn vnpccr(&self) -> &VNPCCR {
        &self.vnpccr
    }
    #[doc = "0x148 - DSI Host Video HSA Current Configuration Register"]
    #[inline(always)]
    pub const fn vhsaccr(&self) -> &VHSACCR {
        &self.vhsaccr
    }
    #[doc = "0x14c - DSI Host Video HBP Current Configuration Register"]
    #[inline(always)]
    pub const fn vhbpccr(&self) -> &VHBPCCR {
        &self.vhbpccr
    }
    #[doc = "0x150 - DSI Host Video Line Current Configuration Register"]
    #[inline(always)]
    pub const fn vlccr(&self) -> &VLCCR {
        &self.vlccr
    }
    #[doc = "0x154 - DSI Host Video VSA Current Configuration Register"]
    #[inline(always)]
    pub const fn vvsaccr(&self) -> &VVSACCR {
        &self.vvsaccr
    }
    #[doc = "0x158 - DSI Host Video VBP Current Configuration Register"]
    #[inline(always)]
    pub const fn vvbpccr(&self) -> &VVBPCCR {
        &self.vvbpccr
    }
    #[doc = "0x15c - DSI Host Video VFP Current Configuration Register"]
    #[inline(always)]
    pub const fn vvfpccr(&self) -> &VVFPCCR {
        &self.vvfpccr
    }
    #[doc = "0x160 - DSI Host Video VA Current Configuration Register"]
    #[inline(always)]
    pub const fn vvaccr(&self) -> &VVACCR {
        &self.vvaccr
    }
    #[doc = "0x400 - DSI Wrapper Configuration Register"]
    #[inline(always)]
    pub const fn wcfgr(&self) -> &WCFGR {
        &self.wcfgr
    }
    #[doc = "0x404 - DSI Wrapper Control Register"]
    #[inline(always)]
    pub const fn wcr(&self) -> &WCR {
        &self.wcr
    }
    #[doc = "0x408 - DSI Wrapper Interrupt Enable Register"]
    #[inline(always)]
    pub const fn wier(&self) -> &WIER {
        &self.wier
    }
    #[doc = "0x40c - DSI Wrapper Interrupt &amp; Status Register"]
    #[inline(always)]
    pub const fn wisr(&self) -> &WISR {
        &self.wisr
    }
    #[doc = "0x410 - DSI Wrapper Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn wifcr(&self) -> &WIFCR {
        &self.wifcr
    }
    #[doc = "0x418 - DSI Wrapper PHY Configuration Register 1"]
    #[inline(always)]
    pub const fn wpcr1(&self) -> &WPCR1 {
        &self.wpcr1
    }
    #[doc = "0x41c - DSI Wrapper PHY Configuration Register 2"]
    #[inline(always)]
    pub const fn wpcr2(&self) -> &WPCR2 {
        &self.wpcr2
    }
    #[doc = "0x420 - DSI Wrapper PHY Configuration Register 3"]
    #[inline(always)]
    pub const fn wpcr3(&self) -> &WPCR3 {
        &self.wpcr3
    }
    #[doc = "0x424 - DSI_WPCR4"]
    #[inline(always)]
    pub const fn wpcr4(&self) -> &WPCR4 {
        &self.wpcr4
    }
    #[doc = "0x428 - DSI Wrapper PHY Configuration Register 5"]
    #[inline(always)]
    pub const fn wpcr5(&self) -> &WPCR5 {
        &self.wpcr5
    }
    #[doc = "0x430 - DSI Wrapper Regulator and PLL Control Register"]
    #[inline(always)]
    pub const fn wrpcr(&self) -> &WRPCR {
        &self.wrpcr
    }
}
#[doc = "VR (rw) register accessor: DSI Host Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vr`]
module"]
pub type VR = crate::Reg<vr::VRrs>;
#[doc = "DSI Host Version Register"]
pub mod vr;
#[doc = "CR (rw) register accessor: DSI Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DSI Host Control Register"]
pub mod cr;
#[doc = "CCR (rw) register accessor: DSI HOST Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "DSI HOST Clock Control Register"]
pub mod ccr;
#[doc = "LVCIDR (rw) register accessor: DSI Host LTDC VCID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvcidr`]
module"]
pub type LVCIDR = crate::Reg<lvcidr::LVCIDRrs>;
#[doc = "DSI Host LTDC VCID Register"]
pub mod lvcidr;
#[doc = "LCOLCR (rw) register accessor: DSI Host LTDC Color Coding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcolcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcolcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcolcr`]
module"]
pub type LCOLCR = crate::Reg<lcolcr::LCOLCRrs>;
#[doc = "DSI Host LTDC Color Coding Register"]
pub mod lcolcr;
#[doc = "LPCR (rw) register accessor: DSI Host LTDC Polarity Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcr`]
module"]
pub type LPCR = crate::Reg<lpcr::LPCRrs>;
#[doc = "DSI Host LTDC Polarity Configuration Register"]
pub mod lpcr;
#[doc = "LPMCR (rw) register accessor: DSI Host Low-Power Mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcr`]
module"]
pub type LPMCR = crate::Reg<lpmcr::LPMCRrs>;
#[doc = "DSI Host Low-Power Mode Configuration Register"]
pub mod lpmcr;
#[doc = "PCR (rw) register accessor: DSI Host Protocol Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCRrs>;
#[doc = "DSI Host Protocol Configuration Register"]
pub mod pcr;
#[doc = "GVCIDR (rw) register accessor: DSI Host Generic VCID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gvcidr`]
module"]
pub type GVCIDR = crate::Reg<gvcidr::GVCIDRrs>;
#[doc = "DSI Host Generic VCID Register"]
pub mod gvcidr;
#[doc = "MCR (rw) register accessor: DSI Host Mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCRrs>;
#[doc = "DSI Host Mode Configuration Register"]
pub mod mcr;
#[doc = "VMCR (rw) register accessor: DSI Host Video mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmcr`]
module"]
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
#[doc = "DSI Host Video mode Configuration Register"]
pub mod vmcr;
#[doc = "VPCR (rw) register accessor: DSI Host Video Packet Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpcr`]
module"]
pub type VPCR = crate::Reg<vpcr::VPCRrs>;
#[doc = "DSI Host Video Packet Configuration Register"]
pub mod vpcr;
#[doc = "VCCR (rw) register accessor: DSI Host Video Chunks Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vccr`]
module"]
pub type VCCR = crate::Reg<vccr::VCCRrs>;
#[doc = "DSI Host Video Chunks Configuration Register"]
pub mod vccr;
#[doc = "VNPCR (rw) register accessor: DSI Host Video Null Packet Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vnpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vnpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vnpcr`]
module"]
pub type VNPCR = crate::Reg<vnpcr::VNPCRrs>;
#[doc = "DSI Host Video Null Packet Configuration Register"]
pub mod vnpcr;
#[doc = "VHSACR (rw) register accessor: DSI Host Video HSA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhsacr`]
module"]
pub type VHSACR = crate::Reg<vhsacr::VHSACRrs>;
#[doc = "DSI Host Video HSA Configuration Register"]
pub mod vhsacr;
#[doc = "VHBPCR (rw) register accessor: DSI Host Video HBP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhbpcr`]
module"]
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCRrs>;
#[doc = "DSI Host Video HBP Configuration Register"]
pub mod vhbpcr;
#[doc = "VLCR (rw) register accessor: DSI Host Video Line Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlcr`]
module"]
pub type VLCR = crate::Reg<vlcr::VLCRrs>;
#[doc = "DSI Host Video Line Configuration Register"]
pub mod vlcr;
#[doc = "VVSACR (rw) register accessor: DSI Host Video VSA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvsacr`]
module"]
pub type VVSACR = crate::Reg<vvsacr::VVSACRrs>;
#[doc = "DSI Host Video VSA Configuration Register"]
pub mod vvsacr;
#[doc = "VVBPCR (rw) register accessor: DSI Host Video VBP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvbpcr`]
module"]
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCRrs>;
#[doc = "DSI Host Video VBP Configuration Register"]
pub mod vvbpcr;
#[doc = "VVFPCR (rw) register accessor: DSI Host Video VFP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvfpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvfpcr`]
module"]
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCRrs>;
#[doc = "DSI Host Video VFP Configuration Register"]
pub mod vvfpcr;
#[doc = "VVACR (rw) register accessor: DSI Host Video VA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvacr`]
module"]
pub type VVACR = crate::Reg<vvacr::VVACRrs>;
#[doc = "DSI Host Video VA Configuration Register"]
pub mod vvacr;
#[doc = "LCCR (rw) register accessor: DSI Host LTDC Command Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lccr`]
module"]
pub type LCCR = crate::Reg<lccr::LCCRrs>;
#[doc = "DSI Host LTDC Command Configuration Register"]
pub mod lccr;
#[doc = "CMCR (rw) register accessor: DSI Host Command mode Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmcr`]
module"]
pub type CMCR = crate::Reg<cmcr::CMCRrs>;
#[doc = "DSI Host Command mode Configuration Register"]
pub mod cmcr;
#[doc = "GHCR (rw) register accessor: DSI Host Generic Header Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ghcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghcr`]
module"]
pub type GHCR = crate::Reg<ghcr::GHCRrs>;
#[doc = "DSI Host Generic Header Configuration Register"]
pub mod ghcr;
#[doc = "GPDR (rw) register accessor: DSI Host Generic Payload Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdr`]
module"]
pub type GPDR = crate::Reg<gpdr::GPDRrs>;
#[doc = "DSI Host Generic Payload Data Register"]
pub mod gpdr;
#[doc = "GPSR (rw) register accessor: DSI Host Generic Packet Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpsr`]
module"]
pub type GPSR = crate::Reg<gpsr::GPSRrs>;
#[doc = "DSI Host Generic Packet Status Register"]
pub mod gpsr;
#[doc = "TCCR1 (rw) register accessor: DSI Host Timeout Counter Configuration Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1`]
module"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1rs>;
#[doc = "DSI Host Timeout Counter Configuration Register1"]
pub mod tccr1;
#[doc = "TCCR2 (rw) register accessor: DSI Host Timeout Counter Configuration Register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr2`]
module"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2rs>;
#[doc = "DSI Host Timeout Counter Configuration Register2"]
pub mod tccr2;
#[doc = "TCCR3 (rw) register accessor: DSI Host Timeout Counter Configuration Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr3`]
module"]
pub type TCCR3 = crate::Reg<tccr3::TCCR3rs>;
#[doc = "DSI Host Timeout Counter Configuration Register3"]
pub mod tccr3;
#[doc = "TCCR4 (rw) register accessor: DSI Host Timeout Counter Configuration Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4`]
module"]
pub type TCCR4 = crate::Reg<tccr4::TCCR4rs>;
#[doc = "DSI Host Timeout Counter Configuration Register4"]
pub mod tccr4;
#[doc = "TCCR5 (rw) register accessor: DSI Host Timeout Counter Configuration Register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr5`]
module"]
pub type TCCR5 = crate::Reg<tccr5::TCCR5rs>;
#[doc = "DSI Host Timeout Counter Configuration Register5"]
pub mod tccr5;
#[doc = "TCCR6 (rw) register accessor: DSI Host Timeout Counter Configuration Register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr6`]
module"]
pub type TCCR6 = crate::Reg<tccr6::TCCR6rs>;
#[doc = "DSI Host Timeout Counter Configuration Register6"]
pub mod tccr6;
#[doc = "CLCR (rw) register accessor: DSI Host Clock Lane Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clcr`]
module"]
pub type CLCR = crate::Reg<clcr::CLCRrs>;
#[doc = "DSI Host Clock Lane Configuration Register"]
pub mod clcr;
#[doc = "CLTCR (rw) register accessor: DSI Host Clock Lane Timer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cltcr`]
module"]
pub type CLTCR = crate::Reg<cltcr::CLTCRrs>;
#[doc = "DSI Host Clock Lane Timer Configuration Register"]
pub mod cltcr;
#[doc = "DLTCR (rw) register accessor: DSI Host Data Lane Timer Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dltcr`]
module"]
pub type DLTCR = crate::Reg<dltcr::DLTCRrs>;
#[doc = "DSI Host Data Lane Timer Configuration Register"]
pub mod dltcr;
#[doc = "PCTLR (rw) register accessor: DSI Host PHY Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pctlr`]
module"]
pub type PCTLR = crate::Reg<pctlr::PCTLRrs>;
#[doc = "DSI Host PHY Control Register"]
pub mod pctlr;
#[doc = "PCCONFR (rw) register accessor: DSI Host PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcconfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcconfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcconfr`]
module"]
pub type PCCONFR = crate::Reg<pcconfr::PCCONFRrs>;
#[doc = "DSI Host PHY Configuration Register"]
pub mod pcconfr;
#[doc = "PUCR (rw) register accessor: DSI Host PHY ULPS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucr`]
module"]
pub type PUCR = crate::Reg<pucr::PUCRrs>;
#[doc = "DSI Host PHY ULPS Control Register"]
pub mod pucr;
#[doc = "PTTCR (rw) register accessor: DSI Host PHY TX Triggers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pttcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pttcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pttcr`]
module"]
pub type PTTCR = crate::Reg<pttcr::PTTCRrs>;
#[doc = "DSI Host PHY TX Triggers Configuration Register"]
pub mod pttcr;
#[doc = "PSR (rw) register accessor: DSI Host PHY Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSRrs>;
#[doc = "DSI Host PHY Status Register"]
pub mod psr;
#[doc = "ISR0 (r) register accessor: DSI Host Interrupt &amp; Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr0`]
module"]
pub type ISR0 = crate::Reg<isr0::ISR0rs>;
#[doc = "DSI Host Interrupt &amp; Status Register 0"]
pub mod isr0;
#[doc = "ISR1 (r) register accessor: DSI Host Interrupt &amp; Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`]
module"]
pub type ISR1 = crate::Reg<isr1::ISR1rs>;
#[doc = "DSI Host Interrupt &amp; Status Register 1"]
pub mod isr1;
#[doc = "IER0 (rw) register accessor: DSI Host Interrupt Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier0`]
module"]
pub type IER0 = crate::Reg<ier0::IER0rs>;
#[doc = "DSI Host Interrupt Enable Register 0"]
pub mod ier0;
#[doc = "IER1 (rw) register accessor: DSI Host Interrupt Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
pub type IER1 = crate::Reg<ier1::IER1rs>;
#[doc = "DSI Host Interrupt Enable Register 1"]
pub mod ier1;
#[doc = "FIR0 (rw) register accessor: DSI Host Force Interrupt Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fir0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fir0`]
module"]
pub type FIR0 = crate::Reg<fir0::FIR0rs>;
#[doc = "DSI Host Force Interrupt Register 0"]
pub mod fir0;
#[doc = "FIR1 (rw) register accessor: DSI Host Force Interrupt Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fir1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fir1`]
module"]
pub type FIR1 = crate::Reg<fir1::FIR1rs>;
#[doc = "DSI Host Force Interrupt Register 1"]
pub mod fir1;
#[doc = "VSCR (rw) register accessor: DSI Host Video Shadow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vscr`]
module"]
pub type VSCR = crate::Reg<vscr::VSCRrs>;
#[doc = "DSI Host Video Shadow Control Register"]
pub mod vscr;
#[doc = "LCVCIDR (rw) register accessor: DSI Host LTDC Current VCID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcvcidr`]
module"]
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDRrs>;
#[doc = "DSI Host LTDC Current VCID Register"]
pub mod lcvcidr;
#[doc = "LCCCR (rw) register accessor: DSI Host LTDC Current Color Coding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcccr`]
module"]
pub type LCCCR = crate::Reg<lcccr::LCCCRrs>;
#[doc = "DSI Host LTDC Current Color Coding Register"]
pub mod lcccr;
#[doc = "LPMCCR (rw) register accessor: DSI Host Low-power Mode Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmccr`]
module"]
pub type LPMCCR = crate::Reg<lpmccr::LPMCCRrs>;
#[doc = "DSI Host Low-power Mode Current Configuration Register"]
pub mod lpmccr;
#[doc = "VMCCR (rw) register accessor: DSI Host Video mode Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmccr`]
module"]
pub type VMCCR = crate::Reg<vmccr::VMCCRrs>;
#[doc = "DSI Host Video mode Current Configuration Register"]
pub mod vmccr;
#[doc = "VPCCR (rw) register accessor: DSI Host Video Packet Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpccr`]
module"]
pub type VPCCR = crate::Reg<vpccr::VPCCRrs>;
#[doc = "DSI Host Video Packet Current Configuration Register"]
pub mod vpccr;
#[doc = "VCCCR (rw) register accessor: DSI Host Video Chunks Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vcccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vcccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vcccr`]
module"]
pub type VCCCR = crate::Reg<vcccr::VCCCRrs>;
#[doc = "DSI Host Video Chunks Current Configuration Register"]
pub mod vcccr;
#[doc = "VNPCCR (rw) register accessor: DSI Host Video Null Packet Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vnpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vnpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vnpccr`]
module"]
pub type VNPCCR = crate::Reg<vnpccr::VNPCCRrs>;
#[doc = "DSI Host Video Null Packet Current Configuration Register"]
pub mod vnpccr;
#[doc = "VHSACCR (rw) register accessor: DSI Host Video HSA Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhsaccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhsaccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhsaccr`]
module"]
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCRrs>;
#[doc = "DSI Host Video HSA Current Configuration Register"]
pub mod vhsaccr;
#[doc = "VHBPCCR (rw) register accessor: DSI Host Video HBP Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhbpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhbpccr`]
module"]
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCRrs>;
#[doc = "DSI Host Video HBP Current Configuration Register"]
pub mod vhbpccr;
#[doc = "VLCCR (rw) register accessor: DSI Host Video Line Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlccr`]
module"]
pub type VLCCR = crate::Reg<vlccr::VLCCRrs>;
#[doc = "DSI Host Video Line Current Configuration Register"]
pub mod vlccr;
#[doc = "VVSACCR (rw) register accessor: DSI Host Video VSA Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsaccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvsaccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvsaccr`]
module"]
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCRrs>;
#[doc = "DSI Host Video VSA Current Configuration Register"]
pub mod vvsaccr;
#[doc = "VVBPCCR (rw) register accessor: DSI Host Video VBP Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvbpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvbpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvbpccr`]
module"]
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCRrs>;
#[doc = "DSI Host Video VBP Current Configuration Register"]
pub mod vvbpccr;
#[doc = "VVFPCCR (rw) register accessor: DSI Host Video VFP Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvfpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvfpccr`]
module"]
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCRrs>;
#[doc = "DSI Host Video VFP Current Configuration Register"]
pub mod vvfpccr;
#[doc = "VVACCR (rw) register accessor: DSI Host Video VA Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvaccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvaccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvaccr`]
module"]
pub type VVACCR = crate::Reg<vvaccr::VVACCRrs>;
#[doc = "DSI Host Video VA Current Configuration Register"]
pub mod vvaccr;
#[doc = "WCFGR (rw) register accessor: DSI Wrapper Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcfgr`]
module"]
pub type WCFGR = crate::Reg<wcfgr::WCFGRrs>;
#[doc = "DSI Wrapper Configuration Register"]
pub mod wcfgr;
#[doc = "WCR (rw) register accessor: DSI Wrapper Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`]
module"]
pub type WCR = crate::Reg<wcr::WCRrs>;
#[doc = "DSI Wrapper Control Register"]
pub mod wcr;
#[doc = "WIER (rw) register accessor: DSI Wrapper Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wier`]
module"]
pub type WIER = crate::Reg<wier::WIERrs>;
#[doc = "DSI Wrapper Interrupt Enable Register"]
pub mod wier;
#[doc = "WISR (r) register accessor: DSI Wrapper Interrupt &amp; Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wisr`]
module"]
pub type WISR = crate::Reg<wisr::WISRrs>;
#[doc = "DSI Wrapper Interrupt &amp; Status Register"]
pub mod wisr;
#[doc = "WIFCR (rw) register accessor: DSI Wrapper Interrupt Flag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifcr`]
module"]
pub type WIFCR = crate::Reg<wifcr::WIFCRrs>;
#[doc = "DSI Wrapper Interrupt Flag Clear Register"]
pub mod wifcr;
#[doc = "WPCR1 (rw) register accessor: DSI Wrapper PHY Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr1`]
module"]
pub type WPCR1 = crate::Reg<wpcr1::WPCR1rs>;
#[doc = "DSI Wrapper PHY Configuration Register 1"]
pub mod wpcr1;
#[doc = "WPCR2 (rw) register accessor: DSI Wrapper PHY Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr2`]
module"]
pub type WPCR2 = crate::Reg<wpcr2::WPCR2rs>;
#[doc = "DSI Wrapper PHY Configuration Register 2"]
pub mod wpcr2;
#[doc = "WPCR3 (rw) register accessor: DSI Wrapper PHY Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr3`]
module"]
pub type WPCR3 = crate::Reg<wpcr3::WPCR3rs>;
#[doc = "DSI Wrapper PHY Configuration Register 3"]
pub mod wpcr3;
#[doc = "WPCR4 (rw) register accessor: DSI_WPCR4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr4`]
module"]
pub type WPCR4 = crate::Reg<wpcr4::WPCR4rs>;
#[doc = "DSI_WPCR4"]
pub mod wpcr4;
#[doc = "WPCR5 (rw) register accessor: DSI Wrapper PHY Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr5`]
module"]
pub type WPCR5 = crate::Reg<wpcr5::WPCR5rs>;
#[doc = "DSI Wrapper PHY Configuration Register 5"]
pub mod wpcr5;
#[doc = "WRPCR (rw) register accessor: DSI Wrapper Regulator and PLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpcr`]
module"]
pub type WRPCR = crate::Reg<wrpcr::WRPCRrs>;
#[doc = "DSI Wrapper Regulator and PLL Control Register"]
pub mod wrpcr;
