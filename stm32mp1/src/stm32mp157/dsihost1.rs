#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    pub dsi_vr: DSI_VR,
    #[doc = "0x04 - DSI Host control register"]
    pub dsi_cr: DSI_CR,
    #[doc = "0x08 - DSI Host clock control register"]
    pub dsi_ccr: DSI_CCR,
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    pub dsi_lvcidr: DSI_LVCIDR,
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    pub dsi_lcolcr: DSI_LCOLCR,
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    pub dsi_lpcr: DSI_LPCR,
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    pub dsi_lpmcr: DSI_LPMCR,
    _reserved7: [u8; 16usize],
    #[doc = "0x2c - DSI Host protocol configuration register"]
    pub dsi_pcr: DSI_PCR,
    #[doc = "0x30 - DSI Host generic VCID register"]
    pub dsi_gvcidr: DSI_GVCIDR,
    #[doc = "0x34 - DSI Host mode configuration register"]
    pub dsi_mcr: DSI_MCR,
    #[doc = "0x38 - DSI Host video mode configuration register"]
    pub dsi_vmcr: DSI_VMCR,
    #[doc = "0x3c - DSI Host video packet configuration register"]
    pub dsi_vpcr: DSI_VPCR,
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    pub dsi_vccr: DSI_VCCR,
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    pub dsi_vnpcr: DSI_VNPCR,
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    pub dsi_vhsacr: DSI_VHSACR,
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    pub dsi_vhbpcr: DSI_VHBPCR,
    #[doc = "0x50 - DSI Host video line configuration register"]
    pub dsi_vlcr: DSI_VLCR,
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    pub dsi_vvsacr: DSI_VVSACR,
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    pub dsi_vvbpcr: DSI_VVBPCR,
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    pub dsi_vvfpcr: DSI_VVFPCR,
    #[doc = "0x60 - DSI Host video VA configuration register"]
    pub dsi_vvacr: DSI_VVACR,
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    pub dsi_lccr: DSI_LCCR,
    #[doc = "0x68 - DSI Host command mode configuration register"]
    pub dsi_cmcr: DSI_CMCR,
    #[doc = "0x6c - DSI Host generic header configuration register"]
    pub dsi_ghcr: DSI_GHCR,
    #[doc = "0x70 - DSI Host generic payload data register"]
    pub dsi_gpdr: DSI_GPDR,
    #[doc = "0x74 - DSI Host generic packet status register"]
    pub dsi_gpsr: DSI_GPSR,
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    pub dsi_tccr0: DSI_TCCR0,
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    pub dsi_tccr1: DSI_TCCR1,
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    pub dsi_tccr2: DSI_TCCR2,
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    pub dsi_tccr3: DSI_TCCR3,
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    pub dsi_tccr4: DSI_TCCR4,
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    pub dsi_tccr5: DSI_TCCR5,
    _reserved32: [u8; 4usize],
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    pub dsi_clcr: DSI_CLCR,
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    pub dsi_cltcr: DSI_CLTCR,
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    pub dsi_dltcr: DSI_DLTCR,
    #[doc = "0xa0 - DSI Host PHY control register"]
    pub dsi_pctlr: DSI_PCTLR,
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    pub dsi_pconfr: DSI_PCONFR,
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    pub dsi_pucr: DSI_PUCR,
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    pub dsi_pttcr: DSI_PTTCR,
    #[doc = "0xb0 - DSI Host PHY status register"]
    pub dsi_psr: DSI_PSR,
    _reserved40: [u8; 8usize],
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    pub dsi_isr0: DSI_ISR0,
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    pub dsi_isr1: DSI_ISR1,
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    pub dsi_ier0: DSI_IER0,
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    pub dsi_ier1: DSI_IER1,
    _reserved44: [u8; 12usize],
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    pub dsi_fir0: DSI_FIR0,
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    pub dsi_fir1: DSI_FIR1,
    _reserved46: [u8; 20usize],
    #[doc = "0xf4 - DSI Host data lane timer read configuration register"]
    pub dsi_dltrcr: DSI_DLTRCR,
    _reserved47: [u8; 8usize],
    #[doc = "0x100 - DSI Host video shadow control register"]
    pub dsi_vscr: DSI_VSCR,
    _reserved48: [u8; 8usize],
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    pub dsi_lcvcidr: DSI_LCVCIDR,
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    pub dsi_lcccr: DSI_LCCCR,
    _reserved50: [u8; 4usize],
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    pub dsi_lpmccr: DSI_LPMCCR,
    _reserved51: [u8; 28usize],
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    pub dsi_vmccr: DSI_VMCCR,
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    pub dsi_vpccr: DSI_VPCCR,
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    pub dsi_vcccr: DSI_VCCCR,
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    pub dsi_vnpccr: DSI_VNPCCR,
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    pub dsi_vhsaccr: DSI_VHSACCR,
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    pub dsi_vhbpccr: DSI_VHBPCCR,
    #[doc = "0x150 - DSI Host video line current configuration register"]
    pub dsi_vlccr: DSI_VLCCR,
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    pub dsi_vvsaccr: DSI_VVSACCR,
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    pub dsi_vvbpccr: DSI_VVBPCCR,
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    pub dsi_vvfpccr: DSI_VVFPCCR,
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    pub dsi_vvaccr: DSI_VVACCR,
    _reserved62: [u8; 668usize],
    #[doc = "0x400 - DSI wrapper configuration register"]
    pub dsi_wcfgr: DSI_WCFGR,
    #[doc = "0x404 - DSI wrapper control register"]
    pub dsi_wcr: DSI_WCR,
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    pub dsi_wier: DSI_WIER,
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    pub dsi_wisr: DSI_WISR,
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    pub dsi_wifcr: DSI_WIFCR,
    _reserved67: [u8; 4usize],
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    pub dsi_wpcr0: DSI_WPCR0,
    #[doc = "0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
    pub dsi_wpcr1: DSI_WPCR1,
    _reserved69: [u8; 16usize],
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    pub dsi_wrpcr: DSI_WRPCR,
    _reserved70: [u8; 956usize],
    #[doc = "0x7f0 - DSI Host hardware configuration register"]
    pub dsi_hwcfgr: DSI_HWCFGR,
    #[doc = "0x7f4 - DSI Host version register"]
    pub dsi_verr: DSI_VERR,
    #[doc = "0x7f8 - DSI Host identification register"]
    pub dsi_ipidr: DSI_IPIDR,
    #[doc = "0x7fc - DSI Host size identification register"]
    pub dsi_sidr: DSI_SIDR,
}
#[doc = "DSI Host version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vr](dsi_vr) module"]
pub type DSI_VR = crate::Reg<u32, _DSI_VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VR;
#[doc = "`read()` method returns [dsi_vr::R](dsi_vr::R) reader structure"]
impl crate::Readable for DSI_VR {}
#[doc = "DSI Host version register"]
pub mod dsi_vr;
#[doc = "DSI Host control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_cr](dsi_cr) module"]
pub type DSI_CR = crate::Reg<u32, _DSI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CR;
#[doc = "`read()` method returns [dsi_cr::R](dsi_cr::R) reader structure"]
impl crate::Readable for DSI_CR {}
#[doc = "`write(|w| ..)` method takes [dsi_cr::W](dsi_cr::W) writer structure"]
impl crate::Writable for DSI_CR {}
#[doc = "DSI Host control register"]
pub mod dsi_cr;
#[doc = "DSI Host clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_ccr](dsi_ccr) module"]
pub type DSI_CCR = crate::Reg<u32, _DSI_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CCR;
#[doc = "`read()` method returns [dsi_ccr::R](dsi_ccr::R) reader structure"]
impl crate::Readable for DSI_CCR {}
#[doc = "`write(|w| ..)` method takes [dsi_ccr::W](dsi_ccr::W) writer structure"]
impl crate::Writable for DSI_CCR {}
#[doc = "DSI Host clock control register"]
pub mod dsi_ccr;
#[doc = "DSI Host LTDC VCID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lvcidr](dsi_lvcidr) module"]
pub type DSI_LVCIDR = crate::Reg<u32, _DSI_LVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LVCIDR;
#[doc = "`read()` method returns [dsi_lvcidr::R](dsi_lvcidr::R) reader structure"]
impl crate::Readable for DSI_LVCIDR {}
#[doc = "`write(|w| ..)` method takes [dsi_lvcidr::W](dsi_lvcidr::W) writer structure"]
impl crate::Writable for DSI_LVCIDR {}
#[doc = "DSI Host LTDC VCID register"]
pub mod dsi_lvcidr;
#[doc = "DSI Host LTDC color coding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lcolcr](dsi_lcolcr) module"]
pub type DSI_LCOLCR = crate::Reg<u32, _DSI_LCOLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCOLCR;
#[doc = "`read()` method returns [dsi_lcolcr::R](dsi_lcolcr::R) reader structure"]
impl crate::Readable for DSI_LCOLCR {}
#[doc = "`write(|w| ..)` method takes [dsi_lcolcr::W](dsi_lcolcr::W) writer structure"]
impl crate::Writable for DSI_LCOLCR {}
#[doc = "DSI Host LTDC color coding register"]
pub mod dsi_lcolcr;
#[doc = "DSI Host LTDC polarity configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lpcr](dsi_lpcr) module"]
pub type DSI_LPCR = crate::Reg<u32, _DSI_LPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LPCR;
#[doc = "`read()` method returns [dsi_lpcr::R](dsi_lpcr::R) reader structure"]
impl crate::Readable for DSI_LPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_lpcr::W](dsi_lpcr::W) writer structure"]
impl crate::Writable for DSI_LPCR {}
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod dsi_lpcr;
#[doc = "DSI Host low-power mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lpmcr](dsi_lpmcr) module"]
pub type DSI_LPMCR = crate::Reg<u32, _DSI_LPMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LPMCR;
#[doc = "`read()` method returns [dsi_lpmcr::R](dsi_lpmcr::R) reader structure"]
impl crate::Readable for DSI_LPMCR {}
#[doc = "`write(|w| ..)` method takes [dsi_lpmcr::W](dsi_lpmcr::W) writer structure"]
impl crate::Writable for DSI_LPMCR {}
#[doc = "DSI Host low-power mode configuration register"]
pub mod dsi_lpmcr;
#[doc = "DSI Host protocol configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_pcr](dsi_pcr) module"]
pub type DSI_PCR = crate::Reg<u32, _DSI_PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PCR;
#[doc = "`read()` method returns [dsi_pcr::R](dsi_pcr::R) reader structure"]
impl crate::Readable for DSI_PCR {}
#[doc = "`write(|w| ..)` method takes [dsi_pcr::W](dsi_pcr::W) writer structure"]
impl crate::Writable for DSI_PCR {}
#[doc = "DSI Host protocol configuration register"]
pub mod dsi_pcr;
#[doc = "DSI Host generic VCID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_gvcidr](dsi_gvcidr) module"]
pub type DSI_GVCIDR = crate::Reg<u32, _DSI_GVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GVCIDR;
#[doc = "`read()` method returns [dsi_gvcidr::R](dsi_gvcidr::R) reader structure"]
impl crate::Readable for DSI_GVCIDR {}
#[doc = "DSI Host generic VCID register"]
pub mod dsi_gvcidr;
#[doc = "DSI Host mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_mcr](dsi_mcr) module"]
pub type DSI_MCR = crate::Reg<u32, _DSI_MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_MCR;
#[doc = "`read()` method returns [dsi_mcr::R](dsi_mcr::R) reader structure"]
impl crate::Readable for DSI_MCR {}
#[doc = "`write(|w| ..)` method takes [dsi_mcr::W](dsi_mcr::W) writer structure"]
impl crate::Writable for DSI_MCR {}
#[doc = "DSI Host mode configuration register"]
pub mod dsi_mcr;
#[doc = "DSI Host video mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vmcr](dsi_vmcr) module"]
pub type DSI_VMCR = crate::Reg<u32, _DSI_VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VMCR;
#[doc = "`read()` method returns [dsi_vmcr::R](dsi_vmcr::R) reader structure"]
impl crate::Readable for DSI_VMCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vmcr::W](dsi_vmcr::W) writer structure"]
impl crate::Writable for DSI_VMCR {}
#[doc = "DSI Host video mode configuration register"]
pub mod dsi_vmcr;
#[doc = "DSI Host video packet configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vpcr](dsi_vpcr) module"]
pub type DSI_VPCR = crate::Reg<u32, _DSI_VPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VPCR;
#[doc = "`read()` method returns [dsi_vpcr::R](dsi_vpcr::R) reader structure"]
impl crate::Readable for DSI_VPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vpcr::W](dsi_vpcr::W) writer structure"]
impl crate::Writable for DSI_VPCR {}
#[doc = "DSI Host video packet configuration register"]
pub mod dsi_vpcr;
#[doc = "DSI Host video chunks configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vccr](dsi_vccr) module"]
pub type DSI_VCCR = crate::Reg<u32, _DSI_VCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VCCR;
#[doc = "`read()` method returns [dsi_vccr::R](dsi_vccr::R) reader structure"]
impl crate::Readable for DSI_VCCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vccr::W](dsi_vccr::W) writer structure"]
impl crate::Writable for DSI_VCCR {}
#[doc = "DSI Host video chunks configuration register"]
pub mod dsi_vccr;
#[doc = "DSI Host video null packet configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vnpcr](dsi_vnpcr) module"]
pub type DSI_VNPCR = crate::Reg<u32, _DSI_VNPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VNPCR;
#[doc = "`read()` method returns [dsi_vnpcr::R](dsi_vnpcr::R) reader structure"]
impl crate::Readable for DSI_VNPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vnpcr::W](dsi_vnpcr::W) writer structure"]
impl crate::Writable for DSI_VNPCR {}
#[doc = "DSI Host video null packet configuration register"]
pub mod dsi_vnpcr;
#[doc = "DSI Host video HSA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vhsacr](dsi_vhsacr) module"]
pub type DSI_VHSACR = crate::Reg<u32, _DSI_VHSACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHSACR;
#[doc = "`read()` method returns [dsi_vhsacr::R](dsi_vhsacr::R) reader structure"]
impl crate::Readable for DSI_VHSACR {}
#[doc = "`write(|w| ..)` method takes [dsi_vhsacr::W](dsi_vhsacr::W) writer structure"]
impl crate::Writable for DSI_VHSACR {}
#[doc = "DSI Host video HSA configuration register"]
pub mod dsi_vhsacr;
#[doc = "DSI Host video HBP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vhbpcr](dsi_vhbpcr) module"]
pub type DSI_VHBPCR = crate::Reg<u32, _DSI_VHBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHBPCR;
#[doc = "`read()` method returns [dsi_vhbpcr::R](dsi_vhbpcr::R) reader structure"]
impl crate::Readable for DSI_VHBPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vhbpcr::W](dsi_vhbpcr::W) writer structure"]
impl crate::Writable for DSI_VHBPCR {}
#[doc = "DSI Host video HBP configuration register"]
pub mod dsi_vhbpcr;
#[doc = "DSI Host video line configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vlcr](dsi_vlcr) module"]
pub type DSI_VLCR = crate::Reg<u32, _DSI_VLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VLCR;
#[doc = "`read()` method returns [dsi_vlcr::R](dsi_vlcr::R) reader structure"]
impl crate::Readable for DSI_VLCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vlcr::W](dsi_vlcr::W) writer structure"]
impl crate::Writable for DSI_VLCR {}
#[doc = "DSI Host video line configuration register"]
pub mod dsi_vlcr;
#[doc = "DSI Host video VSA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvsacr](dsi_vvsacr) module"]
pub type DSI_VVSACR = crate::Reg<u32, _DSI_VVSACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVSACR;
#[doc = "`read()` method returns [dsi_vvsacr::R](dsi_vvsacr::R) reader structure"]
impl crate::Readable for DSI_VVSACR {}
#[doc = "`write(|w| ..)` method takes [dsi_vvsacr::W](dsi_vvsacr::W) writer structure"]
impl crate::Writable for DSI_VVSACR {}
#[doc = "DSI Host video VSA configuration register"]
pub mod dsi_vvsacr;
#[doc = "DSI Host video VBP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvbpcr](dsi_vvbpcr) module"]
pub type DSI_VVBPCR = crate::Reg<u32, _DSI_VVBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVBPCR;
#[doc = "`read()` method returns [dsi_vvbpcr::R](dsi_vvbpcr::R) reader structure"]
impl crate::Readable for DSI_VVBPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vvbpcr::W](dsi_vvbpcr::W) writer structure"]
impl crate::Writable for DSI_VVBPCR {}
#[doc = "DSI Host video VBP configuration register"]
pub mod dsi_vvbpcr;
#[doc = "DSI Host video VFP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvfpcr](dsi_vvfpcr) module"]
pub type DSI_VVFPCR = crate::Reg<u32, _DSI_VVFPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVFPCR;
#[doc = "`read()` method returns [dsi_vvfpcr::R](dsi_vvfpcr::R) reader structure"]
impl crate::Readable for DSI_VVFPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vvfpcr::W](dsi_vvfpcr::W) writer structure"]
impl crate::Writable for DSI_VVFPCR {}
#[doc = "DSI Host video VFP configuration register"]
pub mod dsi_vvfpcr;
#[doc = "DSI Host video VA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvacr](dsi_vvacr) module"]
pub type DSI_VVACR = crate::Reg<u32, _DSI_VVACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVACR;
#[doc = "`read()` method returns [dsi_vvacr::R](dsi_vvacr::R) reader structure"]
impl crate::Readable for DSI_VVACR {}
#[doc = "`write(|w| ..)` method takes [dsi_vvacr::W](dsi_vvacr::W) writer structure"]
impl crate::Writable for DSI_VVACR {}
#[doc = "DSI Host video VA configuration register"]
pub mod dsi_vvacr;
#[doc = "DSI Host LTDC command configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lccr](dsi_lccr) module"]
pub type DSI_LCCR = crate::Reg<u32, _DSI_LCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCCR;
#[doc = "`read()` method returns [dsi_lccr::R](dsi_lccr::R) reader structure"]
impl crate::Readable for DSI_LCCR {}
#[doc = "`write(|w| ..)` method takes [dsi_lccr::W](dsi_lccr::W) writer structure"]
impl crate::Writable for DSI_LCCR {}
#[doc = "DSI Host LTDC command configuration register"]
pub mod dsi_lccr;
#[doc = "DSI Host command mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_cmcr](dsi_cmcr) module"]
pub type DSI_CMCR = crate::Reg<u32, _DSI_CMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CMCR;
#[doc = "`read()` method returns [dsi_cmcr::R](dsi_cmcr::R) reader structure"]
impl crate::Readable for DSI_CMCR {}
#[doc = "`write(|w| ..)` method takes [dsi_cmcr::W](dsi_cmcr::W) writer structure"]
impl crate::Writable for DSI_CMCR {}
#[doc = "DSI Host command mode configuration register"]
pub mod dsi_cmcr;
#[doc = "DSI Host generic header configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_ghcr](dsi_ghcr) module"]
pub type DSI_GHCR = crate::Reg<u32, _DSI_GHCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GHCR;
#[doc = "`read()` method returns [dsi_ghcr::R](dsi_ghcr::R) reader structure"]
impl crate::Readable for DSI_GHCR {}
#[doc = "`write(|w| ..)` method takes [dsi_ghcr::W](dsi_ghcr::W) writer structure"]
impl crate::Writable for DSI_GHCR {}
#[doc = "DSI Host generic header configuration register"]
pub mod dsi_ghcr;
#[doc = "DSI Host generic payload data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_gpdr](dsi_gpdr) module"]
pub type DSI_GPDR = crate::Reg<u32, _DSI_GPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GPDR;
#[doc = "`read()` method returns [dsi_gpdr::R](dsi_gpdr::R) reader structure"]
impl crate::Readable for DSI_GPDR {}
#[doc = "`write(|w| ..)` method takes [dsi_gpdr::W](dsi_gpdr::W) writer structure"]
impl crate::Writable for DSI_GPDR {}
#[doc = "DSI Host generic payload data register"]
pub mod dsi_gpdr;
#[doc = "DSI Host generic packet status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_gpsr](dsi_gpsr) module"]
pub type DSI_GPSR = crate::Reg<u32, _DSI_GPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GPSR;
#[doc = "`read()` method returns [dsi_gpsr::R](dsi_gpsr::R) reader structure"]
impl crate::Readable for DSI_GPSR {}
#[doc = "DSI Host generic packet status register"]
pub mod dsi_gpsr;
#[doc = "DSI Host timeout counter configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr0](dsi_tccr0) module"]
pub type DSI_TCCR0 = crate::Reg<u32, _DSI_TCCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR0;
#[doc = "`read()` method returns [dsi_tccr0::R](dsi_tccr0::R) reader structure"]
impl crate::Readable for DSI_TCCR0 {}
#[doc = "`write(|w| ..)` method takes [dsi_tccr0::W](dsi_tccr0::W) writer structure"]
impl crate::Writable for DSI_TCCR0 {}
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod dsi_tccr0;
#[doc = "DSI Host timeout counter configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr1](dsi_tccr1) module"]
pub type DSI_TCCR1 = crate::Reg<u32, _DSI_TCCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR1;
#[doc = "`read()` method returns [dsi_tccr1::R](dsi_tccr1::R) reader structure"]
impl crate::Readable for DSI_TCCR1 {}
#[doc = "`write(|w| ..)` method takes [dsi_tccr1::W](dsi_tccr1::W) writer structure"]
impl crate::Writable for DSI_TCCR1 {}
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod dsi_tccr1;
#[doc = "DSI Host timeout counter configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr2](dsi_tccr2) module"]
pub type DSI_TCCR2 = crate::Reg<u32, _DSI_TCCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR2;
#[doc = "`read()` method returns [dsi_tccr2::R](dsi_tccr2::R) reader structure"]
impl crate::Readable for DSI_TCCR2 {}
#[doc = "`write(|w| ..)` method takes [dsi_tccr2::W](dsi_tccr2::W) writer structure"]
impl crate::Writable for DSI_TCCR2 {}
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod dsi_tccr2;
#[doc = "DSI Host timeout counter configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr3](dsi_tccr3) module"]
pub type DSI_TCCR3 = crate::Reg<u32, _DSI_TCCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR3;
#[doc = "`read()` method returns [dsi_tccr3::R](dsi_tccr3::R) reader structure"]
impl crate::Readable for DSI_TCCR3 {}
#[doc = "`write(|w| ..)` method takes [dsi_tccr3::W](dsi_tccr3::W) writer structure"]
impl crate::Writable for DSI_TCCR3 {}
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod dsi_tccr3;
#[doc = "DSI Host timeout counter configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr4](dsi_tccr4) module"]
pub type DSI_TCCR4 = crate::Reg<u32, _DSI_TCCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR4;
#[doc = "`read()` method returns [dsi_tccr4::R](dsi_tccr4::R) reader structure"]
impl crate::Readable for DSI_TCCR4 {}
#[doc = "`write(|w| ..)` method takes [dsi_tccr4::W](dsi_tccr4::W) writer structure"]
impl crate::Writable for DSI_TCCR4 {}
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod dsi_tccr4;
#[doc = "DSI Host timeout counter configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr5](dsi_tccr5) module"]
pub type DSI_TCCR5 = crate::Reg<u32, _DSI_TCCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR5;
#[doc = "`read()` method returns [dsi_tccr5::R](dsi_tccr5::R) reader structure"]
impl crate::Readable for DSI_TCCR5 {}
#[doc = "`write(|w| ..)` method takes [dsi_tccr5::W](dsi_tccr5::W) writer structure"]
impl crate::Writable for DSI_TCCR5 {}
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod dsi_tccr5;
#[doc = "DSI Host clock lane configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_clcr](dsi_clcr) module"]
pub type DSI_CLCR = crate::Reg<u32, _DSI_CLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CLCR;
#[doc = "`read()` method returns [dsi_clcr::R](dsi_clcr::R) reader structure"]
impl crate::Readable for DSI_CLCR {}
#[doc = "`write(|w| ..)` method takes [dsi_clcr::W](dsi_clcr::W) writer structure"]
impl crate::Writable for DSI_CLCR {}
#[doc = "DSI Host clock lane configuration register"]
pub mod dsi_clcr;
#[doc = "DSI Host clock lane timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_cltcr](dsi_cltcr) module"]
pub type DSI_CLTCR = crate::Reg<u32, _DSI_CLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CLTCR;
#[doc = "`read()` method returns [dsi_cltcr::R](dsi_cltcr::R) reader structure"]
impl crate::Readable for DSI_CLTCR {}
#[doc = "`write(|w| ..)` method takes [dsi_cltcr::W](dsi_cltcr::W) writer structure"]
impl crate::Writable for DSI_CLTCR {}
#[doc = "DSI Host clock lane timer configuration register"]
pub mod dsi_cltcr;
#[doc = "DSI Host data lane timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_dltcr](dsi_dltcr) module"]
pub type DSI_DLTCR = crate::Reg<u32, _DSI_DLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_DLTCR;
#[doc = "`read()` method returns [dsi_dltcr::R](dsi_dltcr::R) reader structure"]
impl crate::Readable for DSI_DLTCR {}
#[doc = "`write(|w| ..)` method takes [dsi_dltcr::W](dsi_dltcr::W) writer structure"]
impl crate::Writable for DSI_DLTCR {}
#[doc = "DSI Host data lane timer configuration register"]
pub mod dsi_dltcr;
#[doc = "DSI Host PHY control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_pctlr](dsi_pctlr) module"]
pub type DSI_PCTLR = crate::Reg<u32, _DSI_PCTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PCTLR;
#[doc = "`read()` method returns [dsi_pctlr::R](dsi_pctlr::R) reader structure"]
impl crate::Readable for DSI_PCTLR {}
#[doc = "`write(|w| ..)` method takes [dsi_pctlr::W](dsi_pctlr::W) writer structure"]
impl crate::Writable for DSI_PCTLR {}
#[doc = "DSI Host PHY control register"]
pub mod dsi_pctlr;
#[doc = "DSI Host PHY configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_pconfr](dsi_pconfr) module"]
pub type DSI_PCONFR = crate::Reg<u32, _DSI_PCONFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PCONFR;
#[doc = "`read()` method returns [dsi_pconfr::R](dsi_pconfr::R) reader structure"]
impl crate::Readable for DSI_PCONFR {}
#[doc = "`write(|w| ..)` method takes [dsi_pconfr::W](dsi_pconfr::W) writer structure"]
impl crate::Writable for DSI_PCONFR {}
#[doc = "DSI Host PHY configuration register"]
pub mod dsi_pconfr;
#[doc = "DSI Host PHY ULPS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_pucr](dsi_pucr) module"]
pub type DSI_PUCR = crate::Reg<u32, _DSI_PUCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PUCR;
#[doc = "`read()` method returns [dsi_pucr::R](dsi_pucr::R) reader structure"]
impl crate::Readable for DSI_PUCR {}
#[doc = "`write(|w| ..)` method takes [dsi_pucr::W](dsi_pucr::W) writer structure"]
impl crate::Writable for DSI_PUCR {}
#[doc = "DSI Host PHY ULPS control register"]
pub mod dsi_pucr;
#[doc = "DSI Host PHY TX triggers configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_pttcr](dsi_pttcr) module"]
pub type DSI_PTTCR = crate::Reg<u32, _DSI_PTTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PTTCR;
#[doc = "`read()` method returns [dsi_pttcr::R](dsi_pttcr::R) reader structure"]
impl crate::Readable for DSI_PTTCR {}
#[doc = "`write(|w| ..)` method takes [dsi_pttcr::W](dsi_pttcr::W) writer structure"]
impl crate::Writable for DSI_PTTCR {}
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod dsi_pttcr;
#[doc = "DSI Host PHY status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_psr](dsi_psr) module"]
pub type DSI_PSR = crate::Reg<u32, _DSI_PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PSR;
#[doc = "`read()` method returns [dsi_psr::R](dsi_psr::R) reader structure"]
impl crate::Readable for DSI_PSR {}
#[doc = "DSI Host PHY status register"]
pub mod dsi_psr;
#[doc = "DSI Host interrupt and status register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_isr0](dsi_isr0) module"]
pub type DSI_ISR0 = crate::Reg<u32, _DSI_ISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_ISR0;
#[doc = "`read()` method returns [dsi_isr0::R](dsi_isr0::R) reader structure"]
impl crate::Readable for DSI_ISR0 {}
#[doc = "DSI Host interrupt and status register 0"]
pub mod dsi_isr0;
#[doc = "DSI Host interrupt and status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_isr1](dsi_isr1) module"]
pub type DSI_ISR1 = crate::Reg<u32, _DSI_ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_ISR1;
#[doc = "`read()` method returns [dsi_isr1::R](dsi_isr1::R) reader structure"]
impl crate::Readable for DSI_ISR1 {}
#[doc = "DSI Host interrupt and status register 1"]
pub mod dsi_isr1;
#[doc = "DSI Host interrupt enable register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_ier0](dsi_ier0) module"]
pub type DSI_IER0 = crate::Reg<u32, _DSI_IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_IER0;
#[doc = "`read()` method returns [dsi_ier0::R](dsi_ier0::R) reader structure"]
impl crate::Readable for DSI_IER0 {}
#[doc = "`write(|w| ..)` method takes [dsi_ier0::W](dsi_ier0::W) writer structure"]
impl crate::Writable for DSI_IER0 {}
#[doc = "DSI Host interrupt enable register 0"]
pub mod dsi_ier0;
#[doc = "DSI Host interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_ier1](dsi_ier1) module"]
pub type DSI_IER1 = crate::Reg<u32, _DSI_IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_IER1;
#[doc = "`read()` method returns [dsi_ier1::R](dsi_ier1::R) reader structure"]
impl crate::Readable for DSI_IER1 {}
#[doc = "`write(|w| ..)` method takes [dsi_ier1::W](dsi_ier1::W) writer structure"]
impl crate::Writable for DSI_IER1 {}
#[doc = "DSI Host interrupt enable register 1"]
pub mod dsi_ier1;
#[doc = "DSI Host force interrupt register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_fir0](dsi_fir0) module"]
pub type DSI_FIR0 = crate::Reg<u32, _DSI_FIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_FIR0;
#[doc = "`write(|w| ..)` method takes [dsi_fir0::W](dsi_fir0::W) writer structure"]
impl crate::Writable for DSI_FIR0 {}
#[doc = "DSI Host force interrupt register 0"]
pub mod dsi_fir0;
#[doc = "DSI Host force interrupt register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_fir1](dsi_fir1) module"]
pub type DSI_FIR1 = crate::Reg<u32, _DSI_FIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_FIR1;
#[doc = "`write(|w| ..)` method takes [dsi_fir1::W](dsi_fir1::W) writer structure"]
impl crate::Writable for DSI_FIR1 {}
#[doc = "DSI Host force interrupt register 1"]
pub mod dsi_fir1;
#[doc = "DSI Host data lane timer read configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_dltrcr](dsi_dltrcr) module"]
pub type DSI_DLTRCR = crate::Reg<u32, _DSI_DLTRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_DLTRCR;
#[doc = "`read()` method returns [dsi_dltrcr::R](dsi_dltrcr::R) reader structure"]
impl crate::Readable for DSI_DLTRCR {}
#[doc = "`write(|w| ..)` method takes [dsi_dltrcr::W](dsi_dltrcr::W) writer structure"]
impl crate::Writable for DSI_DLTRCR {}
#[doc = "DSI Host data lane timer read configuration register"]
pub mod dsi_dltrcr;
#[doc = "DSI Host video shadow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vscr](dsi_vscr) module"]
pub type DSI_VSCR = crate::Reg<u32, _DSI_VSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VSCR;
#[doc = "`read()` method returns [dsi_vscr::R](dsi_vscr::R) reader structure"]
impl crate::Readable for DSI_VSCR {}
#[doc = "`write(|w| ..)` method takes [dsi_vscr::W](dsi_vscr::W) writer structure"]
impl crate::Writable for DSI_VSCR {}
#[doc = "DSI Host video shadow control register"]
pub mod dsi_vscr;
#[doc = "DSI Host LTDC current VCID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lcvcidr](dsi_lcvcidr) module"]
pub type DSI_LCVCIDR = crate::Reg<u32, _DSI_LCVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCVCIDR;
#[doc = "`read()` method returns [dsi_lcvcidr::R](dsi_lcvcidr::R) reader structure"]
impl crate::Readable for DSI_LCVCIDR {}
#[doc = "`write(|w| ..)` method takes [dsi_lcvcidr::W](dsi_lcvcidr::W) writer structure"]
impl crate::Writable for DSI_LCVCIDR {}
#[doc = "DSI Host LTDC current VCID register"]
pub mod dsi_lcvcidr;
#[doc = "DSI Host LTDC current color coding register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lcccr](dsi_lcccr) module"]
pub type DSI_LCCCR = crate::Reg<u32, _DSI_LCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCCCR;
#[doc = "`read()` method returns [dsi_lcccr::R](dsi_lcccr::R) reader structure"]
impl crate::Readable for DSI_LCCCR {}
#[doc = "DSI Host LTDC current color coding register"]
pub mod dsi_lcccr;
#[doc = "DSI Host low-power mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_lpmccr](dsi_lpmccr) module"]
pub type DSI_LPMCCR = crate::Reg<u32, _DSI_LPMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LPMCCR;
#[doc = "`read()` method returns [dsi_lpmccr::R](dsi_lpmccr::R) reader structure"]
impl crate::Readable for DSI_LPMCCR {}
#[doc = "DSI Host low-power mode current configuration register"]
pub mod dsi_lpmccr;
#[doc = "DSI Host video mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vmccr](dsi_vmccr) module"]
pub type DSI_VMCCR = crate::Reg<u32, _DSI_VMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VMCCR;
#[doc = "`read()` method returns [dsi_vmccr::R](dsi_vmccr::R) reader structure"]
impl crate::Readable for DSI_VMCCR {}
#[doc = "DSI Host video mode current configuration register"]
pub mod dsi_vmccr;
#[doc = "DSI Host video packet current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vpccr](dsi_vpccr) module"]
pub type DSI_VPCCR = crate::Reg<u32, _DSI_VPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VPCCR;
#[doc = "`read()` method returns [dsi_vpccr::R](dsi_vpccr::R) reader structure"]
impl crate::Readable for DSI_VPCCR {}
#[doc = "DSI Host video packet current configuration register"]
pub mod dsi_vpccr;
#[doc = "DSI Host video chunks current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vcccr](dsi_vcccr) module"]
pub type DSI_VCCCR = crate::Reg<u32, _DSI_VCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VCCCR;
#[doc = "`read()` method returns [dsi_vcccr::R](dsi_vcccr::R) reader structure"]
impl crate::Readable for DSI_VCCCR {}
#[doc = "DSI Host video chunks current configuration register"]
pub mod dsi_vcccr;
#[doc = "DSI Host video null packet current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vnpccr](dsi_vnpccr) module"]
pub type DSI_VNPCCR = crate::Reg<u32, _DSI_VNPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VNPCCR;
#[doc = "`read()` method returns [dsi_vnpccr::R](dsi_vnpccr::R) reader structure"]
impl crate::Readable for DSI_VNPCCR {}
#[doc = "DSI Host video null packet current configuration register"]
pub mod dsi_vnpccr;
#[doc = "DSI Host video HSA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vhsaccr](dsi_vhsaccr) module"]
pub type DSI_VHSACCR = crate::Reg<u32, _DSI_VHSACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHSACCR;
#[doc = "`read()` method returns [dsi_vhsaccr::R](dsi_vhsaccr::R) reader structure"]
impl crate::Readable for DSI_VHSACCR {}
#[doc = "DSI Host video HSA current configuration register"]
pub mod dsi_vhsaccr;
#[doc = "DSI Host video HBP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vhbpccr](dsi_vhbpccr) module"]
pub type DSI_VHBPCCR = crate::Reg<u32, _DSI_VHBPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHBPCCR;
#[doc = "`read()` method returns [dsi_vhbpccr::R](dsi_vhbpccr::R) reader structure"]
impl crate::Readable for DSI_VHBPCCR {}
#[doc = "DSI Host video HBP current configuration register"]
pub mod dsi_vhbpccr;
#[doc = "DSI Host video line current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vlccr](dsi_vlccr) module"]
pub type DSI_VLCCR = crate::Reg<u32, _DSI_VLCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VLCCR;
#[doc = "`read()` method returns [dsi_vlccr::R](dsi_vlccr::R) reader structure"]
impl crate::Readable for DSI_VLCCR {}
#[doc = "DSI Host video line current configuration register"]
pub mod dsi_vlccr;
#[doc = "DSI Host video VSA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvsaccr](dsi_vvsaccr) module"]
pub type DSI_VVSACCR = crate::Reg<u32, _DSI_VVSACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVSACCR;
#[doc = "`read()` method returns [dsi_vvsaccr::R](dsi_vvsaccr::R) reader structure"]
impl crate::Readable for DSI_VVSACCR {}
#[doc = "DSI Host video VSA current configuration register"]
pub mod dsi_vvsaccr;
#[doc = "DSI Host video VBP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvbpccr](dsi_vvbpccr) module"]
pub type DSI_VVBPCCR = crate::Reg<u32, _DSI_VVBPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVBPCCR;
#[doc = "`read()` method returns [dsi_vvbpccr::R](dsi_vvbpccr::R) reader structure"]
impl crate::Readable for DSI_VVBPCCR {}
#[doc = "DSI Host video VBP current configuration register"]
pub mod dsi_vvbpccr;
#[doc = "DSI Host video VFP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvfpccr](dsi_vvfpccr) module"]
pub type DSI_VVFPCCR = crate::Reg<u32, _DSI_VVFPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVFPCCR;
#[doc = "`read()` method returns [dsi_vvfpccr::R](dsi_vvfpccr::R) reader structure"]
impl crate::Readable for DSI_VVFPCCR {}
#[doc = "DSI Host video VFP current configuration register"]
pub mod dsi_vvfpccr;
#[doc = "DSI Host video VA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvaccr](dsi_vvaccr) module"]
pub type DSI_VVACCR = crate::Reg<u32, _DSI_VVACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVACCR;
#[doc = "`read()` method returns [dsi_vvaccr::R](dsi_vvaccr::R) reader structure"]
impl crate::Readable for DSI_VVACCR {}
#[doc = "DSI Host video VA current configuration register"]
pub mod dsi_vvaccr;
#[doc = "DSI wrapper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wcfgr](dsi_wcfgr) module"]
pub type DSI_WCFGR = crate::Reg<u32, _DSI_WCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WCFGR;
#[doc = "`read()` method returns [dsi_wcfgr::R](dsi_wcfgr::R) reader structure"]
impl crate::Readable for DSI_WCFGR {}
#[doc = "`write(|w| ..)` method takes [dsi_wcfgr::W](dsi_wcfgr::W) writer structure"]
impl crate::Writable for DSI_WCFGR {}
#[doc = "DSI wrapper configuration register"]
pub mod dsi_wcfgr;
#[doc = "DSI wrapper control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wcr](dsi_wcr) module"]
pub type DSI_WCR = crate::Reg<u32, _DSI_WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WCR;
#[doc = "`read()` method returns [dsi_wcr::R](dsi_wcr::R) reader structure"]
impl crate::Readable for DSI_WCR {}
#[doc = "`write(|w| ..)` method takes [dsi_wcr::W](dsi_wcr::W) writer structure"]
impl crate::Writable for DSI_WCR {}
#[doc = "DSI wrapper control register"]
pub mod dsi_wcr;
#[doc = "DSI wrapper interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wier](dsi_wier) module"]
pub type DSI_WIER = crate::Reg<u32, _DSI_WIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WIER;
#[doc = "`read()` method returns [dsi_wier::R](dsi_wier::R) reader structure"]
impl crate::Readable for DSI_WIER {}
#[doc = "`write(|w| ..)` method takes [dsi_wier::W](dsi_wier::W) writer structure"]
impl crate::Writable for DSI_WIER {}
#[doc = "DSI wrapper interrupt enable register"]
pub mod dsi_wier;
#[doc = "DSI wrapper interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wisr](dsi_wisr) module"]
pub type DSI_WISR = crate::Reg<u32, _DSI_WISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WISR;
#[doc = "`read()` method returns [dsi_wisr::R](dsi_wisr::R) reader structure"]
impl crate::Readable for DSI_WISR {}
#[doc = "DSI wrapper interrupt and status register"]
pub mod dsi_wisr;
#[doc = "DSI wrapper interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wifcr](dsi_wifcr) module"]
pub type DSI_WIFCR = crate::Reg<u32, _DSI_WIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WIFCR;
#[doc = "`write(|w| ..)` method takes [dsi_wifcr::W](dsi_wifcr::W) writer structure"]
impl crate::Writable for DSI_WIFCR {}
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod dsi_wifcr;
#[doc = "DSI wrapper PHY configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wpcr0](dsi_wpcr0) module"]
pub type DSI_WPCR0 = crate::Reg<u32, _DSI_WPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR0;
#[doc = "`read()` method returns [dsi_wpcr0::R](dsi_wpcr0::R) reader structure"]
impl crate::Readable for DSI_WPCR0 {}
#[doc = "`write(|w| ..)` method takes [dsi_wpcr0::W](dsi_wpcr0::W) writer structure"]
impl crate::Writable for DSI_WPCR0 {}
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod dsi_wpcr0;
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wpcr1](dsi_wpcr1) module"]
pub type DSI_WPCR1 = crate::Reg<u32, _DSI_WPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR1;
#[doc = "`read()` method returns [dsi_wpcr1::R](dsi_wpcr1::R) reader structure"]
impl crate::Readable for DSI_WPCR1 {}
#[doc = "`write(|w| ..)` method takes [dsi_wpcr1::W](dsi_wpcr1::W) writer structure"]
impl crate::Writable for DSI_WPCR1 {}
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
pub mod dsi_wpcr1;
#[doc = "DSI wrapper regulator and PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_wrpcr](dsi_wrpcr) module"]
pub type DSI_WRPCR = crate::Reg<u32, _DSI_WRPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WRPCR;
#[doc = "`read()` method returns [dsi_wrpcr::R](dsi_wrpcr::R) reader structure"]
impl crate::Readable for DSI_WRPCR {}
#[doc = "`write(|w| ..)` method takes [dsi_wrpcr::W](dsi_wrpcr::W) writer structure"]
impl crate::Writable for DSI_WRPCR {}
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod dsi_wrpcr;
#[doc = "DSI Host hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_hwcfgr](dsi_hwcfgr) module"]
pub type DSI_HWCFGR = crate::Reg<u32, _DSI_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_HWCFGR;
#[doc = "`read()` method returns [dsi_hwcfgr::R](dsi_hwcfgr::R) reader structure"]
impl crate::Readable for DSI_HWCFGR {}
#[doc = "DSI Host hardware configuration register"]
pub mod dsi_hwcfgr;
#[doc = "DSI Host version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_verr](dsi_verr) module"]
pub type DSI_VERR = crate::Reg<u32, _DSI_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VERR;
#[doc = "`read()` method returns [dsi_verr::R](dsi_verr::R) reader structure"]
impl crate::Readable for DSI_VERR {}
#[doc = "DSI Host version register"]
pub mod dsi_verr;
#[doc = "DSI Host identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_ipidr](dsi_ipidr) module"]
pub type DSI_IPIDR = crate::Reg<u32, _DSI_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_IPIDR;
#[doc = "`read()` method returns [dsi_ipidr::R](dsi_ipidr::R) reader structure"]
impl crate::Readable for DSI_IPIDR {}
#[doc = "DSI Host identification register"]
pub mod dsi_ipidr;
#[doc = "DSI Host size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_sidr](dsi_sidr) module"]
pub type DSI_SIDR = crate::Reg<u32, _DSI_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_SIDR;
#[doc = "`read()` method returns [dsi_sidr::R](dsi_sidr::R) reader structure"]
impl crate::Readable for DSI_SIDR {}
#[doc = "DSI Host size identification register"]
pub mod dsi_sidr;
