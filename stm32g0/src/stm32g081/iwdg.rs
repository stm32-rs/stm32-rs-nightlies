#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    kr: KR,
    _reserved1: [u8; 0x02],
    pr: PR,
    _reserved2: [u8; 0x02],
    rlr: RLR,
    _reserved3: [u8; 0x02],
    sr: SR,
    _reserved4: [u8; 0x02],
    winr: WINR,
    _reserved5: [u8; 0x03de],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - Key register
    #[inline(always)]
    pub const fn kr(&self) -> &KR {
        &self.kr
    }
    ///0x04 - Prescaler register
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    ///0x08 - Reload register
    #[inline(always)]
    pub const fn rlr(&self) -> &RLR {
        &self.rlr
    }
    ///0x0c - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x10 - Window register
    #[inline(always)]
    pub const fn winr(&self) -> &WINR {
        &self.winr
    }
    ///0x3f0 - hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - EXTI IP Version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - EXTI Identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - EXTI Size ID register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**KR (w) register accessor: Key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:KR)

For information about available fields see [`mod@kr`] module*/
pub type KR = crate::Reg<kr::KRrs>;
///Key register
pub mod kr;
/**PR (rw) register accessor: Prescaler register

You can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:PR)

For information about available fields see [`mod@pr`] module*/
pub type PR = crate::Reg<pr::PRrs>;
///Prescaler register
pub mod pr;
/**RLR (rw) register accessor: Reload register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:RLR)

For information about available fields see [`mod@rlr`] module*/
pub type RLR = crate::Reg<rlr::RLRrs>;
///Reload register
pub mod rlr;
/**SR (r) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**WINR (rw) register accessor: Window register

You can [`read`](crate::Reg::read) this register and get [`winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:WINR)

For information about available fields see [`mod@winr`] module*/
pub type WINR = crate::Reg<winr::WINRrs>;
///Window register
pub mod winr;
/**HWCFGR (rw) register accessor: hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: EXTI IP Version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///EXTI IP Version register
pub mod verr;
/**IPIDR (r) register accessor: EXTI Identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///EXTI Identification register
pub mod ipidr;
/**SIDR (r) register accessor: EXTI Size ID register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#IWDG:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///EXTI Size ID register
pub mod sidr;
