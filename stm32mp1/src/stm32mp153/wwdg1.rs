#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    wwdg_cr: WWDG_CR,
    wwdg_cfr: WWDG_CFR,
    wwdg_sr: WWDG_SR,
    _reserved3: [u8; 0x03e4],
    wwdg_hwcfgr: WWDG_HWCFGR,
    wwdg_verr: WWDG_VERR,
    wwdg_ipidr: WWDG_IPIDR,
    wwdg_sidr: WWDG_SIDR,
}
impl RegisterBlock {
    ///0x00 - Control register
    #[inline(always)]
    pub const fn wwdg_cr(&self) -> &WWDG_CR {
        &self.wwdg_cr
    }
    ///0x04 - Configuration register
    #[inline(always)]
    pub const fn wwdg_cfr(&self) -> &WWDG_CFR {
        &self.wwdg_cfr
    }
    ///0x08 - Status register
    #[inline(always)]
    pub const fn wwdg_sr(&self) -> &WWDG_SR {
        &self.wwdg_sr
    }
    ///0x3f0 - WWDG hardware configuration register
    #[inline(always)]
    pub const fn wwdg_hwcfgr(&self) -> &WWDG_HWCFGR {
        &self.wwdg_hwcfgr
    }
    ///0x3f4 - WWDG version register
    #[inline(always)]
    pub const fn wwdg_verr(&self) -> &WWDG_VERR {
        &self.wwdg_verr
    }
    ///0x3f8 - WWDG ID register
    #[inline(always)]
    pub const fn wwdg_ipidr(&self) -> &WWDG_IPIDR {
        &self.wwdg_ipidr
    }
    ///0x3fc - WWDG size ID register
    #[inline(always)]
    pub const fn wwdg_sidr(&self) -> &WWDG_SIDR {
        &self.wwdg_sidr
    }
}
/**WWDG_CR (rw) register accessor: Control register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_CR)

For information about available fields see [`mod@wwdg_cr`]
module*/
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CRrs>;
///Control register
pub mod wwdg_cr;
/**WWDG_CFR (rw) register accessor: Configuration register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_CFR)

For information about available fields see [`mod@wwdg_cfr`]
module*/
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFRrs>;
///Configuration register
pub mod wwdg_cfr;
/**WWDG_SR (rw) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`wwdg_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_SR)

For information about available fields see [`mod@wwdg_sr`]
module*/
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SRrs>;
///Status register
pub mod wwdg_sr;
/**WWDG_HWCFGR (r) register accessor: WWDG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`wwdg_hwcfgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_HWCFGR)

For information about available fields see [`mod@wwdg_hwcfgr`]
module*/
pub type WWDG_HWCFGR = crate::Reg<wwdg_hwcfgr::WWDG_HWCFGRrs>;
///WWDG hardware configuration register
pub mod wwdg_hwcfgr;
/**WWDG_VERR (r) register accessor: WWDG version register

You can [`read`](crate::Reg::read) this register and get [`wwdg_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_VERR)

For information about available fields see [`mod@wwdg_verr`]
module*/
pub type WWDG_VERR = crate::Reg<wwdg_verr::WWDG_VERRrs>;
///WWDG version register
pub mod wwdg_verr;
/**WWDG_IPIDR (r) register accessor: WWDG ID register

You can [`read`](crate::Reg::read) this register and get [`wwdg_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_IPIDR)

For information about available fields see [`mod@wwdg_ipidr`]
module*/
pub type WWDG_IPIDR = crate::Reg<wwdg_ipidr::WWDG_IPIDRrs>;
///WWDG ID register
pub mod wwdg_ipidr;
/**WWDG_SIDR (r) register accessor: WWDG size ID register

You can [`read`](crate::Reg::read) this register and get [`wwdg_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_SIDR)

For information about available fields see [`mod@wwdg_sidr`]
module*/
pub type WWDG_SIDR = crate::Reg<wwdg_sidr::WWDG_SIDRrs>;
///WWDG size ID register
pub mod wwdg_sidr;
