#[repr(C)]
#[derive(Debug)]
///Register block
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
    wpcr0: WPCR0,
    wpcr1: WPCR1,
    wpcr2: WPCR2,
    wpcr3: WPCR3,
    wpcr4: WPCR4,
    _reserved71: [u8; 0x04],
    wrpcr: WRPCR,
}
impl RegisterBlock {
    ///0x00 - DSI Host version register
    #[inline(always)]
    pub const fn vr(&self) -> &VR {
        &self.vr
    }
    ///0x04 - DSI Host control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - DSI Host clock control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x0c - DSI Host LTDC VCID register
    #[inline(always)]
    pub const fn lvcidr(&self) -> &LVCIDR {
        &self.lvcidr
    }
    ///0x10 - DSI Host LTDC color coding register
    #[inline(always)]
    pub const fn lcolcr(&self) -> &LCOLCR {
        &self.lcolcr
    }
    ///0x14 - DSI Host LTDC polarity configuration register
    #[inline(always)]
    pub const fn lpcr(&self) -> &LPCR {
        &self.lpcr
    }
    ///0x18 - DSI Host low-power mode configuration register
    #[inline(always)]
    pub const fn lpmcr(&self) -> &LPMCR {
        &self.lpmcr
    }
    ///0x2c - DSI Host protocol configuration register
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    ///0x30 - DSI Host generic VCID register
    #[inline(always)]
    pub const fn gvcidr(&self) -> &GVCIDR {
        &self.gvcidr
    }
    ///0x34 - DSI Host mode configuration register
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x38 - DSI Host video mode configuration register
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    ///0x3c - DSI Host video packet configuration register
    #[inline(always)]
    pub const fn vpcr(&self) -> &VPCR {
        &self.vpcr
    }
    ///0x40 - DSI Host video chunks configuration register
    #[inline(always)]
    pub const fn vccr(&self) -> &VCCR {
        &self.vccr
    }
    ///0x44 - DSI Host video null packet configuration register
    #[inline(always)]
    pub const fn vnpcr(&self) -> &VNPCR {
        &self.vnpcr
    }
    ///0x48 - DSI Host video HSA configuration register
    #[inline(always)]
    pub const fn vhsacr(&self) -> &VHSACR {
        &self.vhsacr
    }
    ///0x4c - DSI Host video HBP configuration register
    #[inline(always)]
    pub const fn vhbpcr(&self) -> &VHBPCR {
        &self.vhbpcr
    }
    ///0x50 - DSI Host video line configuration register
    #[inline(always)]
    pub const fn vlcr(&self) -> &VLCR {
        &self.vlcr
    }
    ///0x54 - DSI Host video VSA configuration register
    #[inline(always)]
    pub const fn vvsacr(&self) -> &VVSACR {
        &self.vvsacr
    }
    ///0x58 - DSI Host video VBP configuration register
    #[inline(always)]
    pub const fn vvbpcr(&self) -> &VVBPCR {
        &self.vvbpcr
    }
    ///0x5c - DSI Host video VFP configuration register
    #[inline(always)]
    pub const fn vvfpcr(&self) -> &VVFPCR {
        &self.vvfpcr
    }
    ///0x60 - DSI Host video VA configuration register
    #[inline(always)]
    pub const fn vvacr(&self) -> &VVACR {
        &self.vvacr
    }
    ///0x64 - DSI Host LTDC command configuration register
    #[inline(always)]
    pub const fn lccr(&self) -> &LCCR {
        &self.lccr
    }
    ///0x68 - DSI Host command mode configuration register
    #[inline(always)]
    pub const fn cmcr(&self) -> &CMCR {
        &self.cmcr
    }
    ///0x6c - DSI Host generic header configuration register
    #[inline(always)]
    pub const fn ghcr(&self) -> &GHCR {
        &self.ghcr
    }
    ///0x70 - DSI Host generic payload data register
    #[inline(always)]
    pub const fn gpdr(&self) -> &GPDR {
        &self.gpdr
    }
    ///0x74 - DSI Host generic packet status register
    #[inline(always)]
    pub const fn gpsr(&self) -> &GPSR {
        &self.gpsr
    }
    ///0x78 - DSI Host timeout counter configuration register 0
    #[inline(always)]
    pub const fn tccr0(&self) -> &TCCR0 {
        &self.tccr0
    }
    ///0x7c - DSI Host timeout counter configuration register 1
    #[inline(always)]
    pub const fn tccr1(&self) -> &TCCR1 {
        &self.tccr1
    }
    ///0x80 - DSI Host timeout counter configuration register 2
    #[inline(always)]
    pub const fn tccr2(&self) -> &TCCR2 {
        &self.tccr2
    }
    ///0x84 - DSI Host timeout counter configuration register 3
    #[inline(always)]
    pub const fn tccr3(&self) -> &TCCR3 {
        &self.tccr3
    }
    ///0x88 - DSI Host timeout counter configuration register 4
    #[inline(always)]
    pub const fn tccr4(&self) -> &TCCR4 {
        &self.tccr4
    }
    ///0x8c - DSI Host timeout counter configuration register 5
    #[inline(always)]
    pub const fn tccr5(&self) -> &TCCR5 {
        &self.tccr5
    }
    ///0x94 - DSI Host clock lane configuration register
    #[inline(always)]
    pub const fn clcr(&self) -> &CLCR {
        &self.clcr
    }
    ///0x98 - DSI Host clock lane timer configuration register
    #[inline(always)]
    pub const fn cltcr(&self) -> &CLTCR {
        &self.cltcr
    }
    ///0x9c - DSI Host data lane timer configuration register
    #[inline(always)]
    pub const fn dltcr(&self) -> &DLTCR {
        &self.dltcr
    }
    ///0xa0 - DSI Host PHY control register
    #[inline(always)]
    pub const fn pctlr(&self) -> &PCTLR {
        &self.pctlr
    }
    ///0xa4 - DSI Host PHY configuration register
    #[inline(always)]
    pub const fn pconfr(&self) -> &PCONFR {
        &self.pconfr
    }
    ///0xa8 - DSI Host PHY ULPS control register
    #[inline(always)]
    pub const fn pucr(&self) -> &PUCR {
        &self.pucr
    }
    ///0xac - DSI Host PHY TX triggers configuration register
    #[inline(always)]
    pub const fn pttcr(&self) -> &PTTCR {
        &self.pttcr
    }
    ///0xb0 - DSI Host PHY status register
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    ///0xbc - DSI Host interrupt and status register 0
    #[inline(always)]
    pub const fn isr0(&self) -> &ISR0 {
        &self.isr0
    }
    ///0xc0 - DSI Host interrupt and status register 1
    #[inline(always)]
    pub const fn isr1(&self) -> &ISR1 {
        &self.isr1
    }
    ///0xc4 - DSI Host interrupt enable register 0
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    ///0xc8 - DSI Host interrupt enable register 1
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    ///0xd8 - DSI Host force interrupt register 0
    #[inline(always)]
    pub const fn fir0(&self) -> &FIR0 {
        &self.fir0
    }
    ///0xdc - DSI Host force interrupt register 1
    #[inline(always)]
    pub const fn fir1(&self) -> &FIR1 {
        &self.fir1
    }
    ///0x100 - DSI Host video shadow control register
    #[inline(always)]
    pub const fn vscr(&self) -> &VSCR {
        &self.vscr
    }
    ///0x10c - DSI Host LTDC current VCID register
    #[inline(always)]
    pub const fn lcvcidr(&self) -> &LCVCIDR {
        &self.lcvcidr
    }
    ///0x110 - DSI Host LTDC current color coding register
    #[inline(always)]
    pub const fn lcccr(&self) -> &LCCCR {
        &self.lcccr
    }
    ///0x118 - DSI Host low-power mode current configuration register
    #[inline(always)]
    pub const fn lpmccr(&self) -> &LPMCCR {
        &self.lpmccr
    }
    ///0x138 - DSI Host video mode current configuration register
    #[inline(always)]
    pub const fn vmccr(&self) -> &VMCCR {
        &self.vmccr
    }
    ///0x13c - DSI Host video packet current configuration register
    #[inline(always)]
    pub const fn vpccr(&self) -> &VPCCR {
        &self.vpccr
    }
    ///0x140 - DSI Host video chunks current configuration register
    #[inline(always)]
    pub const fn vcccr(&self) -> &VCCCR {
        &self.vcccr
    }
    ///0x144 - DSI Host video null packet current configuration register
    #[inline(always)]
    pub const fn vnpccr(&self) -> &VNPCCR {
        &self.vnpccr
    }
    ///0x148 - DSI Host video HSA current configuration register
    #[inline(always)]
    pub const fn vhsaccr(&self) -> &VHSACCR {
        &self.vhsaccr
    }
    ///0x14c - DSI Host video HBP current configuration register
    #[inline(always)]
    pub const fn vhbpccr(&self) -> &VHBPCCR {
        &self.vhbpccr
    }
    ///0x150 - DSI Host video line current configuration register
    #[inline(always)]
    pub const fn vlccr(&self) -> &VLCCR {
        &self.vlccr
    }
    ///0x154 - DSI Host video VSA current configuration register
    #[inline(always)]
    pub const fn vvsaccr(&self) -> &VVSACCR {
        &self.vvsaccr
    }
    ///0x158 - DSI Host video VBP current configuration register
    #[inline(always)]
    pub const fn vvbpccr(&self) -> &VVBPCCR {
        &self.vvbpccr
    }
    ///0x15c - DSI Host video VFP current configuration register
    #[inline(always)]
    pub const fn vvfpccr(&self) -> &VVFPCCR {
        &self.vvfpccr
    }
    ///0x160 - DSI Host video VA current configuration register
    #[inline(always)]
    pub const fn vvaccr(&self) -> &VVACCR {
        &self.vvaccr
    }
    ///0x400 - DSI wrapper configuration register
    #[inline(always)]
    pub const fn wcfgr(&self) -> &WCFGR {
        &self.wcfgr
    }
    ///0x404 - DSI wrapper control register
    #[inline(always)]
    pub const fn wcr(&self) -> &WCR {
        &self.wcr
    }
    ///0x408 - DSI wrapper interrupt enable register
    #[inline(always)]
    pub const fn wier(&self) -> &WIER {
        &self.wier
    }
    ///0x40c - DSI wrapper interrupt and status register
    #[inline(always)]
    pub const fn wisr(&self) -> &WISR {
        &self.wisr
    }
    ///0x410 - DSI wrapper interrupt flag clear register
    #[inline(always)]
    pub const fn wifcr(&self) -> &WIFCR {
        &self.wifcr
    }
    ///0x418 - DSI wrapper PHY configuration register 0
    #[inline(always)]
    pub const fn wpcr0(&self) -> &WPCR0 {
        &self.wpcr0
    }
    ///0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).
    #[inline(always)]
    pub const fn wpcr1(&self) -> &WPCR1 {
        &self.wpcr1
    }
    ///0x420 - DSI wrapper PHY configuration register 2
    #[inline(always)]
    pub const fn wpcr2(&self) -> &WPCR2 {
        &self.wpcr2
    }
    ///0x424 - DSI wrapper PHY configuration register 3
    #[inline(always)]
    pub const fn wpcr3(&self) -> &WPCR3 {
        &self.wpcr3
    }
    ///0x428 - DSI wrapper PHY configuration register 4
    #[inline(always)]
    pub const fn wpcr4(&self) -> &WPCR4 {
        &self.wpcr4
    }
    ///0x430 - DSI wrapper regulator and PLL control register
    #[inline(always)]
    pub const fn wrpcr(&self) -> &WRPCR {
        &self.wrpcr
    }
}
/**VR (r) register accessor: DSI Host version register

You can [`read`](crate::Reg::read) this register and get [`vr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VR)

For information about available fields see [`mod@vr`] module*/
pub type VR = crate::Reg<vr::VRrs>;
///DSI Host version register
pub mod vr;
/**CR (rw) register accessor: DSI Host control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DSI Host control register
pub mod cr;
/**CCR (rw) register accessor: DSI Host clock control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DSI Host clock control register
pub mod ccr;
/**LVCIDR (rw) register accessor: DSI Host LTDC VCID register

You can [`read`](crate::Reg::read) this register and get [`lvcidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LVCIDR)

For information about available fields see [`mod@lvcidr`] module*/
pub type LVCIDR = crate::Reg<lvcidr::LVCIDRrs>;
///DSI Host LTDC VCID register
pub mod lvcidr;
/**LCOLCR (rw) register accessor: DSI Host LTDC color coding register

You can [`read`](crate::Reg::read) this register and get [`lcolcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcolcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LCOLCR)

For information about available fields see [`mod@lcolcr`] module*/
pub type LCOLCR = crate::Reg<lcolcr::LCOLCRrs>;
///DSI Host LTDC color coding register
pub mod lcolcr;
/**LPCR (rw) register accessor: DSI Host LTDC polarity configuration register

You can [`read`](crate::Reg::read) this register and get [`lpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LPCR)

For information about available fields see [`mod@lpcr`] module*/
pub type LPCR = crate::Reg<lpcr::LPCRrs>;
///DSI Host LTDC polarity configuration register
pub mod lpcr;
/**LPMCR (rw) register accessor: DSI Host low-power mode configuration register

You can [`read`](crate::Reg::read) this register and get [`lpmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LPMCR)

For information about available fields see [`mod@lpmcr`] module*/
pub type LPMCR = crate::Reg<lpmcr::LPMCRrs>;
///DSI Host low-power mode configuration register
pub mod lpmcr;
/**PCR (rw) register accessor: DSI Host protocol configuration register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///DSI Host protocol configuration register
pub mod pcr;
/**GVCIDR (r) register accessor: DSI Host generic VCID register

You can [`read`](crate::Reg::read) this register and get [`gvcidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:GVCIDR)

For information about available fields see [`mod@gvcidr`] module*/
pub type GVCIDR = crate::Reg<gvcidr::GVCIDRrs>;
///DSI Host generic VCID register
pub mod gvcidr;
/**MCR (rw) register accessor: DSI Host mode configuration register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:MCR)

For information about available fields see [`mod@mcr`] module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///DSI Host mode configuration register
pub mod mcr;
/**VMCR (rw) register accessor: DSI Host video mode configuration register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VMCR)

For information about available fields see [`mod@vmcr`] module*/
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
///DSI Host video mode configuration register
pub mod vmcr;
/**VPCR (rw) register accessor: DSI Host video packet configuration register

You can [`read`](crate::Reg::read) this register and get [`vpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VPCR)

For information about available fields see [`mod@vpcr`] module*/
pub type VPCR = crate::Reg<vpcr::VPCRrs>;
///DSI Host video packet configuration register
pub mod vpcr;
/**VCCR (rw) register accessor: DSI Host video chunks configuration register

You can [`read`](crate::Reg::read) this register and get [`vccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VCCR)

For information about available fields see [`mod@vccr`] module*/
pub type VCCR = crate::Reg<vccr::VCCRrs>;
///DSI Host video chunks configuration register
pub mod vccr;
/**VNPCR (rw) register accessor: DSI Host video null packet configuration register

You can [`read`](crate::Reg::read) this register and get [`vnpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vnpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VNPCR)

For information about available fields see [`mod@vnpcr`] module*/
pub type VNPCR = crate::Reg<vnpcr::VNPCRrs>;
///DSI Host video null packet configuration register
pub mod vnpcr;
/**VHSACR (rw) register accessor: DSI Host video HSA configuration register

You can [`read`](crate::Reg::read) this register and get [`vhsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VHSACR)

For information about available fields see [`mod@vhsacr`] module*/
pub type VHSACR = crate::Reg<vhsacr::VHSACRrs>;
///DSI Host video HSA configuration register
pub mod vhsacr;
/**VHBPCR (rw) register accessor: DSI Host video HBP configuration register

You can [`read`](crate::Reg::read) this register and get [`vhbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VHBPCR)

For information about available fields see [`mod@vhbpcr`] module*/
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCRrs>;
///DSI Host video HBP configuration register
pub mod vhbpcr;
/**VLCR (rw) register accessor: DSI Host video line configuration register

You can [`read`](crate::Reg::read) this register and get [`vlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VLCR)

For information about available fields see [`mod@vlcr`] module*/
pub type VLCR = crate::Reg<vlcr::VLCRrs>;
///DSI Host video line configuration register
pub mod vlcr;
/**VVSACR (rw) register accessor: DSI Host video VSA configuration register

You can [`read`](crate::Reg::read) this register and get [`vvsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVSACR)

For information about available fields see [`mod@vvsacr`] module*/
pub type VVSACR = crate::Reg<vvsacr::VVSACRrs>;
///DSI Host video VSA configuration register
pub mod vvsacr;
/**VVBPCR (rw) register accessor: DSI Host video VBP configuration register

You can [`read`](crate::Reg::read) this register and get [`vvbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVBPCR)

For information about available fields see [`mod@vvbpcr`] module*/
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCRrs>;
///DSI Host video VBP configuration register
pub mod vvbpcr;
/**VVFPCR (rw) register accessor: DSI Host video VFP configuration register

You can [`read`](crate::Reg::read) this register and get [`vvfpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvfpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVFPCR)

For information about available fields see [`mod@vvfpcr`] module*/
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCRrs>;
///DSI Host video VFP configuration register
pub mod vvfpcr;
/**VVACR (rw) register accessor: DSI Host video VA configuration register

You can [`read`](crate::Reg::read) this register and get [`vvacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVACR)

For information about available fields see [`mod@vvacr`] module*/
pub type VVACR = crate::Reg<vvacr::VVACRrs>;
///DSI Host video VA configuration register
pub mod vvacr;
/**LCCR (rw) register accessor: DSI Host LTDC command configuration register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LCCR)

For information about available fields see [`mod@lccr`] module*/
pub type LCCR = crate::Reg<lccr::LCCRrs>;
///DSI Host LTDC command configuration register
pub mod lccr;
/**CMCR (rw) register accessor: DSI Host command mode configuration register

You can [`read`](crate::Reg::read) this register and get [`cmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:CMCR)

For information about available fields see [`mod@cmcr`] module*/
pub type CMCR = crate::Reg<cmcr::CMCRrs>;
///DSI Host command mode configuration register
pub mod cmcr;
/**GHCR (rw) register accessor: DSI Host generic header configuration register

You can [`read`](crate::Reg::read) this register and get [`ghcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:GHCR)

For information about available fields see [`mod@ghcr`] module*/
pub type GHCR = crate::Reg<ghcr::GHCRrs>;
///DSI Host generic header configuration register
pub mod ghcr;
/**GPDR (rw) register accessor: DSI Host generic payload data register

You can [`read`](crate::Reg::read) this register and get [`gpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:GPDR)

For information about available fields see [`mod@gpdr`] module*/
pub type GPDR = crate::Reg<gpdr::GPDRrs>;
///DSI Host generic payload data register
pub mod gpdr;
/**GPSR (r) register accessor: DSI Host generic packet status register

You can [`read`](crate::Reg::read) this register and get [`gpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:GPSR)

For information about available fields see [`mod@gpsr`] module*/
pub type GPSR = crate::Reg<gpsr::GPSRrs>;
///DSI Host generic packet status register
pub mod gpsr;
/**TCCR0 (rw) register accessor: DSI Host timeout counter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`tccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR0)

For information about available fields see [`mod@tccr0`] module*/
pub type TCCR0 = crate::Reg<tccr0::TCCR0rs>;
///DSI Host timeout counter configuration register 0
pub mod tccr0;
/**TCCR1 (rw) register accessor: DSI Host timeout counter configuration register 1

You can [`read`](crate::Reg::read) this register and get [`tccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR1)

For information about available fields see [`mod@tccr1`] module*/
pub type TCCR1 = crate::Reg<tccr1::TCCR1rs>;
///DSI Host timeout counter configuration register 1
pub mod tccr1;
/**TCCR2 (rw) register accessor: DSI Host timeout counter configuration register 2

You can [`read`](crate::Reg::read) this register and get [`tccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR2)

For information about available fields see [`mod@tccr2`] module*/
pub type TCCR2 = crate::Reg<tccr2::TCCR2rs>;
///DSI Host timeout counter configuration register 2
pub mod tccr2;
/**TCCR3 (rw) register accessor: DSI Host timeout counter configuration register 3

You can [`read`](crate::Reg::read) this register and get [`tccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR3)

For information about available fields see [`mod@tccr3`] module*/
pub type TCCR3 = crate::Reg<tccr3::TCCR3rs>;
///DSI Host timeout counter configuration register 3
pub mod tccr3;
/**TCCR4 (rw) register accessor: DSI Host timeout counter configuration register 4

You can [`read`](crate::Reg::read) this register and get [`tccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR4)

For information about available fields see [`mod@tccr4`] module*/
pub type TCCR4 = crate::Reg<tccr4::TCCR4rs>;
///DSI Host timeout counter configuration register 4
pub mod tccr4;
/**TCCR5 (rw) register accessor: DSI Host timeout counter configuration register 5

You can [`read`](crate::Reg::read) this register and get [`tccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:TCCR5)

For information about available fields see [`mod@tccr5`] module*/
pub type TCCR5 = crate::Reg<tccr5::TCCR5rs>;
///DSI Host timeout counter configuration register 5
pub mod tccr5;
/**CLCR (rw) register accessor: DSI Host clock lane configuration register

You can [`read`](crate::Reg::read) this register and get [`clcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:CLCR)

For information about available fields see [`mod@clcr`] module*/
pub type CLCR = crate::Reg<clcr::CLCRrs>;
///DSI Host clock lane configuration register
pub mod clcr;
/**CLTCR (rw) register accessor: DSI Host clock lane timer configuration register

You can [`read`](crate::Reg::read) this register and get [`cltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:CLTCR)

For information about available fields see [`mod@cltcr`] module*/
pub type CLTCR = crate::Reg<cltcr::CLTCRrs>;
///DSI Host clock lane timer configuration register
pub mod cltcr;
/**DLTCR (rw) register accessor: DSI Host data lane timer configuration register

You can [`read`](crate::Reg::read) this register and get [`dltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:DLTCR)

For information about available fields see [`mod@dltcr`] module*/
pub type DLTCR = crate::Reg<dltcr::DLTCRrs>;
///DSI Host data lane timer configuration register
pub mod dltcr;
/**PCTLR (rw) register accessor: DSI Host PHY control register

You can [`read`](crate::Reg::read) this register and get [`pctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:PCTLR)

For information about available fields see [`mod@pctlr`] module*/
pub type PCTLR = crate::Reg<pctlr::PCTLRrs>;
///DSI Host PHY control register
pub mod pctlr;
/**PCONFR (rw) register accessor: DSI Host PHY configuration register

You can [`read`](crate::Reg::read) this register and get [`pconfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:PCONFR)

For information about available fields see [`mod@pconfr`] module*/
pub type PCONFR = crate::Reg<pconfr::PCONFRrs>;
///DSI Host PHY configuration register
pub mod pconfr;
/**PUCR (rw) register accessor: DSI Host PHY ULPS control register

You can [`read`](crate::Reg::read) this register and get [`pucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:PUCR)

For information about available fields see [`mod@pucr`] module*/
pub type PUCR = crate::Reg<pucr::PUCRrs>;
///DSI Host PHY ULPS control register
pub mod pucr;
/**PTTCR (rw) register accessor: DSI Host PHY TX triggers configuration register

You can [`read`](crate::Reg::read) this register and get [`pttcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pttcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:PTTCR)

For information about available fields see [`mod@pttcr`] module*/
pub type PTTCR = crate::Reg<pttcr::PTTCRrs>;
///DSI Host PHY TX triggers configuration register
pub mod pttcr;
/**PSR (r) register accessor: DSI Host PHY status register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:PSR)

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSRrs>;
///DSI Host PHY status register
pub mod psr;
/**ISR0 (r) register accessor: DSI Host interrupt and status register 0

You can [`read`](crate::Reg::read) this register and get [`isr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:ISR0)

For information about available fields see [`mod@isr0`] module*/
pub type ISR0 = crate::Reg<isr0::ISR0rs>;
///DSI Host interrupt and status register 0
pub mod isr0;
/**ISR1 (r) register accessor: DSI Host interrupt and status register 1

You can [`read`](crate::Reg::read) this register and get [`isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:ISR1)

For information about available fields see [`mod@isr1`] module*/
pub type ISR1 = crate::Reg<isr1::ISR1rs>;
///DSI Host interrupt and status register 1
pub mod isr1;
/**IER0 (rw) register accessor: DSI Host interrupt enable register 0

You can [`read`](crate::Reg::read) this register and get [`ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:IER0)

For information about available fields see [`mod@ier0`] module*/
pub type IER0 = crate::Reg<ier0::IER0rs>;
///DSI Host interrupt enable register 0
pub mod ier0;
/**IER1 (rw) register accessor: DSI Host interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///DSI Host interrupt enable register 1
pub mod ier1;
/**FIR0 (w) register accessor: DSI Host force interrupt register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:FIR0)

For information about available fields see [`mod@fir0`] module*/
pub type FIR0 = crate::Reg<fir0::FIR0rs>;
///DSI Host force interrupt register 0
pub mod fir0;
/**FIR1 (w) register accessor: DSI Host force interrupt register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:FIR1)

For information about available fields see [`mod@fir1`] module*/
pub type FIR1 = crate::Reg<fir1::FIR1rs>;
///DSI Host force interrupt register 1
pub mod fir1;
/**VSCR (rw) register accessor: DSI Host video shadow control register

You can [`read`](crate::Reg::read) this register and get [`vscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VSCR)

For information about available fields see [`mod@vscr`] module*/
pub type VSCR = crate::Reg<vscr::VSCRrs>;
///DSI Host video shadow control register
pub mod vscr;
/**LCVCIDR (rw) register accessor: DSI Host LTDC current VCID register

You can [`read`](crate::Reg::read) this register and get [`lcvcidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcvcidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LCVCIDR)

For information about available fields see [`mod@lcvcidr`] module*/
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDRrs>;
///DSI Host LTDC current VCID register
pub mod lcvcidr;
/**LCCCR (r) register accessor: DSI Host LTDC current color coding register

You can [`read`](crate::Reg::read) this register and get [`lcccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LCCCR)

For information about available fields see [`mod@lcccr`] module*/
pub type LCCCR = crate::Reg<lcccr::LCCCRrs>;
///DSI Host LTDC current color coding register
pub mod lcccr;
/**LPMCCR (r) register accessor: DSI Host low-power mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`lpmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:LPMCCR)

For information about available fields see [`mod@lpmccr`] module*/
pub type LPMCCR = crate::Reg<lpmccr::LPMCCRrs>;
///DSI Host low-power mode current configuration register
pub mod lpmccr;
/**VMCCR (r) register accessor: DSI Host video mode current configuration register

You can [`read`](crate::Reg::read) this register and get [`vmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VMCCR)

For information about available fields see [`mod@vmccr`] module*/
pub type VMCCR = crate::Reg<vmccr::VMCCRrs>;
///DSI Host video mode current configuration register
pub mod vmccr;
/**VPCCR (r) register accessor: DSI Host video packet current configuration register

You can [`read`](crate::Reg::read) this register and get [`vpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VPCCR)

For information about available fields see [`mod@vpccr`] module*/
pub type VPCCR = crate::Reg<vpccr::VPCCRrs>;
///DSI Host video packet current configuration register
pub mod vpccr;
/**VCCCR (r) register accessor: DSI Host video chunks current configuration register

You can [`read`](crate::Reg::read) this register and get [`vcccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VCCCR)

For information about available fields see [`mod@vcccr`] module*/
pub type VCCCR = crate::Reg<vcccr::VCCCRrs>;
///DSI Host video chunks current configuration register
pub mod vcccr;
/**VNPCCR (r) register accessor: DSI Host video null packet current configuration register

You can [`read`](crate::Reg::read) this register and get [`vnpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VNPCCR)

For information about available fields see [`mod@vnpccr`] module*/
pub type VNPCCR = crate::Reg<vnpccr::VNPCCRrs>;
///DSI Host video null packet current configuration register
pub mod vnpccr;
/**VHSACCR (r) register accessor: DSI Host video HSA current configuration register

You can [`read`](crate::Reg::read) this register and get [`vhsaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VHSACCR)

For information about available fields see [`mod@vhsaccr`] module*/
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCRrs>;
///DSI Host video HSA current configuration register
pub mod vhsaccr;
/**VHBPCCR (r) register accessor: DSI Host video HBP current configuration register

You can [`read`](crate::Reg::read) this register and get [`vhbpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VHBPCCR)

For information about available fields see [`mod@vhbpccr`] module*/
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCRrs>;
///DSI Host video HBP current configuration register
pub mod vhbpccr;
/**VLCCR (r) register accessor: DSI Host video line current configuration register

You can [`read`](crate::Reg::read) this register and get [`vlccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VLCCR)

For information about available fields see [`mod@vlccr`] module*/
pub type VLCCR = crate::Reg<vlccr::VLCCRrs>;
///DSI Host video line current configuration register
pub mod vlccr;
/**VVSACCR (r) register accessor: DSI Host video VSA current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvsaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVSACCR)

For information about available fields see [`mod@vvsaccr`] module*/
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCRrs>;
///DSI Host video VSA current configuration register
pub mod vvsaccr;
/**VVBPCCR (r) register accessor: DSI Host video VBP current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvbpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVBPCCR)

For information about available fields see [`mod@vvbpccr`] module*/
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCRrs>;
///DSI Host video VBP current configuration register
pub mod vvbpccr;
/**VVFPCCR (r) register accessor: DSI Host video VFP current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvfpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVFPCCR)

For information about available fields see [`mod@vvfpccr`] module*/
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCRrs>;
///DSI Host video VFP current configuration register
pub mod vvfpccr;
/**VVACCR (r) register accessor: DSI Host video VA current configuration register

You can [`read`](crate::Reg::read) this register and get [`vvaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:VVACCR)

For information about available fields see [`mod@vvaccr`] module*/
pub type VVACCR = crate::Reg<vvaccr::VVACCRrs>;
///DSI Host video VA current configuration register
pub mod vvaccr;
/**WCFGR (rw) register accessor: DSI wrapper configuration register

You can [`read`](crate::Reg::read) this register and get [`wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WCFGR)

For information about available fields see [`mod@wcfgr`] module*/
pub type WCFGR = crate::Reg<wcfgr::WCFGRrs>;
///DSI wrapper configuration register
pub mod wcfgr;
/**WCR (rw) register accessor: DSI wrapper control register

You can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WCR)

For information about available fields see [`mod@wcr`] module*/
pub type WCR = crate::Reg<wcr::WCRrs>;
///DSI wrapper control register
pub mod wcr;
/**WIER (rw) register accessor: DSI wrapper interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`wier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WIER)

For information about available fields see [`mod@wier`] module*/
pub type WIER = crate::Reg<wier::WIERrs>;
///DSI wrapper interrupt enable register
pub mod wier;
/**WISR (r) register accessor: DSI wrapper interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`wisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WISR)

For information about available fields see [`mod@wisr`] module*/
pub type WISR = crate::Reg<wisr::WISRrs>;
///DSI wrapper interrupt and status register
pub mod wisr;
/**WIFCR (w) register accessor: DSI wrapper interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WIFCR)

For information about available fields see [`mod@wifcr`] module*/
pub type WIFCR = crate::Reg<wifcr::WIFCRrs>;
///DSI wrapper interrupt flag clear register
pub mod wifcr;
/**WPCR0 (rw) register accessor: DSI wrapper PHY configuration register 0

You can [`read`](crate::Reg::read) this register and get [`wpcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WPCR0)

For information about available fields see [`mod@wpcr0`] module*/
pub type WPCR0 = crate::Reg<wpcr0::WPCR0rs>;
///DSI wrapper PHY configuration register 0
pub mod wpcr0;
/**WPCR1 (rw) register accessor: This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).

You can [`read`](crate::Reg::read) this register and get [`wpcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WPCR1)

For information about available fields see [`mod@wpcr1`] module*/
pub type WPCR1 = crate::Reg<wpcr1::WPCR1rs>;
///This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).
pub mod wpcr1;
/**WPCR2 (rw) register accessor: DSI wrapper PHY configuration register 2

You can [`read`](crate::Reg::read) this register and get [`wpcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WPCR2)

For information about available fields see [`mod@wpcr2`] module*/
pub type WPCR2 = crate::Reg<wpcr2::WPCR2rs>;
///DSI wrapper PHY configuration register 2
pub mod wpcr2;
/**WPCR3 (rw) register accessor: DSI wrapper PHY configuration register 3

You can [`read`](crate::Reg::read) this register and get [`wpcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WPCR3)

For information about available fields see [`mod@wpcr3`] module*/
pub type WPCR3 = crate::Reg<wpcr3::WPCR3rs>;
///DSI wrapper PHY configuration register 3
pub mod wpcr3;
/**WPCR4 (rw) register accessor: DSI wrapper PHY configuration register 4

You can [`read`](crate::Reg::read) this register and get [`wpcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WPCR4)

For information about available fields see [`mod@wpcr4`] module*/
pub type WPCR4 = crate::Reg<wpcr4::WPCR4rs>;
///DSI wrapper PHY configuration register 4
pub mod wpcr4;
/**WRPCR (rw) register accessor: DSI wrapper regulator and PLL control register

You can [`read`](crate::Reg::read) this register and get [`wrpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:WRPCR)

For information about available fields see [`mod@wrpcr`] module*/
pub type WRPCR = crate::Reg<wrpcr::WRPCRrs>;
///DSI wrapper regulator and PLL control register
pub mod wrpcr;
