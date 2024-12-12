#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrigr: SWTRIGR,
    dhr12r1: DHR12R1,
    dhr12l1: DHR12L1,
    dhr8r1: DHR8R1,
    _reserved5: [u8; 0x18],
    dor1: DOR1,
    _reserved6: [u8; 0x04],
    sr: SR,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - software trigger register
    #[inline(always)]
    pub const fn swtrigr(&self) -> &SWTRIGR {
        &self.swtrigr
    }
    ///0x08 - channel1 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R1 {
        &self.dhr12r1
    }
    ///0x0c - DAC channel1 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L1 {
        &self.dhr12l1
    }
    ///0x10 - DAC channel1 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R1 {
        &self.dhr8r1
    }
    ///0x2c - DAC channel1 data output register
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR1 {
        &self.dor1
    }
    ///0x34 - DAC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SWTRIGR (w) register accessor: software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:SWTRIGR)

For information about available fields see [`mod@swtrigr`]
module*/
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGRrs>;
///software trigger register
pub mod swtrigr;
/**DHR12R1 (rw) register accessor: channel1 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:DHR12R1)

For information about available fields see [`mod@dhr12r1`]
module*/
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1rs>;
///channel1 12-bit right-aligned data holding register
pub mod dhr12r1;
/**DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:DHR12L1)

For information about available fields see [`mod@dhr12l1`]
module*/
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1rs>;
///DAC channel1 12-bit left aligned data holding register
pub mod dhr12l1;
/**DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:DHR8R1)

For information about available fields see [`mod@dhr8r1`]
module*/
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1rs>;
///DAC channel1 8-bit right aligned data holding register
pub mod dhr8r1;
/**DOR1 (r) register accessor: DAC channel1 data output register

You can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:DOR1)

For information about available fields see [`mod@dor1`]
module*/
pub type DOR1 = crate::Reg<dor1::DOR1rs>;
///DAC channel1 data output register
pub mod dor1;
/**SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#DAC2:SR)

For information about available fields see [`mod@sr`]
module*/
pub type SR = crate::Reg<sr::SRrs>;
///DAC status register
pub mod sr;
