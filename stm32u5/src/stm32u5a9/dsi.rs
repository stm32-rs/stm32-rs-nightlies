#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dsi_vr: DSI_VR,
    dsi_cr: DSI_CR,
    dsi_ccr: DSI_CCR,
    dsi_lvcidr: DSI_LVCIDR,
    dsi_lcolcr: DSI_LCOLCR,
    dsi_lpcr: DSI_LPCR,
    dsi_lpmcr: DSI_LPMCR,
    _reserved7: [u8; 0x10],
    dsi_pcr: DSI_PCR,
    dsi_gvcidr: DSI_GVCIDR,
    dsi_mcr: DSI_MCR,
    dsi_vmcr: DSI_VMCR,
    dsi_vpcr: DSI_VPCR,
    dsi_vccr: DSI_VCCR,
    dsi_vnpcr: DSI_VNPCR,
    dsi_vhsacr: DSI_VHSACR,
    dsi_vhbpcr: DSI_VHBPCR,
    dsi_vlcr: DSI_VLCR,
    dsi_vvsacr: DSI_VVSACR,
    dsi_vvbpcr: DSI_VVBPCR,
    dsi_vvfpcr: DSI_VVFPCR,
    dsi_vvacr: DSI_VVACR,
    dsi_lccr: DSI_LCCR,
    dsi_cmcr: DSI_CMCR,
    dsi_ghcr: DSI_GHCR,
    dsi_gpdr: DSI_GPDR,
    dsi_gpsr: DSI_GPSR,
    dsi_tccr0: DSI_TCCR0,
    dsi_tccr1: DSI_TCCR1,
    dsi_tccr2: DSI_TCCR2,
    dsi_tccr3: DSI_TCCR3,
    dsi_tccr4: DSI_TCCR4,
    dsi_tccr5: DSI_TCCR5,
    _reserved32: [u8; 0x04],
    dsi_clcr: DSI_CLCR,
    dsi_cltcr: DSI_CLTCR,
    dsi_dltcr: DSI_DLTCR,
    dsi_pctlr: DSI_PCTLR,
    dsi_pconfr: DSI_PCONFR,
    dsi_pucr: DSI_PUCR,
    dsi_pttcr: DSI_PTTCR,
    dsi_psr: DSI_PSR,
    _reserved40: [u8; 0x08],
    dsi_isr0: DSI_ISR0,
    dsi_isr1: DSI_ISR1,
    dsi_ier0: DSI_IER0,
    dsi_ier1: DSI_IER1,
    _reserved44: [u8; 0x0c],
    dsi_fir0: DSI_FIR0,
    dsi_fir1: DSI_FIR1,
    _reserved46: [u8; 0x14],
    dsi_dltrcr: DSI_DLTRCR,
    _reserved47: [u8; 0x08],
    dsi_vscr: DSI_VSCR,
    _reserved48: [u8; 0x08],
    dsi_lcvcidr: DSI_LCVCIDR,
    dsi_lcccr: DSI_LCCCR,
    _reserved50: [u8; 0x04],
    dsi_lpmccr: DSI_LPMCCR,
    _reserved51: [u8; 0x1c],
    dsi_vmccr: DSI_VMCCR,
    dsi_vpccr: DSI_VPCCR,
    dsi_vcccr: DSI_VCCCR,
    dsi_vnpccr: DSI_VNPCCR,
    dsi_vhsaccr: DSI_VHSACCR,
    dsi_vhbpccr: DSI_VHBPCCR,
    dsi_vlccr: DSI_VLCCR,
    dsi_vvsaccr: DSI_VVSACCR,
    dsi_vvbpccr: DSI_VVBPCCR,
    dsi_vvfpccr: DSI_VVFPCCR,
    dsi_vvaccr: DSI_VVACCR,
    _reserved62: [u8; 0x04],
    dsi_fbsr: DSI_FBSR,
    _reserved63: [u8; 0x0294],
    dsi_wcfgr: DSI_WCFGR,
    dsi_wcr: DSI_WCR,
    dsi_wier: DSI_WIER,
    dsi_wisr: DSI_WISR,
    dsi_wifcr: DSI_WIFCR,
    _reserved68: [u8; 0x04],
    dsi_wpcr0: DSI_WPCR0,
    _reserved69: [u8; 0x14],
    dsi_wrpcr: DSI_WRPCR,
    _reserved70: [u8; 0x03d4],
    dsi_bcfgr: DSI_BCFGR,
    _reserved71: [u8; 0x03f8],
    dsi_dpcbcr: DSI_DPCBCR,
    _reserved72: [u8; 0x2c],
    dsi_dpcsrcr: DSI_DPCSRCR,
    _reserved73: [u8; 0x38],
    dsi_dpdl0bcr: DSI_DPDL0BCR,
    _reserved74: [u8; 0x2c],
    dsi_dpdl0srcr: DSI_DPDL0SRCR,
    _reserved75: [u8; 0x64],
    dsi_dpdl1bcr: DSI_DPDL1BCR,
    _reserved76: [u8; 0x2c],
    dsi_dpdl1srcr: DSI_DPDL1SRCR,
}
impl RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    #[inline(always)]
    pub const fn dsi_vr(&self) -> &DSI_VR {
        &self.dsi_vr
    }
    #[doc = "0x04 - DSI Host control register"]
    #[inline(always)]
    pub const fn dsi_cr(&self) -> &DSI_CR {
        &self.dsi_cr
    }
    #[doc = "0x08 - DSI Host clock control register"]
    #[inline(always)]
    pub const fn dsi_ccr(&self) -> &DSI_CCR {
        &self.dsi_ccr
    }
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    #[inline(always)]
    pub const fn dsi_lvcidr(&self) -> &DSI_LVCIDR {
        &self.dsi_lvcidr
    }
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    #[inline(always)]
    pub const fn dsi_lcolcr(&self) -> &DSI_LCOLCR {
        &self.dsi_lcolcr
    }
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    #[inline(always)]
    pub const fn dsi_lpcr(&self) -> &DSI_LPCR {
        &self.dsi_lpcr
    }
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    #[inline(always)]
    pub const fn dsi_lpmcr(&self) -> &DSI_LPMCR {
        &self.dsi_lpmcr
    }
    #[doc = "0x2c - DSI Host protocol configuration register"]
    #[inline(always)]
    pub const fn dsi_pcr(&self) -> &DSI_PCR {
        &self.dsi_pcr
    }
    #[doc = "0x30 - DSI Host generic VCID register"]
    #[inline(always)]
    pub const fn dsi_gvcidr(&self) -> &DSI_GVCIDR {
        &self.dsi_gvcidr
    }
    #[doc = "0x34 - DSI Host mode configuration register"]
    #[inline(always)]
    pub const fn dsi_mcr(&self) -> &DSI_MCR {
        &self.dsi_mcr
    }
    #[doc = "0x38 - DSI Host video mode configuration register"]
    #[inline(always)]
    pub const fn dsi_vmcr(&self) -> &DSI_VMCR {
        &self.dsi_vmcr
    }
    #[doc = "0x3c - DSI Host video packet configuration register"]
    #[inline(always)]
    pub const fn dsi_vpcr(&self) -> &DSI_VPCR {
        &self.dsi_vpcr
    }
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    #[inline(always)]
    pub const fn dsi_vccr(&self) -> &DSI_VCCR {
        &self.dsi_vccr
    }
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    #[inline(always)]
    pub const fn dsi_vnpcr(&self) -> &DSI_VNPCR {
        &self.dsi_vnpcr
    }
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    #[inline(always)]
    pub const fn dsi_vhsacr(&self) -> &DSI_VHSACR {
        &self.dsi_vhsacr
    }
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    #[inline(always)]
    pub const fn dsi_vhbpcr(&self) -> &DSI_VHBPCR {
        &self.dsi_vhbpcr
    }
    #[doc = "0x50 - DSI Host video line configuration register"]
    #[inline(always)]
    pub const fn dsi_vlcr(&self) -> &DSI_VLCR {
        &self.dsi_vlcr
    }
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    #[inline(always)]
    pub const fn dsi_vvsacr(&self) -> &DSI_VVSACR {
        &self.dsi_vvsacr
    }
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    #[inline(always)]
    pub const fn dsi_vvbpcr(&self) -> &DSI_VVBPCR {
        &self.dsi_vvbpcr
    }
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    #[inline(always)]
    pub const fn dsi_vvfpcr(&self) -> &DSI_VVFPCR {
        &self.dsi_vvfpcr
    }
    #[doc = "0x60 - DSI Host video VA configuration register"]
    #[inline(always)]
    pub const fn dsi_vvacr(&self) -> &DSI_VVACR {
        &self.dsi_vvacr
    }
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    #[inline(always)]
    pub const fn dsi_lccr(&self) -> &DSI_LCCR {
        &self.dsi_lccr
    }
    #[doc = "0x68 - DSI Host command mode configuration register"]
    #[inline(always)]
    pub const fn dsi_cmcr(&self) -> &DSI_CMCR {
        &self.dsi_cmcr
    }
    #[doc = "0x6c - DSI Host generic header configuration register"]
    #[inline(always)]
    pub const fn dsi_ghcr(&self) -> &DSI_GHCR {
        &self.dsi_ghcr
    }
    #[doc = "0x70 - DSI Host generic payload data register"]
    #[inline(always)]
    pub const fn dsi_gpdr(&self) -> &DSI_GPDR {
        &self.dsi_gpdr
    }
    #[doc = "0x74 - DSI Host generic packet status register"]
    #[inline(always)]
    pub const fn dsi_gpsr(&self) -> &DSI_GPSR {
        &self.dsi_gpsr
    }
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    #[inline(always)]
    pub const fn dsi_tccr0(&self) -> &DSI_TCCR0 {
        &self.dsi_tccr0
    }
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    #[inline(always)]
    pub const fn dsi_tccr1(&self) -> &DSI_TCCR1 {
        &self.dsi_tccr1
    }
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    #[inline(always)]
    pub const fn dsi_tccr2(&self) -> &DSI_TCCR2 {
        &self.dsi_tccr2
    }
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    #[inline(always)]
    pub const fn dsi_tccr3(&self) -> &DSI_TCCR3 {
        &self.dsi_tccr3
    }
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    #[inline(always)]
    pub const fn dsi_tccr4(&self) -> &DSI_TCCR4 {
        &self.dsi_tccr4
    }
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    #[inline(always)]
    pub const fn dsi_tccr5(&self) -> &DSI_TCCR5 {
        &self.dsi_tccr5
    }
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    #[inline(always)]
    pub const fn dsi_clcr(&self) -> &DSI_CLCR {
        &self.dsi_clcr
    }
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    #[inline(always)]
    pub const fn dsi_cltcr(&self) -> &DSI_CLTCR {
        &self.dsi_cltcr
    }
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    #[inline(always)]
    pub const fn dsi_dltcr(&self) -> &DSI_DLTCR {
        &self.dsi_dltcr
    }
    #[doc = "0xa0 - DSI Host PHY control register"]
    #[inline(always)]
    pub const fn dsi_pctlr(&self) -> &DSI_PCTLR {
        &self.dsi_pctlr
    }
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    #[inline(always)]
    pub const fn dsi_pconfr(&self) -> &DSI_PCONFR {
        &self.dsi_pconfr
    }
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    #[inline(always)]
    pub const fn dsi_pucr(&self) -> &DSI_PUCR {
        &self.dsi_pucr
    }
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    #[inline(always)]
    pub const fn dsi_pttcr(&self) -> &DSI_PTTCR {
        &self.dsi_pttcr
    }
    #[doc = "0xb0 - DSI Host PHY status register"]
    #[inline(always)]
    pub const fn dsi_psr(&self) -> &DSI_PSR {
        &self.dsi_psr
    }
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    #[inline(always)]
    pub const fn dsi_isr0(&self) -> &DSI_ISR0 {
        &self.dsi_isr0
    }
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    #[inline(always)]
    pub const fn dsi_isr1(&self) -> &DSI_ISR1 {
        &self.dsi_isr1
    }
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    #[inline(always)]
    pub const fn dsi_ier0(&self) -> &DSI_IER0 {
        &self.dsi_ier0
    }
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    #[inline(always)]
    pub const fn dsi_ier1(&self) -> &DSI_IER1 {
        &self.dsi_ier1
    }
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    #[inline(always)]
    pub const fn dsi_fir0(&self) -> &DSI_FIR0 {
        &self.dsi_fir0
    }
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    #[inline(always)]
    pub const fn dsi_fir1(&self) -> &DSI_FIR1 {
        &self.dsi_fir1
    }
    #[doc = "0xf4 - DSI Host data lane timer read configuration register"]
    #[inline(always)]
    pub const fn dsi_dltrcr(&self) -> &DSI_DLTRCR {
        &self.dsi_dltrcr
    }
    #[doc = "0x100 - DSI Host video shadow control register"]
    #[inline(always)]
    pub const fn dsi_vscr(&self) -> &DSI_VSCR {
        &self.dsi_vscr
    }
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    #[inline(always)]
    pub const fn dsi_lcvcidr(&self) -> &DSI_LCVCIDR {
        &self.dsi_lcvcidr
    }
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    #[inline(always)]
    pub const fn dsi_lcccr(&self) -> &DSI_LCCCR {
        &self.dsi_lcccr
    }
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    #[inline(always)]
    pub const fn dsi_lpmccr(&self) -> &DSI_LPMCCR {
        &self.dsi_lpmccr
    }
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    #[inline(always)]
    pub const fn dsi_vmccr(&self) -> &DSI_VMCCR {
        &self.dsi_vmccr
    }
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    #[inline(always)]
    pub const fn dsi_vpccr(&self) -> &DSI_VPCCR {
        &self.dsi_vpccr
    }
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    #[inline(always)]
    pub const fn dsi_vcccr(&self) -> &DSI_VCCCR {
        &self.dsi_vcccr
    }
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    #[inline(always)]
    pub const fn dsi_vnpccr(&self) -> &DSI_VNPCCR {
        &self.dsi_vnpccr
    }
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    #[inline(always)]
    pub const fn dsi_vhsaccr(&self) -> &DSI_VHSACCR {
        &self.dsi_vhsaccr
    }
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    #[inline(always)]
    pub const fn dsi_vhbpccr(&self) -> &DSI_VHBPCCR {
        &self.dsi_vhbpccr
    }
    #[doc = "0x150 - DSI Host video line current configuration register"]
    #[inline(always)]
    pub const fn dsi_vlccr(&self) -> &DSI_VLCCR {
        &self.dsi_vlccr
    }
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvsaccr(&self) -> &DSI_VVSACCR {
        &self.dsi_vvsaccr
    }
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvbpccr(&self) -> &DSI_VVBPCCR {
        &self.dsi_vvbpccr
    }
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvfpccr(&self) -> &DSI_VVFPCCR {
        &self.dsi_vvfpccr
    }
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvaccr(&self) -> &DSI_VVACCR {
        &self.dsi_vvaccr
    }
    #[doc = "0x168 - DSI Host FIFO and buffer status register"]
    #[inline(always)]
    pub const fn dsi_fbsr(&self) -> &DSI_FBSR {
        &self.dsi_fbsr
    }
    #[doc = "0x400 - DSI Wrapper configuration register"]
    #[inline(always)]
    pub const fn dsi_wcfgr(&self) -> &DSI_WCFGR {
        &self.dsi_wcfgr
    }
    #[doc = "0x404 - DSI Wrapper control register"]
    #[inline(always)]
    pub const fn dsi_wcr(&self) -> &DSI_WCR {
        &self.dsi_wcr
    }
    #[doc = "0x408 - DSI Wrapper interrupt enable register"]
    #[inline(always)]
    pub const fn dsi_wier(&self) -> &DSI_WIER {
        &self.dsi_wier
    }
    #[doc = "0x40c - DSI Wrapper interrupt and status register"]
    #[inline(always)]
    pub const fn dsi_wisr(&self) -> &DSI_WISR {
        &self.dsi_wisr
    }
    #[doc = "0x410 - DSI Wrapper interrupt flag clear register"]
    #[inline(always)]
    pub const fn dsi_wifcr(&self) -> &DSI_WIFCR {
        &self.dsi_wifcr
    }
    #[doc = "0x418 - DSI Wrapper PHY configuration register 0"]
    #[inline(always)]
    pub const fn dsi_wpcr0(&self) -> &DSI_WPCR0 {
        &self.dsi_wpcr0
    }
    #[doc = "0x430 - DSI Wrapper regulator and PLL control register"]
    #[inline(always)]
    pub const fn dsi_wrpcr(&self) -> &DSI_WRPCR {
        &self.dsi_wrpcr
    }
    #[doc = "0x808 - DSI bias configuration register"]
    #[inline(always)]
    pub const fn dsi_bcfgr(&self) -> &DSI_BCFGR {
        &self.dsi_bcfgr
    }
    #[doc = "0xc04 - DSI D-PHY clock band control register"]
    #[inline(always)]
    pub const fn dsi_dpcbcr(&self) -> &DSI_DPCBCR {
        &self.dsi_dpcbcr
    }
    #[doc = "0xc34 - DSI D-PHY clock skew rate control register"]
    #[inline(always)]
    pub const fn dsi_dpcsrcr(&self) -> &DSI_DPCSRCR {
        &self.dsi_dpcsrcr
    }
    #[doc = "0xc70 - DSI D-PHY data lane 0 band control register"]
    #[inline(always)]
    pub const fn dsi_dpdl0bcr(&self) -> &DSI_DPDL0BCR {
        &self.dsi_dpdl0bcr
    }
    #[doc = "0xca0 - DSI D-PHY data lane 0 skew rate control register"]
    #[inline(always)]
    pub const fn dsi_dpdl0srcr(&self) -> &DSI_DPDL0SRCR {
        &self.dsi_dpdl0srcr
    }
    #[doc = "0xd08 - DSI D-PHY data lane 1 band control register"]
    #[inline(always)]
    pub const fn dsi_dpdl1bcr(&self) -> &DSI_DPDL1BCR {
        &self.dsi_dpdl1bcr
    }
    #[doc = "0xd38 - DSI D-PHY data lane 1 skew rate control register"]
    #[inline(always)]
    pub const fn dsi_dpdl1srcr(&self) -> &DSI_DPDL1SRCR {
        &self.dsi_dpdl1srcr
    }
}
#[doc = "DSI_VR (r) register accessor: DSI Host version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vr`]
module"]
pub type DSI_VR = crate::Reg<dsi_vr::DSI_VRrs>;
#[doc = "DSI Host version register"]
pub mod dsi_vr;
#[doc = "DSI_CR (rw) register accessor: DSI Host control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_cr`]
module"]
pub type DSI_CR = crate::Reg<dsi_cr::DSI_CRrs>;
#[doc = "DSI Host control register"]
pub mod dsi_cr;
#[doc = "DSI_CCR (rw) register accessor: DSI Host clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ccr`]
module"]
pub type DSI_CCR = crate::Reg<dsi_ccr::DSI_CCRrs>;
#[doc = "DSI Host clock control register"]
pub mod dsi_ccr;
#[doc = "DSI_LVCIDR (rw) register accessor: DSI Host LTDC VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lvcidr`]
module"]
pub type DSI_LVCIDR = crate::Reg<dsi_lvcidr::DSI_LVCIDRrs>;
#[doc = "DSI Host LTDC VCID register"]
pub mod dsi_lvcidr;
#[doc = "DSI_LCOLCR (rw) register accessor: DSI Host LTDC color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcolcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lcolcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lcolcr`]
module"]
pub type DSI_LCOLCR = crate::Reg<dsi_lcolcr::DSI_LCOLCRrs>;
#[doc = "DSI Host LTDC color coding register"]
pub mod dsi_lcolcr;
#[doc = "DSI_LPCR (rw) register accessor: DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpcr`]
module"]
pub type DSI_LPCR = crate::Reg<dsi_lpcr::DSI_LPCRrs>;
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod dsi_lpcr;
#[doc = "DSI_LPMCR (rw) register accessor: DSI Host low-power mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpmcr`]
module"]
pub type DSI_LPMCR = crate::Reg<dsi_lpmcr::DSI_LPMCRrs>;
#[doc = "DSI Host low-power mode configuration register"]
pub mod dsi_lpmcr;
#[doc = "DSI_PCR (rw) register accessor: DSI Host protocol configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pcr`]
module"]
pub type DSI_PCR = crate::Reg<dsi_pcr::DSI_PCRrs>;
#[doc = "DSI Host protocol configuration register"]
pub mod dsi_pcr;
#[doc = "DSI_GVCIDR (rw) register accessor: DSI Host generic VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_gvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_gvcidr`]
module"]
pub type DSI_GVCIDR = crate::Reg<dsi_gvcidr::DSI_GVCIDRrs>;
#[doc = "DSI Host generic VCID register"]
pub mod dsi_gvcidr;
#[doc = "DSI_MCR (rw) register accessor: DSI Host mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_mcr`]
module"]
pub type DSI_MCR = crate::Reg<dsi_mcr::DSI_MCRrs>;
#[doc = "DSI Host mode configuration register"]
pub mod dsi_mcr;
#[doc = "DSI_VMCR (rw) register accessor: DSI Host video mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vmcr`]
module"]
pub type DSI_VMCR = crate::Reg<dsi_vmcr::DSI_VMCRrs>;
#[doc = "DSI Host video mode configuration register"]
pub mod dsi_vmcr;
#[doc = "DSI_VPCR (rw) register accessor: DSI Host video packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vpcr`]
module"]
pub type DSI_VPCR = crate::Reg<dsi_vpcr::DSI_VPCRrs>;
#[doc = "DSI Host video packet configuration register"]
pub mod dsi_vpcr;
#[doc = "DSI_VCCR (rw) register accessor: DSI Host video chunks configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vccr`]
module"]
pub type DSI_VCCR = crate::Reg<dsi_vccr::DSI_VCCRrs>;
#[doc = "DSI Host video chunks configuration register"]
pub mod dsi_vccr;
#[doc = "DSI_VNPCR (rw) register accessor: DSI Host video null packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vnpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vnpcr`]
module"]
pub type DSI_VNPCR = crate::Reg<dsi_vnpcr::DSI_VNPCRrs>;
#[doc = "DSI Host video null packet configuration register"]
pub mod dsi_vnpcr;
#[doc = "DSI_VHSACR (rw) register accessor: DSI Host video HSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vhsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhsacr`]
module"]
pub type DSI_VHSACR = crate::Reg<dsi_vhsacr::DSI_VHSACRrs>;
#[doc = "DSI Host video HSA configuration register"]
pub mod dsi_vhsacr;
#[doc = "DSI_VHBPCR (rw) register accessor: DSI Host video HBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vhbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhbpcr`]
module"]
pub type DSI_VHBPCR = crate::Reg<dsi_vhbpcr::DSI_VHBPCRrs>;
#[doc = "DSI Host video HBP configuration register"]
pub mod dsi_vhbpcr;
#[doc = "DSI_VLCR (rw) register accessor: DSI Host video line configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vlcr`]
module"]
pub type DSI_VLCR = crate::Reg<dsi_vlcr::DSI_VLCRrs>;
#[doc = "DSI Host video line configuration register"]
pub mod dsi_vlcr;
#[doc = "DSI_VVSACR (rw) register accessor: DSI Host video VSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvsacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvsacr`]
module"]
pub type DSI_VVSACR = crate::Reg<dsi_vvsacr::DSI_VVSACRrs>;
#[doc = "DSI Host video VSA configuration register"]
pub mod dsi_vvsacr;
#[doc = "DSI_VVBPCR (rw) register accessor: DSI Host video VBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvbpcr`]
module"]
pub type DSI_VVBPCR = crate::Reg<dsi_vvbpcr::DSI_VVBPCRrs>;
#[doc = "DSI Host video VBP configuration register"]
pub mod dsi_vvbpcr;
#[doc = "DSI_VVFPCR (rw) register accessor: DSI Host video VFP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvfpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvfpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvfpcr`]
module"]
pub type DSI_VVFPCR = crate::Reg<dsi_vvfpcr::DSI_VVFPCRrs>;
#[doc = "DSI Host video VFP configuration register"]
pub mod dsi_vvfpcr;
#[doc = "DSI_VVACR (rw) register accessor: DSI Host video VA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvacr`]
module"]
pub type DSI_VVACR = crate::Reg<dsi_vvacr::DSI_VVACRrs>;
#[doc = "DSI Host video VA configuration register"]
pub mod dsi_vvacr;
#[doc = "DSI_LCCR (rw) register accessor: DSI Host LTDC command configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lccr`]
module"]
pub type DSI_LCCR = crate::Reg<dsi_lccr::DSI_LCCRrs>;
#[doc = "DSI Host LTDC command configuration register"]
pub mod dsi_lccr;
#[doc = "DSI_CMCR (rw) register accessor: DSI Host command mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_cmcr`]
module"]
pub type DSI_CMCR = crate::Reg<dsi_cmcr::DSI_CMCRrs>;
#[doc = "DSI Host command mode configuration register"]
pub mod dsi_cmcr;
#[doc = "DSI_GHCR (rw) register accessor: DSI Host generic header configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ghcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ghcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ghcr`]
module"]
pub type DSI_GHCR = crate::Reg<dsi_ghcr::DSI_GHCRrs>;
#[doc = "DSI Host generic header configuration register"]
pub mod dsi_ghcr;
#[doc = "DSI_GPDR (rw) register accessor: DSI Host generic payload data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_gpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_gpdr`]
module"]
pub type DSI_GPDR = crate::Reg<dsi_gpdr::DSI_GPDRrs>;
#[doc = "DSI Host generic payload data register"]
pub mod dsi_gpdr;
#[doc = "DSI_GPSR (r) register accessor: DSI Host generic packet status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_gpsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_gpsr`]
module"]
pub type DSI_GPSR = crate::Reg<dsi_gpsr::DSI_GPSRrs>;
#[doc = "DSI Host generic packet status register"]
pub mod dsi_gpsr;
#[doc = "DSI_TCCR0 (rw) register accessor: DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr0`]
module"]
pub type DSI_TCCR0 = crate::Reg<dsi_tccr0::DSI_TCCR0rs>;
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod dsi_tccr0;
#[doc = "DSI_TCCR1 (rw) register accessor: DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr1`]
module"]
pub type DSI_TCCR1 = crate::Reg<dsi_tccr1::DSI_TCCR1rs>;
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod dsi_tccr1;
#[doc = "DSI_TCCR2 (rw) register accessor: DSI Host timeout counter configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr2`]
module"]
pub type DSI_TCCR2 = crate::Reg<dsi_tccr2::DSI_TCCR2rs>;
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod dsi_tccr2;
#[doc = "DSI_TCCR3 (rw) register accessor: DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr3`]
module"]
pub type DSI_TCCR3 = crate::Reg<dsi_tccr3::DSI_TCCR3rs>;
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod dsi_tccr3;
#[doc = "DSI_TCCR4 (rw) register accessor: DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr4`]
module"]
pub type DSI_TCCR4 = crate::Reg<dsi_tccr4::DSI_TCCR4rs>;
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod dsi_tccr4;
#[doc = "DSI_TCCR5 (rw) register accessor: DSI Host timeout counter configuration register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr5`]
module"]
pub type DSI_TCCR5 = crate::Reg<dsi_tccr5::DSI_TCCR5rs>;
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod dsi_tccr5;
#[doc = "DSI_CLCR (rw) register accessor: DSI Host clock lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_clcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_clcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_clcr`]
module"]
pub type DSI_CLCR = crate::Reg<dsi_clcr::DSI_CLCRrs>;
#[doc = "DSI Host clock lane configuration register"]
pub mod dsi_clcr;
#[doc = "DSI_CLTCR (rw) register accessor: DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_cltcr`]
module"]
pub type DSI_CLTCR = crate::Reg<dsi_cltcr::DSI_CLTCRrs>;
#[doc = "DSI Host clock lane timer configuration register"]
pub mod dsi_cltcr;
#[doc = "DSI_DLTCR (rw) register accessor: DSI Host data lane timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dltcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dltcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dltcr`]
module"]
pub type DSI_DLTCR = crate::Reg<dsi_dltcr::DSI_DLTCRrs>;
#[doc = "DSI Host data lane timer configuration register"]
pub mod dsi_dltcr;
#[doc = "DSI_PCTLR (rw) register accessor: DSI Host PHY control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pctlr`]
module"]
pub type DSI_PCTLR = crate::Reg<dsi_pctlr::DSI_PCTLRrs>;
#[doc = "DSI Host PHY control register"]
pub mod dsi_pctlr;
#[doc = "DSI_PCONFR (rw) register accessor: DSI Host PHY configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pconfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pconfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pconfr`]
module"]
pub type DSI_PCONFR = crate::Reg<dsi_pconfr::DSI_PCONFRrs>;
#[doc = "DSI Host PHY configuration register"]
pub mod dsi_pconfr;
#[doc = "DSI_PUCR (rw) register accessor: DSI Host PHY ULPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pucr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pucr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pucr`]
module"]
pub type DSI_PUCR = crate::Reg<dsi_pucr::DSI_PUCRrs>;
#[doc = "DSI Host PHY ULPS control register"]
pub mod dsi_pucr;
#[doc = "DSI_PTTCR (rw) register accessor: DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pttcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pttcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pttcr`]
module"]
pub type DSI_PTTCR = crate::Reg<dsi_pttcr::DSI_PTTCRrs>;
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod dsi_pttcr;
#[doc = "DSI_PSR (r) register accessor: DSI Host PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_psr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_psr`]
module"]
pub type DSI_PSR = crate::Reg<dsi_psr::DSI_PSRrs>;
#[doc = "DSI Host PHY status register"]
pub mod dsi_psr;
#[doc = "DSI_ISR0 (r) register accessor: DSI Host interrupt and status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_isr0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_isr0`]
module"]
pub type DSI_ISR0 = crate::Reg<dsi_isr0::DSI_ISR0rs>;
#[doc = "DSI Host interrupt and status register 0"]
pub mod dsi_isr0;
#[doc = "DSI_ISR1 (r) register accessor: DSI Host interrupt and status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_isr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_isr1`]
module"]
pub type DSI_ISR1 = crate::Reg<dsi_isr1::DSI_ISR1rs>;
#[doc = "DSI Host interrupt and status register 1"]
pub mod dsi_isr1;
#[doc = "DSI_IER0 (rw) register accessor: DSI Host interrupt enable register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ier0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ier0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ier0`]
module"]
pub type DSI_IER0 = crate::Reg<dsi_ier0::DSI_IER0rs>;
#[doc = "DSI Host interrupt enable register 0"]
pub mod dsi_ier0;
#[doc = "DSI_IER1 (rw) register accessor: DSI Host interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ier1`]
module"]
pub type DSI_IER1 = crate::Reg<dsi_ier1::DSI_IER1rs>;
#[doc = "DSI Host interrupt enable register 1"]
pub mod dsi_ier1;
#[doc = "DSI_FIR0 (w) register accessor: DSI Host force interrupt register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_fir0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fir0`]
module"]
pub type DSI_FIR0 = crate::Reg<dsi_fir0::DSI_FIR0rs>;
#[doc = "DSI Host force interrupt register 0"]
pub mod dsi_fir0;
#[doc = "DSI_FIR1 (w) register accessor: DSI Host force interrupt register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_fir1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fir1`]
module"]
pub type DSI_FIR1 = crate::Reg<dsi_fir1::DSI_FIR1rs>;
#[doc = "DSI Host force interrupt register 1"]
pub mod dsi_fir1;
#[doc = "DSI_DLTRCR (rw) register accessor: DSI Host data lane timer read configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dltrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dltrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dltrcr`]
module"]
pub type DSI_DLTRCR = crate::Reg<dsi_dltrcr::DSI_DLTRCRrs>;
#[doc = "DSI Host data lane timer read configuration register"]
pub mod dsi_dltrcr;
#[doc = "DSI_VSCR (rw) register accessor: DSI Host video shadow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vscr`]
module"]
pub type DSI_VSCR = crate::Reg<dsi_vscr::DSI_VSCRrs>;
#[doc = "DSI Host video shadow control register"]
pub mod dsi_vscr;
#[doc = "DSI_LCVCIDR (rw) register accessor: DSI Host LTDC current VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcvcidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lcvcidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lcvcidr`]
module"]
pub type DSI_LCVCIDR = crate::Reg<dsi_lcvcidr::DSI_LCVCIDRrs>;
#[doc = "DSI Host LTDC current VCID register"]
pub mod dsi_lcvcidr;
#[doc = "DSI_LCCCR (r) register accessor: DSI Host LTDC current color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lcccr`]
module"]
pub type DSI_LCCCR = crate::Reg<dsi_lcccr::DSI_LCCCRrs>;
#[doc = "DSI Host LTDC current color coding register"]
pub mod dsi_lcccr;
#[doc = "DSI_LPMCCR (r) register accessor: DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpmccr`]
module"]
pub type DSI_LPMCCR = crate::Reg<dsi_lpmccr::DSI_LPMCCRrs>;
#[doc = "DSI Host low-power mode current configuration register"]
pub mod dsi_lpmccr;
#[doc = "DSI_VMCCR (r) register accessor: DSI Host video mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vmccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vmccr`]
module"]
pub type DSI_VMCCR = crate::Reg<dsi_vmccr::DSI_VMCCRrs>;
#[doc = "DSI Host video mode current configuration register"]
pub mod dsi_vmccr;
#[doc = "DSI_VPCCR (r) register accessor: DSI Host video packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vpccr`]
module"]
pub type DSI_VPCCR = crate::Reg<dsi_vpccr::DSI_VPCCRrs>;
#[doc = "DSI Host video packet current configuration register"]
pub mod dsi_vpccr;
#[doc = "DSI_VCCCR (r) register accessor: DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vcccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vcccr`]
module"]
pub type DSI_VCCCR = crate::Reg<dsi_vcccr::DSI_VCCCRrs>;
#[doc = "DSI Host video chunks current configuration register"]
pub mod dsi_vcccr;
#[doc = "DSI_VNPCCR (r) register accessor: DSI Host video null packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vnpccr`]
module"]
pub type DSI_VNPCCR = crate::Reg<dsi_vnpccr::DSI_VNPCCRrs>;
#[doc = "DSI Host video null packet current configuration register"]
pub mod dsi_vnpccr;
#[doc = "DSI_VHSACCR (r) register accessor: DSI Host video HSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhsaccr`]
module"]
pub type DSI_VHSACCR = crate::Reg<dsi_vhsaccr::DSI_VHSACCRrs>;
#[doc = "DSI Host video HSA current configuration register"]
pub mod dsi_vhsaccr;
#[doc = "DSI_VHBPCCR (r) register accessor: DSI Host video HBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhbpccr`]
module"]
pub type DSI_VHBPCCR = crate::Reg<dsi_vhbpccr::DSI_VHBPCCRrs>;
#[doc = "DSI Host video HBP current configuration register"]
pub mod dsi_vhbpccr;
#[doc = "DSI_VLCCR (r) register accessor: DSI Host video line current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vlccr`]
module"]
pub type DSI_VLCCR = crate::Reg<dsi_vlccr::DSI_VLCCRrs>;
#[doc = "DSI Host video line current configuration register"]
pub mod dsi_vlccr;
#[doc = "DSI_VVSACCR (r) register accessor: DSI Host video VSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvsaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvsaccr`]
module"]
pub type DSI_VVSACCR = crate::Reg<dsi_vvsaccr::DSI_VVSACCRrs>;
#[doc = "DSI Host video VSA current configuration register"]
pub mod dsi_vvsaccr;
#[doc = "DSI_VVBPCCR (r) register accessor: DSI Host video VBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvbpccr`]
module"]
pub type DSI_VVBPCCR = crate::Reg<dsi_vvbpccr::DSI_VVBPCCRrs>;
#[doc = "DSI Host video VBP current configuration register"]
pub mod dsi_vvbpccr;
#[doc = "DSI_VVFPCCR (r) register accessor: DSI Host video VFP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvfpccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvfpccr`]
module"]
pub type DSI_VVFPCCR = crate::Reg<dsi_vvfpccr::DSI_VVFPCCRrs>;
#[doc = "DSI Host video VFP current configuration register"]
pub mod dsi_vvfpccr;
#[doc = "DSI_VVACCR (r) register accessor: DSI Host video VA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvaccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvaccr`]
module"]
pub type DSI_VVACCR = crate::Reg<dsi_vvaccr::DSI_VVACCRrs>;
#[doc = "DSI Host video VA current configuration register"]
pub mod dsi_vvaccr;
#[doc = "DSI_FBSR (r) register accessor: DSI Host FIFO and buffer status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_fbsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fbsr`]
module"]
pub type DSI_FBSR = crate::Reg<dsi_fbsr::DSI_FBSRrs>;
#[doc = "DSI Host FIFO and buffer status register"]
pub mod dsi_fbsr;
#[doc = "DSI_WCFGR (rw) register accessor: DSI Wrapper configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wcfgr`]
module"]
pub type DSI_WCFGR = crate::Reg<dsi_wcfgr::DSI_WCFGRrs>;
#[doc = "DSI Wrapper configuration register"]
pub mod dsi_wcfgr;
#[doc = "DSI_WCR (rw) register accessor: DSI Wrapper control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wcr`]
module"]
pub type DSI_WCR = crate::Reg<dsi_wcr::DSI_WCRrs>;
#[doc = "DSI Wrapper control register"]
pub mod dsi_wcr;
#[doc = "DSI_WIER (rw) register accessor: DSI Wrapper interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wier`]
module"]
pub type DSI_WIER = crate::Reg<dsi_wier::DSI_WIERrs>;
#[doc = "DSI Wrapper interrupt enable register"]
pub mod dsi_wier;
#[doc = "DSI_WISR (r) register accessor: DSI Wrapper interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wisr`]
module"]
pub type DSI_WISR = crate::Reg<dsi_wisr::DSI_WISRrs>;
#[doc = "DSI Wrapper interrupt and status register"]
pub mod dsi_wisr;
#[doc = "DSI_WIFCR (w) register accessor: DSI Wrapper interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wifcr`]
module"]
pub type DSI_WIFCR = crate::Reg<dsi_wifcr::DSI_WIFCRrs>;
#[doc = "DSI Wrapper interrupt flag clear register"]
pub mod dsi_wifcr;
#[doc = "DSI_WPCR0 (rw) register accessor: DSI Wrapper PHY configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wpcr0`]
module"]
pub type DSI_WPCR0 = crate::Reg<dsi_wpcr0::DSI_WPCR0rs>;
#[doc = "DSI Wrapper PHY configuration register 0"]
pub mod dsi_wpcr0;
#[doc = "DSI_WRPCR (rw) register accessor: DSI Wrapper regulator and PLL control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wrpcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wrpcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wrpcr`]
module"]
pub type DSI_WRPCR = crate::Reg<dsi_wrpcr::DSI_WRPCRrs>;
#[doc = "DSI Wrapper regulator and PLL control register"]
pub mod dsi_wrpcr;
#[doc = "DSI_BCFGR (rw) register accessor: DSI bias configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_bcfgr`]
module"]
pub type DSI_BCFGR = crate::Reg<dsi_bcfgr::DSI_BCFGRrs>;
#[doc = "DSI bias configuration register"]
pub mod dsi_bcfgr;
#[doc = "DSI_DPCBCR (rw) register accessor: DSI D-PHY clock band control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpcbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpcbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpcbcr`]
module"]
pub type DSI_DPCBCR = crate::Reg<dsi_dpcbcr::DSI_DPCBCRrs>;
#[doc = "DSI D-PHY clock band control register"]
pub mod dsi_dpcbcr;
#[doc = "DSI_DPCSRCR (rw) register accessor: DSI D-PHY clock skew rate control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpcsrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpcsrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpcsrcr`]
module"]
pub type DSI_DPCSRCR = crate::Reg<dsi_dpcsrcr::DSI_DPCSRCRrs>;
#[doc = "DSI D-PHY clock skew rate control register"]
pub mod dsi_dpcsrcr;
#[doc = "DSI_DPDL0BCR (rw) register accessor: DSI D-PHY data lane 0 band control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpdl0bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpdl0bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpdl0bcr`]
module"]
pub type DSI_DPDL0BCR = crate::Reg<dsi_dpdl0bcr::DSI_DPDL0BCRrs>;
#[doc = "DSI D-PHY data lane 0 band control register"]
pub mod dsi_dpdl0bcr;
#[doc = "DSI_DPDL0SRCR (rw) register accessor: DSI D-PHY data lane 0 skew rate control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpdl0srcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpdl0srcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpdl0srcr`]
module"]
pub type DSI_DPDL0SRCR = crate::Reg<dsi_dpdl0srcr::DSI_DPDL0SRCRrs>;
#[doc = "DSI D-PHY data lane 0 skew rate control register"]
pub mod dsi_dpdl0srcr;
#[doc = "DSI_DPDL1BCR (rw) register accessor: DSI D-PHY data lane 1 band control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpdl1bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpdl1bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpdl1bcr`]
module"]
pub type DSI_DPDL1BCR = crate::Reg<dsi_dpdl1bcr::DSI_DPDL1BCRrs>;
#[doc = "DSI D-PHY data lane 1 band control register"]
pub mod dsi_dpdl1bcr;
#[doc = "DSI_DPDL1SRCR (rw) register accessor: DSI D-PHY data lane 1 skew rate control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_dpdl1srcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_dpdl1srcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dpdl1srcr`]
module"]
pub type DSI_DPDL1SRCR = crate::Reg<dsi_dpdl1srcr::DSI_DPDL1SRCRrs>;
#[doc = "DSI D-PHY data lane 1 skew rate control register"]
pub mod dsi_dpdl1srcr;
