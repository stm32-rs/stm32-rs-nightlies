#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mcr: MCR,
    msr: MSR,
    tsr: TSR,
    rf0r: RF0R,
    rf1r: RF1R,
    ier: IER,
    esr: ESR,
    btr: BTR,
    _reserved8: [u8; 0x0160],
    ti0r: TI0R,
    tdt0r: TDT0R,
    tdl0r: TDL0R,
    tdh0r: TDH0R,
    ti1r: TI1R,
    tdt1r: TDT1R,
    tdl1r: TDL1R,
    tdh1r: TDH1R,
    ti2r: TI2R,
    tdt2r: TDT2R,
    tdl2r: TDL2R,
    tdh2r: TDH2R,
    ri0r: RI0R,
    rdt0r: RDT0R,
    rdl0r: RDL0R,
    rdh0r: RDH0R,
    ri1r: RI1R,
    rdt1r: RDT1R,
    rdl1r: RDL1R,
    rdh1r: RDH1R,
    _reserved28: [u8; 0x30],
    fmr: FMR,
    fm1r: FM1R,
    _reserved30: [u8; 0x04],
    fs1r: FS1R,
    _reserved31: [u8; 0x04],
    ffa1r: FFA1R,
    _reserved32: [u8; 0x04],
    fa1r: FA1R,
    _reserved33: [u8; 0x20],
    f0r1: F0R1,
    f0r2: F0R2,
    f1r1: F1R1,
    f1r2: F1R2,
    f2r1: F2R1,
    f2r2: F2R2,
    f3r1: F3R1,
    f3r2: F3R2,
    f4r1: F4R1,
    f4r2: F4R2,
    f5r1: F5R1,
    f5r2: F5R2,
    f6r1: F6R1,
    f6r2: F6R2,
    f7r1: F7R1,
    f7r2: F7R2,
    f8r1: F8R1,
    f8r2: F8R2,
    f9r1: F9R1,
    f9r2: F9R2,
    f10r1: F10R1,
    f10r2: F10R2,
    f11r1: F11R1,
    f11r2: F11R2,
    f12r1: F12R1,
    f12r2: F12R2,
    f13r1: F13R1,
    f13r2: F13R2,
    f14r1: F14R1,
    f14r2: F14R2,
    f15r1: F15R1,
    f15r2: F15R2,
    f16r1: F16R1,
    f16r2: F16R2,
    f17r1: F17R1,
    f17r2: F17R2,
    f18r1: F18R1,
    f18r2: F18R2,
    f19r1: F19R1,
    f19r2: F19R2,
    f20r1: F20R1,
    f20r2: F20R2,
    f21r1: F21R1,
    f21r2: F21R2,
    f22r1: F22R1,
    f22r2: F22R2,
    f23r1: F23R1,
    f23r2: F23R2,
    f24r1: F24R1,
    f24r2: F24R2,
    f25r1: F25R1,
    f25r2: F25R2,
    f26r1: F26R1,
    f26r2: F26R2,
    f27r1: F27R1,
    f27r2: F27R2,
}
impl RegisterBlock {
    ///0x00 - master control register
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x04 - master status register
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
    ///0x08 - transmit status register
    #[inline(always)]
    pub const fn tsr(&self) -> &TSR {
        &self.tsr
    }
    ///0x0c - receive FIFO 0 register
    #[inline(always)]
    pub const fn rf0r(&self) -> &RF0R {
        &self.rf0r
    }
    ///0x10 - receive FIFO 1 register
    #[inline(always)]
    pub const fn rf1r(&self) -> &RF1R {
        &self.rf1r
    }
    ///0x14 - interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x18 - interrupt enable register
    #[inline(always)]
    pub const fn esr(&self) -> &ESR {
        &self.esr
    }
    ///0x1c - bit timing register
    #[inline(always)]
    pub const fn btr(&self) -> &BTR {
        &self.btr
    }
    ///0x180 - TX mailbox identifier register
    #[inline(always)]
    pub const fn ti0r(&self) -> &TI0R {
        &self.ti0r
    }
    ///0x184 - mailbox data length control and time stamp register
    #[inline(always)]
    pub const fn tdt0r(&self) -> &TDT0R {
        &self.tdt0r
    }
    ///0x188 - mailbox data low register
    #[inline(always)]
    pub const fn tdl0r(&self) -> &TDL0R {
        &self.tdl0r
    }
    ///0x18c - mailbox data high register
    #[inline(always)]
    pub const fn tdh0r(&self) -> &TDH0R {
        &self.tdh0r
    }
    ///0x190 - mailbox identifier register
    #[inline(always)]
    pub const fn ti1r(&self) -> &TI1R {
        &self.ti1r
    }
    ///0x194 - mailbox data length control and time stamp register
    #[inline(always)]
    pub const fn tdt1r(&self) -> &TDT1R {
        &self.tdt1r
    }
    ///0x198 - mailbox data low register
    #[inline(always)]
    pub const fn tdl1r(&self) -> &TDL1R {
        &self.tdl1r
    }
    ///0x19c - mailbox data high register
    #[inline(always)]
    pub const fn tdh1r(&self) -> &TDH1R {
        &self.tdh1r
    }
    ///0x1a0 - mailbox identifier register
    #[inline(always)]
    pub const fn ti2r(&self) -> &TI2R {
        &self.ti2r
    }
    ///0x1a4 - mailbox data length control and time stamp register
    #[inline(always)]
    pub const fn tdt2r(&self) -> &TDT2R {
        &self.tdt2r
    }
    ///0x1a8 - mailbox data low register
    #[inline(always)]
    pub const fn tdl2r(&self) -> &TDL2R {
        &self.tdl2r
    }
    ///0x1ac - mailbox data high register
    #[inline(always)]
    pub const fn tdh2r(&self) -> &TDH2R {
        &self.tdh2r
    }
    ///0x1b0 - receive FIFO mailbox identifier register
    #[inline(always)]
    pub const fn ri0r(&self) -> &RI0R {
        &self.ri0r
    }
    ///0x1b4 - mailbox data high register
    #[inline(always)]
    pub const fn rdt0r(&self) -> &RDT0R {
        &self.rdt0r
    }
    ///0x1b8 - mailbox data high register
    #[inline(always)]
    pub const fn rdl0r(&self) -> &RDL0R {
        &self.rdl0r
    }
    ///0x1bc - receive FIFO mailbox data high register
    #[inline(always)]
    pub const fn rdh0r(&self) -> &RDH0R {
        &self.rdh0r
    }
    ///0x1c0 - mailbox data high register
    #[inline(always)]
    pub const fn ri1r(&self) -> &RI1R {
        &self.ri1r
    }
    ///0x1c4 - mailbox data high register
    #[inline(always)]
    pub const fn rdt1r(&self) -> &RDT1R {
        &self.rdt1r
    }
    ///0x1c8 - mailbox data high register
    #[inline(always)]
    pub const fn rdl1r(&self) -> &RDL1R {
        &self.rdl1r
    }
    ///0x1cc - mailbox data high register
    #[inline(always)]
    pub const fn rdh1r(&self) -> &RDH1R {
        &self.rdh1r
    }
    ///0x200 - filter master register
    #[inline(always)]
    pub const fn fmr(&self) -> &FMR {
        &self.fmr
    }
    ///0x204 - filter mode register
    #[inline(always)]
    pub const fn fm1r(&self) -> &FM1R {
        &self.fm1r
    }
    ///0x20c - filter scale register
    #[inline(always)]
    pub const fn fs1r(&self) -> &FS1R {
        &self.fs1r
    }
    ///0x214 - filter FIFO assignment register
    #[inline(always)]
    pub const fn ffa1r(&self) -> &FFA1R {
        &self.ffa1r
    }
    ///0x21c - filter activation register
    #[inline(always)]
    pub const fn fa1r(&self) -> &FA1R {
        &self.fa1r
    }
    ///0x240 - Filter bank 0 register 1
    #[inline(always)]
    pub const fn f0r1(&self) -> &F0R1 {
        &self.f0r1
    }
    ///0x244 - Filter bank 0 register 2
    #[inline(always)]
    pub const fn f0r2(&self) -> &F0R2 {
        &self.f0r2
    }
    ///0x248 - Filter bank 1 register 1
    #[inline(always)]
    pub const fn f1r1(&self) -> &F1R1 {
        &self.f1r1
    }
    ///0x24c - Filter bank 1 register 2
    #[inline(always)]
    pub const fn f1r2(&self) -> &F1R2 {
        &self.f1r2
    }
    ///0x250 - Filter bank 2 register 1
    #[inline(always)]
    pub const fn f2r1(&self) -> &F2R1 {
        &self.f2r1
    }
    ///0x254 - Filter bank 2 register 2
    #[inline(always)]
    pub const fn f2r2(&self) -> &F2R2 {
        &self.f2r2
    }
    ///0x258 - Filter bank 3 register 1
    #[inline(always)]
    pub const fn f3r1(&self) -> &F3R1 {
        &self.f3r1
    }
    ///0x25c - Filter bank 3 register 2
    #[inline(always)]
    pub const fn f3r2(&self) -> &F3R2 {
        &self.f3r2
    }
    ///0x260 - Filter bank 4 register 1
    #[inline(always)]
    pub const fn f4r1(&self) -> &F4R1 {
        &self.f4r1
    }
    ///0x264 - Filter bank 4 register 2
    #[inline(always)]
    pub const fn f4r2(&self) -> &F4R2 {
        &self.f4r2
    }
    ///0x268 - Filter bank 5 register 1
    #[inline(always)]
    pub const fn f5r1(&self) -> &F5R1 {
        &self.f5r1
    }
    ///0x26c - Filter bank 5 register 2
    #[inline(always)]
    pub const fn f5r2(&self) -> &F5R2 {
        &self.f5r2
    }
    ///0x270 - Filter bank 6 register 1
    #[inline(always)]
    pub const fn f6r1(&self) -> &F6R1 {
        &self.f6r1
    }
    ///0x274 - Filter bank 6 register 2
    #[inline(always)]
    pub const fn f6r2(&self) -> &F6R2 {
        &self.f6r2
    }
    ///0x278 - Filter bank 7 register 1
    #[inline(always)]
    pub const fn f7r1(&self) -> &F7R1 {
        &self.f7r1
    }
    ///0x27c - Filter bank 7 register 2
    #[inline(always)]
    pub const fn f7r2(&self) -> &F7R2 {
        &self.f7r2
    }
    ///0x280 - Filter bank 8 register 1
    #[inline(always)]
    pub const fn f8r1(&self) -> &F8R1 {
        &self.f8r1
    }
    ///0x284 - Filter bank 8 register 2
    #[inline(always)]
    pub const fn f8r2(&self) -> &F8R2 {
        &self.f8r2
    }
    ///0x288 - Filter bank 9 register 1
    #[inline(always)]
    pub const fn f9r1(&self) -> &F9R1 {
        &self.f9r1
    }
    ///0x28c - Filter bank 9 register 2
    #[inline(always)]
    pub const fn f9r2(&self) -> &F9R2 {
        &self.f9r2
    }
    ///0x290 - Filter bank 10 register 1
    #[inline(always)]
    pub const fn f10r1(&self) -> &F10R1 {
        &self.f10r1
    }
    ///0x294 - Filter bank 10 register 2
    #[inline(always)]
    pub const fn f10r2(&self) -> &F10R2 {
        &self.f10r2
    }
    ///0x298 - Filter bank 11 register 1
    #[inline(always)]
    pub const fn f11r1(&self) -> &F11R1 {
        &self.f11r1
    }
    ///0x29c - Filter bank 11 register 2
    #[inline(always)]
    pub const fn f11r2(&self) -> &F11R2 {
        &self.f11r2
    }
    ///0x2a0 - Filter bank 4 register 1
    #[inline(always)]
    pub const fn f12r1(&self) -> &F12R1 {
        &self.f12r1
    }
    ///0x2a4 - Filter bank 12 register 2
    #[inline(always)]
    pub const fn f12r2(&self) -> &F12R2 {
        &self.f12r2
    }
    ///0x2a8 - Filter bank 13 register 1
    #[inline(always)]
    pub const fn f13r1(&self) -> &F13R1 {
        &self.f13r1
    }
    ///0x2ac - Filter bank 13 register 2
    #[inline(always)]
    pub const fn f13r2(&self) -> &F13R2 {
        &self.f13r2
    }
    ///0x2b0 - Filter bank 14 register 1
    #[inline(always)]
    pub const fn f14r1(&self) -> &F14R1 {
        &self.f14r1
    }
    ///0x2b4 - Filter bank 14 register 2
    #[inline(always)]
    pub const fn f14r2(&self) -> &F14R2 {
        &self.f14r2
    }
    ///0x2b8 - Filter bank 15 register 1
    #[inline(always)]
    pub const fn f15r1(&self) -> &F15R1 {
        &self.f15r1
    }
    ///0x2bc - Filter bank 15 register 2
    #[inline(always)]
    pub const fn f15r2(&self) -> &F15R2 {
        &self.f15r2
    }
    ///0x2c0 - Filter bank 16 register 1
    #[inline(always)]
    pub const fn f16r1(&self) -> &F16R1 {
        &self.f16r1
    }
    ///0x2c4 - Filter bank 16 register 2
    #[inline(always)]
    pub const fn f16r2(&self) -> &F16R2 {
        &self.f16r2
    }
    ///0x2c8 - Filter bank 17 register 1
    #[inline(always)]
    pub const fn f17r1(&self) -> &F17R1 {
        &self.f17r1
    }
    ///0x2cc - Filter bank 17 register 2
    #[inline(always)]
    pub const fn f17r2(&self) -> &F17R2 {
        &self.f17r2
    }
    ///0x2d0 - Filter bank 18 register 1
    #[inline(always)]
    pub const fn f18r1(&self) -> &F18R1 {
        &self.f18r1
    }
    ///0x2d4 - Filter bank 18 register 2
    #[inline(always)]
    pub const fn f18r2(&self) -> &F18R2 {
        &self.f18r2
    }
    ///0x2d8 - Filter bank 19 register 1
    #[inline(always)]
    pub const fn f19r1(&self) -> &F19R1 {
        &self.f19r1
    }
    ///0x2dc - Filter bank 19 register 2
    #[inline(always)]
    pub const fn f19r2(&self) -> &F19R2 {
        &self.f19r2
    }
    ///0x2e0 - Filter bank 20 register 1
    #[inline(always)]
    pub const fn f20r1(&self) -> &F20R1 {
        &self.f20r1
    }
    ///0x2e4 - Filter bank 20 register 2
    #[inline(always)]
    pub const fn f20r2(&self) -> &F20R2 {
        &self.f20r2
    }
    ///0x2e8 - Filter bank 21 register 1
    #[inline(always)]
    pub const fn f21r1(&self) -> &F21R1 {
        &self.f21r1
    }
    ///0x2ec - Filter bank 21 register 2
    #[inline(always)]
    pub const fn f21r2(&self) -> &F21R2 {
        &self.f21r2
    }
    ///0x2f0 - Filter bank 22 register 1
    #[inline(always)]
    pub const fn f22r1(&self) -> &F22R1 {
        &self.f22r1
    }
    ///0x2f4 - Filter bank 22 register 2
    #[inline(always)]
    pub const fn f22r2(&self) -> &F22R2 {
        &self.f22r2
    }
    ///0x2f8 - Filter bank 23 register 1
    #[inline(always)]
    pub const fn f23r1(&self) -> &F23R1 {
        &self.f23r1
    }
    ///0x2fc - Filter bank 23 register 2
    #[inline(always)]
    pub const fn f23r2(&self) -> &F23R2 {
        &self.f23r2
    }
    ///0x300 - Filter bank 24 register 1
    #[inline(always)]
    pub const fn f24r1(&self) -> &F24R1 {
        &self.f24r1
    }
    ///0x304 - Filter bank 24 register 2
    #[inline(always)]
    pub const fn f24r2(&self) -> &F24R2 {
        &self.f24r2
    }
    ///0x308 - Filter bank 25 register 1
    #[inline(always)]
    pub const fn f25r1(&self) -> &F25R1 {
        &self.f25r1
    }
    ///0x30c - Filter bank 25 register 2
    #[inline(always)]
    pub const fn f25r2(&self) -> &F25R2 {
        &self.f25r2
    }
    ///0x310 - Filter bank 26 register 1
    #[inline(always)]
    pub const fn f26r1(&self) -> &F26R1 {
        &self.f26r1
    }
    ///0x314 - Filter bank 26 register 2
    #[inline(always)]
    pub const fn f26r2(&self) -> &F26R2 {
        &self.f26r2
    }
    ///0x318 - Filter bank 27 register 1
    #[inline(always)]
    pub const fn f27r1(&self) -> &F27R1 {
        &self.f27r1
    }
    ///0x31c - Filter bank 27 register 2
    #[inline(always)]
    pub const fn f27r2(&self) -> &F27R2 {
        &self.f27r2
    }
}
/**MCR (rw) register accessor: master control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:MCR)

For information about available fields see [`mod@mcr`] module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///master control register
pub mod mcr;
/**MSR (rw) register accessor: master status register

You can [`read`](crate::Reg::read) this register and get [`msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:MSR)

For information about available fields see [`mod@msr`] module*/
pub type MSR = crate::Reg<msr::MSRrs>;
///master status register
pub mod msr;
/**TSR (rw) register accessor: transmit status register

You can [`read`](crate::Reg::read) this register and get [`tsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TSR)

For information about available fields see [`mod@tsr`] module*/
pub type TSR = crate::Reg<tsr::TSRrs>;
///transmit status register
pub mod tsr;
/**RF0R (rw) register accessor: receive FIFO 0 register

You can [`read`](crate::Reg::read) this register and get [`rf0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RF0R)

For information about available fields see [`mod@rf0r`] module*/
pub type RF0R = crate::Reg<rf0r::RF0Rrs>;
///receive FIFO 0 register
pub mod rf0r;
/**RF1R (rw) register accessor: receive FIFO 1 register

You can [`read`](crate::Reg::read) this register and get [`rf1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RF1R)

For information about available fields see [`mod@rf1r`] module*/
pub type RF1R = crate::Reg<rf1r::RF1Rrs>;
///receive FIFO 1 register
pub mod rf1r;
/**IER (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///interrupt enable register
pub mod ier;
/**ESR (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`esr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:ESR)

For information about available fields see [`mod@esr`] module*/
pub type ESR = crate::Reg<esr::ESRrs>;
///interrupt enable register
pub mod esr;
/**BTR (rw) register accessor: bit timing register

You can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:BTR)

For information about available fields see [`mod@btr`] module*/
pub type BTR = crate::Reg<btr::BTRrs>;
///bit timing register
pub mod btr;
/**TI0R (rw) register accessor: TX mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`ti0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TI0R)

For information about available fields see [`mod@ti0r`] module*/
pub type TI0R = crate::Reg<ti0r::TI0Rrs>;
///TX mailbox identifier register
pub mod ti0r;
/**TDT0R (rw) register accessor: mailbox data length control and time stamp register

You can [`read`](crate::Reg::read) this register and get [`tdt0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdt0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDT0R)

For information about available fields see [`mod@tdt0r`] module*/
pub type TDT0R = crate::Reg<tdt0r::TDT0Rrs>;
///mailbox data length control and time stamp register
pub mod tdt0r;
/**TDL0R (rw) register accessor: mailbox data low register

You can [`read`](crate::Reg::read) this register and get [`tdl0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdl0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDL0R)

For information about available fields see [`mod@tdl0r`] module*/
pub type TDL0R = crate::Reg<tdl0r::TDL0Rrs>;
///mailbox data low register
pub mod tdl0r;
/**TDH0R (rw) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`tdh0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdh0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDH0R)

For information about available fields see [`mod@tdh0r`] module*/
pub type TDH0R = crate::Reg<tdh0r::TDH0Rrs>;
///mailbox data high register
pub mod tdh0r;
/**TI1R (rw) register accessor: mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`ti1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TI1R)

For information about available fields see [`mod@ti1r`] module*/
pub type TI1R = crate::Reg<ti1r::TI1Rrs>;
///mailbox identifier register
pub mod ti1r;
/**TDT1R (rw) register accessor: mailbox data length control and time stamp register

You can [`read`](crate::Reg::read) this register and get [`tdt1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdt1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDT1R)

For information about available fields see [`mod@tdt1r`] module*/
pub type TDT1R = crate::Reg<tdt1r::TDT1Rrs>;
///mailbox data length control and time stamp register
pub mod tdt1r;
/**TDL1R (rw) register accessor: mailbox data low register

You can [`read`](crate::Reg::read) this register and get [`tdl1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdl1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDL1R)

For information about available fields see [`mod@tdl1r`] module*/
pub type TDL1R = crate::Reg<tdl1r::TDL1Rrs>;
///mailbox data low register
pub mod tdl1r;
/**TDH1R (rw) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`tdh1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdh1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDH1R)

For information about available fields see [`mod@tdh1r`] module*/
pub type TDH1R = crate::Reg<tdh1r::TDH1Rrs>;
///mailbox data high register
pub mod tdh1r;
/**TI2R (rw) register accessor: mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`ti2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TI2R)

For information about available fields see [`mod@ti2r`] module*/
pub type TI2R = crate::Reg<ti2r::TI2Rrs>;
///mailbox identifier register
pub mod ti2r;
/**TDT2R (rw) register accessor: mailbox data length control and time stamp register

You can [`read`](crate::Reg::read) this register and get [`tdt2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdt2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDT2R)

For information about available fields see [`mod@tdt2r`] module*/
pub type TDT2R = crate::Reg<tdt2r::TDT2Rrs>;
///mailbox data length control and time stamp register
pub mod tdt2r;
/**TDL2R (rw) register accessor: mailbox data low register

You can [`read`](crate::Reg::read) this register and get [`tdl2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdl2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDL2R)

For information about available fields see [`mod@tdl2r`] module*/
pub type TDL2R = crate::Reg<tdl2r::TDL2Rrs>;
///mailbox data low register
pub mod tdl2r;
/**TDH2R (rw) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`tdh2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdh2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:TDH2R)

For information about available fields see [`mod@tdh2r`] module*/
pub type TDH2R = crate::Reg<tdh2r::TDH2Rrs>;
///mailbox data high register
pub mod tdh2r;
/**RI0R (r) register accessor: receive FIFO mailbox identifier register

You can [`read`](crate::Reg::read) this register and get [`ri0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RI0R)

For information about available fields see [`mod@ri0r`] module*/
pub type RI0R = crate::Reg<ri0r::RI0Rrs>;
///receive FIFO mailbox identifier register
pub mod ri0r;
/**RDT0R (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdt0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDT0R)

For information about available fields see [`mod@rdt0r`] module*/
pub type RDT0R = crate::Reg<rdt0r::RDT0Rrs>;
///mailbox data high register
pub mod rdt0r;
/**RDL0R (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdl0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDL0R)

For information about available fields see [`mod@rdl0r`] module*/
pub type RDL0R = crate::Reg<rdl0r::RDL0Rrs>;
///mailbox data high register
pub mod rdl0r;
/**RDH0R (r) register accessor: receive FIFO mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdh0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDH0R)

For information about available fields see [`mod@rdh0r`] module*/
pub type RDH0R = crate::Reg<rdh0r::RDH0Rrs>;
///receive FIFO mailbox data high register
pub mod rdh0r;
/**RI1R (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`ri1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RI1R)

For information about available fields see [`mod@ri1r`] module*/
pub type RI1R = crate::Reg<ri1r::RI1Rrs>;
///mailbox data high register
pub mod ri1r;
/**RDT1R (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdt1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDT1R)

For information about available fields see [`mod@rdt1r`] module*/
pub type RDT1R = crate::Reg<rdt1r::RDT1Rrs>;
///mailbox data high register
pub mod rdt1r;
/**RDL1R (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdl1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDL1R)

For information about available fields see [`mod@rdl1r`] module*/
pub type RDL1R = crate::Reg<rdl1r::RDL1Rrs>;
///mailbox data high register
pub mod rdl1r;
/**RDH1R (r) register accessor: mailbox data high register

You can [`read`](crate::Reg::read) this register and get [`rdh1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:RDH1R)

For information about available fields see [`mod@rdh1r`] module*/
pub type RDH1R = crate::Reg<rdh1r::RDH1Rrs>;
///mailbox data high register
pub mod rdh1r;
/**FMR (rw) register accessor: filter master register

You can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:FMR)

For information about available fields see [`mod@fmr`] module*/
pub type FMR = crate::Reg<fmr::FMRrs>;
///filter master register
pub mod fmr;
/**FM1R (rw) register accessor: filter mode register

You can [`read`](crate::Reg::read) this register and get [`fm1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:FM1R)

For information about available fields see [`mod@fm1r`] module*/
pub type FM1R = crate::Reg<fm1r::FM1Rrs>;
///filter mode register
pub mod fm1r;
/**FS1R (rw) register accessor: filter scale register

You can [`read`](crate::Reg::read) this register and get [`fs1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:FS1R)

For information about available fields see [`mod@fs1r`] module*/
pub type FS1R = crate::Reg<fs1r::FS1Rrs>;
///filter scale register
pub mod fs1r;
/**FFA1R (rw) register accessor: filter FIFO assignment register

You can [`read`](crate::Reg::read) this register and get [`ffa1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ffa1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:FFA1R)

For information about available fields see [`mod@ffa1r`] module*/
pub type FFA1R = crate::Reg<ffa1r::FFA1Rrs>;
///filter FIFO assignment register
pub mod ffa1r;
/**FA1R (rw) register accessor: filter activation register

You can [`read`](crate::Reg::read) this register and get [`fa1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fa1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:FA1R)

For information about available fields see [`mod@fa1r`] module*/
pub type FA1R = crate::Reg<fa1r::FA1Rrs>;
///filter activation register
pub mod fa1r;
/**F0R1 (rw) register accessor: Filter bank 0 register 1

You can [`read`](crate::Reg::read) this register and get [`f0r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f0r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F0R1)

For information about available fields see [`mod@f0r1`] module*/
pub type F0R1 = crate::Reg<f0r1::F0R1rs>;
///Filter bank 0 register 1
pub mod f0r1;
/**F0R2 (rw) register accessor: Filter bank 0 register 2

You can [`read`](crate::Reg::read) this register and get [`f0r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f0r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F0R2)

For information about available fields see [`mod@f0r2`] module*/
pub type F0R2 = crate::Reg<f0r2::F0R2rs>;
///Filter bank 0 register 2
pub mod f0r2;
/**F1R1 (rw) register accessor: Filter bank 1 register 1

You can [`read`](crate::Reg::read) this register and get [`f1r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F1R1)

For information about available fields see [`mod@f1r1`] module*/
pub type F1R1 = crate::Reg<f1r1::F1R1rs>;
///Filter bank 1 register 1
pub mod f1r1;
/**F1R2 (rw) register accessor: Filter bank 1 register 2

You can [`read`](crate::Reg::read) this register and get [`f1r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f1r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F1R2)

For information about available fields see [`mod@f1r2`] module*/
pub type F1R2 = crate::Reg<f1r2::F1R2rs>;
///Filter bank 1 register 2
pub mod f1r2;
/**F2R1 (rw) register accessor: Filter bank 2 register 1

You can [`read`](crate::Reg::read) this register and get [`f2r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F2R1)

For information about available fields see [`mod@f2r1`] module*/
pub type F2R1 = crate::Reg<f2r1::F2R1rs>;
///Filter bank 2 register 1
pub mod f2r1;
/**F2R2 (rw) register accessor: Filter bank 2 register 2

You can [`read`](crate::Reg::read) this register and get [`f2r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F2R2)

For information about available fields see [`mod@f2r2`] module*/
pub type F2R2 = crate::Reg<f2r2::F2R2rs>;
///Filter bank 2 register 2
pub mod f2r2;
/**F3R1 (rw) register accessor: Filter bank 3 register 1

You can [`read`](crate::Reg::read) this register and get [`f3r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f3r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F3R1)

For information about available fields see [`mod@f3r1`] module*/
pub type F3R1 = crate::Reg<f3r1::F3R1rs>;
///Filter bank 3 register 1
pub mod f3r1;
/**F3R2 (rw) register accessor: Filter bank 3 register 2

You can [`read`](crate::Reg::read) this register and get [`f3r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f3r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F3R2)

For information about available fields see [`mod@f3r2`] module*/
pub type F3R2 = crate::Reg<f3r2::F3R2rs>;
///Filter bank 3 register 2
pub mod f3r2;
/**F4R1 (rw) register accessor: Filter bank 4 register 1

You can [`read`](crate::Reg::read) this register and get [`f4r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f4r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F4R1)

For information about available fields see [`mod@f4r1`] module*/
pub type F4R1 = crate::Reg<f4r1::F4R1rs>;
///Filter bank 4 register 1
pub mod f4r1;
/**F4R2 (rw) register accessor: Filter bank 4 register 2

You can [`read`](crate::Reg::read) this register and get [`f4r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f4r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F4R2)

For information about available fields see [`mod@f4r2`] module*/
pub type F4R2 = crate::Reg<f4r2::F4R2rs>;
///Filter bank 4 register 2
pub mod f4r2;
/**F5R1 (rw) register accessor: Filter bank 5 register 1

You can [`read`](crate::Reg::read) this register and get [`f5r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f5r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F5R1)

For information about available fields see [`mod@f5r1`] module*/
pub type F5R1 = crate::Reg<f5r1::F5R1rs>;
///Filter bank 5 register 1
pub mod f5r1;
/**F5R2 (rw) register accessor: Filter bank 5 register 2

You can [`read`](crate::Reg::read) this register and get [`f5r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f5r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F5R2)

For information about available fields see [`mod@f5r2`] module*/
pub type F5R2 = crate::Reg<f5r2::F5R2rs>;
///Filter bank 5 register 2
pub mod f5r2;
/**F6R1 (rw) register accessor: Filter bank 6 register 1

You can [`read`](crate::Reg::read) this register and get [`f6r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f6r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F6R1)

For information about available fields see [`mod@f6r1`] module*/
pub type F6R1 = crate::Reg<f6r1::F6R1rs>;
///Filter bank 6 register 1
pub mod f6r1;
/**F6R2 (rw) register accessor: Filter bank 6 register 2

You can [`read`](crate::Reg::read) this register and get [`f6r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f6r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F6R2)

For information about available fields see [`mod@f6r2`] module*/
pub type F6R2 = crate::Reg<f6r2::F6R2rs>;
///Filter bank 6 register 2
pub mod f6r2;
/**F7R1 (rw) register accessor: Filter bank 7 register 1

You can [`read`](crate::Reg::read) this register and get [`f7r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f7r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F7R1)

For information about available fields see [`mod@f7r1`] module*/
pub type F7R1 = crate::Reg<f7r1::F7R1rs>;
///Filter bank 7 register 1
pub mod f7r1;
/**F7R2 (rw) register accessor: Filter bank 7 register 2

You can [`read`](crate::Reg::read) this register and get [`f7r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f7r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F7R2)

For information about available fields see [`mod@f7r2`] module*/
pub type F7R2 = crate::Reg<f7r2::F7R2rs>;
///Filter bank 7 register 2
pub mod f7r2;
/**F8R1 (rw) register accessor: Filter bank 8 register 1

You can [`read`](crate::Reg::read) this register and get [`f8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F8R1)

For information about available fields see [`mod@f8r1`] module*/
pub type F8R1 = crate::Reg<f8r1::F8R1rs>;
///Filter bank 8 register 1
pub mod f8r1;
/**F8R2 (rw) register accessor: Filter bank 8 register 2

You can [`read`](crate::Reg::read) this register and get [`f8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F8R2)

For information about available fields see [`mod@f8r2`] module*/
pub type F8R2 = crate::Reg<f8r2::F8R2rs>;
///Filter bank 8 register 2
pub mod f8r2;
/**F9R1 (rw) register accessor: Filter bank 9 register 1

You can [`read`](crate::Reg::read) this register and get [`f9r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f9r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F9R1)

For information about available fields see [`mod@f9r1`] module*/
pub type F9R1 = crate::Reg<f9r1::F9R1rs>;
///Filter bank 9 register 1
pub mod f9r1;
/**F9R2 (rw) register accessor: Filter bank 9 register 2

You can [`read`](crate::Reg::read) this register and get [`f9r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f9r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F9R2)

For information about available fields see [`mod@f9r2`] module*/
pub type F9R2 = crate::Reg<f9r2::F9R2rs>;
///Filter bank 9 register 2
pub mod f9r2;
/**F10R1 (rw) register accessor: Filter bank 10 register 1

You can [`read`](crate::Reg::read) this register and get [`f10r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f10r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F10R1)

For information about available fields see [`mod@f10r1`] module*/
pub type F10R1 = crate::Reg<f10r1::F10R1rs>;
///Filter bank 10 register 1
pub mod f10r1;
/**F10R2 (rw) register accessor: Filter bank 10 register 2

You can [`read`](crate::Reg::read) this register and get [`f10r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f10r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F10R2)

For information about available fields see [`mod@f10r2`] module*/
pub type F10R2 = crate::Reg<f10r2::F10R2rs>;
///Filter bank 10 register 2
pub mod f10r2;
/**F11R1 (rw) register accessor: Filter bank 11 register 1

You can [`read`](crate::Reg::read) this register and get [`f11r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f11r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F11R1)

For information about available fields see [`mod@f11r1`] module*/
pub type F11R1 = crate::Reg<f11r1::F11R1rs>;
///Filter bank 11 register 1
pub mod f11r1;
/**F11R2 (rw) register accessor: Filter bank 11 register 2

You can [`read`](crate::Reg::read) this register and get [`f11r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f11r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F11R2)

For information about available fields see [`mod@f11r2`] module*/
pub type F11R2 = crate::Reg<f11r2::F11R2rs>;
///Filter bank 11 register 2
pub mod f11r2;
/**F12R1 (rw) register accessor: Filter bank 4 register 1

You can [`read`](crate::Reg::read) this register and get [`f12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F12R1)

For information about available fields see [`mod@f12r1`] module*/
pub type F12R1 = crate::Reg<f12r1::F12R1rs>;
///Filter bank 4 register 1
pub mod f12r1;
/**F12R2 (rw) register accessor: Filter bank 12 register 2

You can [`read`](crate::Reg::read) this register and get [`f12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F12R2)

For information about available fields see [`mod@f12r2`] module*/
pub type F12R2 = crate::Reg<f12r2::F12R2rs>;
///Filter bank 12 register 2
pub mod f12r2;
/**F13R1 (rw) register accessor: Filter bank 13 register 1

You can [`read`](crate::Reg::read) this register and get [`f13r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f13r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F13R1)

For information about available fields see [`mod@f13r1`] module*/
pub type F13R1 = crate::Reg<f13r1::F13R1rs>;
///Filter bank 13 register 1
pub mod f13r1;
/**F13R2 (rw) register accessor: Filter bank 13 register 2

You can [`read`](crate::Reg::read) this register and get [`f13r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f13r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F13R2)

For information about available fields see [`mod@f13r2`] module*/
pub type F13R2 = crate::Reg<f13r2::F13R2rs>;
///Filter bank 13 register 2
pub mod f13r2;
/**F14R1 (rw) register accessor: Filter bank 14 register 1

You can [`read`](crate::Reg::read) this register and get [`f14r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f14r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F14R1)

For information about available fields see [`mod@f14r1`] module*/
pub type F14R1 = crate::Reg<f14r1::F14R1rs>;
///Filter bank 14 register 1
pub mod f14r1;
/**F14R2 (rw) register accessor: Filter bank 14 register 2

You can [`read`](crate::Reg::read) this register and get [`f14r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f14r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F14R2)

For information about available fields see [`mod@f14r2`] module*/
pub type F14R2 = crate::Reg<f14r2::F14R2rs>;
///Filter bank 14 register 2
pub mod f14r2;
/**F15R1 (rw) register accessor: Filter bank 15 register 1

You can [`read`](crate::Reg::read) this register and get [`f15r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f15r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F15R1)

For information about available fields see [`mod@f15r1`] module*/
pub type F15R1 = crate::Reg<f15r1::F15R1rs>;
///Filter bank 15 register 1
pub mod f15r1;
/**F15R2 (rw) register accessor: Filter bank 15 register 2

You can [`read`](crate::Reg::read) this register and get [`f15r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f15r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F15R2)

For information about available fields see [`mod@f15r2`] module*/
pub type F15R2 = crate::Reg<f15r2::F15R2rs>;
///Filter bank 15 register 2
pub mod f15r2;
/**F16R1 (rw) register accessor: Filter bank 16 register 1

You can [`read`](crate::Reg::read) this register and get [`f16r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f16r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F16R1)

For information about available fields see [`mod@f16r1`] module*/
pub type F16R1 = crate::Reg<f16r1::F16R1rs>;
///Filter bank 16 register 1
pub mod f16r1;
/**F16R2 (rw) register accessor: Filter bank 16 register 2

You can [`read`](crate::Reg::read) this register and get [`f16r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f16r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F16R2)

For information about available fields see [`mod@f16r2`] module*/
pub type F16R2 = crate::Reg<f16r2::F16R2rs>;
///Filter bank 16 register 2
pub mod f16r2;
/**F17R1 (rw) register accessor: Filter bank 17 register 1

You can [`read`](crate::Reg::read) this register and get [`f17r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f17r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F17R1)

For information about available fields see [`mod@f17r1`] module*/
pub type F17R1 = crate::Reg<f17r1::F17R1rs>;
///Filter bank 17 register 1
pub mod f17r1;
/**F17R2 (rw) register accessor: Filter bank 17 register 2

You can [`read`](crate::Reg::read) this register and get [`f17r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f17r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F17R2)

For information about available fields see [`mod@f17r2`] module*/
pub type F17R2 = crate::Reg<f17r2::F17R2rs>;
///Filter bank 17 register 2
pub mod f17r2;
/**F18R1 (rw) register accessor: Filter bank 18 register 1

You can [`read`](crate::Reg::read) this register and get [`f18r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f18r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F18R1)

For information about available fields see [`mod@f18r1`] module*/
pub type F18R1 = crate::Reg<f18r1::F18R1rs>;
///Filter bank 18 register 1
pub mod f18r1;
/**F18R2 (rw) register accessor: Filter bank 18 register 2

You can [`read`](crate::Reg::read) this register and get [`f18r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f18r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F18R2)

For information about available fields see [`mod@f18r2`] module*/
pub type F18R2 = crate::Reg<f18r2::F18R2rs>;
///Filter bank 18 register 2
pub mod f18r2;
/**F19R1 (rw) register accessor: Filter bank 19 register 1

You can [`read`](crate::Reg::read) this register and get [`f19r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f19r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F19R1)

For information about available fields see [`mod@f19r1`] module*/
pub type F19R1 = crate::Reg<f19r1::F19R1rs>;
///Filter bank 19 register 1
pub mod f19r1;
/**F19R2 (rw) register accessor: Filter bank 19 register 2

You can [`read`](crate::Reg::read) this register and get [`f19r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f19r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F19R2)

For information about available fields see [`mod@f19r2`] module*/
pub type F19R2 = crate::Reg<f19r2::F19R2rs>;
///Filter bank 19 register 2
pub mod f19r2;
/**F20R1 (rw) register accessor: Filter bank 20 register 1

You can [`read`](crate::Reg::read) this register and get [`f20r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f20r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F20R1)

For information about available fields see [`mod@f20r1`] module*/
pub type F20R1 = crate::Reg<f20r1::F20R1rs>;
///Filter bank 20 register 1
pub mod f20r1;
/**F20R2 (rw) register accessor: Filter bank 20 register 2

You can [`read`](crate::Reg::read) this register and get [`f20r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f20r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F20R2)

For information about available fields see [`mod@f20r2`] module*/
pub type F20R2 = crate::Reg<f20r2::F20R2rs>;
///Filter bank 20 register 2
pub mod f20r2;
/**F21R1 (rw) register accessor: Filter bank 21 register 1

You can [`read`](crate::Reg::read) this register and get [`f21r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f21r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F21R1)

For information about available fields see [`mod@f21r1`] module*/
pub type F21R1 = crate::Reg<f21r1::F21R1rs>;
///Filter bank 21 register 1
pub mod f21r1;
/**F21R2 (rw) register accessor: Filter bank 21 register 2

You can [`read`](crate::Reg::read) this register and get [`f21r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f21r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F21R2)

For information about available fields see [`mod@f21r2`] module*/
pub type F21R2 = crate::Reg<f21r2::F21R2rs>;
///Filter bank 21 register 2
pub mod f21r2;
/**F22R1 (rw) register accessor: Filter bank 22 register 1

You can [`read`](crate::Reg::read) this register and get [`f22r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f22r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F22R1)

For information about available fields see [`mod@f22r1`] module*/
pub type F22R1 = crate::Reg<f22r1::F22R1rs>;
///Filter bank 22 register 1
pub mod f22r1;
/**F22R2 (rw) register accessor: Filter bank 22 register 2

You can [`read`](crate::Reg::read) this register and get [`f22r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f22r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F22R2)

For information about available fields see [`mod@f22r2`] module*/
pub type F22R2 = crate::Reg<f22r2::F22R2rs>;
///Filter bank 22 register 2
pub mod f22r2;
/**F23R1 (rw) register accessor: Filter bank 23 register 1

You can [`read`](crate::Reg::read) this register and get [`f23r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f23r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F23R1)

For information about available fields see [`mod@f23r1`] module*/
pub type F23R1 = crate::Reg<f23r1::F23R1rs>;
///Filter bank 23 register 1
pub mod f23r1;
/**F23R2 (rw) register accessor: Filter bank 23 register 2

You can [`read`](crate::Reg::read) this register and get [`f23r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f23r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F23R2)

For information about available fields see [`mod@f23r2`] module*/
pub type F23R2 = crate::Reg<f23r2::F23R2rs>;
///Filter bank 23 register 2
pub mod f23r2;
/**F24R1 (rw) register accessor: Filter bank 24 register 1

You can [`read`](crate::Reg::read) this register and get [`f24r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f24r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F24R1)

For information about available fields see [`mod@f24r1`] module*/
pub type F24R1 = crate::Reg<f24r1::F24R1rs>;
///Filter bank 24 register 1
pub mod f24r1;
/**F24R2 (rw) register accessor: Filter bank 24 register 2

You can [`read`](crate::Reg::read) this register and get [`f24r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f24r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F24R2)

For information about available fields see [`mod@f24r2`] module*/
pub type F24R2 = crate::Reg<f24r2::F24R2rs>;
///Filter bank 24 register 2
pub mod f24r2;
/**F25R1 (rw) register accessor: Filter bank 25 register 1

You can [`read`](crate::Reg::read) this register and get [`f25r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f25r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F25R1)

For information about available fields see [`mod@f25r1`] module*/
pub type F25R1 = crate::Reg<f25r1::F25R1rs>;
///Filter bank 25 register 1
pub mod f25r1;
/**F25R2 (rw) register accessor: Filter bank 25 register 2

You can [`read`](crate::Reg::read) this register and get [`f25r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f25r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F25R2)

For information about available fields see [`mod@f25r2`] module*/
pub type F25R2 = crate::Reg<f25r2::F25R2rs>;
///Filter bank 25 register 2
pub mod f25r2;
/**F26R1 (rw) register accessor: Filter bank 26 register 1

You can [`read`](crate::Reg::read) this register and get [`f26r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f26r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F26R1)

For information about available fields see [`mod@f26r1`] module*/
pub type F26R1 = crate::Reg<f26r1::F26R1rs>;
///Filter bank 26 register 1
pub mod f26r1;
/**F26R2 (rw) register accessor: Filter bank 26 register 2

You can [`read`](crate::Reg::read) this register and get [`f26r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f26r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F26R2)

For information about available fields see [`mod@f26r2`] module*/
pub type F26R2 = crate::Reg<f26r2::F26R2rs>;
///Filter bank 26 register 2
pub mod f26r2;
/**F27R1 (rw) register accessor: Filter bank 27 register 1

You can [`read`](crate::Reg::read) this register and get [`f27r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f27r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F27R1)

For information about available fields see [`mod@f27r1`] module*/
pub type F27R1 = crate::Reg<f27r1::F27R1rs>;
///Filter bank 27 register 1
pub mod f27r1;
/**F27R2 (rw) register accessor: Filter bank 27 register 2

You can [`read`](crate::Reg::read) this register and get [`f27r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f27r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#CAN1:F27R2)

For information about available fields see [`mod@f27r2`] module*/
pub type F27R2 = crate::Reg<f27r2::F27R2rs>;
///Filter bank 27 register 2
pub mod f27r2;
