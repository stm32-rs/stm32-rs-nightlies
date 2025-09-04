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
    ///0x00 - DSI Host Version Register
    #[inline(always)]
    pub const fn vr(&self) -> &VR {
        &self.vr
    }
    ///0x04 - DSI Host Control Register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - DSI HOST Clock Control Register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x0c - DSI Host LTDC VCID Register
    #[inline(always)]
    pub const fn lvcidr(&self) -> &LVCIDR {
        &self.lvcidr
    }
    ///0x10 - DSI Host LTDC Color Coding Register
    #[inline(always)]
    pub const fn lcolcr(&self) -> &LCOLCR {
        &self.lcolcr
    }
    ///0x14 - DSI Host LTDC Polarity Configuration Register
    #[inline(always)]
    pub const fn lpcr(&self) -> &LPCR {
        &self.lpcr
    }
    ///0x18 - DSI Host Low-Power mode Configuration Register
    #[inline(always)]
    pub const fn lpmcr(&self) -> &LPMCR {
        &self.lpmcr
    }
    ///0x2c - DSI Host Protocol Configuration Register
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    ///0x30 - DSI Host Generic VCID Register
    #[inline(always)]
    pub const fn gvcidr(&self) -> &GVCIDR {
        &self.gvcidr
    }
    ///0x34 - DSI Host mode Configuration Register
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x38 - DSI Host Video mode Configuration Register
    #[inline(always)]
    pub const fn vmcr(&self) -> &VMCR {
        &self.vmcr
    }
    ///0x3c - DSI Host Video Packet Configuration Register
    #[inline(always)]
    pub const fn vpcr(&self) -> &VPCR {
        &self.vpcr
    }
    ///0x40 - DSI Host Video Chunks Configuration Register
    #[inline(always)]
    pub const fn vccr(&self) -> &VCCR {
        &self.vccr
    }
    ///0x44 - DSI Host Video Null Packet Configuration Register
    #[inline(always)]
    pub const fn vnpcr(&self) -> &VNPCR {
        &self.vnpcr
    }
    ///0x48 - DSI Host Video HSA Configuration Register
    #[inline(always)]
    pub const fn vhsacr(&self) -> &VHSACR {
        &self.vhsacr
    }
    ///0x4c - DSI Host Video HBP Configuration Register
    #[inline(always)]
    pub const fn vhbpcr(&self) -> &VHBPCR {
        &self.vhbpcr
    }
    ///0x50 - DSI Host Video Line Configuration Register
    #[inline(always)]
    pub const fn vlcr(&self) -> &VLCR {
        &self.vlcr
    }
    ///0x54 - DSI Host Video VSA Configuration Register
    #[inline(always)]
    pub const fn vvsacr(&self) -> &VVSACR {
        &self.vvsacr
    }
    ///0x58 - DSI Host Video VBP Configuration Register
    #[inline(always)]
    pub const fn vvbpcr(&self) -> &VVBPCR {
        &self.vvbpcr
    }
    ///0x5c - DSI Host Video VFP Configuration Register
    #[inline(always)]
    pub const fn vvfpcr(&self) -> &VVFPCR {
        &self.vvfpcr
    }
    ///0x60 - DSI Host Video VA Configuration Register
    #[inline(always)]
    pub const fn vvacr(&self) -> &VVACR {
        &self.vvacr
    }
    ///0x64 - DSI Host LTDC Command Configuration Register
    #[inline(always)]
    pub const fn lccr(&self) -> &LCCR {
        &self.lccr
    }
    ///0x68 - DSI Host Command mode Configuration Register
    #[inline(always)]
    pub const fn cmcr(&self) -> &CMCR {
        &self.cmcr
    }
    ///0x6c - DSI Host Generic Header Configuration Register
    #[inline(always)]
    pub const fn ghcr(&self) -> &GHCR {
        &self.ghcr
    }
    ///0x70 - DSI Host Generic Payload Data Register
    #[inline(always)]
    pub const fn gpdr(&self) -> &GPDR {
        &self.gpdr
    }
    ///0x74 - DSI Host Generic Packet Status Register
    #[inline(always)]
    pub const fn gpsr(&self) -> &GPSR {
        &self.gpsr
    }
    ///0x78 - DSI Host Timeout Counter Configuration Register 0
    #[inline(always)]
    pub const fn tccr0(&self) -> &TCCR0 {
        &self.tccr0
    }
    ///0x7c - DSI Host Timeout Counter Configuration Register 1
    #[inline(always)]
    pub const fn tccr1(&self) -> &TCCR1 {
        &self.tccr1
    }
    ///0x80 - DSI Host Timeout Counter Configuration Register 2
    #[inline(always)]
    pub const fn tccr2(&self) -> &TCCR2 {
        &self.tccr2
    }
    ///0x84 - DSI Host Timeout Counter Configuration Register 3
    #[inline(always)]
    pub const fn tccr3(&self) -> &TCCR3 {
        &self.tccr3
    }
    ///0x88 - DSI Host Timeout Counter Configuration Register 4
    #[inline(always)]
    pub const fn tccr4(&self) -> &TCCR4 {
        &self.tccr4
    }
    ///0x8c - DSI Host Timeout Counter Configuration Register 5
    #[inline(always)]
    pub const fn tccr5(&self) -> &TCCR5 {
        &self.tccr5
    }
    ///0x94 - DSI Host Clock Lane Configuration Register
    #[inline(always)]
    pub const fn clcr(&self) -> &CLCR {
        &self.clcr
    }
    ///0x98 - DSI Host Clock Lane Timer Configuration Register
    #[inline(always)]
    pub const fn cltcr(&self) -> &CLTCR {
        &self.cltcr
    }
    ///0x9c - DSI Host Data Lane Timer Configuration Register
    #[inline(always)]
    pub const fn dltcr(&self) -> &DLTCR {
        &self.dltcr
    }
    ///0xa0 - DSI Host PHY Control Register
    #[inline(always)]
    pub const fn pctlr(&self) -> &PCTLR {
        &self.pctlr
    }
    ///0xa4 - DSI Host PHY Configuration Register
    #[inline(always)]
    pub const fn pconfr(&self) -> &PCONFR {
        &self.pconfr
    }
    ///0xa8 - DSI Host PHY ULPS Control Register
    #[inline(always)]
    pub const fn pucr(&self) -> &PUCR {
        &self.pucr
    }
    ///0xac - DSI Host PHY TX Triggers Configuration Register
    #[inline(always)]
    pub const fn pttcr(&self) -> &PTTCR {
        &self.pttcr
    }
    ///0xb0 - DSI Host PHY Status Register
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    ///0xbc - DSI Host Interrupt & Status Register 0
    #[inline(always)]
    pub const fn isr0(&self) -> &ISR0 {
        &self.isr0
    }
    ///0xc0 - DSI Host Interrupt & Status Register 1
    #[inline(always)]
    pub const fn isr1(&self) -> &ISR1 {
        &self.isr1
    }
    ///0xc4 - DSI Host Interrupt Enable Register 0
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    ///0xc8 - DSI Host Interrupt Enable Register 1
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    ///0xd8 - DSI Host Force Interrupt Register 0
    #[inline(always)]
    pub const fn fir0(&self) -> &FIR0 {
        &self.fir0
    }
    ///0xdc - DSI Host Force Interrupt Register 1
    #[inline(always)]
    pub const fn fir1(&self) -> &FIR1 {
        &self.fir1
    }
    ///0x100 - DSI Host Video Shadow Control Register
    #[inline(always)]
    pub const fn vscr(&self) -> &VSCR {
        &self.vscr
    }
    ///0x10c - DSI Host LTDC Current VCID Register
    #[inline(always)]
    pub const fn lcvcidr(&self) -> &LCVCIDR {
        &self.lcvcidr
    }
    ///0x110 - DSI Host LTDC Current Color Coding Register
    #[inline(always)]
    pub const fn lcccr(&self) -> &LCCCR {
        &self.lcccr
    }
    ///0x118 - DSI Host Low-Power mode Current Configuration Register
    #[inline(always)]
    pub const fn lpmccr(&self) -> &LPMCCR {
        &self.lpmccr
    }
    ///0x138 - DSI Host Video mode Current Configuration Register
    #[inline(always)]
    pub const fn vmccr(&self) -> &VMCCR {
        &self.vmccr
    }
    ///0x13c - DSI Host Video Packet Current Configuration Register
    #[inline(always)]
    pub const fn vpccr(&self) -> &VPCCR {
        &self.vpccr
    }
    ///0x140 - DSI Host Video Chunks Current Configuration Register
    #[inline(always)]
    pub const fn vcccr(&self) -> &VCCCR {
        &self.vcccr
    }
    ///0x144 - DSI Host Video Null Packet Current Configuration Register
    #[inline(always)]
    pub const fn vnpccr(&self) -> &VNPCCR {
        &self.vnpccr
    }
    ///0x148 - DSI Host Video HSA Current Configuration Register
    #[inline(always)]
    pub const fn vhsaccr(&self) -> &VHSACCR {
        &self.vhsaccr
    }
    ///0x14c - DSI Host Video HBP Current Configuration Register
    #[inline(always)]
    pub const fn vhbpccr(&self) -> &VHBPCCR {
        &self.vhbpccr
    }
    ///0x150 - DSI Host Video Line Current Configuration Register
    #[inline(always)]
    pub const fn vlccr(&self) -> &VLCCR {
        &self.vlccr
    }
    ///0x154 - DSI Host Video VSA Current Configuration Register
    #[inline(always)]
    pub const fn vvsaccr(&self) -> &VVSACCR {
        &self.vvsaccr
    }
    ///0x158 - DSI Host Video VBP Current Configuration Register
    #[inline(always)]
    pub const fn vvbpccr(&self) -> &VVBPCCR {
        &self.vvbpccr
    }
    ///0x15c - DSI Host Video VFP Current Configuration Register
    #[inline(always)]
    pub const fn vvfpccr(&self) -> &VVFPCCR {
        &self.vvfpccr
    }
    ///0x160 - DSI Host Video VA Current Configuration Register
    #[inline(always)]
    pub const fn vvaccr(&self) -> &VVACCR {
        &self.vvaccr
    }
    ///0x400 - DSI Wrapper Configuration Register
    #[inline(always)]
    pub const fn wcfgr(&self) -> &WCFGR {
        &self.wcfgr
    }
    ///0x404 - DSI Wrapper Control Register
    #[inline(always)]
    pub const fn wcr(&self) -> &WCR {
        &self.wcr
    }
    ///0x408 - DSI Wrapper Interrupt Enable Register
    #[inline(always)]
    pub const fn wier(&self) -> &WIER {
        &self.wier
    }
    ///0x40c - DSI Wrapper Interrupt & Status Register
    #[inline(always)]
    pub const fn wisr(&self) -> &WISR {
        &self.wisr
    }
    ///0x410 - DSI Wrapper Interrupt Flag Clear Register
    #[inline(always)]
    pub const fn wifcr(&self) -> &WIFCR {
        &self.wifcr
    }
    ///0x418 - DSI Wrapper PHY Configuration Register 1
    #[inline(always)]
    pub const fn wpcr0(&self) -> &WPCR0 {
        &self.wpcr0
    }
    ///0x41c - DSI Wrapper PHY Configuration Register 2
    #[inline(always)]
    pub const fn wpcr1(&self) -> &WPCR1 {
        &self.wpcr1
    }
    ///0x420 - DSI Wrapper PHY Configuration Register 3
    #[inline(always)]
    pub const fn wpcr2(&self) -> &WPCR2 {
        &self.wpcr2
    }
    ///0x424 - DSI_WPCR4
    #[inline(always)]
    pub const fn wpcr3(&self) -> &WPCR3 {
        &self.wpcr3
    }
    ///0x428 - DSI Wrapper PHY Configuration Register 5
    #[inline(always)]
    pub const fn wpcr4(&self) -> &WPCR4 {
        &self.wpcr4
    }
    ///0x430 - DSI Wrapper Regulator and PLL Control Register
    #[inline(always)]
    pub const fn wrpcr(&self) -> &WRPCR {
        &self.wrpcr
    }
}
/**VR (r) register accessor: DSI Host Version Register

You can [`read`](crate::Reg::read) this register and get [`vr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VR)

For information about available fields see [`mod@vr`] module*/
pub type VR = crate::Reg<vr::VRrs>;
///DSI Host Version Register
pub mod vr;
/**CR (rw) register accessor: DSI Host Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DSI Host Control Register
pub mod cr;
/**CCR (rw) register accessor: DSI HOST Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DSI HOST Clock Control Register
pub mod ccr;
/**LVCIDR (rw) register accessor: DSI Host LTDC VCID Register

You can [`read`](crate::Reg::read) this register and get [`lvcidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvcidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LVCIDR)

For information about available fields see [`mod@lvcidr`] module*/
pub type LVCIDR = crate::Reg<lvcidr::LVCIDRrs>;
///DSI Host LTDC VCID Register
pub mod lvcidr;
/**LCOLCR (rw) register accessor: DSI Host LTDC Color Coding Register

You can [`read`](crate::Reg::read) this register and get [`lcolcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcolcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LCOLCR)

For information about available fields see [`mod@lcolcr`] module*/
pub type LCOLCR = crate::Reg<lcolcr::LCOLCRrs>;
///DSI Host LTDC Color Coding Register
pub mod lcolcr;
/**LPCR (rw) register accessor: DSI Host LTDC Polarity Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LPCR)

For information about available fields see [`mod@lpcr`] module*/
pub type LPCR = crate::Reg<lpcr::LPCRrs>;
///DSI Host LTDC Polarity Configuration Register
pub mod lpcr;
/**LPMCR (rw) register accessor: DSI Host Low-Power mode Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lpmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LPMCR)

For information about available fields see [`mod@lpmcr`] module*/
pub type LPMCR = crate::Reg<lpmcr::LPMCRrs>;
///DSI Host Low-Power mode Configuration Register
pub mod lpmcr;
/**PCR (rw) register accessor: DSI Host Protocol Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///DSI Host Protocol Configuration Register
pub mod pcr;
/**GVCIDR (rw) register accessor: DSI Host Generic VCID Register

You can [`read`](crate::Reg::read) this register and get [`gvcidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gvcidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:GVCIDR)

For information about available fields see [`mod@gvcidr`] module*/
pub type GVCIDR = crate::Reg<gvcidr::GVCIDRrs>;
///DSI Host Generic VCID Register
pub mod gvcidr;
/**MCR (rw) register accessor: DSI Host mode Configuration Register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:MCR)

For information about available fields see [`mod@mcr`] module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///DSI Host mode Configuration Register
pub mod mcr;
/**VMCR (rw) register accessor: DSI Host Video mode Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VMCR)

For information about available fields see [`mod@vmcr`] module*/
pub type VMCR = crate::Reg<vmcr::VMCRrs>;
///DSI Host Video mode Configuration Register
pub mod vmcr;
/**VPCR (rw) register accessor: DSI Host Video Packet Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VPCR)

For information about available fields see [`mod@vpcr`] module*/
pub type VPCR = crate::Reg<vpcr::VPCRrs>;
///DSI Host Video Packet Configuration Register
pub mod vpcr;
/**VCCR (rw) register accessor: DSI Host Video Chunks Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VCCR)

For information about available fields see [`mod@vccr`] module*/
pub type VCCR = crate::Reg<vccr::VCCRrs>;
///DSI Host Video Chunks Configuration Register
pub mod vccr;
/**VNPCR (rw) register accessor: DSI Host Video Null Packet Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vnpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vnpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VNPCR)

For information about available fields see [`mod@vnpcr`] module*/
pub type VNPCR = crate::Reg<vnpcr::VNPCRrs>;
///DSI Host Video Null Packet Configuration Register
pub mod vnpcr;
/**VHSACR (rw) register accessor: DSI Host Video HSA Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vhsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VHSACR)

For information about available fields see [`mod@vhsacr`] module*/
pub type VHSACR = crate::Reg<vhsacr::VHSACRrs>;
///DSI Host Video HSA Configuration Register
pub mod vhsacr;
/**VHBPCR (rw) register accessor: DSI Host Video HBP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vhbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VHBPCR)

For information about available fields see [`mod@vhbpcr`] module*/
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCRrs>;
///DSI Host Video HBP Configuration Register
pub mod vhbpcr;
/**VLCR (rw) register accessor: DSI Host Video Line Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VLCR)

For information about available fields see [`mod@vlcr`] module*/
pub type VLCR = crate::Reg<vlcr::VLCRrs>;
///DSI Host Video Line Configuration Register
pub mod vlcr;
/**VVSACR (rw) register accessor: DSI Host Video VSA Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVSACR)

For information about available fields see [`mod@vvsacr`] module*/
pub type VVSACR = crate::Reg<vvsacr::VVSACRrs>;
///DSI Host Video VSA Configuration Register
pub mod vvsacr;
/**VVBPCR (rw) register accessor: DSI Host Video VBP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVBPCR)

For information about available fields see [`mod@vvbpcr`] module*/
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCRrs>;
///DSI Host Video VBP Configuration Register
pub mod vvbpcr;
/**VVFPCR (rw) register accessor: DSI Host Video VFP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvfpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvfpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVFPCR)

For information about available fields see [`mod@vvfpcr`] module*/
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCRrs>;
///DSI Host Video VFP Configuration Register
pub mod vvfpcr;
/**VVACR (rw) register accessor: DSI Host Video VA Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vvacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVACR)

For information about available fields see [`mod@vvacr`] module*/
pub type VVACR = crate::Reg<vvacr::VVACRrs>;
///DSI Host Video VA Configuration Register
pub mod vvacr;
/**LCCR (rw) register accessor: DSI Host LTDC Command Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LCCR)

For information about available fields see [`mod@lccr`] module*/
pub type LCCR = crate::Reg<lccr::LCCRrs>;
///DSI Host LTDC Command Configuration Register
pub mod lccr;
/**CMCR (rw) register accessor: DSI Host Command mode Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:CMCR)

For information about available fields see [`mod@cmcr`] module*/
pub type CMCR = crate::Reg<cmcr::CMCRrs>;
///DSI Host Command mode Configuration Register
pub mod cmcr;
/**GHCR (rw) register accessor: DSI Host Generic Header Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ghcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:GHCR)

For information about available fields see [`mod@ghcr`] module*/
pub type GHCR = crate::Reg<ghcr::GHCRrs>;
///DSI Host Generic Header Configuration Register
pub mod ghcr;
/**GPDR (rw) register accessor: DSI Host Generic Payload Data Register

You can [`read`](crate::Reg::read) this register and get [`gpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:GPDR)

For information about available fields see [`mod@gpdr`] module*/
pub type GPDR = crate::Reg<gpdr::GPDRrs>;
///DSI Host Generic Payload Data Register
pub mod gpdr;
/**GPSR (r) register accessor: DSI Host Generic Packet Status Register

You can [`read`](crate::Reg::read) this register and get [`gpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:GPSR)

For information about available fields see [`mod@gpsr`] module*/
pub type GPSR = crate::Reg<gpsr::GPSRrs>;
///DSI Host Generic Packet Status Register
pub mod gpsr;
/**TCCR0 (rw) register accessor: DSI Host Timeout Counter Configuration Register 0

You can [`read`](crate::Reg::read) this register and get [`tccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR0)

For information about available fields see [`mod@tccr0`] module*/
pub type TCCR0 = crate::Reg<tccr0::TCCR0rs>;
///DSI Host Timeout Counter Configuration Register 0
pub mod tccr0;
/**TCCR1 (rw) register accessor: DSI Host Timeout Counter Configuration Register 1

You can [`read`](crate::Reg::read) this register and get [`tccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR1)

For information about available fields see [`mod@tccr1`] module*/
pub type TCCR1 = crate::Reg<tccr1::TCCR1rs>;
///DSI Host Timeout Counter Configuration Register 1
pub mod tccr1;
/**TCCR2 (rw) register accessor: DSI Host Timeout Counter Configuration Register 2

You can [`read`](crate::Reg::read) this register and get [`tccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR2)

For information about available fields see [`mod@tccr2`] module*/
pub type TCCR2 = crate::Reg<tccr2::TCCR2rs>;
///DSI Host Timeout Counter Configuration Register 2
pub mod tccr2;
/**TCCR3 (rw) register accessor: DSI Host Timeout Counter Configuration Register 3

You can [`read`](crate::Reg::read) this register and get [`tccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR3)

For information about available fields see [`mod@tccr3`] module*/
pub type TCCR3 = crate::Reg<tccr3::TCCR3rs>;
///DSI Host Timeout Counter Configuration Register 3
pub mod tccr3;
/**TCCR4 (rw) register accessor: DSI Host Timeout Counter Configuration Register 4

You can [`read`](crate::Reg::read) this register and get [`tccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR4)

For information about available fields see [`mod@tccr4`] module*/
pub type TCCR4 = crate::Reg<tccr4::TCCR4rs>;
///DSI Host Timeout Counter Configuration Register 4
pub mod tccr4;
/**TCCR5 (rw) register accessor: DSI Host Timeout Counter Configuration Register 5

You can [`read`](crate::Reg::read) this register and get [`tccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:TCCR5)

For information about available fields see [`mod@tccr5`] module*/
pub type TCCR5 = crate::Reg<tccr5::TCCR5rs>;
///DSI Host Timeout Counter Configuration Register 5
pub mod tccr5;
/**CLCR (rw) register accessor: DSI Host Clock Lane Configuration Register

You can [`read`](crate::Reg::read) this register and get [`clcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:CLCR)

For information about available fields see [`mod@clcr`] module*/
pub type CLCR = crate::Reg<clcr::CLCRrs>;
///DSI Host Clock Lane Configuration Register
pub mod clcr;
/**CLTCR (rw) register accessor: DSI Host Clock Lane Timer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:CLTCR)

For information about available fields see [`mod@cltcr`] module*/
pub type CLTCR = crate::Reg<cltcr::CLTCRrs>;
///DSI Host Clock Lane Timer Configuration Register
pub mod cltcr;
/**DLTCR (rw) register accessor: DSI Host Data Lane Timer Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:DLTCR)

For information about available fields see [`mod@dltcr`] module*/
pub type DLTCR = crate::Reg<dltcr::DLTCRrs>;
///DSI Host Data Lane Timer Configuration Register
pub mod dltcr;
/**PCTLR (rw) register accessor: DSI Host PHY Control Register

You can [`read`](crate::Reg::read) this register and get [`pctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:PCTLR)

For information about available fields see [`mod@pctlr`] module*/
pub type PCTLR = crate::Reg<pctlr::PCTLRrs>;
///DSI Host PHY Control Register
pub mod pctlr;
/**PCONFR (rw) register accessor: DSI Host PHY Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pconfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:PCONFR)

For information about available fields see [`mod@pconfr`] module*/
pub type PCONFR = crate::Reg<pconfr::PCONFRrs>;
///DSI Host PHY Configuration Register
pub mod pconfr;
/**PUCR (rw) register accessor: DSI Host PHY ULPS Control Register

You can [`read`](crate::Reg::read) this register and get [`pucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:PUCR)

For information about available fields see [`mod@pucr`] module*/
pub type PUCR = crate::Reg<pucr::PUCRrs>;
///DSI Host PHY ULPS Control Register
pub mod pucr;
/**PTTCR (rw) register accessor: DSI Host PHY TX Triggers Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pttcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pttcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:PTTCR)

For information about available fields see [`mod@pttcr`] module*/
pub type PTTCR = crate::Reg<pttcr::PTTCRrs>;
///DSI Host PHY TX Triggers Configuration Register
pub mod pttcr;
/**PSR (r) register accessor: DSI Host PHY Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:PSR)

For information about available fields see [`mod@psr`] module*/
pub type PSR = crate::Reg<psr::PSRrs>;
///DSI Host PHY Status Register
pub mod psr;
/**ISR0 (r) register accessor: DSI Host Interrupt & Status Register 0

You can [`read`](crate::Reg::read) this register and get [`isr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:ISR0)

For information about available fields see [`mod@isr0`] module*/
pub type ISR0 = crate::Reg<isr0::ISR0rs>;
///DSI Host Interrupt & Status Register 0
pub mod isr0;
/**ISR1 (r) register accessor: DSI Host Interrupt & Status Register 1

You can [`read`](crate::Reg::read) this register and get [`isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:ISR1)

For information about available fields see [`mod@isr1`] module*/
pub type ISR1 = crate::Reg<isr1::ISR1rs>;
///DSI Host Interrupt & Status Register 1
pub mod isr1;
/**IER0 (rw) register accessor: DSI Host Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:IER0)

For information about available fields see [`mod@ier0`] module*/
pub type IER0 = crate::Reg<ier0::IER0rs>;
///DSI Host Interrupt Enable Register 0
pub mod ier0;
/**IER1 (rw) register accessor: DSI Host Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///DSI Host Interrupt Enable Register 1
pub mod ier1;
/**FIR0 (w) register accessor: DSI Host Force Interrupt Register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:FIR0)

For information about available fields see [`mod@fir0`] module*/
pub type FIR0 = crate::Reg<fir0::FIR0rs>;
///DSI Host Force Interrupt Register 0
pub mod fir0;
/**FIR1 (w) register accessor: DSI Host Force Interrupt Register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fir1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:FIR1)

For information about available fields see [`mod@fir1`] module*/
pub type FIR1 = crate::Reg<fir1::FIR1rs>;
///DSI Host Force Interrupt Register 1
pub mod fir1;
/**VSCR (rw) register accessor: DSI Host Video Shadow Control Register

You can [`read`](crate::Reg::read) this register and get [`vscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VSCR)

For information about available fields see [`mod@vscr`] module*/
pub type VSCR = crate::Reg<vscr::VSCRrs>;
///DSI Host Video Shadow Control Register
pub mod vscr;
/**LCVCIDR (r) register accessor: DSI Host LTDC Current VCID Register

You can [`read`](crate::Reg::read) this register and get [`lcvcidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LCVCIDR)

For information about available fields see [`mod@lcvcidr`] module*/
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDRrs>;
///DSI Host LTDC Current VCID Register
pub mod lcvcidr;
/**LCCCR (r) register accessor: DSI Host LTDC Current Color Coding Register

You can [`read`](crate::Reg::read) this register and get [`lcccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LCCCR)

For information about available fields see [`mod@lcccr`] module*/
pub type LCCCR = crate::Reg<lcccr::LCCCRrs>;
///DSI Host LTDC Current Color Coding Register
pub mod lcccr;
/**LPMCCR (r) register accessor: DSI Host Low-Power mode Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lpmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:LPMCCR)

For information about available fields see [`mod@lpmccr`] module*/
pub type LPMCCR = crate::Reg<lpmccr::LPMCCRrs>;
///DSI Host Low-Power mode Current Configuration Register
pub mod lpmccr;
/**VMCCR (r) register accessor: DSI Host Video mode Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VMCCR)

For information about available fields see [`mod@vmccr`] module*/
pub type VMCCR = crate::Reg<vmccr::VMCCRrs>;
///DSI Host Video mode Current Configuration Register
pub mod vmccr;
/**VPCCR (r) register accessor: DSI Host Video Packet Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VPCCR)

For information about available fields see [`mod@vpccr`] module*/
pub type VPCCR = crate::Reg<vpccr::VPCCRrs>;
///DSI Host Video Packet Current Configuration Register
pub mod vpccr;
/**VCCCR (r) register accessor: DSI Host Video Chunks Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vcccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VCCCR)

For information about available fields see [`mod@vcccr`] module*/
pub type VCCCR = crate::Reg<vcccr::VCCCRrs>;
///DSI Host Video Chunks Current Configuration Register
pub mod vcccr;
/**VNPCCR (r) register accessor: DSI Host Video Null Packet Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vnpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VNPCCR)

For information about available fields see [`mod@vnpccr`] module*/
pub type VNPCCR = crate::Reg<vnpccr::VNPCCRrs>;
///DSI Host Video Null Packet Current Configuration Register
pub mod vnpccr;
/**VHSACCR (r) register accessor: DSI Host Video HSA Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vhsaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VHSACCR)

For information about available fields see [`mod@vhsaccr`] module*/
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCRrs>;
///DSI Host Video HSA Current Configuration Register
pub mod vhsaccr;
/**VHBPCCR (r) register accessor: DSI Host Video HBP Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vhbpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VHBPCCR)

For information about available fields see [`mod@vhbpccr`] module*/
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCRrs>;
///DSI Host Video HBP Current Configuration Register
pub mod vhbpccr;
/**VLCCR (r) register accessor: DSI Host Video Line Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vlccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VLCCR)

For information about available fields see [`mod@vlccr`] module*/
pub type VLCCR = crate::Reg<vlccr::VLCCRrs>;
///DSI Host Video Line Current Configuration Register
pub mod vlccr;
/**VVSACCR (r) register accessor: DSI Host Video VSA Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvsaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVSACCR)

For information about available fields see [`mod@vvsaccr`] module*/
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCRrs>;
///DSI Host Video VSA Current Configuration Register
pub mod vvsaccr;
/**VVBPCCR (r) register accessor: DSI Host Video VBP Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvbpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVBPCCR)

For information about available fields see [`mod@vvbpccr`] module*/
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCRrs>;
///DSI Host Video VBP Current Configuration Register
pub mod vvbpccr;
/**VVFPCCR (r) register accessor: DSI Host Video VFP Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvfpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVFPCCR)

For information about available fields see [`mod@vvfpccr`] module*/
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCRrs>;
///DSI Host Video VFP Current Configuration Register
pub mod vvfpccr;
/**VVACCR (r) register accessor: DSI Host Video VA Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vvaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:VVACCR)

For information about available fields see [`mod@vvaccr`] module*/
pub type VVACCR = crate::Reg<vvaccr::VVACCRrs>;
///DSI Host Video VA Current Configuration Register
pub mod vvaccr;
/**WCFGR (rw) register accessor: DSI Wrapper Configuration Register

You can [`read`](crate::Reg::read) this register and get [`wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WCFGR)

For information about available fields see [`mod@wcfgr`] module*/
pub type WCFGR = crate::Reg<wcfgr::WCFGRrs>;
///DSI Wrapper Configuration Register
pub mod wcfgr;
/**WCR (rw) register accessor: DSI Wrapper Control Register

You can [`read`](crate::Reg::read) this register and get [`wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WCR)

For information about available fields see [`mod@wcr`] module*/
pub type WCR = crate::Reg<wcr::WCRrs>;
///DSI Wrapper Control Register
pub mod wcr;
/**WIER (rw) register accessor: DSI Wrapper Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`wier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WIER)

For information about available fields see [`mod@wier`] module*/
pub type WIER = crate::Reg<wier::WIERrs>;
///DSI Wrapper Interrupt Enable Register
pub mod wier;
/**WISR (r) register accessor: DSI Wrapper Interrupt & Status Register

You can [`read`](crate::Reg::read) this register and get [`wisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WISR)

For information about available fields see [`mod@wisr`] module*/
pub type WISR = crate::Reg<wisr::WISRrs>;
///DSI Wrapper Interrupt & Status Register
pub mod wisr;
/**WIFCR (rw) register accessor: DSI Wrapper Interrupt Flag Clear Register

You can [`read`](crate::Reg::read) this register and get [`wifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WIFCR)

For information about available fields see [`mod@wifcr`] module*/
pub type WIFCR = crate::Reg<wifcr::WIFCRrs>;
///DSI Wrapper Interrupt Flag Clear Register
pub mod wifcr;
/**WPCR0 (rw) register accessor: DSI Wrapper PHY Configuration Register 1

You can [`read`](crate::Reg::read) this register and get [`wpcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WPCR0)

For information about available fields see [`mod@wpcr0`] module*/
pub type WPCR0 = crate::Reg<wpcr0::WPCR0rs>;
///DSI Wrapper PHY Configuration Register 1
pub mod wpcr0;
/**WPCR1 (rw) register accessor: DSI Wrapper PHY Configuration Register 2

You can [`read`](crate::Reg::read) this register and get [`wpcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WPCR1)

For information about available fields see [`mod@wpcr1`] module*/
pub type WPCR1 = crate::Reg<wpcr1::WPCR1rs>;
///DSI Wrapper PHY Configuration Register 2
pub mod wpcr1;
/**WPCR2 (rw) register accessor: DSI Wrapper PHY Configuration Register 3

You can [`read`](crate::Reg::read) this register and get [`wpcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WPCR2)

For information about available fields see [`mod@wpcr2`] module*/
pub type WPCR2 = crate::Reg<wpcr2::WPCR2rs>;
///DSI Wrapper PHY Configuration Register 3
pub mod wpcr2;
/**WPCR3 (rw) register accessor: DSI_WPCR4

You can [`read`](crate::Reg::read) this register and get [`wpcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WPCR3)

For information about available fields see [`mod@wpcr3`] module*/
pub type WPCR3 = crate::Reg<wpcr3::WPCR3rs>;
///DSI_WPCR4
pub mod wpcr3;
/**WPCR4 (rw) register accessor: DSI Wrapper PHY Configuration Register 5

You can [`read`](crate::Reg::read) this register and get [`wpcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WPCR4)

For information about available fields see [`mod@wpcr4`] module*/
pub type WPCR4 = crate::Reg<wpcr4::WPCR4rs>;
///DSI Wrapper PHY Configuration Register 5
pub mod wpcr4;
/**WRPCR (rw) register accessor: DSI Wrapper Regulator and PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`wrpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#DSI:WRPCR)

For information about available fields see [`mod@wrpcr`] module*/
pub type WRPCR = crate::Reg<wrpcr::WRPCRrs>;
///DSI Wrapper Regulator and PLL Control Register
pub mod wrpcr;
