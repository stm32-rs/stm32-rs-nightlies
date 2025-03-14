#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrigr: SWTRIGR,
    dhr12r1: DHR12R1,
    dhr12l1: DHR12L1,
    dhr8r1: DHR8R1,
    dhr12r2: DHR12R2,
    dhr12l2: DHR12L2,
    dhr8r2: DHR8R2,
    dhr12rd: DHR12RD,
    dhr12ld: DHR12LD,
    dhr8rd: DHR8RD,
    dor1: DOR1,
    dor2: DOR2,
    sr: SR,
}
impl RegisterBlock {
    ///0x00 - Control register (DAC_CR)
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DAC software trigger register (DAC_SWTRIGR)
    #[inline(always)]
    pub const fn swtrigr(&self) -> &SWTRIGR {
        &self.swtrigr
    }
    ///0x08 - DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R1 {
        &self.dhr12r1
    }
    ///0x0c - DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L1 {
        &self.dhr12l1
    }
    ///0x10 - DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R1 {
        &self.dhr8r1
    }
    ///0x14 - DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &DHR12R2 {
        &self.dhr12r2
    }
    ///0x18 - DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &DHR12L2 {
        &self.dhr12l2
    }
    ///0x1c - DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &DHR8R2 {
        &self.dhr8r2
    }
    ///0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &DHR12RD {
        &self.dhr12rd
    }
    ///0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &DHR12LD {
        &self.dhr12ld
    }
    ///0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &DHR8RD {
        &self.dhr8rd
    }
    ///0x2c - DAC channel1 data output register (DAC_DOR1)
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR1 {
        &self.dor1
    }
    ///0x30 - DAC channel2 data output register (DAC_DOR2)
    #[inline(always)]
    pub const fn dor2(&self) -> &DOR2 {
        &self.dor2
    }
    ///0x34 - DAC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
/**CR (rw) register accessor: Control register (DAC_CR)

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Control register (DAC_CR)
pub mod cr;
/**SWTRIGR (w) register accessor: DAC software trigger register (DAC_SWTRIGR)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:SWTRIGR)

For information about available fields see [`mod@swtrigr`] module*/
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGRrs>;
///DAC software trigger register (DAC_SWTRIGR)
pub mod swtrigr;
/**DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)

You can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12R1)

For information about available fields see [`mod@dhr12r1`] module*/
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1rs>;
///DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)
pub mod dhr12r1;
/**DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)

You can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12L1)

For information about available fields see [`mod@dhr12l1`] module*/
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1rs>;
///DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)
pub mod dhr12l1;
/**DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)

You can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR8R1)

For information about available fields see [`mod@dhr8r1`] module*/
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1rs>;
///DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)
pub mod dhr8r1;
/**DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)

You can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12R2)

For information about available fields see [`mod@dhr12r2`] module*/
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2rs>;
///DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)
pub mod dhr12r2;
/**DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)

You can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12L2)

For information about available fields see [`mod@dhr12l2`] module*/
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2rs>;
///DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)
pub mod dhr12l2;
/**DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)

You can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR8R2)

For information about available fields see [`mod@dhr8r2`] module*/
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2rs>;
///DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)
pub mod dhr8r2;
/**DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved

You can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12RD)

For information about available fields see [`mod@dhr12rd`] module*/
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RDrs>;
///Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved
pub mod dhr12rd;
/**DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved

You can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12LD)

For information about available fields see [`mod@dhr12ld`] module*/
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LDrs>;
///DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved
pub mod dhr12ld;
/**DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved

You can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR8RD)

For information about available fields see [`mod@dhr8rd`] module*/
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RDrs>;
///DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved
pub mod dhr8rd;
/**DOR1 (r) register accessor: DAC channel1 data output register (DAC_DOR1)

You can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DOR1)

For information about available fields see [`mod@dor1`] module*/
pub type DOR1 = crate::Reg<dor1::DOR1rs>;
///DAC channel1 data output register (DAC_DOR1)
pub mod dor1;
/**DOR2 (r) register accessor: DAC channel2 data output register (DAC_DOR2)

You can [`read`](crate::Reg::read) this register and get [`dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DOR2)

For information about available fields see [`mod@dor2`] module*/
pub type DOR2 = crate::Reg<dor2::DOR2rs>;
///DAC channel2 data output register (DAC_DOR2)
pub mod dor2;
/**SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DAC status register
pub mod sr;
