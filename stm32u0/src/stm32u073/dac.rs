#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrgr: SWTRGR,
    dhr12r: [DHR12R; 1],
    dhr12l: [DHR12L; 1],
    dhr8r: [DHR8R; 1],
    _reserved5: [u8; 0x18],
    dor: [DOR; 1],
    _reserved6: [u8; 0x04],
    sr: SR,
    ccr: CCR,
    mcr: MCR,
    shsr: [SHSR; 1],
    _reserved10: [u8; 0x04],
    shhr: SHHR,
    shrr: SHRR,
}
impl RegisterBlock {
    ///0x00 - DAC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DAC software trigger register
    #[inline(always)]
    pub const fn swtrgr(&self) -> &SWTRGR {
        &self.swtrgr
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
    ///0x38 - DAC calibration control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    ///0x3c - DAC mode control register
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    ///0x40 - DAC channel%s sample and hold sample time register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SHSR1` register.</div>
    #[inline(always)]
    pub const fn shsr(&self, n: usize) -> &SHSR {
        &self.shsr[n]
    }
    ///Iterator for array of:
    ///0x40 - DAC channel%s sample and hold sample time register
    #[inline(always)]
    pub fn shsr_iter(&self) -> impl Iterator<Item = &SHSR> {
        self.shsr.iter()
    }
    ///0x40 - DAC channel1 sample and hold sample time register
    #[inline(always)]
    pub const fn shsr1(&self) -> &SHSR {
        self.shsr(0)
    }
    ///0x48 - DAC sample and hold time register
    #[inline(always)]
    pub const fn shhr(&self) -> &SHHR {
        &self.shhr
    }
    ///0x4c - DAC sample and hold refresh time register
    #[inline(always)]
    pub const fn shrr(&self) -> &SHRR {
        &self.shrr
    }
}
/**CR (rw) register accessor: DAC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DAC control register
pub mod cr;
/**SWTRGR (w) register accessor: DAC software trigger register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:SWTRGR)

For information about available fields see [`mod@swtrgr`] module*/
pub type SWTRGR = crate::Reg<swtrgr::SWTRGRrs>;
///DAC software trigger register
pub mod swtrgr;
/**DHR12R (rw) register accessor: channel%s 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:DHR12R[1])

For information about available fields see [`mod@dhr12r`] module*/
pub type DHR12R = crate::Reg<dhr12r::DHR12Rrs>;
///channel%s 12-bit right-aligned data holding register
pub mod dhr12r;
/**DHR12L (rw) register accessor: channel%s 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:DHR12L[1])

For information about available fields see [`mod@dhr12l`] module*/
pub type DHR12L = crate::Reg<dhr12l::DHR12Lrs>;
///channel%s 12-bit left aligned data holding register
pub mod dhr12l;
/**DHR8R (rw) register accessor: channel%s 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:DHR8R[1])

For information about available fields see [`mod@dhr8r`] module*/
pub type DHR8R = crate::Reg<dhr8r::DHR8Rrs>;
///channel%s 8-bit right aligned data holding register
pub mod dhr8r;
/**DOR (r) register accessor: channel%s data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:DOR[1])

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///channel%s data output register
pub mod dor;
/**SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DAC status register
pub mod sr;
/**CCR (rw) register accessor: DAC calibration control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DAC calibration control register
pub mod ccr;
/**MCR (rw) register accessor: DAC mode control register

You can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:MCR)

For information about available fields see [`mod@mcr`] module*/
pub type MCR = crate::Reg<mcr::MCRrs>;
///DAC mode control register
pub mod mcr;
/**SHSR (rw) register accessor: DAC channel%s sample and hold sample time register

You can [`read`](crate::Reg::read) this register and get [`shsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:SHSR[1])

For information about available fields see [`mod@shsr`] module*/
pub type SHSR = crate::Reg<shsr::SHSRrs>;
///DAC channel%s sample and hold sample time register
pub mod shsr;
/**SHHR (rw) register accessor: DAC sample and hold time register

You can [`read`](crate::Reg::read) this register and get [`shhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:SHHR)

For information about available fields see [`mod@shhr`] module*/
pub type SHHR = crate::Reg<shhr::SHHRrs>;
///DAC sample and hold time register
pub mod shhr;
/**SHRR (rw) register accessor: DAC sample and hold refresh time register

You can [`read`](crate::Reg::read) this register and get [`shrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#DAC:SHRR)

For information about available fields see [`mod@shrr`] module*/
pub type SHRR = crate::Reg<shrr::SHRRrs>;
///DAC sample and hold refresh time register
pub mod shrr;
