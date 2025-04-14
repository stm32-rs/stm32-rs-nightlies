#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    swtrigr: SWTRIGR,
    dhr12r: (),
    _reserved3: [u8; 0x04],
    dhr12l: (),
    _reserved4: [u8; 0x04],
    dhr8r: (),
    _reserved5: [u8; 0x10],
    dhr12rd: DHR12RD,
    dhr12ld: DHR12LD,
    dhr8rd: DHR8RD,
    dor: [DOR; 2],
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
    ///0x08..0x10 - channel%s 12-bit right-aligned data holding register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DHR12R1` register.</div>
    #[inline(always)]
    pub const fn dhr12r(&self, n: usize) -> &DHR12R {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(12 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x08..0x10 - channel%s 12-bit right-aligned data holding register
    #[inline(always)]
    pub fn dhr12r_iter(&self) -> impl Iterator<Item = &DHR12R> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(12 * n)
                .cast()
        })
    }
    ///0x08 - channel1 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &DHR12R {
        self.dhr12r(0)
    }
    ///0x14 - channel2 12-bit right-aligned data holding register
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &DHR12R {
        self.dhr12r(1)
    }
    ///0x0c..0x14 - channel%s 12-bit left aligned data holding register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DHR12L1` register.</div>
    #[inline(always)]
    pub const fn dhr12l(&self, n: usize) -> &DHR12L {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(12 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x0c..0x14 - channel%s 12-bit left aligned data holding register
    #[inline(always)]
    pub fn dhr12l_iter(&self) -> impl Iterator<Item = &DHR12L> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(12 * n)
                .cast()
        })
    }
    ///0x0c - channel1 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &DHR12L {
        self.dhr12l(0)
    }
    ///0x18 - channel2 12-bit left aligned data holding register
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &DHR12L {
        self.dhr12l(1)
    }
    ///0x10..0x18 - channel%s 8-bit right aligned data holding register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DHR8R1` register.</div>
    #[inline(always)]
    pub const fn dhr8r(&self, n: usize) -> &DHR8R {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(12 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x10..0x18 - channel%s 8-bit right aligned data holding register
    #[inline(always)]
    pub fn dhr8r_iter(&self) -> impl Iterator<Item = &DHR8R> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(12 * n)
                .cast()
        })
    }
    ///0x10 - channel1 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &DHR8R {
        self.dhr8r(0)
    }
    ///0x1c - channel2 8-bit right aligned data holding register
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &DHR8R {
        self.dhr8r(1)
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
    ///0x2c..0x34 - channel%s data output register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `DOR1` register.</div>
    #[inline(always)]
    pub const fn dor(&self, n: usize) -> &DOR {
        &self.dor[n]
    }
    ///Iterator for array of:
    ///0x2c..0x34 - channel%s data output register
    #[inline(always)]
    pub fn dor_iter(&self) -> impl Iterator<Item = &DOR> {
        self.dor.iter()
    }
    ///0x2c - channel1 data output register
    #[inline(always)]
    pub const fn dor1(&self) -> &DOR {
        self.dor(0)
    }
    ///0x30 - channel2 data output register
    #[inline(always)]
    pub const fn dor2(&self) -> &DOR {
        self.dor(1)
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
/**DHR12R (rw) register accessor: channel%s 12-bit right-aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12R[1])

For information about available fields see [`mod@dhr12r`] module*/
pub type DHR12R = crate::Reg<dhr12r::DHR12Rrs>;
///channel%s 12-bit right-aligned data holding register
pub mod dhr12r;
/**DHR12L (rw) register accessor: channel%s 12-bit left aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr12l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR12L[1])

For information about available fields see [`mod@dhr12l`] module*/
pub type DHR12L = crate::Reg<dhr12l::DHR12Lrs>;
///channel%s 12-bit left aligned data holding register
pub mod dhr12l;
/**DHR8R (rw) register accessor: channel%s 8-bit right aligned data holding register

You can [`read`](crate::Reg::read) this register and get [`dhr8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DHR8R[1])

For information about available fields see [`mod@dhr8r`] module*/
pub type DHR8R = crate::Reg<dhr8r::DHR8Rrs>;
///channel%s 8-bit right aligned data holding register
pub mod dhr8r;
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
/**DOR (r) register accessor: channel%s data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:DOR[1])

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///channel%s data output register
pub mod dor;
/**SR (rw) register accessor: DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#DAC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///DAC status register
pub mod sr;
