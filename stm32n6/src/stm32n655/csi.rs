#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    pcr: PCR,
    _reserved2: [u8; 0x08],
    vc0cfgr1: VC0CFGR1,
    vc0cfgr2: VC0CFGR2,
    vc0cfgr3: VC0CFGR3,
    vc0cfgr4: VC0CFGR4,
    vc1cfgr1: VC1CFGR1,
    vc1cfgr2: VC1CFGR2,
    vc1cfgr3: VC1CFGR3,
    vc1cfgr4: VC1CFGR4,
    vc2cfgr1: VC2CFGR1,
    vc2cfgr2: VC2CFGR2,
    vc2cfgr3: VC2CFGR3,
    vc2cfgr4: VC2CFGR4,
    vc3cfgr1: VC3CFGR1,
    vc3cfgr2: VC3CFGR2,
    vc3cfgr3: VC3CFGR3,
    vc3cfgr4: VC3CFGR4,
    lb0cfgr: LB0CFGR,
    lb1cfgr: LB1CFGR,
    lb2cfgr: LB2CFGR,
    lb3cfgr: LB3CFGR,
    tim0cfgr: TIM0CFGR,
    tim1cfgr: TIM1CFGR,
    tim2cfgr: TIM2CFGR,
    tim3cfgr: TIM3CFGR,
    lmcfgr: LMCFGR,
    prgitr: PRGITR,
    wdr: WDR,
    _reserved29: [u8; 0x04],
    ier0: IER0,
    ier1: IER1,
    _reserved31: [u8; 0x08],
    sr0: SR0,
    sr1: SR1,
    _reserved33: [u8; 0x68],
    fcr0: FCR0,
    fcr1: FCR1,
    _reserved35: [u8; 0x08],
    spdfr: SPDFR,
    err1: ERR1,
    err2: ERR2,
    _reserved38: [u8; 0x0ee4],
    prcr: PRCR,
    pmcr: PMCR,
    pfcr: PFCR,
    _reserved41: [u8; 0x04],
    ptcr0: PTCR0,
    ptcr1: PTCR1,
    ptsr: PTSR,
}
impl RegisterBlock {
    ///0x00 - CSI-2 Host control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - CSI-2 Host DPHY_RX control register
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    ///0x10 - CSI-2 Host virtual channel 0 configuration register 1
    #[inline(always)]
    pub const fn vc0cfgr1(&self) -> &VC0CFGR1 {
        &self.vc0cfgr1
    }
    ///0x14 - CSI-2 Host virtual channel 0 configuration register 2
    #[inline(always)]
    pub const fn vc0cfgr2(&self) -> &VC0CFGR2 {
        &self.vc0cfgr2
    }
    ///0x18 - CSI-2 Host virtual channel 0 configuration register 3
    #[inline(always)]
    pub const fn vc0cfgr3(&self) -> &VC0CFGR3 {
        &self.vc0cfgr3
    }
    ///0x1c - CSI-2 Host virtual channel 0 configuration register 4
    #[inline(always)]
    pub const fn vc0cfgr4(&self) -> &VC0CFGR4 {
        &self.vc0cfgr4
    }
    ///0x20 - CSI-2 Host virtual channel 1 configuration register 1
    #[inline(always)]
    pub const fn vc1cfgr1(&self) -> &VC1CFGR1 {
        &self.vc1cfgr1
    }
    ///0x24 - CSI-2 Host virtual channel 1 configuration register 2
    #[inline(always)]
    pub const fn vc1cfgr2(&self) -> &VC1CFGR2 {
        &self.vc1cfgr2
    }
    ///0x28 - CSI-2 Host virtual channel 1 configuration register 3
    #[inline(always)]
    pub const fn vc1cfgr3(&self) -> &VC1CFGR3 {
        &self.vc1cfgr3
    }
    ///0x2c - CSI-2 Host virtual channel 1 configuration register 4
    #[inline(always)]
    pub const fn vc1cfgr4(&self) -> &VC1CFGR4 {
        &self.vc1cfgr4
    }
    ///0x30 - CSI-2 Host virtual channel 2 configuration register 1
    #[inline(always)]
    pub const fn vc2cfgr1(&self) -> &VC2CFGR1 {
        &self.vc2cfgr1
    }
    ///0x34 - CSI-2 Host virtual channel 2 configuration register 2
    #[inline(always)]
    pub const fn vc2cfgr2(&self) -> &VC2CFGR2 {
        &self.vc2cfgr2
    }
    ///0x38 - CSI-2 Host virtual channel 2 configuration register 3
    #[inline(always)]
    pub const fn vc2cfgr3(&self) -> &VC2CFGR3 {
        &self.vc2cfgr3
    }
    ///0x3c - CSI-2 Host virtual channel 2 configuration register 4
    #[inline(always)]
    pub const fn vc2cfgr4(&self) -> &VC2CFGR4 {
        &self.vc2cfgr4
    }
    ///0x40 - CSI-2 Host virtual channel 3 configuration register 1
    #[inline(always)]
    pub const fn vc3cfgr1(&self) -> &VC3CFGR1 {
        &self.vc3cfgr1
    }
    ///0x44 - CSI-2 Host virtual channel 3 configuration register 2
    #[inline(always)]
    pub const fn vc3cfgr2(&self) -> &VC3CFGR2 {
        &self.vc3cfgr2
    }
    ///0x48 - CSI-2 Host virtual channel 3 configuration register 3
    #[inline(always)]
    pub const fn vc3cfgr3(&self) -> &VC3CFGR3 {
        &self.vc3cfgr3
    }
    ///0x4c - CSI-2 Host virtual channel 3 configuration register 4
    #[inline(always)]
    pub const fn vc3cfgr4(&self) -> &VC3CFGR4 {
        &self.vc3cfgr4
    }
    ///0x50 - CSI-2 Host line byte 0 configuration register
    #[inline(always)]
    pub const fn lb0cfgr(&self) -> &LB0CFGR {
        &self.lb0cfgr
    }
    ///0x54 - CSI-2 Host line byte 1 configuration register
    #[inline(always)]
    pub const fn lb1cfgr(&self) -> &LB1CFGR {
        &self.lb1cfgr
    }
    ///0x58 - CSI-2 Host line byte 2 configuration register
    #[inline(always)]
    pub const fn lb2cfgr(&self) -> &LB2CFGR {
        &self.lb2cfgr
    }
    ///0x5c - CSI-2 Host line byte 3 configuration register
    #[inline(always)]
    pub const fn lb3cfgr(&self) -> &LB3CFGR {
        &self.lb3cfgr
    }
    ///0x60 - CSI-2 Host timer 0 configuration register
    #[inline(always)]
    pub const fn tim0cfgr(&self) -> &TIM0CFGR {
        &self.tim0cfgr
    }
    ///0x64 - CSI-2 Host timer 1 configuration register
    #[inline(always)]
    pub const fn tim1cfgr(&self) -> &TIM1CFGR {
        &self.tim1cfgr
    }
    ///0x68 - CSI-2 Host timer 2 configuration register
    #[inline(always)]
    pub const fn tim2cfgr(&self) -> &TIM2CFGR {
        &self.tim2cfgr
    }
    ///0x6c - CSI-2 Host timer 3 configuration register
    #[inline(always)]
    pub const fn tim3cfgr(&self) -> &TIM3CFGR {
        &self.tim3cfgr
    }
    ///0x70 - CSI-2 Host lane merger configuration register
    #[inline(always)]
    pub const fn lmcfgr(&self) -> &LMCFGR {
        &self.lmcfgr
    }
    ///0x74 - CSI-2 Host program interrupt register
    #[inline(always)]
    pub const fn prgitr(&self) -> &PRGITR {
        &self.prgitr
    }
    ///0x78 - CSI-2 Host watchdog register
    #[inline(always)]
    pub const fn wdr(&self) -> &WDR {
        &self.wdr
    }
    ///0x80 - CSI-2 Host interrupt enable register 0
    #[inline(always)]
    pub const fn ier0(&self) -> &IER0 {
        &self.ier0
    }
    ///0x84 - CSI-2 Host interrupt enable register 1
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    ///0x90 - CSI-2 Host status register 0
    #[inline(always)]
    pub const fn sr0(&self) -> &SR0 {
        &self.sr0
    }
    ///0x94 - CSI-2 Host status register 1
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    ///0x100 - CSI-2 Host flag clear register 0
    #[inline(always)]
    pub const fn fcr0(&self) -> &FCR0 {
        &self.fcr0
    }
    ///0x104 - CSI-2 Host flag clear register 1
    #[inline(always)]
    pub const fn fcr1(&self) -> &FCR1 {
        &self.fcr1
    }
    ///0x110 - CSI-2 Host short packet data field register
    #[inline(always)]
    pub const fn spdfr(&self) -> &SPDFR {
        &self.spdfr
    }
    ///0x114 - CSI-2 Host error register 1
    #[inline(always)]
    pub const fn err1(&self) -> &ERR1 {
        &self.err1
    }
    ///0x118 - CSI-2 Host error register 2
    #[inline(always)]
    pub const fn err2(&self) -> &ERR2 {
        &self.err2
    }
    ///0x1000 - CSI PHY reset control register
    #[inline(always)]
    pub const fn prcr(&self) -> &PRCR {
        &self.prcr
    }
    ///0x1004 - CSI PHY mode control register
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    ///0x1008 - CSI PHY frequency control register
    #[inline(always)]
    pub const fn pfcr(&self) -> &PFCR {
        &self.pfcr
    }
    ///0x1010 - CSI PHY test control register 0
    #[inline(always)]
    pub const fn ptcr0(&self) -> &PTCR0 {
        &self.ptcr0
    }
    ///0x1014 - CSI PHY test control register 1
    #[inline(always)]
    pub const fn ptcr1(&self) -> &PTCR1 {
        &self.ptcr1
    }
    ///0x1018 - CSI PHY test status register
    #[inline(always)]
    pub const fn ptsr(&self) -> &PTSR {
        &self.ptsr
    }
}
/**CR (rw) register accessor: CSI-2 Host control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CSI-2 Host control register
pub mod cr;
/**PCR (rw) register accessor: CSI-2 Host DPHY_RX control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///CSI-2 Host DPHY_RX control register
pub mod pcr;
/**VC0CFGR1 (rw) register accessor: CSI-2 Host virtual channel 0 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`vc0cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc0cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC0CFGR1)

For information about available fields see [`mod@vc0cfgr1`] module*/
pub type VC0CFGR1 = crate::Reg<vc0cfgr1::VC0CFGR1rs>;
///CSI-2 Host virtual channel 0 configuration register 1
pub mod vc0cfgr1;
/**VC0CFGR2 (rw) register accessor: CSI-2 Host virtual channel 0 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`vc0cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc0cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC0CFGR2)

For information about available fields see [`mod@vc0cfgr2`] module*/
pub type VC0CFGR2 = crate::Reg<vc0cfgr2::VC0CFGR2rs>;
///CSI-2 Host virtual channel 0 configuration register 2
pub mod vc0cfgr2;
/**VC0CFGR3 (rw) register accessor: CSI-2 Host virtual channel 0 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`vc0cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc0cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC0CFGR3)

For information about available fields see [`mod@vc0cfgr3`] module*/
pub type VC0CFGR3 = crate::Reg<vc0cfgr3::VC0CFGR3rs>;
///CSI-2 Host virtual channel 0 configuration register 3
pub mod vc0cfgr3;
/**VC0CFGR4 (rw) register accessor: CSI-2 Host virtual channel 0 configuration register 4

You can [`read`](crate::Reg::read) this register and get [`vc0cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc0cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC0CFGR4)

For information about available fields see [`mod@vc0cfgr4`] module*/
pub type VC0CFGR4 = crate::Reg<vc0cfgr4::VC0CFGR4rs>;
///CSI-2 Host virtual channel 0 configuration register 4
pub mod vc0cfgr4;
/**VC1CFGR1 (rw) register accessor: CSI-2 Host virtual channel 1 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`vc1cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc1cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC1CFGR1)

For information about available fields see [`mod@vc1cfgr1`] module*/
pub type VC1CFGR1 = crate::Reg<vc1cfgr1::VC1CFGR1rs>;
///CSI-2 Host virtual channel 1 configuration register 1
pub mod vc1cfgr1;
/**VC1CFGR2 (rw) register accessor: CSI-2 Host virtual channel 1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`vc1cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc1cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC1CFGR2)

For information about available fields see [`mod@vc1cfgr2`] module*/
pub type VC1CFGR2 = crate::Reg<vc1cfgr2::VC1CFGR2rs>;
///CSI-2 Host virtual channel 1 configuration register 2
pub mod vc1cfgr2;
/**VC1CFGR3 (rw) register accessor: CSI-2 Host virtual channel 1 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`vc1cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc1cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC1CFGR3)

For information about available fields see [`mod@vc1cfgr3`] module*/
pub type VC1CFGR3 = crate::Reg<vc1cfgr3::VC1CFGR3rs>;
///CSI-2 Host virtual channel 1 configuration register 3
pub mod vc1cfgr3;
/**VC1CFGR4 (rw) register accessor: CSI-2 Host virtual channel 1 configuration register 4

You can [`read`](crate::Reg::read) this register and get [`vc1cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc1cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC1CFGR4)

For information about available fields see [`mod@vc1cfgr4`] module*/
pub type VC1CFGR4 = crate::Reg<vc1cfgr4::VC1CFGR4rs>;
///CSI-2 Host virtual channel 1 configuration register 4
pub mod vc1cfgr4;
/**VC2CFGR1 (rw) register accessor: CSI-2 Host virtual channel 2 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`vc2cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc2cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC2CFGR1)

For information about available fields see [`mod@vc2cfgr1`] module*/
pub type VC2CFGR1 = crate::Reg<vc2cfgr1::VC2CFGR1rs>;
///CSI-2 Host virtual channel 2 configuration register 1
pub mod vc2cfgr1;
/**VC2CFGR2 (rw) register accessor: CSI-2 Host virtual channel 2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`vc2cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc2cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC2CFGR2)

For information about available fields see [`mod@vc2cfgr2`] module*/
pub type VC2CFGR2 = crate::Reg<vc2cfgr2::VC2CFGR2rs>;
///CSI-2 Host virtual channel 2 configuration register 2
pub mod vc2cfgr2;
/**VC2CFGR3 (rw) register accessor: CSI-2 Host virtual channel 2 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`vc2cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc2cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC2CFGR3)

For information about available fields see [`mod@vc2cfgr3`] module*/
pub type VC2CFGR3 = crate::Reg<vc2cfgr3::VC2CFGR3rs>;
///CSI-2 Host virtual channel 2 configuration register 3
pub mod vc2cfgr3;
/**VC2CFGR4 (rw) register accessor: CSI-2 Host virtual channel 2 configuration register 4

You can [`read`](crate::Reg::read) this register and get [`vc2cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc2cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC2CFGR4)

For information about available fields see [`mod@vc2cfgr4`] module*/
pub type VC2CFGR4 = crate::Reg<vc2cfgr4::VC2CFGR4rs>;
///CSI-2 Host virtual channel 2 configuration register 4
pub mod vc2cfgr4;
/**VC3CFGR1 (rw) register accessor: CSI-2 Host virtual channel 3 configuration register 1

You can [`read`](crate::Reg::read) this register and get [`vc3cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc3cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC3CFGR1)

For information about available fields see [`mod@vc3cfgr1`] module*/
pub type VC3CFGR1 = crate::Reg<vc3cfgr1::VC3CFGR1rs>;
///CSI-2 Host virtual channel 3 configuration register 1
pub mod vc3cfgr1;
/**VC3CFGR2 (rw) register accessor: CSI-2 Host virtual channel 3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`vc3cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc3cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC3CFGR2)

For information about available fields see [`mod@vc3cfgr2`] module*/
pub type VC3CFGR2 = crate::Reg<vc3cfgr2::VC3CFGR2rs>;
///CSI-2 Host virtual channel 3 configuration register 2
pub mod vc3cfgr2;
/**VC3CFGR3 (rw) register accessor: CSI-2 Host virtual channel 3 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`vc3cfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc3cfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC3CFGR3)

For information about available fields see [`mod@vc3cfgr3`] module*/
pub type VC3CFGR3 = crate::Reg<vc3cfgr3::VC3CFGR3rs>;
///CSI-2 Host virtual channel 3 configuration register 3
pub mod vc3cfgr3;
/**VC3CFGR4 (rw) register accessor: CSI-2 Host virtual channel 3 configuration register 4

You can [`read`](crate::Reg::read) this register and get [`vc3cfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vc3cfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:VC3CFGR4)

For information about available fields see [`mod@vc3cfgr4`] module*/
pub type VC3CFGR4 = crate::Reg<vc3cfgr4::VC3CFGR4rs>;
///CSI-2 Host virtual channel 3 configuration register 4
pub mod vc3cfgr4;
/**LB0CFGR (rw) register accessor: CSI-2 Host line byte 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`lb0cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lb0cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:LB0CFGR)

For information about available fields see [`mod@lb0cfgr`] module*/
pub type LB0CFGR = crate::Reg<lb0cfgr::LB0CFGRrs>;
///CSI-2 Host line byte 0 configuration register
pub mod lb0cfgr;
/**LB1CFGR (rw) register accessor: CSI-2 Host line byte 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`lb1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lb1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:LB1CFGR)

For information about available fields see [`mod@lb1cfgr`] module*/
pub type LB1CFGR = crate::Reg<lb1cfgr::LB1CFGRrs>;
///CSI-2 Host line byte 1 configuration register
pub mod lb1cfgr;
/**LB2CFGR (rw) register accessor: CSI-2 Host line byte 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`lb2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lb2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:LB2CFGR)

For information about available fields see [`mod@lb2cfgr`] module*/
pub type LB2CFGR = crate::Reg<lb2cfgr::LB2CFGRrs>;
///CSI-2 Host line byte 2 configuration register
pub mod lb2cfgr;
/**LB3CFGR (rw) register accessor: CSI-2 Host line byte 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`lb3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lb3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:LB3CFGR)

For information about available fields see [`mod@lb3cfgr`] module*/
pub type LB3CFGR = crate::Reg<lb3cfgr::LB3CFGRrs>;
///CSI-2 Host line byte 3 configuration register
pub mod lb3cfgr;
/**TIM0CFGR (rw) register accessor: CSI-2 Host timer 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim0cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim0cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:TIM0CFGR)

For information about available fields see [`mod@tim0cfgr`] module*/
pub type TIM0CFGR = crate::Reg<tim0cfgr::TIM0CFGRrs>;
///CSI-2 Host timer 0 configuration register
pub mod tim0cfgr;
/**TIM1CFGR (rw) register accessor: CSI-2 Host timer 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:TIM1CFGR)

For information about available fields see [`mod@tim1cfgr`] module*/
pub type TIM1CFGR = crate::Reg<tim1cfgr::TIM1CFGRrs>;
///CSI-2 Host timer 1 configuration register
pub mod tim1cfgr;
/**TIM2CFGR (rw) register accessor: CSI-2 Host timer 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:TIM2CFGR)

For information about available fields see [`mod@tim2cfgr`] module*/
pub type TIM2CFGR = crate::Reg<tim2cfgr::TIM2CFGRrs>;
///CSI-2 Host timer 2 configuration register
pub mod tim2cfgr;
/**TIM3CFGR (rw) register accessor: CSI-2 Host timer 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:TIM3CFGR)

For information about available fields see [`mod@tim3cfgr`] module*/
pub type TIM3CFGR = crate::Reg<tim3cfgr::TIM3CFGRrs>;
///CSI-2 Host timer 3 configuration register
pub mod tim3cfgr;
/**LMCFGR (rw) register accessor: CSI-2 Host lane merger configuration register

You can [`read`](crate::Reg::read) this register and get [`lmcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lmcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:LMCFGR)

For information about available fields see [`mod@lmcfgr`] module*/
pub type LMCFGR = crate::Reg<lmcfgr::LMCFGRrs>;
///CSI-2 Host lane merger configuration register
pub mod lmcfgr;
/**PRGITR (rw) register accessor: CSI-2 Host program interrupt register

You can [`read`](crate::Reg::read) this register and get [`prgitr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgitr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PRGITR)

For information about available fields see [`mod@prgitr`] module*/
pub type PRGITR = crate::Reg<prgitr::PRGITRrs>;
///CSI-2 Host program interrupt register
pub mod prgitr;
/**WDR (rw) register accessor: CSI-2 Host watchdog register

You can [`read`](crate::Reg::read) this register and get [`wdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:WDR)

For information about available fields see [`mod@wdr`] module*/
pub type WDR = crate::Reg<wdr::WDRrs>;
///CSI-2 Host watchdog register
pub mod wdr;
/**IER0 (rw) register accessor: CSI-2 Host interrupt enable register 0

You can [`read`](crate::Reg::read) this register and get [`ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:IER0)

For information about available fields see [`mod@ier0`] module*/
pub type IER0 = crate::Reg<ier0::IER0rs>;
///CSI-2 Host interrupt enable register 0
pub mod ier0;
/**IER1 (rw) register accessor: CSI-2 Host interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:IER1)

For information about available fields see [`mod@ier1`] module*/
pub type IER1 = crate::Reg<ier1::IER1rs>;
///CSI-2 Host interrupt enable register 1
pub mod ier1;
/**SR0 (r) register accessor: CSI-2 Host status register 0

You can [`read`](crate::Reg::read) this register and get [`sr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:SR0)

For information about available fields see [`mod@sr0`] module*/
pub type SR0 = crate::Reg<sr0::SR0rs>;
///CSI-2 Host status register 0
pub mod sr0;
/**SR1 (r) register accessor: CSI-2 Host status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:SR1)

For information about available fields see [`mod@sr1`] module*/
pub type SR1 = crate::Reg<sr1::SR1rs>;
///CSI-2 Host status register 1
pub mod sr1;
/**FCR0 (w) register accessor: CSI-2 Host flag clear register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:FCR0)

For information about available fields see [`mod@fcr0`] module*/
pub type FCR0 = crate::Reg<fcr0::FCR0rs>;
///CSI-2 Host flag clear register 0
pub mod fcr0;
/**FCR1 (w) register accessor: CSI-2 Host flag clear register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:FCR1)

For information about available fields see [`mod@fcr1`] module*/
pub type FCR1 = crate::Reg<fcr1::FCR1rs>;
///CSI-2 Host flag clear register 1
pub mod fcr1;
/**SPDFR (r) register accessor: CSI-2 Host short packet data field register

You can [`read`](crate::Reg::read) this register and get [`spdfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:SPDFR)

For information about available fields see [`mod@spdfr`] module*/
pub type SPDFR = crate::Reg<spdfr::SPDFRrs>;
///CSI-2 Host short packet data field register
pub mod spdfr;
/**ERR1 (r) register accessor: CSI-2 Host error register 1

You can [`read`](crate::Reg::read) this register and get [`err1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:ERR1)

For information about available fields see [`mod@err1`] module*/
pub type ERR1 = crate::Reg<err1::ERR1rs>;
///CSI-2 Host error register 1
pub mod err1;
/**ERR2 (r) register accessor: CSI-2 Host error register 2

You can [`read`](crate::Reg::read) this register and get [`err2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:ERR2)

For information about available fields see [`mod@err2`] module*/
pub type ERR2 = crate::Reg<err2::ERR2rs>;
///CSI-2 Host error register 2
pub mod err2;
/**PRCR (rw) register accessor: CSI PHY reset control register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PRCR)

For information about available fields see [`mod@prcr`] module*/
pub type PRCR = crate::Reg<prcr::PRCRrs>;
///CSI PHY reset control register
pub mod prcr;
/**PMCR (rw) register accessor: CSI PHY mode control register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///CSI PHY mode control register
pub mod pmcr;
/**PFCR (rw) register accessor: CSI PHY frequency control register

You can [`read`](crate::Reg::read) this register and get [`pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PFCR)

For information about available fields see [`mod@pfcr`] module*/
pub type PFCR = crate::Reg<pfcr::PFCRrs>;
///CSI PHY frequency control register
pub mod pfcr;
/**PTCR0 (rw) register accessor: CSI PHY test control register 0

You can [`read`](crate::Reg::read) this register and get [`ptcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PTCR0)

For information about available fields see [`mod@ptcr0`] module*/
pub type PTCR0 = crate::Reg<ptcr0::PTCR0rs>;
///CSI PHY test control register 0
pub mod ptcr0;
/**PTCR1 (rw) register accessor: CSI PHY test control register 1

You can [`read`](crate::Reg::read) this register and get [`ptcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PTCR1)

For information about available fields see [`mod@ptcr1`] module*/
pub type PTCR1 = crate::Reg<ptcr1::PTCR1rs>;
///CSI PHY test control register 1
pub mod ptcr1;
/**PTSR (r) register accessor: CSI PHY test status register

You can [`read`](crate::Reg::read) this register and get [`ptsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CSI:PTSR)

For information about available fields see [`mod@ptsr`] module*/
pub type PTSR = crate::Reg<ptsr::PTSRrs>;
///CSI PHY test status register
pub mod ptsr;
