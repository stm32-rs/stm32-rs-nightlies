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
    tccr0: TCCR0,
    tccr1: TCCR1,
    tccr2: TCCR2,
    tccr3: TCCR3,
    tccr4: TCCR4,
    tccr5: TCCR5,
    _reserved32: [u8; 0x04],
    clcr: CLCR,
    cltcr: CLTCR,
    dltcr: DLTCR,
    pctlr: PCTLR,
    pconfr: PCONFR,
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
    _reserved46: [u8; 0x14],
    dltrcr: DLTRCR,
    _reserved47: [u8; 0x08],
    vscr: VSCR,
    _reserved48: [u8; 0x08],
    lcvcidr: LCVCIDR,
    lcccr: LCCCR,
    _reserved50: [u8; 0x04],
    lpmccr: LPMCCR,
    _reserved51: [u8; 0x1c],
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
    _reserved62: [u8; 0x029c],
    wcfgr: WCFGR,
    wcr: WCR,
    wier: WIER,
    wisr: WISR,
    wifcr: WIFCR,
    _reserved67: [u8; 0x04],
    wpcr0: WPCR0,
    wpcr1: WPCR1,
    _reserved69: [u8; 0x10],
    wrpcr: WRPCR,
    _reserved70: [u8; 0x03bc],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    #[inline(always)]
    pub const fn vr(&self) -> &VR {
        &self.vr
    }
    #[doc = "0x04 - DSI Host control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - DSI Host clock control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    #[inline(always)]
    pub const fn lvcidr(&self) -> &LVCIDR {
        &self.lvcidr
    }
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    #[inline(always)]
    pub const fn lcolcr(&self) -> &LCOLCR {
        &self.lcolcr
    }
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    #[inline(always)]
    pub const fn lpcr(&self) -> &LPCR {
        &self.lpcr
    }
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    #[inline(always)]
    pub const fn lpmcr(&self) -> &LPMCR {
        &self.lpmcr
    }
    #[doc = "0x2c - DSI Host protocol configuration register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    #[doc = "0x30 - DSI Host generic VCID register"]
    #[inline(always)]
    pub const fn gvcidr(&self) -> &GVCIDR {
        &self.gvcidr
    }
    #[doc = "0x34 - DSI Host mode configuration register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x38 - DSI Host video mode configuration register"]
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    #[doc = "0x3c - DSI Host video packet configuration register"]
    #[inline(always)]
    pub const fn vpcr(&self) -> &VPCR {
        &self.vpcr
    }
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    #[inline(always)]
    pub const fn vccr(&self) -> &VCCR {
        &self.vccr
    }
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    #[inline(always)]
    pub const fn vnpcr(&self) -> &VNPCR {
        &self.vnpcr
    }
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    #[inline(always)]
    pub const fn vhsacr(&self) -> &VHSACR {
        &self.vhsacr
    }
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    #[inline(always)]
    pub const fn vhbpcr(&self) -> &VHBPCR {
        &self.vhbpcr
    }
    #[doc = "0x50 - DSI Host video line configuration register"]
    #[inline(always)]
    pub const fn vlcr(&self) -> &VLCR {
        &self.vlcr
    }
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    #[inline(always)]
    pub const fn vvsacr(&self) -> &VVSACR {
        &self.vvsacr
    }
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    #[inline(always)]
    pub const fn vvbpcr(&self) -> &VVBPCR {
        &self.vvbpcr
    }
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    #[inline(always)]
    pub const fn vvfpcr(&self) -> &VVFPCR {
        &self.vvfpcr
    }
    #[doc = "0x60 - DSI Host video VA configuration register"]
    #[inline(always)]
    pub const fn vvacr(&self) -> &VVACR {
        &self.vvacr
    }
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    #[inline(always)]
    pub const fn lccr(&self) -> &LCCR {
        &self.lccr
    }
    #[doc = "0x68 - DSI Host command mode configuration register"]
    #[inline(always)]
    pub const fn cmcr(&self) -> &CMCR {
        &self.cmcr
    }
    #[doc = "0x6c - DSI Host generic header configuration register"]
    #[inline(always)]
    pub const fn ghcr(&self) -> &GHCR {
        &self.ghcr
    }
    #[doc = "0x70 - DSI Host generic payload data register"]
    #[inline(always)]
    pub const fn gpdr(&self) -> &GPDR {
        &self.gpdr
    }
    #[doc = "0x74 - DSI Host generic packet status register"]
    #[inline(always)]
    pub const fn gpsr(&self) -> &GPSR {
        &self.gpsr
    }
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    #[inline(always)]
    pub const fn tccr0(&self) -> &TCCR0 {
        &self.tccr0
    }
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    #[inline(always)]
    pub const fn tccr1(&self) -> &TCCR1 {
        &self.tccr1
    }
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    #[inline(always)]
    pub const fn tccr2(&self) -> &TCCR2 {
        &self.tccr2
    }
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    #[inline(always)]
    pub const fn tccr3(&self) -> &TCCR3 {
        &self.tccr3
    }
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    #[inline(always)]
    pub const fn tccr4(&self) -> &TCCR4 {
        &self.tccr4
    }
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    #[inline(always)]
    pub const fn tccr5(&self) -> &TCCR5 {
        &self.tccr5
    }
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    #[inline(always)]
    pub const fn clcr(&self) -> &CLCR {
        &self.clcr
    }
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    #[inline(always)]
    pub const fn cltcr(&self) -> &CLTCR {
        &self.cltcr
    }
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    #[inline(always)]
    pub const fn dltcr(&self) -> &DLTCR {
        &self.dltcr
    }
    #[doc = "0xa0 - DSI Host PHY control register"]
    #[inline(always)]
    pub const fn pctlr(&self) -> &PCTLR {
        &self.pctlr
    }
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    #[inline(always)]
    pub const fn pconfr(&self) -> &PCONFR {
        &self.pconfr
    }
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    #[inline(always)]
    pub const fn pucr(&self) -> &PUCR {
        &self.pucr
    }
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    #[inline(always)]
    pub const fn pttcr(&self) -> &PTTCR {
        &self.pttcr
    }
    #[doc = "0xb0 - DSI Host PHY status register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    #[inline(always)]
    pub const fn isr0(&self) -> &ISR0 {
        &self.isr0
    }
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    #[inline(always)]
    pub const fn isr1(&self) -> &ISR1 {
        &self.isr1
    }
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    #[inline(always)]
    pub const fn fir0(&self) -> &FIR0 {
        &self.fir0
    }
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    #[inline(always)]
    pub const fn fir1(&self) -> &FIR1 {
        &self.fir1
    }
    #[doc = "0xf4 - DSI Host data lane timer read configuration register"]
    #[inline(always)]
    pub const fn dltrcr(&self) -> &DLTRCR {
        &self.dltrcr
    }
    #[doc = "0x100 - DSI Host video shadow control register"]
    #[inline(always)]
    pub const fn vscr(&self) -> &VSCR {
        &self.vscr
    }
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    #[inline(always)]
    pub const fn lcvcidr(&self) -> &LCVCIDR {
        &self.lcvcidr
    }
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    #[inline(always)]
    pub const fn lcccr(&self) -> &LCCCR {
        &self.lcccr
    }
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    #[inline(always)]
    pub const fn lpmccr(&self) -> &LPMCCR {
        &self.lpmccr
    }
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    #[inline(always)]
    pub const fn vmccr(&self) -> &VMCCR {
        &self.vmccr
    }
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    #[inline(always)]
    pub const fn vpccr(&self) -> &VPCCR {
        &self.vpccr
    }
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    #[inline(always)]
    pub const fn vcccr(&self) -> &VCCCR {
        &self.vcccr
    }
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    #[inline(always)]
    pub const fn vnpccr(&self) -> &VNPCCR {
        &self.vnpccr
    }
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    #[inline(always)]
    pub const fn vhsaccr(&self) -> &VHSACCR {
        &self.vhsaccr
    }
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    #[inline(always)]
    pub const fn vhbpccr(&self) -> &VHBPCCR {
        &self.vhbpccr
    }
    #[doc = "0x150 - DSI Host video line current configuration register"]
    #[inline(always)]
    pub const fn vlccr(&self) -> &VLCCR {
        &self.vlccr
    }
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    #[inline(always)]
    pub const fn vvsaccr(&self) -> &VVSACCR {
        &self.vvsaccr
    }
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    #[inline(always)]
    pub const fn vvbpccr(&self) -> &VVBPCCR {
        &self.vvbpccr
    }
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    #[inline(always)]
    pub const fn vvfpccr(&self) -> &VVFPCCR {
        &self.vvfpccr
    }
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    #[inline(always)]
    pub const fn vvaccr(&self) -> &VVACCR {
        &self.vvaccr
    }
    #[doc = "0x400 - DSI wrapper configuration register"]
    #[inline(always)]
    pub const fn wcfgr(&self) -> &WCFGR {
        &self.wcfgr
    }
    #[doc = "0x404 - DSI wrapper control register"]
    #[inline(always)]
    pub const fn wcr(&self) -> &WCR {
        &self.wcr
    }
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    #[inline(always)]
    pub const fn wier(&self) -> &WIER {
        &self.wier
    }
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    #[inline(always)]
    pub const fn wisr(&self) -> &WISR {
        &self.wisr
    }
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    #[inline(always)]
    pub const fn wifcr(&self) -> &WIFCR {
        &self.wifcr
    }
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    #[inline(always)]
    pub const fn wpcr0(&self) -> &WPCR0 {
        &self.wpcr0
    }
    #[doc = "0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
    #[inline(always)]
    pub const fn wpcr1(&self) -> &WPCR1 {
        &self.wpcr1
    }
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    #[inline(always)]
    pub const fn wrpcr(&self) -> &WRPCR {
        &self.wrpcr
    }
    #[doc = "0x7f0 - DSI Host hardware configuration register"]
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    #[doc = "0x7f4 - DSI Host version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x7f8 - DSI Host identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x7fc - DSI Host size identification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "VR (r) register accessor: DSI Host version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vr`]
module"]
pub type VR = crate::Reg<vr::VRrs>;
#[doc = "DSI Host version register"]
pub mod vr;
#[doc = "CR (rw) register accessor: DSI Host control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "DSI Host control register"]
pub mod cr;
#[doc = "CCR (rw) register accessor: DSI Host clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "DSI Host clock control register"]
pub mod ccr;
#[doc = "LVCIDR (rw) register accessor: DSI Host LTDC VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvcidr`]
module"]
pub type LVCIDR = crate::Reg<lvcidr::LVCIDRrs>;
#[doc = "DSI Host LTDC VCID register"]
pub mod lvcidr;
#[doc = "LCOLCR (rw) register accessor: DSI Host LTDC color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcolcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcolcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcolcr`]
module"]
pub type LCOLCR = crate::Reg<lcolcr::LCOLCRrs>;
#[doc = "DSI Host LTDC color coding register"]
pub mod lcolcr;
#[doc = "LPCR (rw) register accessor: DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcr`]
module"]
pub type LPCR = crate::Reg<lpcr::LPCRrs>;
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod lpcr;
#[doc = "LPMCR (rw) register accessor: DSI Host low-power mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmcr`]
module"]
pub type LPMCR = crate::Reg<lpmcr::LPMCRrs>;
#[doc = "DSI Host low-power mode configuration register"]
pub mod lpmcr;
#[doc = "PCR (rw) register accessor: DSI Host protocol configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCRrs>;
#[doc = "DSI Host protocol configuration register"]
pub mod pcr;
#[doc = "GVCIDR (r) register accessor: DSI Host generic VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gvcidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gvcidr`]
module"]
pub type GVCIDR = crate::Reg<gvcidr::GVCIDRrs>;
#[doc = "DSI Host generic VCID register"]
pub mod gvcidr;
#[doc = "MCR (rw) register accessor: DSI Host mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCRrs>;
#[doc = "DSI Host mode configuration register"]
pub mod mcr;
#[doc = "VMCR (rw) register accessor: DSI Host video mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmcr`]
module"]
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
#[doc = "DSI Host video mode configuration register"]
pub mod vmcr;
#[doc = "VPCR (rw) register accessor: DSI Host video packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpcr`]
module"]
pub type VPCR = crate::Reg<vpcr::VPCRrs>;
#[doc = "DSI Host video packet configuration register"]
pub mod vpcr;
#[doc = "VCCR (rw) register accessor: DSI Host video chunks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vccr`]
module"]
pub type VCCR = crate::Reg<vccr::VCCRrs>;
#[doc = "DSI Host video chunks configuration register"]
pub mod vccr;
#[doc = "VNPCR (rw) register accessor: DSI Host video null packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vnpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vnpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vnpcr`]
module"]
pub type VNPCR = crate::Reg<vnpcr::VNPCRrs>;
#[doc = "DSI Host video null packet configuration register"]
pub mod vnpcr;
#[doc = "VHSACR (rw) register accessor: DSI Host video HSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhsacr`]
module"]
pub type VHSACR = crate::Reg<vhsacr::VHSACRrs>;
#[doc = "DSI Host video HSA configuration register"]
pub mod vhsacr;
#[doc = "VHBPCR (rw) register accessor: DSI Host video HBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vhbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhbpcr`]
module"]
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCRrs>;
#[doc = "DSI Host video HBP configuration register"]
pub mod vhbpcr;
#[doc = "VLCR (rw) register accessor: DSI Host video line configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlcr`]
module"]
pub type VLCR = crate::Reg<vlcr::VLCRrs>;
#[doc = "DSI Host video line configuration register"]
pub mod vlcr;
#[doc = "VVSACR (rw) register accessor: DSI Host video VSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvsacr`]
module"]
pub type VVSACR = crate::Reg<vvsacr::VVSACRrs>;
#[doc = "DSI Host video VSA configuration register"]
pub mod vvsacr;
#[doc = "VVBPCR (rw) register accessor: DSI Host video VBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvbpcr`]
module"]
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCRrs>;
#[doc = "DSI Host video VBP configuration register"]
pub mod vvbpcr;
#[doc = "VVFPCR (rw) register accessor: DSI Host video VFP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvfpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvfpcr`]
module"]
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCRrs>;
#[doc = "DSI Host video VFP configuration register"]
pub mod vvfpcr;
#[doc = "VVACR (rw) register accessor: DSI Host video VA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvacr`]
module"]
pub type VVACR = crate::Reg<vvacr::VVACRrs>;
#[doc = "DSI Host video VA configuration register"]
pub mod vvacr;
#[doc = "LCCR (rw) register accessor: DSI Host LTDC command configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lccr`]
module"]
pub type LCCR = crate::Reg<lccr::LCCRrs>;
#[doc = "DSI Host LTDC command configuration register"]
pub mod lccr;
#[doc = "CMCR (rw) register accessor: DSI Host command mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmcr`]
module"]
pub type CMCR = crate::Reg<cmcr::CMCRrs>;
#[doc = "DSI Host command mode configuration register"]
pub mod cmcr;
#[doc = "GHCR (rw) register accessor: DSI Host generic header configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ghcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ghcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghcr`]
module"]
pub type GHCR = crate::Reg<ghcr::GHCRrs>;
#[doc = "DSI Host generic header configuration register"]
pub mod ghcr;
#[doc = "GPDR (rw) register accessor: DSI Host generic payload data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdr`]
module"]
pub type GPDR = crate::Reg<gpdr::GPDRrs>;
#[doc = "DSI Host generic payload data register"]
pub mod gpdr;
#[doc = "GPSR (r) register accessor: DSI Host generic packet status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpsr`]
module"]
pub type GPSR = crate::Reg<gpsr::GPSRrs>;
#[doc = "DSI Host generic packet status register"]
pub mod gpsr;
#[doc = "TCCR0 (rw) register accessor: DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr0`]
module"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0rs>;
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod tccr0;
#[doc = "TCCR1 (rw) register accessor: DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr1`]
module"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1rs>;
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod tccr1;
#[doc = "TCCR2 (rw) register accessor: DSI Host timeout counter configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr2`]
module"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2rs>;
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod tccr2;
#[doc = "TCCR3 (rw) register accessor: DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr3`]
module"]
pub type TCCR3 = crate::Reg<tccr3::TCCR3rs>;
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod tccr3;
#[doc = "TCCR4 (rw) register accessor: DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr4`]
module"]
pub type TCCR4 = crate::Reg<tccr4::TCCR4rs>;
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod tccr4;
#[doc = "TCCR5 (rw) register accessor: DSI Host timeout counter configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr5`]
module"]
pub type TCCR5 = crate::Reg<tccr5::TCCR5rs>;
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod tccr5;
#[doc = "CLCR (rw) register accessor: DSI Host clock lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clcr`]
module"]
pub type CLCR = crate::Reg<clcr::CLCRrs>;
#[doc = "DSI Host clock lane configuration register"]
pub mod clcr;
#[doc = "CLTCR (rw) register accessor: DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cltcr`]
module"]
pub type CLTCR = crate::Reg<cltcr::CLTCRrs>;
#[doc = "DSI Host clock lane timer configuration register"]
pub mod cltcr;
#[doc = "DLTCR (rw) register accessor: DSI Host data lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dltcr`]
module"]
pub type DLTCR = crate::Reg<dltcr::DLTCRrs>;
#[doc = "DSI Host data lane timer configuration register"]
pub mod dltcr;
#[doc = "PCTLR (rw) register accessor: DSI Host PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pctlr`]
module"]
pub type PCTLR = crate::Reg<pctlr::PCTLRrs>;
#[doc = "DSI Host PHY control register"]
pub mod pctlr;
#[doc = "PCONFR (rw) register accessor: DSI Host PHY configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pconfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pconfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconfr`]
module"]
pub type PCONFR = crate::Reg<pconfr::PCONFRrs>;
#[doc = "DSI Host PHY configuration register"]
pub mod pconfr;
#[doc = "PUCR (rw) register accessor: DSI Host PHY ULPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucr`]
module"]
pub type PUCR = crate::Reg<pucr::PUCRrs>;
#[doc = "DSI Host PHY ULPS control register"]
pub mod pucr;
#[doc = "PTTCR (rw) register accessor: DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pttcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pttcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pttcr`]
module"]
pub type PTTCR = crate::Reg<pttcr::PTTCRrs>;
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod pttcr;
#[doc = "PSR (r) register accessor: DSI Host PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSRrs>;
#[doc = "DSI Host PHY status register"]
pub mod psr;
#[doc = "ISR0 (r) register accessor: DSI Host interrupt and status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr0`]
module"]
pub type ISR0 = crate::Reg<isr0::ISR0rs>;
#[doc = "DSI Host interrupt and status register 0"]
pub mod isr0;
#[doc = "ISR1 (r) register accessor: DSI Host interrupt and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr1`]
module"]
pub type ISR1 = crate::Reg<isr1::ISR1rs>;
#[doc = "DSI Host interrupt and status register 1"]
pub mod isr1;
#[doc = "IER0 (rw) register accessor: DSI Host interrupt enable register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier0`]
module"]
pub type IER0 = crate::Reg<ier0::IER0rs>;
#[doc = "DSI Host interrupt enable register 0"]
pub mod ier0;
#[doc = "IER1 (rw) register accessor: DSI Host interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
pub type IER1 = crate::Reg<ier1::IER1rs>;
#[doc = "DSI Host interrupt enable register 1"]
pub mod ier1;
#[doc = "FIR0 (w) register accessor: DSI Host force interrupt register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fir0`]
module"]
pub type FIR0 = crate::Reg<fir0::FIR0rs>;
#[doc = "DSI Host force interrupt register 0"]
pub mod fir0;
#[doc = "FIR1 (w) register accessor: DSI Host force interrupt register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fir1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fir1`]
module"]
pub type FIR1 = crate::Reg<fir1::FIR1rs>;
#[doc = "DSI Host force interrupt register 1"]
pub mod fir1;
#[doc = "DLTRCR (rw) register accessor: DSI Host data lane timer read configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dltrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dltrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dltrcr`]
module"]
pub type DLTRCR = crate::Reg<dltrcr::DLTRCRrs>;
#[doc = "DSI Host data lane timer read configuration register"]
pub mod dltrcr;
#[doc = "VSCR (rw) register accessor: DSI Host video shadow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vscr`]
module"]
pub type VSCR = crate::Reg<vscr::VSCRrs>;
#[doc = "DSI Host video shadow control register"]
pub mod vscr;
#[doc = "LCVCIDR (rw) register accessor: DSI Host LTDC current VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcvcidr`]
module"]
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDRrs>;
#[doc = "DSI Host LTDC current VCID register"]
pub mod lcvcidr;
#[doc = "LCCCR (r) register accessor: DSI Host LTDC current color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcccr`]
module"]
pub type LCCCR = crate::Reg<lcccr::LCCCRrs>;
#[doc = "DSI Host LTDC current color coding register"]
pub mod lcccr;
#[doc = "LPMCCR (r) register accessor: DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmccr`]
module"]
pub type LPMCCR = crate::Reg<lpmccr::LPMCCRrs>;
#[doc = "DSI Host low-power mode current configuration register"]
pub mod lpmccr;
#[doc = "VMCCR (r) register accessor: DSI Host video mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmccr`]
module"]
pub type VMCCR = crate::Reg<vmccr::VMCCRrs>;
#[doc = "DSI Host video mode current configuration register"]
pub mod vmccr;
#[doc = "VPCCR (r) register accessor: DSI Host video packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vpccr`]
module"]
pub type VPCCR = crate::Reg<vpccr::VPCCRrs>;
#[doc = "DSI Host video packet current configuration register"]
pub mod vpccr;
#[doc = "VCCCR (r) register accessor: DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vcccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vcccr`]
module"]
pub type VCCCR = crate::Reg<vcccr::VCCCRrs>;
#[doc = "DSI Host video chunks current configuration register"]
pub mod vcccr;
#[doc = "VNPCCR (r) register accessor: DSI Host video null packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vnpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vnpccr`]
module"]
pub type VNPCCR = crate::Reg<vnpccr::VNPCCRrs>;
#[doc = "DSI Host video null packet current configuration register"]
pub mod vnpccr;
#[doc = "VHSACCR (r) register accessor: DSI Host video HSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhsaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhsaccr`]
module"]
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCRrs>;
#[doc = "DSI Host video HSA current configuration register"]
pub mod vhsaccr;
#[doc = "VHBPCCR (r) register accessor: DSI Host video HBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vhbpccr`]
module"]
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCRrs>;
#[doc = "DSI Host video HBP current configuration register"]
pub mod vhbpccr;
#[doc = "VLCCR (r) register accessor: DSI Host video line current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlccr`]
module"]
pub type VLCCR = crate::Reg<vlccr::VLCCRrs>;
#[doc = "DSI Host video line current configuration register"]
pub mod vlccr;
#[doc = "VVSACCR (r) register accessor: DSI Host video VSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvsaccr`]
module"]
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCRrs>;
#[doc = "DSI Host video VSA current configuration register"]
pub mod vvsaccr;
#[doc = "VVBPCCR (r) register accessor: DSI Host video VBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvbpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvbpccr`]
module"]
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCRrs>;
#[doc = "DSI Host video VBP current configuration register"]
pub mod vvbpccr;
#[doc = "VVFPCCR (r) register accessor: DSI Host video VFP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvfpccr`]
module"]
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCRrs>;
#[doc = "DSI Host video VFP current configuration register"]
pub mod vvfpccr;
#[doc = "VVACCR (r) register accessor: DSI Host video VA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vvaccr`]
module"]
pub type VVACCR = crate::Reg<vvaccr::VVACCRrs>;
#[doc = "DSI Host video VA current configuration register"]
pub mod vvaccr;
#[doc = "WCFGR (rw) register accessor: DSI wrapper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcfgr`]
module"]
pub type WCFGR = crate::Reg<wcfgr::WCFGRrs>;
#[doc = "DSI wrapper configuration register"]
pub mod wcfgr;
#[doc = "WCR (rw) register accessor: DSI wrapper control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`]
module"]
pub type WCR = crate::Reg<wcr::WCRrs>;
#[doc = "DSI wrapper control register"]
pub mod wcr;
#[doc = "WIER (rw) register accessor: DSI wrapper interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wier`]
module"]
pub type WIER = crate::Reg<wier::WIERrs>;
#[doc = "DSI wrapper interrupt enable register"]
pub mod wier;
#[doc = "WISR (r) register accessor: DSI wrapper interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wisr`]
module"]
pub type WISR = crate::Reg<wisr::WISRrs>;
#[doc = "DSI wrapper interrupt and status register"]
pub mod wisr;
#[doc = "WIFCR (w) register accessor: DSI wrapper interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifcr`]
module"]
pub type WIFCR = crate::Reg<wifcr::WIFCRrs>;
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod wifcr;
#[doc = "WPCR0 (rw) register accessor: DSI wrapper PHY configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr0`]
module"]
pub type WPCR0 = crate::Reg<wpcr0::WPCR0rs>;
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod wpcr0;
#[doc = "WPCR1 (rw) register accessor: This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpcr1`]
module"]
pub type WPCR1 = crate::Reg<wpcr1::WPCR1rs>;
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
pub mod wpcr1;
#[doc = "WRPCR (rw) register accessor: DSI wrapper regulator and PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpcr`]
module"]
pub type WRPCR = crate::Reg<wrpcr::WRPCRrs>;
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod wrpcr;
#[doc = "HWCFGR (r) register accessor: DSI Host hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr`]
module"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
#[doc = "DSI Host hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR (r) register accessor: DSI Host version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "DSI Host version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: DSI Host identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "DSI Host identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: DSI Host size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "DSI Host size identification register"]
pub mod sidr;
