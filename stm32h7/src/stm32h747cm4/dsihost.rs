#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    pub vr: VR,
    #[doc = "0x04 - DSI Host control register"]
    pub cr: CR,
    #[doc = "0x08 - DSI Host clock control register"]
    pub ccr: CCR,
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    pub lvcidr: LVCIDR,
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    pub lcolcr: LCOLCR,
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    pub lpcr: LPCR,
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    pub lpmcr: LPMCR,
    _reserved7: [u8; 16usize],
    #[doc = "0x2c - DSI Host protocol configuration register"]
    pub pcr: PCR,
    #[doc = "0x30 - DSI Host generic VCID register"]
    pub gvcidr: GVCIDR,
    #[doc = "0x34 - DSI Host mode configuration register"]
    pub mcr: MCR,
    #[doc = "0x38 - DSI Host video mode configuration register"]
    pub vmcr: VMCR,
    #[doc = "0x3c - DSI Host video packet configuration register"]
    pub vpcr: VPCR,
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    pub vccr: VCCR,
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    pub vnpcr: VNPCR,
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    pub vhsacr: VHSACR,
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    pub vhbpcr: VHBPCR,
    #[doc = "0x50 - DSI Host video line configuration register"]
    pub vlcr: VLCR,
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    pub vvsacr: VVSACR,
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    pub vvbpcr: VVBPCR,
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    pub vvfpcr: VVFPCR,
    #[doc = "0x60 - DSI Host video VA configuration register"]
    pub vvacr: VVACR,
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    pub lccr: LCCR,
    #[doc = "0x68 - DSI Host command mode configuration register"]
    pub cmcr: CMCR,
    #[doc = "0x6c - DSI Host generic header configuration register"]
    pub ghcr: GHCR,
    #[doc = "0x70 - DSI Host generic payload data register"]
    pub gpdr: GPDR,
    #[doc = "0x74 - DSI Host generic packet status register"]
    pub gpsr: GPSR,
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    pub tccr0: TCCR0,
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    pub tccr1: TCCR1,
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    pub tccr2: TCCR2,
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    pub tccr3: TCCR3,
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    pub tccr4: TCCR4,
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    pub tccr5: TCCR5,
    _reserved32: [u8; 4usize],
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    pub clcr: CLCR,
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    pub cltcr: CLTCR,
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    pub dltcr: DLTCR,
    #[doc = "0xa0 - DSI Host PHY control register"]
    pub pctlr: PCTLR,
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    pub pconfr: PCONFR,
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    pub pucr: PUCR,
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    pub pttcr: PTTCR,
    _reserved39: [u8; 12usize],
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    pub psr: PSR,
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    pub isr1: ISR1,
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    pub ier0: IER0,
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    pub ier1: IER1,
    _reserved43: [u8; 12usize],
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    pub fir0: FIR0,
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    pub fir1: FIR1,
    _reserved45: [u8; 32usize],
    #[doc = "0x100 - DSI Host video shadow control register"]
    pub vscr: VSCR,
    _reserved46: [u8; 8usize],
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    pub lcvcidr: LCVCIDR,
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    pub lcccr: LCCCR,
    _reserved48: [u8; 4usize],
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    pub lpmccr: LPMCCR,
    _reserved49: [u8; 28usize],
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    pub vmccr: VMCCR,
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    pub vpccr: VPCCR,
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    pub vcccr: VCCCR,
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    pub vnpccr: VNPCCR,
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    pub vhsaccr: VHSACCR,
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    pub vhbpccr: VHBPCCR,
    #[doc = "0x150 - DSI Host video line current configuration register"]
    pub vlccr: VLCCR,
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    pub vvsaccr: VVSACCR,
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    pub vvpbccr: VVPBCCR,
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    pub vvfpccr: VVFPCCR,
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    pub vvaccr: VVACCR,
    _reserved60: [u8; 668usize],
    #[doc = "0x400 - DSI wrapper configuration register"]
    pub wcfgr: WCFGR,
    #[doc = "0x404 - DSI wrapper control register"]
    pub wcr: WCR,
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    pub wier: WIER,
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    pub wisr: WISR,
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    pub wifcr: WIFCR,
    _reserved65: [u8; 4usize],
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    pub wpcr0: WPCR0,
    #[doc = "0x41c - DSI wrapper PHY configuration register 1"]
    pub wpcr1: WPCR1,
    _reserved67: [u8; 8usize],
    #[doc = "0x428 - DSI wrapper PHY configuration register 4"]
    pub wpcr2: WPCR2,
    _reserved68: [u8; 4usize],
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    pub wrpcr: WRPCR,
}
#[doc = "DSI Host version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vr](vr) module"]
pub type VR = crate::Reg<u32, _VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VR;
#[doc = "`read()` method returns [vr::R](vr::R) reader structure"]
impl crate::Readable for VR {}
#[doc = "DSI Host version register"]
pub mod vr;
#[doc = "DSI Host control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "DSI Host control register"]
pub mod cr;
#[doc = "DSI Host clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "DSI Host clock control register"]
pub mod ccr;
#[doc = "DSI Host LTDC VCID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvcidr](lvcidr) module"]
pub type LVCIDR = crate::Reg<u32, _LVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LVCIDR;
#[doc = "`read()` method returns [lvcidr::R](lvcidr::R) reader structure"]
impl crate::Readable for LVCIDR {}
#[doc = "`write(|w| ..)` method takes [lvcidr::W](lvcidr::W) writer structure"]
impl crate::Writable for LVCIDR {}
#[doc = "DSI Host LTDC VCID register"]
pub mod lvcidr;
#[doc = "DSI Host LTDC color coding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcolcr](lcolcr) module"]
pub type LCOLCR = crate::Reg<u32, _LCOLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCOLCR;
#[doc = "`read()` method returns [lcolcr::R](lcolcr::R) reader structure"]
impl crate::Readable for LCOLCR {}
#[doc = "`write(|w| ..)` method takes [lcolcr::W](lcolcr::W) writer structure"]
impl crate::Writable for LCOLCR {}
#[doc = "DSI Host LTDC color coding register"]
pub mod lcolcr;
#[doc = "DSI Host LTDC polarity configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](lpcr) module"]
pub type LPCR = crate::Reg<u32, _LPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCR;
#[doc = "`read()` method returns [lpcr::R](lpcr::R) reader structure"]
impl crate::Readable for LPCR {}
#[doc = "`write(|w| ..)` method takes [lpcr::W](lpcr::W) writer structure"]
impl crate::Writable for LPCR {}
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod lpcr;
#[doc = "DSI Host low-power mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcr](lpmcr) module"]
pub type LPMCR = crate::Reg<u32, _LPMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCR;
#[doc = "`read()` method returns [lpmcr::R](lpmcr::R) reader structure"]
impl crate::Readable for LPMCR {}
#[doc = "`write(|w| ..)` method takes [lpmcr::W](lpmcr::W) writer structure"]
impl crate::Writable for LPMCR {}
#[doc = "DSI Host low-power mode configuration register"]
pub mod lpmcr;
#[doc = "DSI Host protocol configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "DSI Host protocol configuration register"]
pub mod pcr;
#[doc = "DSI Host generic VCID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gvcidr](gvcidr) module"]
pub type GVCIDR = crate::Reg<u32, _GVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GVCIDR;
#[doc = "`read()` method returns [gvcidr::R](gvcidr::R) reader structure"]
impl crate::Readable for GVCIDR {}
#[doc = "`write(|w| ..)` method takes [gvcidr::W](gvcidr::W) writer structure"]
impl crate::Writable for GVCIDR {}
#[doc = "DSI Host generic VCID register"]
pub mod gvcidr;
#[doc = "DSI Host mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "DSI Host mode configuration register"]
pub mod mcr;
#[doc = "DSI Host video mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmcr](vmcr) module"]
pub type VMCR = crate::Reg<u32, _VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMCR;
#[doc = "`read()` method returns [vmcr::R](vmcr::R) reader structure"]
impl crate::Readable for VMCR {}
#[doc = "`write(|w| ..)` method takes [vmcr::W](vmcr::W) writer structure"]
impl crate::Writable for VMCR {}
#[doc = "DSI Host video mode configuration register"]
pub mod vmcr;
#[doc = "DSI Host video packet configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vpcr](vpcr) module"]
pub type VPCR = crate::Reg<u32, _VPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VPCR;
#[doc = "`read()` method returns [vpcr::R](vpcr::R) reader structure"]
impl crate::Readable for VPCR {}
#[doc = "`write(|w| ..)` method takes [vpcr::W](vpcr::W) writer structure"]
impl crate::Writable for VPCR {}
#[doc = "DSI Host video packet configuration register"]
pub mod vpcr;
#[doc = "DSI Host video chunks configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vccr](vccr) module"]
pub type VCCR = crate::Reg<u32, _VCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCCR;
#[doc = "`read()` method returns [vccr::R](vccr::R) reader structure"]
impl crate::Readable for VCCR {}
#[doc = "`write(|w| ..)` method takes [vccr::W](vccr::W) writer structure"]
impl crate::Writable for VCCR {}
#[doc = "DSI Host video chunks configuration register"]
pub mod vccr;
#[doc = "DSI Host video null packet configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vnpcr](vnpcr) module"]
pub type VNPCR = crate::Reg<u32, _VNPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VNPCR;
#[doc = "`read()` method returns [vnpcr::R](vnpcr::R) reader structure"]
impl crate::Readable for VNPCR {}
#[doc = "`write(|w| ..)` method takes [vnpcr::W](vnpcr::W) writer structure"]
impl crate::Writable for VNPCR {}
#[doc = "DSI Host video null packet configuration register"]
pub mod vnpcr;
#[doc = "DSI Host video HSA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhsacr](vhsacr) module"]
pub type VHSACR = crate::Reg<u32, _VHSACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VHSACR;
#[doc = "`read()` method returns [vhsacr::R](vhsacr::R) reader structure"]
impl crate::Readable for VHSACR {}
#[doc = "`write(|w| ..)` method takes [vhsacr::W](vhsacr::W) writer structure"]
impl crate::Writable for VHSACR {}
#[doc = "DSI Host video HSA configuration register"]
pub mod vhsacr;
#[doc = "DSI Host video HBP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhbpcr](vhbpcr) module"]
pub type VHBPCR = crate::Reg<u32, _VHBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VHBPCR;
#[doc = "`read()` method returns [vhbpcr::R](vhbpcr::R) reader structure"]
impl crate::Readable for VHBPCR {}
#[doc = "`write(|w| ..)` method takes [vhbpcr::W](vhbpcr::W) writer structure"]
impl crate::Writable for VHBPCR {}
#[doc = "DSI Host video HBP configuration register"]
pub mod vhbpcr;
#[doc = "DSI Host video line configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlcr](vlcr) module"]
pub type VLCR = crate::Reg<u32, _VLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLCR;
#[doc = "`read()` method returns [vlcr::R](vlcr::R) reader structure"]
impl crate::Readable for VLCR {}
#[doc = "`write(|w| ..)` method takes [vlcr::W](vlcr::W) writer structure"]
impl crate::Writable for VLCR {}
#[doc = "DSI Host video line configuration register"]
pub mod vlcr;
#[doc = "DSI Host video VSA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvsacr](vvsacr) module"]
pub type VVSACR = crate::Reg<u32, _VVSACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVSACR;
#[doc = "`read()` method returns [vvsacr::R](vvsacr::R) reader structure"]
impl crate::Readable for VVSACR {}
#[doc = "`write(|w| ..)` method takes [vvsacr::W](vvsacr::W) writer structure"]
impl crate::Writable for VVSACR {}
#[doc = "DSI Host video VSA configuration register"]
pub mod vvsacr;
#[doc = "DSI Host video VBP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvbpcr](vvbpcr) module"]
pub type VVBPCR = crate::Reg<u32, _VVBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVBPCR;
#[doc = "`read()` method returns [vvbpcr::R](vvbpcr::R) reader structure"]
impl crate::Readable for VVBPCR {}
#[doc = "`write(|w| ..)` method takes [vvbpcr::W](vvbpcr::W) writer structure"]
impl crate::Writable for VVBPCR {}
#[doc = "DSI Host video VBP configuration register"]
pub mod vvbpcr;
#[doc = "DSI Host video VFP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvfpcr](vvfpcr) module"]
pub type VVFPCR = crate::Reg<u32, _VVFPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVFPCR;
#[doc = "`read()` method returns [vvfpcr::R](vvfpcr::R) reader structure"]
impl crate::Readable for VVFPCR {}
#[doc = "`write(|w| ..)` method takes [vvfpcr::W](vvfpcr::W) writer structure"]
impl crate::Writable for VVFPCR {}
#[doc = "DSI Host video VFP configuration register"]
pub mod vvfpcr;
#[doc = "DSI Host video VA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvacr](vvacr) module"]
pub type VVACR = crate::Reg<u32, _VVACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVACR;
#[doc = "`read()` method returns [vvacr::R](vvacr::R) reader structure"]
impl crate::Readable for VVACR {}
#[doc = "`write(|w| ..)` method takes [vvacr::W](vvacr::W) writer structure"]
impl crate::Writable for VVACR {}
#[doc = "DSI Host video VA configuration register"]
pub mod vvacr;
#[doc = "DSI Host LTDC command configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccr](lccr) module"]
pub type LCCR = crate::Reg<u32, _LCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCCR;
#[doc = "`read()` method returns [lccr::R](lccr::R) reader structure"]
impl crate::Readable for LCCR {}
#[doc = "`write(|w| ..)` method takes [lccr::W](lccr::W) writer structure"]
impl crate::Writable for LCCR {}
#[doc = "DSI Host LTDC command configuration register"]
pub mod lccr;
#[doc = "DSI Host command mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcr](cmcr) module"]
pub type CMCR = crate::Reg<u32, _CMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMCR;
#[doc = "`read()` method returns [cmcr::R](cmcr::R) reader structure"]
impl crate::Readable for CMCR {}
#[doc = "`write(|w| ..)` method takes [cmcr::W](cmcr::W) writer structure"]
impl crate::Writable for CMCR {}
#[doc = "DSI Host command mode configuration register"]
pub mod cmcr;
#[doc = "DSI Host generic header configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghcr](ghcr) module"]
pub type GHCR = crate::Reg<u32, _GHCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GHCR;
#[doc = "`read()` method returns [ghcr::R](ghcr::R) reader structure"]
impl crate::Readable for GHCR {}
#[doc = "`write(|w| ..)` method takes [ghcr::W](ghcr::W) writer structure"]
impl crate::Writable for GHCR {}
#[doc = "DSI Host generic header configuration register"]
pub mod ghcr;
#[doc = "DSI Host generic payload data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdr](gpdr) module"]
pub type GPDR = crate::Reg<u32, _GPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPDR;
#[doc = "`read()` method returns [gpdr::R](gpdr::R) reader structure"]
impl crate::Readable for GPDR {}
#[doc = "`write(|w| ..)` method takes [gpdr::W](gpdr::W) writer structure"]
impl crate::Writable for GPDR {}
#[doc = "DSI Host generic payload data register"]
pub mod gpdr;
#[doc = "DSI Host generic packet status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpsr](gpsr) module"]
pub type GPSR = crate::Reg<u32, _GPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPSR;
#[doc = "`read()` method returns [gpsr::R](gpsr::R) reader structure"]
impl crate::Readable for GPSR {}
#[doc = "`write(|w| ..)` method takes [gpsr::W](gpsr::W) writer structure"]
impl crate::Writable for GPSR {}
#[doc = "DSI Host generic packet status register"]
pub mod gpsr;
#[doc = "DSI Host timeout counter configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr0](tccr0) module"]
pub type TCCR0 = crate::Reg<u32, _TCCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR0;
#[doc = "`read()` method returns [tccr0::R](tccr0::R) reader structure"]
impl crate::Readable for TCCR0 {}
#[doc = "`write(|w| ..)` method takes [tccr0::W](tccr0::W) writer structure"]
impl crate::Writable for TCCR0 {}
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod tccr0;
#[doc = "DSI Host timeout counter configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1](tccr1) module"]
pub type TCCR1 = crate::Reg<u32, _TCCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR1;
#[doc = "`read()` method returns [tccr1::R](tccr1::R) reader structure"]
impl crate::Readable for TCCR1 {}
#[doc = "`write(|w| ..)` method takes [tccr1::W](tccr1::W) writer structure"]
impl crate::Writable for TCCR1 {}
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod tccr1;
#[doc = "DSI Host timeout counter configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr2](tccr2) module"]
pub type TCCR2 = crate::Reg<u32, _TCCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR2;
#[doc = "`read()` method returns [tccr2::R](tccr2::R) reader structure"]
impl crate::Readable for TCCR2 {}
#[doc = "`write(|w| ..)` method takes [tccr2::W](tccr2::W) writer structure"]
impl crate::Writable for TCCR2 {}
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod tccr2;
#[doc = "DSI Host timeout counter configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3](tccr3) module"]
pub type TCCR3 = crate::Reg<u32, _TCCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR3;
#[doc = "`read()` method returns [tccr3::R](tccr3::R) reader structure"]
impl crate::Readable for TCCR3 {}
#[doc = "`write(|w| ..)` method takes [tccr3::W](tccr3::W) writer structure"]
impl crate::Writable for TCCR3 {}
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod tccr3;
#[doc = "DSI Host timeout counter configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr4](tccr4) module"]
pub type TCCR4 = crate::Reg<u32, _TCCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR4;
#[doc = "`read()` method returns [tccr4::R](tccr4::R) reader structure"]
impl crate::Readable for TCCR4 {}
#[doc = "`write(|w| ..)` method takes [tccr4::W](tccr4::W) writer structure"]
impl crate::Writable for TCCR4 {}
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod tccr4;
#[doc = "DSI Host timeout counter configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr5](tccr5) module"]
pub type TCCR5 = crate::Reg<u32, _TCCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR5;
#[doc = "`read()` method returns [tccr5::R](tccr5::R) reader structure"]
impl crate::Readable for TCCR5 {}
#[doc = "`write(|w| ..)` method takes [tccr5::W](tccr5::W) writer structure"]
impl crate::Writable for TCCR5 {}
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod tccr5;
#[doc = "DSI Host clock lane configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clcr](clcr) module"]
pub type CLCR = crate::Reg<u32, _CLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLCR;
#[doc = "`read()` method returns [clcr::R](clcr::R) reader structure"]
impl crate::Readable for CLCR {}
#[doc = "`write(|w| ..)` method takes [clcr::W](clcr::W) writer structure"]
impl crate::Writable for CLCR {}
#[doc = "DSI Host clock lane configuration register"]
pub mod clcr;
#[doc = "DSI Host clock lane timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cltcr](cltcr) module"]
pub type CLTCR = crate::Reg<u32, _CLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLTCR;
#[doc = "`read()` method returns [cltcr::R](cltcr::R) reader structure"]
impl crate::Readable for CLTCR {}
#[doc = "`write(|w| ..)` method takes [cltcr::W](cltcr::W) writer structure"]
impl crate::Writable for CLTCR {}
#[doc = "DSI Host clock lane timer configuration register"]
pub mod cltcr;
#[doc = "DSI Host data lane timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dltcr](dltcr) module"]
pub type DLTCR = crate::Reg<u32, _DLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLTCR;
#[doc = "`read()` method returns [dltcr::R](dltcr::R) reader structure"]
impl crate::Readable for DLTCR {}
#[doc = "`write(|w| ..)` method takes [dltcr::W](dltcr::W) writer structure"]
impl crate::Writable for DLTCR {}
#[doc = "DSI Host data lane timer configuration register"]
pub mod dltcr;
#[doc = "DSI Host PHY control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pctlr](pctlr) module"]
pub type PCTLR = crate::Reg<u32, _PCTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCTLR;
#[doc = "`read()` method returns [pctlr::R](pctlr::R) reader structure"]
impl crate::Readable for PCTLR {}
#[doc = "`write(|w| ..)` method takes [pctlr::W](pctlr::W) writer structure"]
impl crate::Writable for PCTLR {}
#[doc = "DSI Host PHY control register"]
pub mod pctlr;
#[doc = "DSI Host PHY configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconfr](pconfr) module"]
pub type PCONFR = crate::Reg<u32, _PCONFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCONFR;
#[doc = "`read()` method returns [pconfr::R](pconfr::R) reader structure"]
impl crate::Readable for PCONFR {}
#[doc = "`write(|w| ..)` method takes [pconfr::W](pconfr::W) writer structure"]
impl crate::Writable for PCONFR {}
#[doc = "DSI Host PHY configuration register"]
pub mod pconfr;
#[doc = "DSI Host PHY ULPS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr](pucr) module"]
pub type PUCR = crate::Reg<u32, _PUCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCR;
#[doc = "`read()` method returns [pucr::R](pucr::R) reader structure"]
impl crate::Readable for PUCR {}
#[doc = "`write(|w| ..)` method takes [pucr::W](pucr::W) writer structure"]
impl crate::Writable for PUCR {}
#[doc = "DSI Host PHY ULPS control register"]
pub mod pucr;
#[doc = "DSI Host PHY TX triggers configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pttcr](pttcr) module"]
pub type PTTCR = crate::Reg<u32, _PTTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTTCR;
#[doc = "`read()` method returns [pttcr::R](pttcr::R) reader structure"]
impl crate::Readable for PTTCR {}
#[doc = "`write(|w| ..)` method takes [pttcr::W](pttcr::W) writer structure"]
impl crate::Writable for PTTCR {}
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod pttcr;
#[doc = "DSI Host interrupt and status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "`write(|w| ..)` method takes [psr::W](psr::W) writer structure"]
impl crate::Writable for PSR {}
#[doc = "DSI Host interrupt and status register 0"]
pub mod psr;
#[doc = "DSI Host interrupt and status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr1](isr1) module"]
pub type ISR1 = crate::Reg<u32, _ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR1;
#[doc = "`read()` method returns [isr1::R](isr1::R) reader structure"]
impl crate::Readable for ISR1 {}
#[doc = "`write(|w| ..)` method takes [isr1::W](isr1::W) writer structure"]
impl crate::Writable for ISR1 {}
#[doc = "DSI Host interrupt and status register 1"]
pub mod isr1;
#[doc = "DSI Host interrupt enable register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier0](ier0) module"]
pub type IER0 = crate::Reg<u32, _IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER0;
#[doc = "`read()` method returns [ier0::R](ier0::R) reader structure"]
impl crate::Readable for IER0 {}
#[doc = "`write(|w| ..)` method takes [ier0::W](ier0::W) writer structure"]
impl crate::Writable for IER0 {}
#[doc = "DSI Host interrupt enable register 0"]
pub mod ier0;
#[doc = "DSI Host interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`read()` method returns [ier1::R](ier1::R) reader structure"]
impl crate::Readable for IER1 {}
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "DSI Host interrupt enable register 1"]
pub mod ier1;
#[doc = "DSI Host force interrupt register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir0](fir0) module"]
pub type FIR0 = crate::Reg<u32, _FIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIR0;
#[doc = "`read()` method returns [fir0::R](fir0::R) reader structure"]
impl crate::Readable for FIR0 {}
#[doc = "`write(|w| ..)` method takes [fir0::W](fir0::W) writer structure"]
impl crate::Writable for FIR0 {}
#[doc = "DSI Host force interrupt register 0"]
pub mod fir0;
#[doc = "DSI Host force interrupt register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fir1](fir1) module"]
pub type FIR1 = crate::Reg<u32, _FIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIR1;
#[doc = "`read()` method returns [fir1::R](fir1::R) reader structure"]
impl crate::Readable for FIR1 {}
#[doc = "`write(|w| ..)` method takes [fir1::W](fir1::W) writer structure"]
impl crate::Writable for FIR1 {}
#[doc = "DSI Host force interrupt register 1"]
pub mod fir1;
#[doc = "DSI Host video shadow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vscr](vscr) module"]
pub type VSCR = crate::Reg<u32, _VSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VSCR;
#[doc = "`read()` method returns [vscr::R](vscr::R) reader structure"]
impl crate::Readable for VSCR {}
#[doc = "`write(|w| ..)` method takes [vscr::W](vscr::W) writer structure"]
impl crate::Writable for VSCR {}
#[doc = "DSI Host video shadow control register"]
pub mod vscr;
#[doc = "DSI Host LTDC current VCID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcvcidr](lcvcidr) module"]
pub type LCVCIDR = crate::Reg<u32, _LCVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCVCIDR;
#[doc = "`read()` method returns [lcvcidr::R](lcvcidr::R) reader structure"]
impl crate::Readable for LCVCIDR {}
#[doc = "`write(|w| ..)` method takes [lcvcidr::W](lcvcidr::W) writer structure"]
impl crate::Writable for LCVCIDR {}
#[doc = "DSI Host LTDC current VCID register"]
pub mod lcvcidr;
#[doc = "DSI Host LTDC current color coding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcccr](lcccr) module"]
pub type LCCCR = crate::Reg<u32, _LCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCCCR;
#[doc = "`read()` method returns [lcccr::R](lcccr::R) reader structure"]
impl crate::Readable for LCCCR {}
#[doc = "`write(|w| ..)` method takes [lcccr::W](lcccr::W) writer structure"]
impl crate::Writable for LCCCR {}
#[doc = "DSI Host LTDC current color coding register"]
pub mod lcccr;
#[doc = "DSI Host low-power mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmccr](lpmccr) module"]
pub type LPMCCR = crate::Reg<u32, _LPMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCCR;
#[doc = "`read()` method returns [lpmccr::R](lpmccr::R) reader structure"]
impl crate::Readable for LPMCCR {}
#[doc = "`write(|w| ..)` method takes [lpmccr::W](lpmccr::W) writer structure"]
impl crate::Writable for LPMCCR {}
#[doc = "DSI Host low-power mode current configuration register"]
pub mod lpmccr;
#[doc = "DSI Host video mode current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmccr](vmccr) module"]
pub type VMCCR = crate::Reg<u32, _VMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMCCR;
#[doc = "`read()` method returns [vmccr::R](vmccr::R) reader structure"]
impl crate::Readable for VMCCR {}
#[doc = "`write(|w| ..)` method takes [vmccr::W](vmccr::W) writer structure"]
impl crate::Writable for VMCCR {}
#[doc = "DSI Host video mode current configuration register"]
pub mod vmccr;
#[doc = "DSI Host video packet current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vpccr](vpccr) module"]
pub type VPCCR = crate::Reg<u32, _VPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VPCCR;
#[doc = "`read()` method returns [vpccr::R](vpccr::R) reader structure"]
impl crate::Readable for VPCCR {}
#[doc = "`write(|w| ..)` method takes [vpccr::W](vpccr::W) writer structure"]
impl crate::Writable for VPCCR {}
#[doc = "DSI Host video packet current configuration register"]
pub mod vpccr;
#[doc = "DSI Host video chunks current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcccr](vcccr) module"]
pub type VCCCR = crate::Reg<u32, _VCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCCCR;
#[doc = "`read()` method returns [vcccr::R](vcccr::R) reader structure"]
impl crate::Readable for VCCCR {}
#[doc = "`write(|w| ..)` method takes [vcccr::W](vcccr::W) writer structure"]
impl crate::Writable for VCCCR {}
#[doc = "DSI Host video chunks current configuration register"]
pub mod vcccr;
#[doc = "DSI Host video null packet current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vnpccr](vnpccr) module"]
pub type VNPCCR = crate::Reg<u32, _VNPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VNPCCR;
#[doc = "`read()` method returns [vnpccr::R](vnpccr::R) reader structure"]
impl crate::Readable for VNPCCR {}
#[doc = "`write(|w| ..)` method takes [vnpccr::W](vnpccr::W) writer structure"]
impl crate::Writable for VNPCCR {}
#[doc = "DSI Host video null packet current configuration register"]
pub mod vnpccr;
#[doc = "DSI Host video HSA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhsaccr](vhsaccr) module"]
pub type VHSACCR = crate::Reg<u32, _VHSACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VHSACCR;
#[doc = "`read()` method returns [vhsaccr::R](vhsaccr::R) reader structure"]
impl crate::Readable for VHSACCR {}
#[doc = "`write(|w| ..)` method takes [vhsaccr::W](vhsaccr::W) writer structure"]
impl crate::Writable for VHSACCR {}
#[doc = "DSI Host video HSA current configuration register"]
pub mod vhsaccr;
#[doc = "DSI Host video HBP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhbpccr](vhbpccr) module"]
pub type VHBPCCR = crate::Reg<u32, _VHBPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VHBPCCR;
#[doc = "`read()` method returns [vhbpccr::R](vhbpccr::R) reader structure"]
impl crate::Readable for VHBPCCR {}
#[doc = "`write(|w| ..)` method takes [vhbpccr::W](vhbpccr::W) writer structure"]
impl crate::Writable for VHBPCCR {}
#[doc = "DSI Host video HBP current configuration register"]
pub mod vhbpccr;
#[doc = "DSI Host video line current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlccr](vlccr) module"]
pub type VLCCR = crate::Reg<u32, _VLCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VLCCR;
#[doc = "`read()` method returns [vlccr::R](vlccr::R) reader structure"]
impl crate::Readable for VLCCR {}
#[doc = "`write(|w| ..)` method takes [vlccr::W](vlccr::W) writer structure"]
impl crate::Writable for VLCCR {}
#[doc = "DSI Host video line current configuration register"]
pub mod vlccr;
#[doc = "DSI Host video VSA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvsaccr](vvsaccr) module"]
pub type VVSACCR = crate::Reg<u32, _VVSACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVSACCR;
#[doc = "`read()` method returns [vvsaccr::R](vvsaccr::R) reader structure"]
impl crate::Readable for VVSACCR {}
#[doc = "`write(|w| ..)` method takes [vvsaccr::W](vvsaccr::W) writer structure"]
impl crate::Writable for VVSACCR {}
#[doc = "DSI Host video VSA current configuration register"]
pub mod vvsaccr;
#[doc = "DSI Host video VBP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvpbccr](vvpbccr) module"]
pub type VVPBCCR = crate::Reg<u32, _VVPBCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVPBCCR;
#[doc = "`read()` method returns [vvpbccr::R](vvpbccr::R) reader structure"]
impl crate::Readable for VVPBCCR {}
#[doc = "`write(|w| ..)` method takes [vvpbccr::W](vvpbccr::W) writer structure"]
impl crate::Writable for VVPBCCR {}
#[doc = "DSI Host video VBP current configuration register"]
pub mod vvpbccr;
#[doc = "DSI Host video VFP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvfpccr](vvfpccr) module"]
pub type VVFPCCR = crate::Reg<u32, _VVFPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVFPCCR;
#[doc = "`read()` method returns [vvfpccr::R](vvfpccr::R) reader structure"]
impl crate::Readable for VVFPCCR {}
#[doc = "`write(|w| ..)` method takes [vvfpccr::W](vvfpccr::W) writer structure"]
impl crate::Writable for VVFPCCR {}
#[doc = "DSI Host video VFP current configuration register"]
pub mod vvfpccr;
#[doc = "DSI Host video VA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvaccr](vvaccr) module"]
pub type VVACCR = crate::Reg<u32, _VVACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VVACCR;
#[doc = "`read()` method returns [vvaccr::R](vvaccr::R) reader structure"]
impl crate::Readable for VVACCR {}
#[doc = "`write(|w| ..)` method takes [vvaccr::W](vvaccr::W) writer structure"]
impl crate::Writable for VVACCR {}
#[doc = "DSI Host video VA current configuration register"]
pub mod vvaccr;
#[doc = "DSI wrapper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcfgr](wcfgr) module"]
pub type WCFGR = crate::Reg<u32, _WCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCFGR;
#[doc = "`read()` method returns [wcfgr::R](wcfgr::R) reader structure"]
impl crate::Readable for WCFGR {}
#[doc = "`write(|w| ..)` method takes [wcfgr::W](wcfgr::W) writer structure"]
impl crate::Writable for WCFGR {}
#[doc = "DSI wrapper configuration register"]
pub mod wcfgr;
#[doc = "DSI wrapper control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](wcr) module"]
pub type WCR = crate::Reg<u32, _WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WCR;
#[doc = "`read()` method returns [wcr::R](wcr::R) reader structure"]
impl crate::Readable for WCR {}
#[doc = "`write(|w| ..)` method takes [wcr::W](wcr::W) writer structure"]
impl crate::Writable for WCR {}
#[doc = "DSI wrapper control register"]
pub mod wcr;
#[doc = "DSI wrapper interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wier](wier) module"]
pub type WIER = crate::Reg<u32, _WIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIER;
#[doc = "`read()` method returns [wier::R](wier::R) reader structure"]
impl crate::Readable for WIER {}
#[doc = "`write(|w| ..)` method takes [wier::W](wier::W) writer structure"]
impl crate::Writable for WIER {}
#[doc = "DSI wrapper interrupt enable register"]
pub mod wier;
#[doc = "DSI wrapper interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wisr](wisr) module"]
pub type WISR = crate::Reg<u32, _WISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WISR;
#[doc = "`read()` method returns [wisr::R](wisr::R) reader structure"]
impl crate::Readable for WISR {}
#[doc = "`write(|w| ..)` method takes [wisr::W](wisr::W) writer structure"]
impl crate::Writable for WISR {}
#[doc = "DSI wrapper interrupt and status register"]
pub mod wisr;
#[doc = "DSI wrapper interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifcr](wifcr) module"]
pub type WIFCR = crate::Reg<u32, _WIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WIFCR;
#[doc = "`read()` method returns [wifcr::R](wifcr::R) reader structure"]
impl crate::Readable for WIFCR {}
#[doc = "`write(|w| ..)` method takes [wifcr::W](wifcr::W) writer structure"]
impl crate::Writable for WIFCR {}
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod wifcr;
#[doc = "DSI wrapper PHY configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr0](wpcr0) module"]
pub type WPCR0 = crate::Reg<u32, _WPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCR0;
#[doc = "`read()` method returns [wpcr0::R](wpcr0::R) reader structure"]
impl crate::Readable for WPCR0 {}
#[doc = "`write(|w| ..)` method takes [wpcr0::W](wpcr0::W) writer structure"]
impl crate::Writable for WPCR0 {}
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod wpcr0;
#[doc = "DSI wrapper PHY configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr1](wpcr1) module"]
pub type WPCR1 = crate::Reg<u32, _WPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCR1;
#[doc = "`read()` method returns [wpcr1::R](wpcr1::R) reader structure"]
impl crate::Readable for WPCR1 {}
#[doc = "`write(|w| ..)` method takes [wpcr1::W](wpcr1::W) writer structure"]
impl crate::Writable for WPCR1 {}
#[doc = "DSI wrapper PHY configuration register 1"]
pub mod wpcr1;
#[doc = "DSI wrapper PHY configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr2](wpcr2) module"]
pub type WPCR2 = crate::Reg<u32, _WPCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPCR2;
#[doc = "`read()` method returns [wpcr2::R](wpcr2::R) reader structure"]
impl crate::Readable for WPCR2 {}
#[doc = "`write(|w| ..)` method takes [wpcr2::W](wpcr2::W) writer structure"]
impl crate::Writable for WPCR2 {}
#[doc = "DSI wrapper PHY configuration register 4"]
pub mod wpcr2;
#[doc = "DSI wrapper regulator and PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpcr](wrpcr) module"]
pub type WRPCR = crate::Reg<u32, _WRPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRPCR;
#[doc = "`read()` method returns [wrpcr::R](wrpcr::R) reader structure"]
impl crate::Readable for WRPCR {}
#[doc = "`write(|w| ..)` method takes [wrpcr::W](wrpcr::W) writer structure"]
impl crate::Writable for WRPCR {}
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod wrpcr;
