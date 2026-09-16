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
    ///0x0c - channel1 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L1 {
        &self.dhr12l1
    }
    ///0x10 - channel1 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R1 {
        &self.dhr8r1
    }
    ///0x14 - channel2 12-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &DHR12R2 {
        &self.dhr12r2
    }
    ///0x18 - channel2 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &DHR12L2 {
        &self.dhr12l2
    }
    ///0x1c - channel2 8-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &DHR8R2 {
        &self.dhr8r2
    }
    ///0x20 - Dual DAC 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &DHR12RD {
        &self.dhr12rd
    }
    ///0x24 - DUAL DAC 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &DHR12LD {
        &self.dhr12ld
    }
    ///0x28 - DUAL DAC 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &DHR8RD {
        &self.dhr8rd
    }
    ///0x2c - channel1 data output register
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR1 {
        &self.dor1
    }
    ///0x30 - channel2 data output register
    #[inline(always)]
    pub const fn dor2(&self) -> &DOR2 {
        &self.dor2
    }
    ///0x34 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SWTRIGR (w) register accessor: software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:SWTRIGR)

For information about available fields see [`mod@swtrigr`] module*/
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGRrs>;
///software trigger register
pub mod swtrigr;
/**DHR12R1 (rw) register accessor: channel1 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12R1)

For information about available fields see [`mod@dhr12r1`] module*/
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1rs>;
///channel1 12-bit right-aligned data holding register
pub mod dhr12r1;
/**DHR12L1 (rw) register accessor: channel1 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12L1)

For information about available fields see [`mod@dhr12l1`] module*/
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1rs>;
///channel1 12-bit left aligned data holding register
pub mod dhr12l1;
/**DHR8R1 (rw) register accessor: channel1 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR8R1)

For information about available fields see [`mod@dhr8r1`] module*/
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1rs>;
///channel1 8-bit right aligned data holding register
pub mod dhr8r1;
/**DHR12R2 (rw) register accessor: channel2 12-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12R2)

For information about available fields see [`mod@dhr12r2`] module*/
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2rs>;
///channel2 12-bit right aligned data holding register
pub mod dhr12r2;
/**DHR12L2 (rw) register accessor: channel2 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12L2)

For information about available fields see [`mod@dhr12l2`] module*/
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2rs>;
///channel2 12-bit left aligned data holding register
pub mod dhr12l2;
/**DHR8R2 (rw) register accessor: channel2 8-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR8R2)

For information about available fields see [`mod@dhr8r2`] module*/
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2rs>;
///channel2 8-bit right-aligned data holding register
pub mod dhr8r2;
/**DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12RD)

For information about available fields see [`mod@dhr12rd`] module*/
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RDrs>;
///Dual DAC 12-bit right-aligned data holding register
pub mod dhr12rd;
/**DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR12LD)

For information about available fields see [`mod@dhr12ld`] module*/
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LDrs>;
///DUAL DAC 12-bit left aligned data holding register
pub mod dhr12ld;
/**DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DHR8RD)

For information about available fields see [`mod@dhr8rd`] module*/
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RDrs>;
///DUAL DAC 8-bit right aligned data holding register
pub mod dhr8rd;
/**DOR1 (r) register accessor: channel1 data output register

You can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DOR1)

For information about available fields see [`mod@dor1`] module*/
pub type DOR1 = crate::Reg<dor1::DOR1rs>;
///channel1 data output register
pub mod dor1;
/**DOR2 (r) register accessor: channel2 data output register

You can [`read`](crate::Reg::read) this register and get [`dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:DOR2)

For information about available fields see [`mod@dor2`] module*/
pub type DOR2 = crate::Reg<dor2::DOR2rs>;
///channel2 data output register
pub mod dor2;
/**SR (rw) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#DAC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
