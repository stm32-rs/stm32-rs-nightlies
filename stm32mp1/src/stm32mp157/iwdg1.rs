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
    _reserved5: [u8; 0x02],
    ewcr: EWCR,
    _reserved6: [u8; 0x03d8],
    hwcfgr: HWCFGR,
    verr: VERR,
    idr: IDR,
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
    ///0x14 - IWDG early wake-up interrupt register
    #[inline(always)]
    pub const fn ewcr(&self) -> &EWCR {
        &self.ewcr
    }
    ///0x3f0 - IWDG hardware configuration register
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    ///0x3f4 - IWDG version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - IWDG identification register
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    ///0x3fc - IWDG size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**KR (w) register accessor: Key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:KR)

For information about available fields see [`mod@kr`] module*/
pub type KR = crate::Reg<kr::KRrs>;
///Key register
pub mod kr;
/**PR (rw) register accessor: Prescaler register

You can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:PR)

For information about available fields see [`mod@pr`] module*/
pub type PR = crate::Reg<pr::PRrs>;
///Prescaler register
pub mod pr;
/**RLR (rw) register accessor: Reload register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:RLR)

For information about available fields see [`mod@rlr`] module*/
pub type RLR = crate::Reg<rlr::RLRrs>;
///Reload register
pub mod rlr;
/**SR (r) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**WINR (rw) register accessor: Window register

You can [`read`](crate::Reg::read) this register and get [`winr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:WINR)

For information about available fields see [`mod@winr`] module*/
pub type WINR = crate::Reg<winr::WINRrs>;
///Window register
pub mod winr;
/**EWCR (rw) register accessor: IWDG early wake-up interrupt register

You can [`read`](crate::Reg::read) this register and get [`ewcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:EWCR)

For information about available fields see [`mod@ewcr`] module*/
pub type EWCR = crate::Reg<ewcr::EWCRrs>;
///IWDG early wake-up interrupt register
pub mod ewcr;
/**HWCFGR (r) register accessor: IWDG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:HWCFGR)

For information about available fields see [`mod@hwcfgr`] module*/
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
///IWDG hardware configuration register
pub mod hwcfgr;
/**VERR (r) register accessor: IWDG version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///IWDG version register
pub mod verr;
/**IDR (r) register accessor: IWDG identification register

You can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:IDR)

For information about available fields see [`mod@idr`] module*/
pub type IDR = crate::Reg<idr::IDRrs>;
///IWDG identification register
pub mod idr;
/**SIDR (r) register accessor: IWDG size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IWDG1:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///IWDG size identification register
pub mod sidr;
