#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    pub vr: crate::Reg<vr::VR_SPEC>,
    #[doc = "0x04 - DSI Host control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - DSI Host clock control register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    pub lvcidr: crate::Reg<lvcidr::LVCIDR_SPEC>,
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    pub lcolcr: crate::Reg<lcolcr::LCOLCR_SPEC>,
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    pub lpcr: crate::Reg<lpcr::LPCR_SPEC>,
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    pub lpmcr: crate::Reg<lpmcr::LPMCR_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - DSI Host protocol configuration register"]
    pub pcr: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x30 - DSI Host generic VCID register"]
    pub gvcidr: crate::Reg<gvcidr::GVCIDR_SPEC>,
    #[doc = "0x34 - DSI Host mode configuration register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x38 - DSI Host video mode configuration register"]
    pub vmcr: crate::Reg<vmcr::VMCR_SPEC>,
    #[doc = "0x3c - DSI Host video packet configuration register"]
    pub vpcr: crate::Reg<vpcr::VPCR_SPEC>,
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    pub vccr: crate::Reg<vccr::VCCR_SPEC>,
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    pub vnpcr: crate::Reg<vnpcr::VNPCR_SPEC>,
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    pub vhsacr: crate::Reg<vhsacr::VHSACR_SPEC>,
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    pub vhbpcr: crate::Reg<vhbpcr::VHBPCR_SPEC>,
    #[doc = "0x50 - DSI Host video line configuration register"]
    pub vlcr: crate::Reg<vlcr::VLCR_SPEC>,
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    pub vvsacr: crate::Reg<vvsacr::VVSACR_SPEC>,
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    pub vvbpcr: crate::Reg<vvbpcr::VVBPCR_SPEC>,
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    pub vvfpcr: crate::Reg<vvfpcr::VVFPCR_SPEC>,
    #[doc = "0x60 - DSI Host video VA configuration register"]
    pub vvacr: crate::Reg<vvacr::VVACR_SPEC>,
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    pub lccr: crate::Reg<lccr::LCCR_SPEC>,
    #[doc = "0x68 - DSI Host command mode configuration register"]
    pub cmcr: crate::Reg<cmcr::CMCR_SPEC>,
    #[doc = "0x6c - DSI Host generic header configuration register"]
    pub ghcr: crate::Reg<ghcr::GHCR_SPEC>,
    #[doc = "0x70 - DSI Host generic payload data register"]
    pub gpdr: crate::Reg<gpdr::GPDR_SPEC>,
    #[doc = "0x74 - DSI Host generic packet status register"]
    pub gpsr: crate::Reg<gpsr::GPSR_SPEC>,
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    pub tccr0: crate::Reg<tccr0::TCCR0_SPEC>,
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    pub tccr1: crate::Reg<tccr1::TCCR1_SPEC>,
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    pub tccr2: crate::Reg<tccr2::TCCR2_SPEC>,
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    pub tccr3: crate::Reg<tccr3::TCCR3_SPEC>,
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    pub tccr4: crate::Reg<tccr4::TCCR4_SPEC>,
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    pub tccr5: crate::Reg<tccr5::TCCR5_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    pub clcr: crate::Reg<clcr::CLCR_SPEC>,
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    pub cltcr: crate::Reg<cltcr::CLTCR_SPEC>,
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    pub dltcr: crate::Reg<dltcr::DLTCR_SPEC>,
    #[doc = "0xa0 - DSI Host PHY control register"]
    pub pctlr: crate::Reg<pctlr::PCTLR_SPEC>,
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    pub pconfr: crate::Reg<pconfr::PCONFR_SPEC>,
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    pub pucr: crate::Reg<pucr::PUCR_SPEC>,
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    pub pttcr: crate::Reg<pttcr::PTTCR_SPEC>,
    #[doc = "0xb0 - DSI Host PHY status register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    pub isr0: crate::Reg<isr0::ISR0_SPEC>,
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    pub isr1: crate::Reg<isr1::ISR1_SPEC>,
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    pub ier0: crate::Reg<ier0::IER0_SPEC>,
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    pub ier1: crate::Reg<ier1::IER1_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    pub fir0: crate::Reg<fir0::FIR0_SPEC>,
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    pub fir1: crate::Reg<fir1::FIR1_SPEC>,
    _reserved46: [u8; 0x20],
    #[doc = "0x100 - DSI Host video shadow control register"]
    pub vscr: crate::Reg<vscr::VSCR_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    pub lcvcidr: crate::Reg<lcvcidr::LCVCIDR_SPEC>,
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    pub lcccr: crate::Reg<lcccr::LCCCR_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    pub lpmccr: crate::Reg<lpmccr::LPMCCR_SPEC>,
    _reserved50: [u8; 0x1c],
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    pub vmccr: crate::Reg<vmccr::VMCCR_SPEC>,
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    pub vpccr: crate::Reg<vpccr::VPCCR_SPEC>,
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    pub vcccr: crate::Reg<vcccr::VCCCR_SPEC>,
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    pub vnpccr: crate::Reg<vnpccr::VNPCCR_SPEC>,
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    pub vhsaccr: crate::Reg<vhsaccr::VHSACCR_SPEC>,
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    pub vhbpccr: crate::Reg<vhbpccr::VHBPCCR_SPEC>,
    #[doc = "0x150 - DSI Host video line current configuration register"]
    pub vlccr: crate::Reg<vlccr::VLCCR_SPEC>,
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    pub vvsaccr: crate::Reg<vvsaccr::VVSACCR_SPEC>,
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    pub vvpbccr: crate::Reg<vvpbccr::VVPBCCR_SPEC>,
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    pub vvfpccr: crate::Reg<vvfpccr::VVFPCCR_SPEC>,
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    pub vvaccr: crate::Reg<vvaccr::VVACCR_SPEC>,
    _reserved61: [u8; 0x029c],
    #[doc = "0x400 - DSI wrapper configuration register"]
    pub wcfgr: crate::Reg<wcfgr::WCFGR_SPEC>,
    #[doc = "0x404 - DSI wrapper control register"]
    pub wcr: crate::Reg<wcr::WCR_SPEC>,
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    pub wier: crate::Reg<wier::WIER_SPEC>,
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    pub wisr: crate::Reg<wisr::WISR_SPEC>,
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    pub wifcr: crate::Reg<wifcr::WIFCR_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    pub wpcr0: crate::Reg<wpcr0::WPCR0_SPEC>,
    #[doc = "0x41c - DSI wrapper PHY configuration register 1"]
    pub wpcr1: crate::Reg<wpcr1::WPCR1_SPEC>,
    #[doc = "0x420 - DSI wrapper PHY configuration register 2"]
    pub wpcr2: crate::Reg<wpcr2::WPCR2_SPEC>,
    #[doc = "0x424 - DSI wrapper PHY configuration register 3"]
    pub wpcr3: crate::Reg<wpcr3::WPCR3_SPEC>,
    #[doc = "0x428 - DSI wrapper PHY configuration register 4"]
    pub wpcr4: crate::Reg<wpcr4::WPCR4_SPEC>,
    _reserved71: [u8; 0x04],
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    pub wrpcr: crate::Reg<wrpcr::WRPCR_SPEC>,
}
#[doc = "VR register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "DSI Host version register"]
pub mod vr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DSI Host control register"]
pub mod cr;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "DSI Host clock control register"]
pub mod ccr;
#[doc = "LVCIDR register accessor: an alias for `Reg<LVCIDR_SPEC>`"]
pub type LVCIDR = crate::Reg<lvcidr::LVCIDR_SPEC>;
#[doc = "DSI Host LTDC VCID register"]
pub mod lvcidr;
#[doc = "LCOLCR register accessor: an alias for `Reg<LCOLCR_SPEC>`"]
pub type LCOLCR = crate::Reg<lcolcr::LCOLCR_SPEC>;
#[doc = "DSI Host LTDC color coding register"]
pub mod lcolcr;
#[doc = "LPCR register accessor: an alias for `Reg<LPCR_SPEC>`"]
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod lpcr;
#[doc = "LPMCR register accessor: an alias for `Reg<LPMCR_SPEC>`"]
pub type LPMCR = crate::Reg<lpmcr::LPMCR_SPEC>;
#[doc = "DSI Host low-power mode configuration register"]
pub mod lpmcr;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "DSI Host protocol configuration register"]
pub mod pcr;
#[doc = "GVCIDR register accessor: an alias for `Reg<GVCIDR_SPEC>`"]
pub type GVCIDR = crate::Reg<gvcidr::GVCIDR_SPEC>;
#[doc = "DSI Host generic VCID register"]
pub mod gvcidr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "DSI Host mode configuration register"]
pub mod mcr;
#[doc = "VMCR register accessor: an alias for `Reg<VMCR_SPEC>`"]
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
#[doc = "DSI Host video mode configuration register"]
pub mod vmcr;
#[doc = "VPCR register accessor: an alias for `Reg<VPCR_SPEC>`"]
pub type VPCR = crate::Reg<vpcr::VPCR_SPEC>;
#[doc = "DSI Host video packet configuration register"]
pub mod vpcr;
#[doc = "VCCR register accessor: an alias for `Reg<VCCR_SPEC>`"]
pub type VCCR = crate::Reg<vccr::VCCR_SPEC>;
#[doc = "DSI Host video chunks configuration register"]
pub mod vccr;
#[doc = "VNPCR register accessor: an alias for `Reg<VNPCR_SPEC>`"]
pub type VNPCR = crate::Reg<vnpcr::VNPCR_SPEC>;
#[doc = "DSI Host video null packet configuration register"]
pub mod vnpcr;
#[doc = "VHSACR register accessor: an alias for `Reg<VHSACR_SPEC>`"]
pub type VHSACR = crate::Reg<vhsacr::VHSACR_SPEC>;
#[doc = "DSI Host video HSA configuration register"]
pub mod vhsacr;
#[doc = "VHBPCR register accessor: an alias for `Reg<VHBPCR_SPEC>`"]
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCR_SPEC>;
#[doc = "DSI Host video HBP configuration register"]
pub mod vhbpcr;
#[doc = "VLCR register accessor: an alias for `Reg<VLCR_SPEC>`"]
pub type VLCR = crate::Reg<vlcr::VLCR_SPEC>;
#[doc = "DSI Host video line configuration register"]
pub mod vlcr;
#[doc = "VVSACR register accessor: an alias for `Reg<VVSACR_SPEC>`"]
pub type VVSACR = crate::Reg<vvsacr::VVSACR_SPEC>;
#[doc = "DSI Host video VSA configuration register"]
pub mod vvsacr;
#[doc = "VVBPCR register accessor: an alias for `Reg<VVBPCR_SPEC>`"]
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCR_SPEC>;
#[doc = "DSI Host video VBP configuration register"]
pub mod vvbpcr;
#[doc = "VVFPCR register accessor: an alias for `Reg<VVFPCR_SPEC>`"]
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCR_SPEC>;
#[doc = "DSI Host video VFP configuration register"]
pub mod vvfpcr;
#[doc = "VVACR register accessor: an alias for `Reg<VVACR_SPEC>`"]
pub type VVACR = crate::Reg<vvacr::VVACR_SPEC>;
#[doc = "DSI Host video VA configuration register"]
pub mod vvacr;
#[doc = "LCCR register accessor: an alias for `Reg<LCCR_SPEC>`"]
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
#[doc = "DSI Host LTDC command configuration register"]
pub mod lccr;
#[doc = "CMCR register accessor: an alias for `Reg<CMCR_SPEC>`"]
pub type CMCR = crate::Reg<cmcr::CMCR_SPEC>;
#[doc = "DSI Host command mode configuration register"]
pub mod cmcr;
#[doc = "GHCR register accessor: an alias for `Reg<GHCR_SPEC>`"]
pub type GHCR = crate::Reg<ghcr::GHCR_SPEC>;
#[doc = "DSI Host generic header configuration register"]
pub mod ghcr;
#[doc = "GPDR register accessor: an alias for `Reg<GPDR_SPEC>`"]
pub type GPDR = crate::Reg<gpdr::GPDR_SPEC>;
#[doc = "DSI Host generic payload data register"]
pub mod gpdr;
#[doc = "GPSR register accessor: an alias for `Reg<GPSR_SPEC>`"]
pub type GPSR = crate::Reg<gpsr::GPSR_SPEC>;
#[doc = "DSI Host generic packet status register"]
pub mod gpsr;
#[doc = "TCCR0 register accessor: an alias for `Reg<TCCR0_SPEC>`"]
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod tccr0;
#[doc = "TCCR1 register accessor: an alias for `Reg<TCCR1_SPEC>`"]
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod tccr1;
#[doc = "TCCR2 register accessor: an alias for `Reg<TCCR2_SPEC>`"]
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod tccr2;
#[doc = "TCCR3 register accessor: an alias for `Reg<TCCR3_SPEC>`"]
pub type TCCR3 = crate::Reg<tccr3::TCCR3_SPEC>;
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod tccr3;
#[doc = "TCCR4 register accessor: an alias for `Reg<TCCR4_SPEC>`"]
pub type TCCR4 = crate::Reg<tccr4::TCCR4_SPEC>;
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod tccr4;
#[doc = "TCCR5 register accessor: an alias for `Reg<TCCR5_SPEC>`"]
pub type TCCR5 = crate::Reg<tccr5::TCCR5_SPEC>;
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod tccr5;
#[doc = "CLCR register accessor: an alias for `Reg<CLCR_SPEC>`"]
pub type CLCR = crate::Reg<clcr::CLCR_SPEC>;
#[doc = "DSI Host clock lane configuration register"]
pub mod clcr;
#[doc = "CLTCR register accessor: an alias for `Reg<CLTCR_SPEC>`"]
pub type CLTCR = crate::Reg<cltcr::CLTCR_SPEC>;
#[doc = "DSI Host clock lane timer configuration register"]
pub mod cltcr;
#[doc = "DLTCR register accessor: an alias for `Reg<DLTCR_SPEC>`"]
pub type DLTCR = crate::Reg<dltcr::DLTCR_SPEC>;
#[doc = "DSI Host data lane timer configuration register"]
pub mod dltcr;
#[doc = "PCTLR register accessor: an alias for `Reg<PCTLR_SPEC>`"]
pub type PCTLR = crate::Reg<pctlr::PCTLR_SPEC>;
#[doc = "DSI Host PHY control register"]
pub mod pctlr;
#[doc = "PCONFR register accessor: an alias for `Reg<PCONFR_SPEC>`"]
pub type PCONFR = crate::Reg<pconfr::PCONFR_SPEC>;
#[doc = "DSI Host PHY configuration register"]
pub mod pconfr;
#[doc = "PUCR register accessor: an alias for `Reg<PUCR_SPEC>`"]
pub type PUCR = crate::Reg<pucr::PUCR_SPEC>;
#[doc = "DSI Host PHY ULPS control register"]
pub mod pucr;
#[doc = "PTTCR register accessor: an alias for `Reg<PTTCR_SPEC>`"]
pub type PTTCR = crate::Reg<pttcr::PTTCR_SPEC>;
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod pttcr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "DSI Host PHY status register"]
pub mod psr;
#[doc = "ISR0 register accessor: an alias for `Reg<ISR0_SPEC>`"]
pub type ISR0 = crate::Reg<isr0::ISR0_SPEC>;
#[doc = "DSI Host interrupt and status register 0"]
pub mod isr0;
#[doc = "ISR1 register accessor: an alias for `Reg<ISR1_SPEC>`"]
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
#[doc = "DSI Host interrupt and status register 1"]
pub mod isr1;
#[doc = "IER0 register accessor: an alias for `Reg<IER0_SPEC>`"]
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
#[doc = "DSI Host interrupt enable register 0"]
pub mod ier0;
#[doc = "IER1 register accessor: an alias for `Reg<IER1_SPEC>`"]
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
#[doc = "DSI Host interrupt enable register 1"]
pub mod ier1;
#[doc = "FIR0 register accessor: an alias for `Reg<FIR0_SPEC>`"]
pub type FIR0 = crate::Reg<fir0::FIR0_SPEC>;
#[doc = "DSI Host force interrupt register 0"]
pub mod fir0;
#[doc = "FIR1 register accessor: an alias for `Reg<FIR1_SPEC>`"]
pub type FIR1 = crate::Reg<fir1::FIR1_SPEC>;
#[doc = "DSI Host force interrupt register 1"]
pub mod fir1;
#[doc = "VSCR register accessor: an alias for `Reg<VSCR_SPEC>`"]
pub type VSCR = crate::Reg<vscr::VSCR_SPEC>;
#[doc = "DSI Host video shadow control register"]
pub mod vscr;
#[doc = "LCVCIDR register accessor: an alias for `Reg<LCVCIDR_SPEC>`"]
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDR_SPEC>;
#[doc = "DSI Host LTDC current VCID register"]
pub mod lcvcidr;
#[doc = "LCCCR register accessor: an alias for `Reg<LCCCR_SPEC>`"]
pub type LCCCR = crate::Reg<lcccr::LCCCR_SPEC>;
#[doc = "DSI Host LTDC current color coding register"]
pub mod lcccr;
#[doc = "LPMCCR register accessor: an alias for `Reg<LPMCCR_SPEC>`"]
pub type LPMCCR = crate::Reg<lpmccr::LPMCCR_SPEC>;
#[doc = "DSI Host low-power mode current configuration register"]
pub mod lpmccr;
#[doc = "VMCCR register accessor: an alias for `Reg<VMCCR_SPEC>`"]
pub type VMCCR = crate::Reg<vmccr::VMCCR_SPEC>;
#[doc = "DSI Host video mode current configuration register"]
pub mod vmccr;
#[doc = "VPCCR register accessor: an alias for `Reg<VPCCR_SPEC>`"]
pub type VPCCR = crate::Reg<vpccr::VPCCR_SPEC>;
#[doc = "DSI Host video packet current configuration register"]
pub mod vpccr;
#[doc = "VCCCR register accessor: an alias for `Reg<VCCCR_SPEC>`"]
pub type VCCCR = crate::Reg<vcccr::VCCCR_SPEC>;
#[doc = "DSI Host video chunks current configuration register"]
pub mod vcccr;
#[doc = "VNPCCR register accessor: an alias for `Reg<VNPCCR_SPEC>`"]
pub type VNPCCR = crate::Reg<vnpccr::VNPCCR_SPEC>;
#[doc = "DSI Host video null packet current configuration register"]
pub mod vnpccr;
#[doc = "VHSACCR register accessor: an alias for `Reg<VHSACCR_SPEC>`"]
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCR_SPEC>;
#[doc = "DSI Host video HSA current configuration register"]
pub mod vhsaccr;
#[doc = "VHBPCCR register accessor: an alias for `Reg<VHBPCCR_SPEC>`"]
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCR_SPEC>;
#[doc = "DSI Host video HBP current configuration register"]
pub mod vhbpccr;
#[doc = "VLCCR register accessor: an alias for `Reg<VLCCR_SPEC>`"]
pub type VLCCR = crate::Reg<vlccr::VLCCR_SPEC>;
#[doc = "DSI Host video line current configuration register"]
pub mod vlccr;
#[doc = "VVSACCR register accessor: an alias for `Reg<VVSACCR_SPEC>`"]
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCR_SPEC>;
#[doc = "DSI Host video VSA current configuration register"]
pub mod vvsaccr;
#[doc = "VVPBCCR register accessor: an alias for `Reg<VVPBCCR_SPEC>`"]
pub type VVPBCCR = crate::Reg<vvpbccr::VVPBCCR_SPEC>;
#[doc = "DSI Host video VBP current configuration register"]
pub mod vvpbccr;
#[doc = "VVFPCCR register accessor: an alias for `Reg<VVFPCCR_SPEC>`"]
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCR_SPEC>;
#[doc = "DSI Host video VFP current configuration register"]
pub mod vvfpccr;
#[doc = "VVACCR register accessor: an alias for `Reg<VVACCR_SPEC>`"]
pub type VVACCR = crate::Reg<vvaccr::VVACCR_SPEC>;
#[doc = "DSI Host video VA current configuration register"]
pub mod vvaccr;
#[doc = "WCFGR register accessor: an alias for `Reg<WCFGR_SPEC>`"]
pub type WCFGR = crate::Reg<wcfgr::WCFGR_SPEC>;
#[doc = "DSI wrapper configuration register"]
pub mod wcfgr;
#[doc = "WCR register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "DSI wrapper control register"]
pub mod wcr;
#[doc = "WIER register accessor: an alias for `Reg<WIER_SPEC>`"]
pub type WIER = crate::Reg<wier::WIER_SPEC>;
#[doc = "DSI wrapper interrupt enable register"]
pub mod wier;
#[doc = "WISR register accessor: an alias for `Reg<WISR_SPEC>`"]
pub type WISR = crate::Reg<wisr::WISR_SPEC>;
#[doc = "DSI wrapper interrupt and status register"]
pub mod wisr;
#[doc = "WIFCR register accessor: an alias for `Reg<WIFCR_SPEC>`"]
pub type WIFCR = crate::Reg<wifcr::WIFCR_SPEC>;
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod wifcr;
#[doc = "WPCR0 register accessor: an alias for `Reg<WPCR0_SPEC>`"]
pub type WPCR0 = crate::Reg<wpcr0::WPCR0_SPEC>;
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod wpcr0;
#[doc = "WPCR1 register accessor: an alias for `Reg<WPCR1_SPEC>`"]
pub type WPCR1 = crate::Reg<wpcr1::WPCR1_SPEC>;
#[doc = "DSI wrapper PHY configuration register 1"]
pub mod wpcr1;
#[doc = "WPCR2 register accessor: an alias for `Reg<WPCR2_SPEC>`"]
pub type WPCR2 = crate::Reg<wpcr2::WPCR2_SPEC>;
#[doc = "DSI wrapper PHY configuration register 2"]
pub mod wpcr2;
#[doc = "WPCR3 register accessor: an alias for `Reg<WPCR3_SPEC>`"]
pub type WPCR3 = crate::Reg<wpcr3::WPCR3_SPEC>;
#[doc = "DSI wrapper PHY configuration register 3"]
pub mod wpcr3;
#[doc = "WPCR4 register accessor: an alias for `Reg<WPCR4_SPEC>`"]
pub type WPCR4 = crate::Reg<wpcr4::WPCR4_SPEC>;
#[doc = "DSI wrapper PHY configuration register 4"]
pub mod wpcr4;
#[doc = "WRPCR register accessor: an alias for `Reg<WRPCR_SPEC>`"]
pub type WRPCR = crate::Reg<wrpcr::WRPCR_SPEC>;
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod wrpcr;
