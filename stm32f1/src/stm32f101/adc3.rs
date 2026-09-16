#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sr: SR,
    cr1: CR1,
    cr2: CR2,
    smpr1: SMPR1,
    smpr2: SMPR2,
    jofr: [JOFR; 4],
    htr: HTR,
    ltr: LTR,
    sqr1: SQR1,
    sqr2: SQR2,
    sqr3: SQR3,
    jsqr: JSQR,
    jdr: [JDR; 4],
    dr: DR,
}
impl RegisterBlock {
    ///0x00 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x04 - control register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    ///0x08 - control register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    ///0x0c - sample time register 1
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    ///0x10 - sample time register 2
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    ///0x14..0x24 - injected channel data offset register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `JOFR1` register.</div>
    #[inline(always)]
    pub const fn jofr(&self, n: usize) -> &JOFR {
        &self.jofr[n]
    }
    ///Iterator for array of:
    ///0x14..0x24 - injected channel data offset register %s
    #[inline(always)]
    pub fn jofr_iter(&self) -> impl Iterator<Item = &JOFR> {
        self.jofr.iter()
    }
    ///0x14 - injected channel data offset register 1
    #[inline(always)]
    pub const fn jofr1(&self) -> &JOFR {
        self.jofr(0)
    }
    ///0x18 - injected channel data offset register 2
    #[inline(always)]
    pub const fn jofr2(&self) -> &JOFR {
        self.jofr(1)
    }
    ///0x1c - injected channel data offset register 3
    #[inline(always)]
    pub const fn jofr3(&self) -> &JOFR {
        self.jofr(2)
    }
    ///0x20 - injected channel data offset register 4
    #[inline(always)]
    pub const fn jofr4(&self) -> &JOFR {
        self.jofr(3)
    }
    ///0x24 - watchdog higher threshold register
    #[inline(always)]
    pub const fn htr(&self) -> &HTR {
        &self.htr
    }
    ///0x28 - watchdog lower threshold register
    #[inline(always)]
    pub const fn ltr(&self) -> &LTR {
        &self.ltr
    }
    ///0x2c - regular sequence register 1
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    ///0x30 - regular sequence register 2
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    ///0x34 - regular sequence register 3
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    ///0x38 - injected sequence register
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    ///0x3c..0x4c - injected data register x
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `JDR1` register.</div>
    #[inline(always)]
    pub const fn jdr(&self, n: usize) -> &JDR {
        &self.jdr[n]
    }
    ///Iterator for array of:
    ///0x3c..0x4c - injected data register x
    #[inline(always)]
    pub fn jdr_iter(&self) -> impl Iterator<Item = &JDR> {
        self.jdr.iter()
    }
    ///0x3c - injected data register x
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR {
        self.jdr(0)
    }
    ///0x40 - injected data register x
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR {
        self.jdr(1)
    }
    ///0x44 - injected data register x
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR {
        self.jdr(2)
    }
    ///0x48 - injected data register x
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR {
        self.jdr(3)
    }
    ///0x4c - regular data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
}
pub use crate::stm32f101::adc1::sr;
pub use crate::stm32f101::adc1::SR;
/**CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC3:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///control register 1
pub mod cr1;
/**CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC3:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///control register 2
pub mod cr2;
pub use crate::stm32f101::adc1::htr;
pub use crate::stm32f101::adc1::jdr;
pub use crate::stm32f101::adc1::jofr;
pub use crate::stm32f101::adc1::jsqr;
pub use crate::stm32f101::adc1::ltr;
pub use crate::stm32f101::adc1::smpr1;
pub use crate::stm32f101::adc1::smpr2;
pub use crate::stm32f101::adc1::sqr1;
pub use crate::stm32f101::adc1::sqr2;
pub use crate::stm32f101::adc1::sqr3;
pub use crate::stm32f101::adc1::HTR;
pub use crate::stm32f101::adc1::JDR;
pub use crate::stm32f101::adc1::JOFR;
pub use crate::stm32f101::adc1::JSQR;
pub use crate::stm32f101::adc1::LTR;
pub use crate::stm32f101::adc1::SMPR1;
pub use crate::stm32f101::adc1::SMPR2;
pub use crate::stm32f101::adc1::SQR1;
pub use crate::stm32f101::adc1::SQR2;
pub use crate::stm32f101::adc1::SQR3;
/**DR (r) register accessor: regular data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC3:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///regular data register
pub mod dr;
