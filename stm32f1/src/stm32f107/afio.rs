#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    evcr: EVCR,
    mapr: MAPR,
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
    _reserved6: [u8; 0x04],
    mapr2: MAPR2,
}
impl RegisterBlock {
    ///0x00 - Event Control Register (AFIO_EVCR)
    #[inline(always)]
    pub const fn evcr(&self) -> &EVCR {
        &self.evcr
    }
    ///0x04 - AF remap and debug I/O configuration register (AFIO_MAPR)
    #[inline(always)]
    pub const fn mapr(&self) -> &MAPR {
        &self.mapr
    }
    ///0x08 - External interrupt configuration register 1 (AFIO_EXTICR1)
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    ///0x0c - External interrupt configuration register 2 (AFIO_EXTICR2)
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    ///0x10 - External interrupt configuration register 3 (AFIO_EXTICR3)
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    ///0x14 - External interrupt configuration register 4 (AFIO_EXTICR4)
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
    ///0x1c - AF remap and debug I/O configuration register
    #[inline(always)]
    pub const fn mapr2(&self) -> &MAPR2 {
        &self.mapr2
    }
}
/**EVCR (rw) register accessor: Event Control Register (AFIO_EVCR)

You can [`read`](crate::Reg::read) this register and get [`evcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:EVCR)

For information about available fields see [`mod@evcr`] module*/
pub type EVCR = crate::Reg<evcr::EVCRrs>;
///Event Control Register (AFIO_EVCR)
pub mod evcr;
/**MAPR (rw) register accessor: AF remap and debug I/O configuration register (AFIO_MAPR)

You can [`read`](crate::Reg::read) this register and get [`mapr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:MAPR)

For information about available fields see [`mod@mapr`] module*/
pub type MAPR = crate::Reg<mapr::MAPRrs>;
///AF remap and debug I/O configuration register (AFIO_MAPR)
pub mod mapr;
/**EXTICR1 (rw) register accessor: External interrupt configuration register 1 (AFIO_EXTICR1)

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///External interrupt configuration register 1 (AFIO_EXTICR1)
pub mod exticr1;
/**EXTICR2 (rw) register accessor: External interrupt configuration register 2 (AFIO_EXTICR2)

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///External interrupt configuration register 2 (AFIO_EXTICR2)
pub mod exticr2;
/**EXTICR3 (rw) register accessor: External interrupt configuration register 3 (AFIO_EXTICR3)

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///External interrupt configuration register 3 (AFIO_EXTICR3)
pub mod exticr3;
/**EXTICR4 (rw) register accessor: External interrupt configuration register 4 (AFIO_EXTICR4)

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///External interrupt configuration register 4 (AFIO_EXTICR4)
pub mod exticr4;
/**MAPR2 (rw) register accessor: AF remap and debug I/O configuration register

You can [`read`](crate::Reg::read) this register and get [`mapr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mapr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#AFIO:MAPR2)

For information about available fields see [`mod@mapr2`] module*/
pub type MAPR2 = crate::Reg<mapr2::MAPR2rs>;
///AF remap and debug I/O configuration register
pub mod mapr2;
