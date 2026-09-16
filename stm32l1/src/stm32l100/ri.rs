#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    icr: ICR,
    ascr1: ASCR1,
    ascr2: ASCR2,
    hyscr1: HYSCR1,
    hyscr2: HYSCR2,
    hyscr3: HYSCR3,
    hyscr4: HYSCR4,
    asmr1: ASMR1,
    cmr1: CMR1,
    cicr1: CICR1,
    asmr2: ASMR2,
    cmr2: CMR2,
    cicr2: CICR2,
    asmr3: ASMR3,
    cmr3: CMR3,
    cicr3: CICR3,
    asmr4: ASMR4,
    cmr4: CMR4,
    cicr4: CICR4,
    asmr5: ASMR5,
    cmr5: CMR5,
    cicr5: CICR5,
}
impl RegisterBlock {
    ///0x00 - RI input capture register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x04 - RI analog switches control register 1
    #[inline(always)]
    pub const fn ascr1(&self) -> &ASCR1 {
        &self.ascr1
    }
    ///0x08 - RI analog switches control register 2
    #[inline(always)]
    pub const fn ascr2(&self) -> &ASCR2 {
        &self.ascr2
    }
    ///0x0c - RI hysteresis control register 1
    #[inline(always)]
    pub const fn hyscr1(&self) -> &HYSCR1 {
        &self.hyscr1
    }
    ///0x10 - RI hysteresis control register 2
    #[inline(always)]
    pub const fn hyscr2(&self) -> &HYSCR2 {
        &self.hyscr2
    }
    ///0x14 - RI hysteresis control register 3
    #[inline(always)]
    pub const fn hyscr3(&self) -> &HYSCR3 {
        &self.hyscr3
    }
    ///0x18 - Hysteresis control register
    #[inline(always)]
    pub const fn hyscr4(&self) -> &HYSCR4 {
        &self.hyscr4
    }
    ///0x1c - Analog switch mode register
    #[inline(always)]
    pub const fn asmr1(&self) -> &ASMR1 {
        &self.asmr1
    }
    ///0x20 - Channel mask register
    #[inline(always)]
    pub const fn cmr1(&self) -> &CMR1 {
        &self.cmr1
    }
    ///0x24 - Channel identification for capture register
    #[inline(always)]
    pub const fn cicr1(&self) -> &CICR1 {
        &self.cicr1
    }
    ///0x28 - Analog switch mode register
    #[inline(always)]
    pub const fn asmr2(&self) -> &ASMR2 {
        &self.asmr2
    }
    ///0x2c - Channel mask register
    #[inline(always)]
    pub const fn cmr2(&self) -> &CMR2 {
        &self.cmr2
    }
    ///0x30 - Channel identification for capture register
    #[inline(always)]
    pub const fn cicr2(&self) -> &CICR2 {
        &self.cicr2
    }
    ///0x34 - Analog switch mode register
    #[inline(always)]
    pub const fn asmr3(&self) -> &ASMR3 {
        &self.asmr3
    }
    ///0x38 - Channel mask register
    #[inline(always)]
    pub const fn cmr3(&self) -> &CMR3 {
        &self.cmr3
    }
    ///0x3c - Channel identification for capture register
    #[inline(always)]
    pub const fn cicr3(&self) -> &CICR3 {
        &self.cicr3
    }
    ///0x40 - Analog switch mode register
    #[inline(always)]
    pub const fn asmr4(&self) -> &ASMR4 {
        &self.asmr4
    }
    ///0x44 - Channel mask register
    #[inline(always)]
    pub const fn cmr4(&self) -> &CMR4 {
        &self.cmr4
    }
    ///0x48 - Channel identification for capture register
    #[inline(always)]
    pub const fn cicr4(&self) -> &CICR4 {
        &self.cicr4
    }
    ///0x4c - Analog switch mode register
    #[inline(always)]
    pub const fn asmr5(&self) -> &ASMR5 {
        &self.asmr5
    }
    ///0x50 - Channel mask register
    #[inline(always)]
    pub const fn cmr5(&self) -> &CMR5 {
        &self.cmr5
    }
    ///0x54 - Channel identification for capture register
    #[inline(always)]
    pub const fn cicr5(&self) -> &CICR5 {
        &self.cicr5
    }
}
/**ICR (rw) register accessor: RI input capture register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///RI input capture register
pub mod icr;
/**ASCR1 (rw) register accessor: RI analog switches control register 1

You can [`read`](crate::Reg::read) this register and get [`ascr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASCR1)

For information about available fields see [`mod@ascr1`] module*/
pub type ASCR1 = crate::Reg<ascr1::ASCR1rs>;
///RI analog switches control register 1
pub mod ascr1;
/**ASCR2 (rw) register accessor: RI analog switches control register 2

You can [`read`](crate::Reg::read) this register and get [`ascr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASCR2)

For information about available fields see [`mod@ascr2`] module*/
pub type ASCR2 = crate::Reg<ascr2::ASCR2rs>;
///RI analog switches control register 2
pub mod ascr2;
/**HYSCR1 (rw) register accessor: RI hysteresis control register 1

You can [`read`](crate::Reg::read) this register and get [`hyscr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:HYSCR1)

For information about available fields see [`mod@hyscr1`] module*/
pub type HYSCR1 = crate::Reg<hyscr1::HYSCR1rs>;
///RI hysteresis control register 1
pub mod hyscr1;
/**HYSCR2 (rw) register accessor: RI hysteresis control register 2

You can [`read`](crate::Reg::read) this register and get [`hyscr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:HYSCR2)

For information about available fields see [`mod@hyscr2`] module*/
pub type HYSCR2 = crate::Reg<hyscr2::HYSCR2rs>;
///RI hysteresis control register 2
pub mod hyscr2;
/**HYSCR3 (rw) register accessor: RI hysteresis control register 3

You can [`read`](crate::Reg::read) this register and get [`hyscr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:HYSCR3)

For information about available fields see [`mod@hyscr3`] module*/
pub type HYSCR3 = crate::Reg<hyscr3::HYSCR3rs>;
///RI hysteresis control register 3
pub mod hyscr3;
/**HYSCR4 (rw) register accessor: Hysteresis control register

You can [`read`](crate::Reg::read) this register and get [`hyscr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hyscr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:HYSCR4)

For information about available fields see [`mod@hyscr4`] module*/
pub type HYSCR4 = crate::Reg<hyscr4::HYSCR4rs>;
///Hysteresis control register
pub mod hyscr4;
/**ASMR1 (rw) register accessor: Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASMR1)

For information about available fields see [`mod@asmr1`] module*/
pub type ASMR1 = crate::Reg<asmr1::ASMR1rs>;
///Analog switch mode register
pub mod asmr1;
/**CMR1 (rw) register accessor: Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CMR1)

For information about available fields see [`mod@cmr1`] module*/
pub type CMR1 = crate::Reg<cmr1::CMR1rs>;
///Channel mask register
pub mod cmr1;
/**CICR1 (rw) register accessor: Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR1)

For information about available fields see [`mod@cicr1`] module*/
pub type CICR1 = crate::Reg<cicr1::CICR1rs>;
///Channel identification for capture register
pub mod cicr1;
/**ASMR2 (rw) register accessor: Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASMR2)

For information about available fields see [`mod@asmr2`] module*/
pub type ASMR2 = crate::Reg<asmr2::ASMR2rs>;
///Analog switch mode register
pub mod asmr2;
/**CMR2 (rw) register accessor: Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CMR2)

For information about available fields see [`mod@cmr2`] module*/
pub type CMR2 = crate::Reg<cmr2::CMR2rs>;
///Channel mask register
pub mod cmr2;
/**CICR2 (rw) register accessor: Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR2)

For information about available fields see [`mod@cicr2`] module*/
pub type CICR2 = crate::Reg<cicr2::CICR2rs>;
///Channel identification for capture register
pub mod cicr2;
/**ASMR3 (rw) register accessor: Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASMR3)

For information about available fields see [`mod@asmr3`] module*/
pub type ASMR3 = crate::Reg<asmr3::ASMR3rs>;
///Analog switch mode register
pub mod asmr3;
/**CMR3 (rw) register accessor: Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CMR3)

For information about available fields see [`mod@cmr3`] module*/
pub type CMR3 = crate::Reg<cmr3::CMR3rs>;
///Channel mask register
pub mod cmr3;
/**CICR3 (rw) register accessor: Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR3)

For information about available fields see [`mod@cicr3`] module*/
pub type CICR3 = crate::Reg<cicr3::CICR3rs>;
///Channel identification for capture register
pub mod cicr3;
/**ASMR4 (rw) register accessor: Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASMR4)

For information about available fields see [`mod@asmr4`] module*/
pub type ASMR4 = crate::Reg<asmr4::ASMR4rs>;
///Analog switch mode register
pub mod asmr4;
/**CMR4 (rw) register accessor: Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CMR4)

For information about available fields see [`mod@cmr4`] module*/
pub type CMR4 = crate::Reg<cmr4::CMR4rs>;
///Channel mask register
pub mod cmr4;
/**CICR4 (rw) register accessor: Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR4)

For information about available fields see [`mod@cicr4`] module*/
pub type CICR4 = crate::Reg<cicr4::CICR4rs>;
///Channel identification for capture register
pub mod cicr4;
/**ASMR5 (rw) register accessor: Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:ASMR5)

For information about available fields see [`mod@asmr5`] module*/
pub type ASMR5 = crate::Reg<asmr5::ASMR5rs>;
///Analog switch mode register
pub mod asmr5;
/**CMR5 (rw) register accessor: Channel mask register

You can [`read`](crate::Reg::read) this register and get [`cmr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CMR5)

For information about available fields see [`mod@cmr5`] module*/
pub type CMR5 = crate::Reg<cmr5::CMR5rs>;
///Channel mask register
pub mod cmr5;
/**CICR5 (rw) register accessor: Channel identification for capture register

You can [`read`](crate::Reg::read) this register and get [`cicr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RI:CICR5)

For information about available fields see [`mod@cicr5`] module*/
pub type CICR5 = crate::Reg<cicr5::CICR5rs>;
///Channel identification for capture register
pub mod cicr5;
