#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    seccfgr1: SECCFGR1,
    seccfgr2: SECCFGR2,
    seccfgr3: SECCFGR3,
    _reserved4: [u8; 0x04],
    privcfgr1: PRIVCFGR1,
    privcfgr2: PRIVCFGR2,
    privcfgr3: PRIVCFGR3,
}
impl RegisterBlock {
    ///0x00 - GTZC1 TZSC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x10 - GTZC1 TZSC secure configuration register 1
    #[inline(always)]
    pub const fn seccfgr1(&self) -> &SECCFGR1 {
        &self.seccfgr1
    }
    ///0x14 - GTZC1 TZSC secure configuration register 2
    #[inline(always)]
    pub const fn seccfgr2(&self) -> &SECCFGR2 {
        &self.seccfgr2
    }
    ///0x18 - GTZC1 TZSC secure configuration register 3
    #[inline(always)]
    pub const fn seccfgr3(&self) -> &SECCFGR3 {
        &self.seccfgr3
    }
    ///0x20 - GTZC1 TZSC privilege configuration register 1
    #[inline(always)]
    pub const fn privcfgr1(&self) -> &PRIVCFGR1 {
        &self.privcfgr1
    }
    ///0x24 - GTZC1 TZSC privilege configuration register 2
    #[inline(always)]
    pub const fn privcfgr2(&self) -> &PRIVCFGR2 {
        &self.privcfgr2
    }
    ///0x28 - GTZC1 TZSC privilege configuration register 3
    #[inline(always)]
    pub const fn privcfgr3(&self) -> &PRIVCFGR3 {
        &self.privcfgr3
    }
}
/**CR (rw) register accessor: GTZC1 TZSC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///GTZC1 TZSC control register
pub mod cr;
/**SECCFGR1 (rw) register accessor: GTZC1 TZSC secure configuration register 1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:SECCFGR1)

For information about available fields see [`mod@seccfgr1`] module*/
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1rs>;
///GTZC1 TZSC secure configuration register 1
pub mod seccfgr1;
/**SECCFGR2 (rw) register accessor: GTZC1 TZSC secure configuration register 2

You can [`read`](crate::Reg::read) this register and get [`seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:SECCFGR2)

For information about available fields see [`mod@seccfgr2`] module*/
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2rs>;
///GTZC1 TZSC secure configuration register 2
pub mod seccfgr2;
/**SECCFGR3 (rw) register accessor: GTZC1 TZSC secure configuration register 3

You can [`read`](crate::Reg::read) this register and get [`seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:SECCFGR3)

For information about available fields see [`mod@seccfgr3`] module*/
pub type SECCFGR3 = crate::Reg<seccfgr3::SECCFGR3rs>;
///GTZC1 TZSC secure configuration register 3
pub mod seccfgr3;
/**PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:PRIVCFGR1)

For information about available fields see [`mod@privcfgr1`] module*/
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1rs>;
///GTZC1 TZSC privilege configuration register 1
pub mod privcfgr1;
/**PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:PRIVCFGR2)

For information about available fields see [`mod@privcfgr2`] module*/
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2rs>;
///GTZC1 TZSC privilege configuration register 2
pub mod privcfgr2;
/**PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GTZC1_TZSC:PRIVCFGR3)

For information about available fields see [`mod@privcfgr3`] module*/
pub type PRIVCFGR3 = crate::Reg<privcfgr3::PRIVCFGR3rs>;
///GTZC1 TZSC privilege configuration register 3
pub mod privcfgr3;
