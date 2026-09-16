#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cfgr1: CFGR1,
    _reserved1: [u8; 0x04],
    t0valr1: T0VALR1,
    _reserved2: [u8; 0x04],
    rampvalr: RAMPVALR,
    itr1: ITR1,
    _reserved4: [u8; 0x04],
    dr: DR,
    sr: SR,
    itenr: ITENR,
    icifr: ICIFR,
    or: OR,
}
impl RegisterBlock {
    ///0x00 - Temperature sensor configuration register 1
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x08 - Temperature sensor T0 value register 1
    #[inline(always)]
    pub const fn t0valr1(&self) -> &T0VALR1 {
        &self.t0valr1
    }
    ///0x10 - Temperature sensor ramp value register
    #[inline(always)]
    pub const fn rampvalr(&self) -> &RAMPVALR {
        &self.rampvalr
    }
    ///0x14 - Temperature sensor interrupt threshold register 1
    #[inline(always)]
    pub const fn itr1(&self) -> &ITR1 {
        &self.itr1
    }
    ///0x1c - Temperature sensor data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x20 - Temperature sensor status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x24 - Temperature sensor interrupt enable register
    #[inline(always)]
    pub const fn itenr(&self) -> &ITENR {
        &self.itenr
    }
    ///0x28 - Temperature sensor clear interrupt flag register
    #[inline(always)]
    pub const fn icifr(&self) -> &ICIFR {
        &self.icifr
    }
    ///0x2c - Temperature sensor option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
/**CFGR1 (rw) register accessor: Temperature sensor configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///Temperature sensor configuration register 1
pub mod cfgr1;
/**T0VALR1 (r) register accessor: Temperature sensor T0 value register 1

You can [`read`](crate::Reg::read) this register and get [`t0valr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:T0VALR1)

For information about available fields see [`mod@t0valr1`] module*/
pub type T0VALR1 = crate::Reg<t0valr1::T0VALR1rs>;
///Temperature sensor T0 value register 1
pub mod t0valr1;
/**RAMPVALR (r) register accessor: Temperature sensor ramp value register

You can [`read`](crate::Reg::read) this register and get [`rampvalr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:RAMPVALR)

For information about available fields see [`mod@rampvalr`] module*/
pub type RAMPVALR = crate::Reg<rampvalr::RAMPVALRrs>;
///Temperature sensor ramp value register
pub mod rampvalr;
/**ITR1 (rw) register accessor: Temperature sensor interrupt threshold register 1

You can [`read`](crate::Reg::read) this register and get [`itr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:ITR1)

For information about available fields see [`mod@itr1`] module*/
pub type ITR1 = crate::Reg<itr1::ITR1rs>;
///Temperature sensor interrupt threshold register 1
pub mod itr1;
/**DR (rw) register accessor: Temperature sensor data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///Temperature sensor data register
pub mod dr;
/**SR (r) register accessor: Temperature sensor status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Temperature sensor status register
pub mod sr;
/**ITENR (rw) register accessor: Temperature sensor interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`itenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:ITENR)

For information about available fields see [`mod@itenr`] module*/
pub type ITENR = crate::Reg<itenr::ITENRrs>;
///Temperature sensor interrupt enable register
pub mod itenr;
/**ICIFR (rw) register accessor: Temperature sensor clear interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`icifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:ICIFR)

For information about available fields see [`mod@icifr`] module*/
pub type ICIFR = crate::Reg<icifr::ICIFRrs>;
///Temperature sensor clear interrupt flag register
pub mod icifr;
/**OR (rw) register accessor: Temperature sensor option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#DTS:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///Temperature sensor option register
pub mod or;
