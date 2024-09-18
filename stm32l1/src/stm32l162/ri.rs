#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    icr: ICR,
    ascr1: ASCR1,
    ascr2: ASCR2,
    hyscr1: HYSCR1,
    hyscr2: HYSCR2,
    hyscr3: HYSCR3,
    hyscr4: HYSCR4,
}
impl RegisterBlock {
    ///0x04 - RI input capture register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x08 - RI analog switches control register 1
    #[inline(always)]
    pub const fn ascr1(&self) -> &ASCR1 {
        &self.ascr1
    }
    ///0x0c - RI analog switches control register 2
    #[inline(always)]
    pub const fn ascr2(&self) -> &ASCR2 {
        &self.ascr2
    }
    ///0x10 - RI hysteresis control register 1
    #[inline(always)]
    pub const fn hyscr1(&self) -> &HYSCR1 {
        &self.hyscr1
    }
    ///0x14 - RI hysteresis control register 2
    #[inline(always)]
    pub const fn hyscr2(&self) -> &HYSCR2 {
        &self.hyscr2
    }
    ///0x18 - RI hysteresis control register 3
    #[inline(always)]
    pub const fn hyscr3(&self) -> &HYSCR3 {
        &self.hyscr3
    }
    ///0x1c - Hysteresis control register
    #[inline(always)]
    pub const fn hyscr4(&self) -> &HYSCR4 {
        &self.hyscr4
    }
}
/**ICR (rw) register accessor: RI input capture register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:ICR)

For information about available fields see [`mod@icr`]
module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///RI input capture register
pub mod icr;
/**ASCR1 (rw) register accessor: RI analog switches control register 1

You can [`read`](crate::Reg::read) this register and get [`ascr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:ASCR1)

For information about available fields see [`mod@ascr1`]
module*/
pub type ASCR1 = crate::Reg<ascr1::ASCR1rs>;
///RI analog switches control register 1
pub mod ascr1;
/**ASCR2 (rw) register accessor: RI analog switches control register 2

You can [`read`](crate::Reg::read) this register and get [`ascr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:ASCR2)

For information about available fields see [`mod@ascr2`]
module*/
pub type ASCR2 = crate::Reg<ascr2::ASCR2rs>;
///RI analog switches control register 2
pub mod ascr2;
/**HYSCR1 (rw) register accessor: RI hysteresis control register 1

You can [`read`](crate::Reg::read) this register and get [`hyscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:HYSCR1)

For information about available fields see [`mod@hyscr1`]
module*/
pub type HYSCR1 = crate::Reg<hyscr1::HYSCR1rs>;
///RI hysteresis control register 1
pub mod hyscr1;
/**HYSCR2 (rw) register accessor: RI hysteresis control register 2

You can [`read`](crate::Reg::read) this register and get [`hyscr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:HYSCR2)

For information about available fields see [`mod@hyscr2`]
module*/
pub type HYSCR2 = crate::Reg<hyscr2::HYSCR2rs>;
///RI hysteresis control register 2
pub mod hyscr2;
/**HYSCR3 (rw) register accessor: RI hysteresis control register 3

You can [`read`](crate::Reg::read) this register and get [`hyscr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:HYSCR3)

For information about available fields see [`mod@hyscr3`]
module*/
pub type HYSCR3 = crate::Reg<hyscr3::HYSCR3rs>;
///RI hysteresis control register 3
pub mod hyscr3;
/**HYSCR4 (rw) register accessor: Hysteresis control register

You can [`read`](crate::Reg::read) this register and get [`hyscr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:HYSCR4)

For information about available fields see [`mod@hyscr4`]
module*/
pub type HYSCR4 = crate::Reg<hyscr4::HYSCR4rs>;
///Hysteresis control register
pub mod hyscr4;
