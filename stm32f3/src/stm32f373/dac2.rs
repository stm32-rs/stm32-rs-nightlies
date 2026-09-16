#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrigr: SWTRIGR,
    dhr12r: [DHR12R; 1],
    dhr12l: [DHR12L; 1],
    dhr8r: [DHR8R; 1],
    _reserved5: [u8; 0x18],
    dor: [DOR; 1],
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
    ///0x08 - channel%s 12-bit right-aligned data holding register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DHR12R1` register.</div>
    #[inline(always)]
    pub const fn dhr12r(&self, n: usize) -> &DHR12R {
        &self.dhr12r[n]
    }
    ///Iterator for array of:
    ///0x08 - channel%s 12-bit right-aligned data holding register
    #[inline(always)]
    pub fn dhr12r_iter(&self) -> impl Iterator<Item = &DHR12R> {
        self.dhr12r.iter()
    }
    ///0x08 - channel1 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R {
        self.dhr12r(0)
    }
    ///0x0c - channel%s 12-bit left aligned data holding register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DHR12L1` register.</div>
    #[inline(always)]
    pub const fn dhr12l(&self, n: usize) -> &DHR12L {
        &self.dhr12l[n]
    }
    ///Iterator for array of:
    ///0x0c - channel%s 12-bit left aligned data holding register
    #[inline(always)]
    pub fn dhr12l_iter(&self) -> impl Iterator<Item = &DHR12L> {
        self.dhr12l.iter()
    }
    ///0x0c - channel1 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L {
        self.dhr12l(0)
    }
    ///0x10 - channel%s 8-bit right aligned data holding register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DHR8R1` register.</div>
    #[inline(always)]
    pub const fn dhr8r(&self, n: usize) -> &DHR8R {
        &self.dhr8r[n]
    }
    ///Iterator for array of:
    ///0x10 - channel%s 8-bit right aligned data holding register
    #[inline(always)]
    pub fn dhr8r_iter(&self) -> impl Iterator<Item = &DHR8R> {
        self.dhr8r.iter()
    }
    ///0x10 - channel1 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R {
        self.dhr8r(0)
    }
    ///0x2c - channel%s data output register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DOR1` register.</div>
    #[inline(always)]
    pub const fn dor(&self, n: usize) -> &DOR {
        &self.dor[n]
    }
    ///Iterator for array of:
    ///0x2c - channel%s data output register
    #[inline(always)]
    pub fn dor_iter(&self) -> impl Iterator<Item = &DOR> {
        self.dor.iter()
    }
    ///0x2c - channel1 data output register
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR {
        self.dor(0)
    }
    ///0x34 - DAC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SWTRIGR (w) register accessor: software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrigr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:SWTRIGR)

For information about available fields see [`mod@swtrigr`] module*/
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGRrs>;
///software trigger register
pub mod swtrigr;
/**DHR12R (rw) register accessor: channel%s 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:DHR12R[1])

For information about available fields see [`mod@dhr12r`] module*/
pub type DHR12R = crate::Reg<dhr12r::DHR12Rrs>;
///channel%s 12-bit right-aligned data holding register
pub mod dhr12r;
/**DHR12L (rw) register accessor: channel%s 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:DHR12L[1])

For information about available fields see [`mod@dhr12l`] module*/
pub type DHR12L = crate::Reg<dhr12l::DHR12Lrs>;
///channel%s 12-bit left aligned data holding register
pub mod dhr12l;
/**DHR8R (rw) register accessor: channel%s 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:DHR8R[1])

For information about available fields see [`mod@dhr8r`] module*/
pub type DHR8R = crate::Reg<dhr8r::DHR8Rrs>;
///channel%s 8-bit right aligned data holding register
pub mod dhr8r;
/**DOR (r) register accessor: channel%s data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:DOR[1])

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///channel%s data output register
pub mod dor;
/**SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#DAC2:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DAC status register
pub mod sr;
