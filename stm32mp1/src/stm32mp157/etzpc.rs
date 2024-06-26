#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    etzpc_tzma0_size: ETZPC_TZMA0_SIZE,
    etzpc_tzma1_size: ETZPC_TZMA1_SIZE,
    _reserved2: [u8; 0x08],
    etzpc_decprot0: ETZPC_DECPROT0,
    etzpc_decprot1: ETZPC_DECPROT1,
    etzpc_decprot2: ETZPC_DECPROT2,
    etzpc_decprot3: ETZPC_DECPROT3,
    etzpc_decprot4: ETZPC_DECPROT4,
    etzpc_decprot5: ETZPC_DECPROT5,
    _reserved8: [u8; 0x08],
    etzpc_decprot_lock0: ETZPC_DECPROT_LOCK0,
    etzpc_decprot_lock1: ETZPC_DECPROT_LOCK1,
    etzpc_decprot_lock2: ETZPC_DECPROT_LOCK2,
    _reserved11: [u8; 0x03b4],
    etzpc_hwcfgr: ETZPC_HWCFGR,
    etzpc_verr: ETZPC_VERR,
    etzpc_idr: ETZPC_IDR,
    etzpc_sidr: ETZPC_SIDR,
}
impl RegisterBlock {
    ///0x00 - ETZPC ROM secure size definition
    #[inline(always)]
    pub const fn etzpc_tzma0_size(&self) -> &ETZPC_TZMA0_SIZE {
        &self.etzpc_tzma0_size
    }
    ///0x04 - ETZPC RAM secure size definition
    #[inline(always)]
    pub const fn etzpc_tzma1_size(&self) -> &ETZPC_TZMA1_SIZE {
        &self.etzpc_tzma1_size
    }
    ///0x10 - Register reset values
    #[inline(always)]
    pub const fn etzpc_decprot0(&self) -> &ETZPC_DECPROT0 {
        &self.etzpc_decprot0
    }
    ///0x14 - Register reset values
    #[inline(always)]
    pub const fn etzpc_decprot1(&self) -> &ETZPC_DECPROT1 {
        &self.etzpc_decprot1
    }
    ///0x18 - Register reset values
    #[inline(always)]
    pub const fn etzpc_decprot2(&self) -> &ETZPC_DECPROT2 {
        &self.etzpc_decprot2
    }
    ///0x1c - Register reset values
    #[inline(always)]
    pub const fn etzpc_decprot3(&self) -> &ETZPC_DECPROT3 {
        &self.etzpc_decprot3
    }
    ///0x20 - Register reset values
    #[inline(always)]
    pub const fn etzpc_decprot4(&self) -> &ETZPC_DECPROT4 {
        &self.etzpc_decprot4
    }
    ///0x24 - Register reset values
    #[inline(always)]
    pub const fn etzpc_decprot5(&self) -> &ETZPC_DECPROT5 {
        &self.etzpc_decprot5
    }
    ///0x30 - ETZPC decprot lock 0 register
    #[inline(always)]
    pub const fn etzpc_decprot_lock0(&self) -> &ETZPC_DECPROT_LOCK0 {
        &self.etzpc_decprot_lock0
    }
    ///0x34 - ETZPC decprot lock 1 register
    #[inline(always)]
    pub const fn etzpc_decprot_lock1(&self) -> &ETZPC_DECPROT_LOCK1 {
        &self.etzpc_decprot_lock1
    }
    ///0x38 - ETZPC decprot lock 2 register
    #[inline(always)]
    pub const fn etzpc_decprot_lock2(&self) -> &ETZPC_DECPROT_LOCK2 {
        &self.etzpc_decprot_lock2
    }
    ///0x3f0 - ETZPC IP HW configuration register
    #[inline(always)]
    pub const fn etzpc_hwcfgr(&self) -> &ETZPC_HWCFGR {
        &self.etzpc_hwcfgr
    }
    ///0x3f4 - ETZPC IP version register
    #[inline(always)]
    pub const fn etzpc_verr(&self) -> &ETZPC_VERR {
        &self.etzpc_verr
    }
    ///0x3f8 - ETZPC IP version register
    #[inline(always)]
    pub const fn etzpc_idr(&self) -> &ETZPC_IDR {
        &self.etzpc_idr
    }
    ///0x3fc - ETZPC IP version register
    #[inline(always)]
    pub const fn etzpc_sidr(&self) -> &ETZPC_SIDR {
        &self.etzpc_sidr
    }
}
/**ETZPC_TZMA0_SIZE (rw) register accessor: ETZPC ROM secure size definition

You can [`read`](crate::Reg::read) this register and get [`etzpc_tzma0_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_tzma0_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_TZMA0_SIZE)

For information about available fields see [`mod@etzpc_tzma0_size`]
module*/
pub type ETZPC_TZMA0_SIZE = crate::Reg<etzpc_tzma0_size::ETZPC_TZMA0_SIZErs>;
///ETZPC ROM secure size definition
pub mod etzpc_tzma0_size;
/**ETZPC_TZMA1_SIZE (rw) register accessor: ETZPC RAM secure size definition

You can [`read`](crate::Reg::read) this register and get [`etzpc_tzma1_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_tzma1_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_TZMA1_SIZE)

For information about available fields see [`mod@etzpc_tzma1_size`]
module*/
pub type ETZPC_TZMA1_SIZE = crate::Reg<etzpc_tzma1_size::ETZPC_TZMA1_SIZErs>;
///ETZPC RAM secure size definition
pub mod etzpc_tzma1_size;
/**ETZPC_DECPROT0 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT0)

For information about available fields see [`mod@etzpc_decprot0`]
module*/
pub type ETZPC_DECPROT0 = crate::Reg<etzpc_decprot0::ETZPC_DECPROT0rs>;
///Register reset values
pub mod etzpc_decprot0;
/**ETZPC_DECPROT1 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT1)

For information about available fields see [`mod@etzpc_decprot1`]
module*/
pub type ETZPC_DECPROT1 = crate::Reg<etzpc_decprot1::ETZPC_DECPROT1rs>;
///Register reset values
pub mod etzpc_decprot1;
/**ETZPC_DECPROT2 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT2)

For information about available fields see [`mod@etzpc_decprot2`]
module*/
pub type ETZPC_DECPROT2 = crate::Reg<etzpc_decprot2::ETZPC_DECPROT2rs>;
///Register reset values
pub mod etzpc_decprot2;
/**ETZPC_DECPROT3 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT3)

For information about available fields see [`mod@etzpc_decprot3`]
module*/
pub type ETZPC_DECPROT3 = crate::Reg<etzpc_decprot3::ETZPC_DECPROT3rs>;
///Register reset values
pub mod etzpc_decprot3;
/**ETZPC_DECPROT4 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT4)

For information about available fields see [`mod@etzpc_decprot4`]
module*/
pub type ETZPC_DECPROT4 = crate::Reg<etzpc_decprot4::ETZPC_DECPROT4rs>;
///Register reset values
pub mod etzpc_decprot4;
/**ETZPC_DECPROT5 (rw) register accessor: Register reset values

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT5)

For information about available fields see [`mod@etzpc_decprot5`]
module*/
pub type ETZPC_DECPROT5 = crate::Reg<etzpc_decprot5::ETZPC_DECPROT5rs>;
///Register reset values
pub mod etzpc_decprot5;
/**ETZPC_DECPROT_LOCK0 (rw) register accessor: ETZPC decprot lock 0 register

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot_lock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot_lock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT_LOCK0)

For information about available fields see [`mod@etzpc_decprot_lock0`]
module*/
pub type ETZPC_DECPROT_LOCK0 = crate::Reg<etzpc_decprot_lock0::ETZPC_DECPROT_LOCK0rs>;
///ETZPC decprot lock 0 register
pub mod etzpc_decprot_lock0;
/**ETZPC_DECPROT_LOCK1 (rw) register accessor: ETZPC decprot lock 1 register

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot_lock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot_lock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT_LOCK1)

For information about available fields see [`mod@etzpc_decprot_lock1`]
module*/
pub type ETZPC_DECPROT_LOCK1 = crate::Reg<etzpc_decprot_lock1::ETZPC_DECPROT_LOCK1rs>;
///ETZPC decprot lock 1 register
pub mod etzpc_decprot_lock1;
/**ETZPC_DECPROT_LOCK2 (rw) register accessor: ETZPC decprot lock 2 register

You can [`read`](crate::Reg::read) this register and get [`etzpc_decprot_lock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etzpc_decprot_lock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_DECPROT_LOCK2)

For information about available fields see [`mod@etzpc_decprot_lock2`]
module*/
pub type ETZPC_DECPROT_LOCK2 = crate::Reg<etzpc_decprot_lock2::ETZPC_DECPROT_LOCK2rs>;
///ETZPC decprot lock 2 register
pub mod etzpc_decprot_lock2;
/**ETZPC_HWCFGR (r) register accessor: ETZPC IP HW configuration register

You can [`read`](crate::Reg::read) this register and get [`etzpc_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_HWCFGR)

For information about available fields see [`mod@etzpc_hwcfgr`]
module*/
pub type ETZPC_HWCFGR = crate::Reg<etzpc_hwcfgr::ETZPC_HWCFGRrs>;
///ETZPC IP HW configuration register
pub mod etzpc_hwcfgr;
/**ETZPC_VERR (r) register accessor: ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`etzpc_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_VERR)

For information about available fields see [`mod@etzpc_verr`]
module*/
pub type ETZPC_VERR = crate::Reg<etzpc_verr::ETZPC_VERRrs>;
///ETZPC IP version register
pub mod etzpc_verr;
/**ETZPC_IDR (r) register accessor: ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`etzpc_idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_IDR)

For information about available fields see [`mod@etzpc_idr`]
module*/
pub type ETZPC_IDR = crate::Reg<etzpc_idr::ETZPC_IDRrs>;
///ETZPC IP version register
pub mod etzpc_idr;
/**ETZPC_SIDR (r) register accessor: ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`etzpc_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:ETZPC_SIDR)

For information about available fields see [`mod@etzpc_sidr`]
module*/
pub type ETZPC_SIDR = crate::Reg<etzpc_sidr::ETZPC_SIDRrs>;
///ETZPC IP version register
pub mod etzpc_sidr;
