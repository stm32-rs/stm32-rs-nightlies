#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrigr: SWTRIGR,
    _reserved2: [u8; 0x08],
    dhr: DHR,
    _reserved3: [u8; 0x18],
    dor: DOR,
    _reserved4: [u8; 0x04],
    sr: SR,
}
impl RegisterBlock {
    ///0x00 - CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - SWTRIGR register
    #[inline(always)]
    pub const fn swtrigr(&self) -> &SWTRIGR {
        &self.swtrigr
    }
    ///0x10 - DHR register
    #[inline(always)]
    pub const fn dhr(&self) -> &DHR {
        &self.dhr
    }
    ///0x2c - DOR register
    #[inline(always)]
    pub const fn dor(&self) -> &DOR {
        &self.dor
    }
    ///0x34 - SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
/**CR (rw) register accessor: CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CR register
pub mod cr;
/**SWTRIGR (rw) register accessor: SWTRIGR register

You can [`read`](crate::Reg::read) this register and get [`swtrigr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:SWTRIGR)

For information about available fields see [`mod@swtrigr`] module*/
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGRrs>;
///SWTRIGR register
pub mod swtrigr;
/**DHR (rw) register accessor: DHR register

You can [`read`](crate::Reg::read) this register and get [`dhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:DHR)

For information about available fields see [`mod@dhr`] module*/
pub type DHR = crate::Reg<dhr::DHRrs>;
///DHR register
pub mod dhr;
/**DOR (r) register accessor: DOR register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:DOR)

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///DOR register
pub mod dor;
/**SR (rw) register accessor: SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DAC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SR register
pub mod sr;
