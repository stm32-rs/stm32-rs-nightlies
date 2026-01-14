#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sr: SR,
    cr1: CR1,
    cr2: CR2,
    smpr1: SMPR1,
    smpr2: SMPR2,
    smpr3: SMPR3,
    jofr: [JOFR; 4],
    htr: HTR,
    ltr: LTR,
    sqr1: SQR1,
    sqr2: SQR2,
    sqr3: SQR3,
    sqr4: SQR4,
    sqr5: SQR5,
    jsqr: JSQR,
    jdr: [JDR; 4],
    dr: DR,
    smpr0: SMPR0,
    _reserved18: [u8; 0x02a0],
    csr: CSR,
    ccr: CCR,
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
    ///0x14 - sample time register 3
    #[inline(always)]
    pub const fn smpr3(&self) -> &SMPR3 {
        &self.smpr3
    }
    ///0x18..0x28 - injected channel data offset register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `JOFR1` register.</div>
    #[inline(always)]
    pub const fn jofr(&self, n: usize) -> &JOFR {
        &self.jofr[n]
    }
    ///Iterator for array of:
    ///0x18..0x28 - injected channel data offset register %s
    #[inline(always)]
    pub fn jofr_iter(&self) -> impl Iterator<Item = &JOFR> {
        self.jofr.iter()
    }
    ///0x18 - injected channel data offset register 1
    #[inline(always)]
    pub const fn jofr1(&self) -> &JOFR {
        self.jofr(0)
    }
    ///0x1c - injected channel data offset register 2
    #[inline(always)]
    pub const fn jofr2(&self) -> &JOFR {
        self.jofr(1)
    }
    ///0x20 - injected channel data offset register 3
    #[inline(always)]
    pub const fn jofr3(&self) -> &JOFR {
        self.jofr(2)
    }
    ///0x24 - injected channel data offset register 4
    #[inline(always)]
    pub const fn jofr4(&self) -> &JOFR {
        self.jofr(3)
    }
    ///0x28 - watchdog higher threshold register
    #[inline(always)]
    pub const fn htr(&self) -> &HTR {
        &self.htr
    }
    ///0x2c - watchdog lower threshold register
    #[inline(always)]
    pub const fn ltr(&self) -> &LTR {
        &self.ltr
    }
    ///0x30 - regular sequence register 1
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    ///0x34 - regular sequence register 2
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    ///0x38 - regular sequence register 3
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    ///0x3c - regular sequence register 4
    #[inline(always)]
    pub const fn sqr4(&self) -> &SQR4 {
        &self.sqr4
    }
    ///0x40 - regular sequence register 5
    #[inline(always)]
    pub const fn sqr5(&self) -> &SQR5 {
        &self.sqr5
    }
    ///0x44 - injected sequence register
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    ///0x48..0x58 - injected data register x
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `JDR1` register.</div>
    #[inline(always)]
    pub const fn jdr(&self, n: usize) -> &JDR {
        &self.jdr[n]
    }
    ///Iterator for array of:
    ///0x48..0x58 - injected data register x
    #[inline(always)]
    pub fn jdr_iter(&self) -> impl Iterator<Item = &JDR> {
        self.jdr.iter()
    }
    ///0x48 - injected data register x
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR {
        self.jdr(0)
    }
    ///0x4c - injected data register x
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR {
        self.jdr(1)
    }
    ///0x50 - injected data register x
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR {
        self.jdr(2)
    }
    ///0x54 - injected data register x
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR {
        self.jdr(3)
    }
    ///0x58 - regular data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x5c - sample time register 0
    #[inline(always)]
    pub const fn smpr0(&self) -> &SMPR0 {
        &self.smpr0
    }
    ///0x300 - ADC common status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x304 - ADC common control register
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
}
/**SR (rw) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**CR1 (rw) register accessor: control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:CR1)

For information about available fields see [`mod@cr1`] module*/
pub type CR1 = crate::Reg<cr1::CR1rs>;
///control register 1
pub mod cr1;
/**CR2 (rw) register accessor: control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:CR2)

For information about available fields see [`mod@cr2`] module*/
pub type CR2 = crate::Reg<cr2::CR2rs>;
///control register 2
pub mod cr2;
/**SMPR1 (rw) register accessor: sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SMPR1)

For information about available fields see [`mod@smpr1`] module*/
pub type SMPR1 = crate::Reg<smpr1::SMPR1rs>;
///sample time register 1
pub mod smpr1;
/**SMPR2 (rw) register accessor: sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SMPR2)

For information about available fields see [`mod@smpr2`] module*/
pub type SMPR2 = crate::Reg<smpr2::SMPR2rs>;
///sample time register 2
pub mod smpr2;
/**SMPR3 (rw) register accessor: sample time register 3

You can [`read`](crate::Reg::read) this register and get [`smpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SMPR3)

For information about available fields see [`mod@smpr3`] module*/
pub type SMPR3 = crate::Reg<smpr3::SMPR3rs>;
///sample time register 3
pub mod smpr3;
/**JOFR (rw) register accessor: injected channel data offset register %s

You can [`read`](crate::Reg::read) this register and get [`jofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:JOFR[1])

For information about available fields see [`mod@jofr`] module*/
pub type JOFR = crate::Reg<jofr::JOFRrs>;
///injected channel data offset register %s
pub mod jofr;
/**HTR (rw) register accessor: watchdog higher threshold register

You can [`read`](crate::Reg::read) this register and get [`htr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:HTR)

For information about available fields see [`mod@htr`] module*/
pub type HTR = crate::Reg<htr::HTRrs>;
///watchdog higher threshold register
pub mod htr;
/**LTR (rw) register accessor: watchdog lower threshold register

You can [`read`](crate::Reg::read) this register and get [`ltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:LTR)

For information about available fields see [`mod@ltr`] module*/
pub type LTR = crate::Reg<ltr::LTRrs>;
///watchdog lower threshold register
pub mod ltr;
/**SQR1 (rw) register accessor: regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SQR1)

For information about available fields see [`mod@sqr1`] module*/
pub type SQR1 = crate::Reg<sqr1::SQR1rs>;
///regular sequence register 1
pub mod sqr1;
/**SQR2 (rw) register accessor: regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SQR2)

For information about available fields see [`mod@sqr2`] module*/
pub type SQR2 = crate::Reg<sqr2::SQR2rs>;
///regular sequence register 2
pub mod sqr2;
/**SQR3 (rw) register accessor: regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SQR3)

For information about available fields see [`mod@sqr3`] module*/
pub type SQR3 = crate::Reg<sqr3::SQR3rs>;
///regular sequence register 3
pub mod sqr3;
/**SQR4 (rw) register accessor: regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SQR4)

For information about available fields see [`mod@sqr4`] module*/
pub type SQR4 = crate::Reg<sqr4::SQR4rs>;
///regular sequence register 4
pub mod sqr4;
/**SQR5 (rw) register accessor: regular sequence register 5

You can [`read`](crate::Reg::read) this register and get [`sqr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SQR5)

For information about available fields see [`mod@sqr5`] module*/
pub type SQR5 = crate::Reg<sqr5::SQR5rs>;
///regular sequence register 5
pub mod sqr5;
/**JSQR (rw) register accessor: injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:JSQR)

For information about available fields see [`mod@jsqr`] module*/
pub type JSQR = crate::Reg<jsqr::JSQRrs>;
///injected sequence register
pub mod jsqr;
/**JDR (r) register accessor: injected data register x

You can [`read`](crate::Reg::read) this register and get [`jdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:JDR[1])

For information about available fields see [`mod@jdr`] module*/
pub type JDR = crate::Reg<jdr::JDRrs>;
///injected data register x
pub mod jdr;
/**DR (r) register accessor: regular data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///regular data register
pub mod dr;
/**SMPR0 (rw) register accessor: sample time register 0

You can [`read`](crate::Reg::read) this register and get [`smpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:SMPR0)

For information about available fields see [`mod@smpr0`] module*/
pub type SMPR0 = crate::Reg<smpr0::SMPR0rs>;
///sample time register 0
pub mod smpr0;
/**CSR (r) register accessor: ADC common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///ADC common status register
pub mod csr;
/**CCR (rw) register accessor: ADC common control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#ADC:CCR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///ADC common control register
pub mod ccr;
