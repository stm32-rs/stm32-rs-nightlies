#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tzma0_size: TZMA0_SIZE,
    tzma1_size: TZMA1_SIZE,
    _reserved2: [u8; 0x08],
    decprot0: DECPROT0,
    decprot1: DECPROT1,
    decprot2: DECPROT2,
    decprot3: DECPROT3,
    decprot4: DECPROT4,
    decprot5: DECPROT5,
    _reserved8: [u8; 0x08],
    decprot_lock0: DECPROT_LOCK0,
    decprot_lock1: DECPROT_LOCK1,
    decprot_lock2: DECPROT_LOCK2,
    _reserved11: [u8; 0x03b4],
    hwcfgr: HWCFGR,
    verr: VERR,
    idr: IDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - ETZPC ROM secure size definition
    #[inline(always)]
    pub const fn tzma0_size(&self) -> &TZMA0_SIZE {
        &self.tzma0_size
    }
    ///0x04 - ETZPC RAM secure size definition
    #[inline(always)]
    pub const fn tzma1_size(&self) -> &TZMA1_SIZE {
        &self.tzma1_size
    }
    ///0x10 - Register reset values
    #[inline(always)]
    pub const fn decprot0(&self) -> &DECPROT0 {
        &self.decprot0
    }
    ///0x14 - Register reset values
    #[inline(always)]
    pub const fn decprot1(&self) -> &DECPROT1 {
        &self.decprot1
    }
    ///0x18 - Register reset values
    #[inline(always)]
    pub const fn decprot2(&self) -> &DECPROT2 {
        &self.decprot2
    }
    ///0x1c - Register reset values
    #[inline(always)]
    pub const fn decprot3(&self) -> &DECPROT3 {
        &self.decprot3
    }
    ///0x20 - Register reset values
    #[inline(always)]
    pub const fn decprot4(&self) -> &DECPROT4 {
        &self.decprot4
    }
    ///0x24 - Register reset values
    #[inline(always)]
    pub const fn decprot5(&self) -> &DECPROT5 {
        &self.decprot5
    }
    ///0x30 - ETZPC decprot lock 0 register
    #[inline(always)]
    pub const fn decprot_lock0(&self) -> &DECPROT_LOCK0 {
        &self.decprot_lock0
    }
    ///0x34 - ETZPC decprot lock 1 register
    #[inline(always)]
    pub const fn decprot_lock1(&self) -> &DECPROT_LOCK1 {
        &self.decprot_lock1
    }
    ///0x38 - ETZPC decprot lock 2 register
    #[inline(always)]
    pub const fn decprot_lock2(&self) -> &DECPROT_LOCK2 {
        &self.decprot_lock2
    }
    ///0x3f0 - ETZPC IP HW configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - ETZPC IP version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - ETZPC IP version register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x3fc - ETZPC IP version register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**TZMA0_SIZE (rw) register accessor: ETZPC ROM secure size definition

You can [`read`](crate::Reg::read) this register and get [`tzma0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzma0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:TZMA0_SIZE)

For information about available fields see [`mod@tzma0_size`] module*/
pub type TZMA0_SIZE = crate::Reg<tzma0_size::TZMA0_SIZErs>;
///ETZPC ROM secure size definition
pub mod tzma0_size;
/**TZMA1_SIZE (rw) register accessor: ETZPC RAM secure size definition

You can [`read`](crate::Reg::read) this register and get [`tzma1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzma1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:TZMA1_SIZE)

For information about available fields see [`mod@tzma1_size`] module*/
pub type TZMA1_SIZE = crate::Reg<tzma1_size::TZMA1_SIZErs>;
///ETZPC RAM secure size definition
pub mod tzma1_size;
/**DECPROT0 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT0)

For information about available fields see [`mod@decprot0`] module*/
pub type DECPROT0 = crate::Reg<decprot0::DECPROT0rs>;
///Register reset values
pub mod decprot0;
/**DECPROT1 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT1)

For information about available fields see [`mod@decprot1`] module*/
pub type DECPROT1 = crate::Reg<decprot1::DECPROT1rs>;
///Register reset values
pub mod decprot1;
/**DECPROT2 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT2)

For information about available fields see [`mod@decprot2`] module*/
pub type DECPROT2 = crate::Reg<decprot2::DECPROT2rs>;
///Register reset values
pub mod decprot2;
/**DECPROT3 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT3)

For information about available fields see [`mod@decprot3`] module*/
pub type DECPROT3 = crate::Reg<decprot3::DECPROT3rs>;
///Register reset values
pub mod decprot3;
/**DECPROT4 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT4)

For information about available fields see [`mod@decprot4`] module*/
pub type DECPROT4 = crate::Reg<decprot4::DECPROT4rs>;
///Register reset values
pub mod decprot4;
/**DECPROT5 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`decprot5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT5)

For information about available fields see [`mod@decprot5`] module*/
pub type DECPROT5 = crate::Reg<decprot5::DECPROT5rs>;
///Register reset values
pub mod decprot5;
/**DECPROT_LOCK0 (rw) register accessor: ETZPC decprot lock 0 register

You can [`read`](crate::Reg::read) this register and get [`decprot_lock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot_lock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT_LOCK0)

For information about available fields see [`mod@decprot_lock0`] module*/
pub type DECPROT_LOCK0 = crate::Reg<decprot_lock0::DECPROT_LOCK0rs>;
///ETZPC decprot lock 0 register
pub mod decprot_lock0;
/**DECPROT_LOCK1 (rw) register accessor: ETZPC decprot lock 1 register

You can [`read`](crate::Reg::read) this register and get [`decprot_lock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot_lock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT_LOCK1)

For information about available fields see [`mod@decprot_lock1`] module*/
pub type DECPROT_LOCK1 = crate::Reg<decprot_lock1::DECPROT_LOCK1rs>;
///ETZPC decprot lock 1 register
pub mod decprot_lock1;
/**DECPROT_LOCK2 (rw) register accessor: ETZPC decprot lock 2 register

You can [`read`](crate::Reg::read) this register and get [`decprot_lock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decprot_lock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:DECPROT_LOCK2)

For information about available fields see [`mod@decprot_lock2`] module*/
pub type DECPROT_LOCK2 = crate::Reg<decprot_lock2::DECPROT_LOCK2rs>;
///ETZPC decprot lock 2 register
pub mod decprot_lock2;
/**HWCFGR (r) register accessor: ETZPC IP HW configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///ETZPC IP HW configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///ETZPC IP version register
pub mod verr;
/**IDR (r) register accessor: ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///ETZPC IP version register
pub mod idr;
/**SIDR (r) register accessor: ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///ETZPC IP version register
pub mod sidr;
