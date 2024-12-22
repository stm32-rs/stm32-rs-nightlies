#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    aes_cr: AES_CR,
    aes_sr: AES_SR,
    aes_dinr: AES_DINR,
    aes_doutr: AES_DOUTR,
    aes_keyr0: AES_KEYR0,
    aes_keyr1: AES_KEYR1,
    aes_keyr2: AES_KEYR2,
    aes_keyr3: AES_KEYR3,
    aes_ivr0: AES_IVR0,
    aes_ivr1: AES_IVR1,
    aes_ivr2: AES_IVR2,
    aes_ivr3: AES_IVR3,
    aes_keyr4: AES_KEYR4,
    aes_keyr5: AES_KEYR5,
    aes_keyr6: AES_KEYR6,
    aes_keyr7: AES_KEYR7,
    aes_susp0r: AES_SUSP0R,
    aes_susp1r: AES_SUSP1R,
    aes_susp2r: AES_SUSP2R,
    aes_susp3r: AES_SUSP3R,
    aes_susp4r: AES_SUSP4R,
    aes_susp5r: AES_SUSP5R,
    aes_susp6r: AES_SUSP6R,
    aes_susp7r: AES_SUSP7R,
}
impl RegisterBlock {
    ///0x00 - AES control register
    #[inline(always)]
    pub const fn aes_cr(&self) -> &AES_CR {
        &self.aes_cr
    }
    ///0x04 - AES control register
    #[inline(always)]
    pub const fn aes_sr(&self) -> &AES_SR {
        &self.aes_sr
    }
    ///0x08 - AES data input register
    #[inline(always)]
    pub const fn aes_dinr(&self) -> &AES_DINR {
        &self.aes_dinr
    }
    ///0x0c - AES data output register
    #[inline(always)]
    pub const fn aes_doutr(&self) -> &AES_DOUTR {
        &self.aes_doutr
    }
    ///0x10 - AES key register 0
    #[inline(always)]
    pub const fn aes_keyr0(&self) -> &AES_KEYR0 {
        &self.aes_keyr0
    }
    ///0x14 - AES key register 1
    #[inline(always)]
    pub const fn aes_keyr1(&self) -> &AES_KEYR1 {
        &self.aes_keyr1
    }
    ///0x18 - AES key register 2
    #[inline(always)]
    pub const fn aes_keyr2(&self) -> &AES_KEYR2 {
        &self.aes_keyr2
    }
    ///0x1c - AES key register 3
    #[inline(always)]
    pub const fn aes_keyr3(&self) -> &AES_KEYR3 {
        &self.aes_keyr3
    }
    ///0x20 - AES initialization vector register 0
    #[inline(always)]
    pub const fn aes_ivr0(&self) -> &AES_IVR0 {
        &self.aes_ivr0
    }
    ///0x24 - AES initialization vector register 1
    #[inline(always)]
    pub const fn aes_ivr1(&self) -> &AES_IVR1 {
        &self.aes_ivr1
    }
    ///0x28 - AES initialization vector register 2
    #[inline(always)]
    pub const fn aes_ivr2(&self) -> &AES_IVR2 {
        &self.aes_ivr2
    }
    ///0x2c - AES initialization vector register 3
    #[inline(always)]
    pub const fn aes_ivr3(&self) -> &AES_IVR3 {
        &self.aes_ivr3
    }
    ///0x30 - AES key register 4
    #[inline(always)]
    pub const fn aes_keyr4(&self) -> &AES_KEYR4 {
        &self.aes_keyr4
    }
    ///0x34 - AES key register 5
    #[inline(always)]
    pub const fn aes_keyr5(&self) -> &AES_KEYR5 {
        &self.aes_keyr5
    }
    ///0x38 - AES key register 6
    #[inline(always)]
    pub const fn aes_keyr6(&self) -> &AES_KEYR6 {
        &self.aes_keyr6
    }
    ///0x3c - AES key register 7
    #[inline(always)]
    pub const fn aes_keyr7(&self) -> &AES_KEYR7 {
        &self.aes_keyr7
    }
    ///0x40 - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp0r(&self) -> &AES_SUSP0R {
        &self.aes_susp0r
    }
    ///0x44 - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp1r(&self) -> &AES_SUSP1R {
        &self.aes_susp1r
    }
    ///0x48 - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp2r(&self) -> &AES_SUSP2R {
        &self.aes_susp2r
    }
    ///0x4c - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp3r(&self) -> &AES_SUSP3R {
        &self.aes_susp3r
    }
    ///0x50 - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp4r(&self) -> &AES_SUSP4R {
        &self.aes_susp4r
    }
    ///0x54 - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp5r(&self) -> &AES_SUSP5R {
        &self.aes_susp5r
    }
    ///0x58 - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp6r(&self) -> &AES_SUSP6R {
        &self.aes_susp6r
    }
    ///0x5c - AES suspend registers
    #[inline(always)]
    pub const fn aes_susp7r(&self) -> &AES_SUSP7R {
        &self.aes_susp7r
    }
}
/**AES_CR (rw) register accessor: AES control register

You can [`read`](crate::Reg::read) this register and get [`aes_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_CR)

For information about available fields see [`mod@aes_cr`]
module*/
pub type AES_CR = crate::Reg<aes_cr::AES_CRrs>;
///AES control register
pub mod aes_cr;
/**AES_SR (r) register accessor: AES control register

You can [`read`](crate::Reg::read) this register and get [`aes_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SR)

For information about available fields see [`mod@aes_sr`]
module*/
pub type AES_SR = crate::Reg<aes_sr::AES_SRrs>;
///AES control register
pub mod aes_sr;
/**AES_DINR (rw) register accessor: AES data input register

You can [`read`](crate::Reg::read) this register and get [`aes_dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_DINR)

For information about available fields see [`mod@aes_dinr`]
module*/
pub type AES_DINR = crate::Reg<aes_dinr::AES_DINRrs>;
///AES data input register
pub mod aes_dinr;
/**AES_DOUTR (r) register accessor: AES data output register

You can [`read`](crate::Reg::read) this register and get [`aes_doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_DOUTR)

For information about available fields see [`mod@aes_doutr`]
module*/
pub type AES_DOUTR = crate::Reg<aes_doutr::AES_DOUTRrs>;
///AES data output register
pub mod aes_doutr;
/**AES_KEYR0 (rw) register accessor: AES key register 0

You can [`read`](crate::Reg::read) this register and get [`aes_keyr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR0)

For information about available fields see [`mod@aes_keyr0`]
module*/
pub type AES_KEYR0 = crate::Reg<aes_keyr0::AES_KEYR0rs>;
///AES key register 0
pub mod aes_keyr0;
/**AES_KEYR1 (rw) register accessor: AES key register 1

You can [`read`](crate::Reg::read) this register and get [`aes_keyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR1)

For information about available fields see [`mod@aes_keyr1`]
module*/
pub type AES_KEYR1 = crate::Reg<aes_keyr1::AES_KEYR1rs>;
///AES key register 1
pub mod aes_keyr1;
/**AES_KEYR2 (rw) register accessor: AES key register 2

You can [`read`](crate::Reg::read) this register and get [`aes_keyr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR2)

For information about available fields see [`mod@aes_keyr2`]
module*/
pub type AES_KEYR2 = crate::Reg<aes_keyr2::AES_KEYR2rs>;
///AES key register 2
pub mod aes_keyr2;
/**AES_KEYR3 (rw) register accessor: AES key register 3

You can [`read`](crate::Reg::read) this register and get [`aes_keyr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR3)

For information about available fields see [`mod@aes_keyr3`]
module*/
pub type AES_KEYR3 = crate::Reg<aes_keyr3::AES_KEYR3rs>;
///AES key register 3
pub mod aes_keyr3;
/**AES_IVR0 (rw) register accessor: AES initialization vector register 0

You can [`read`](crate::Reg::read) this register and get [`aes_ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_IVR0)

For information about available fields see [`mod@aes_ivr0`]
module*/
pub type AES_IVR0 = crate::Reg<aes_ivr0::AES_IVR0rs>;
///AES initialization vector register 0
pub mod aes_ivr0;
/**AES_IVR1 (rw) register accessor: AES initialization vector register 1

You can [`read`](crate::Reg::read) this register and get [`aes_ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_IVR1)

For information about available fields see [`mod@aes_ivr1`]
module*/
pub type AES_IVR1 = crate::Reg<aes_ivr1::AES_IVR1rs>;
///AES initialization vector register 1
pub mod aes_ivr1;
/**AES_IVR2 (rw) register accessor: AES initialization vector register 2

You can [`read`](crate::Reg::read) this register and get [`aes_ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_IVR2)

For information about available fields see [`mod@aes_ivr2`]
module*/
pub type AES_IVR2 = crate::Reg<aes_ivr2::AES_IVR2rs>;
///AES initialization vector register 2
pub mod aes_ivr2;
/**AES_IVR3 (rw) register accessor: AES initialization vector register 3

You can [`read`](crate::Reg::read) this register and get [`aes_ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_IVR3)

For information about available fields see [`mod@aes_ivr3`]
module*/
pub type AES_IVR3 = crate::Reg<aes_ivr3::AES_IVR3rs>;
///AES initialization vector register 3
pub mod aes_ivr3;
/**AES_KEYR4 (rw) register accessor: AES key register 4

You can [`read`](crate::Reg::read) this register and get [`aes_keyr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR4)

For information about available fields see [`mod@aes_keyr4`]
module*/
pub type AES_KEYR4 = crate::Reg<aes_keyr4::AES_KEYR4rs>;
///AES key register 4
pub mod aes_keyr4;
/**AES_KEYR5 (rw) register accessor: AES key register 5

You can [`read`](crate::Reg::read) this register and get [`aes_keyr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR5)

For information about available fields see [`mod@aes_keyr5`]
module*/
pub type AES_KEYR5 = crate::Reg<aes_keyr5::AES_KEYR5rs>;
///AES key register 5
pub mod aes_keyr5;
/**AES_KEYR6 (rw) register accessor: AES key register 6

You can [`read`](crate::Reg::read) this register and get [`aes_keyr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR6)

For information about available fields see [`mod@aes_keyr6`]
module*/
pub type AES_KEYR6 = crate::Reg<aes_keyr6::AES_KEYR6rs>;
///AES key register 6
pub mod aes_keyr6;
/**AES_KEYR7 (rw) register accessor: AES key register 7

You can [`read`](crate::Reg::read) this register and get [`aes_keyr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_KEYR7)

For information about available fields see [`mod@aes_keyr7`]
module*/
pub type AES_KEYR7 = crate::Reg<aes_keyr7::AES_KEYR7rs>;
///AES key register 7
pub mod aes_keyr7;
/**AES_SUSP0R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP0R)

For information about available fields see [`mod@aes_susp0r`]
module*/
pub type AES_SUSP0R = crate::Reg<aes_susp0r::AES_SUSP0Rrs>;
///AES suspend registers
pub mod aes_susp0r;
/**AES_SUSP1R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP1R)

For information about available fields see [`mod@aes_susp1r`]
module*/
pub type AES_SUSP1R = crate::Reg<aes_susp1r::AES_SUSP1Rrs>;
///AES suspend registers
pub mod aes_susp1r;
/**AES_SUSP2R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP2R)

For information about available fields see [`mod@aes_susp2r`]
module*/
pub type AES_SUSP2R = crate::Reg<aes_susp2r::AES_SUSP2Rrs>;
///AES suspend registers
pub mod aes_susp2r;
/**AES_SUSP3R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP3R)

For information about available fields see [`mod@aes_susp3r`]
module*/
pub type AES_SUSP3R = crate::Reg<aes_susp3r::AES_SUSP3Rrs>;
///AES suspend registers
pub mod aes_susp3r;
/**AES_SUSP4R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP4R)

For information about available fields see [`mod@aes_susp4r`]
module*/
pub type AES_SUSP4R = crate::Reg<aes_susp4r::AES_SUSP4Rrs>;
///AES suspend registers
pub mod aes_susp4r;
/**AES_SUSP5R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP5R)

For information about available fields see [`mod@aes_susp5r`]
module*/
pub type AES_SUSP5R = crate::Reg<aes_susp5r::AES_SUSP5Rrs>;
///AES suspend registers
pub mod aes_susp5r;
/**AES_SUSP6R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP6R)

For information about available fields see [`mod@aes_susp6r`]
module*/
pub type AES_SUSP6R = crate::Reg<aes_susp6r::AES_SUSP6Rrs>;
///AES suspend registers
pub mod aes_susp6r;
/**AES_SUSP7R (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_susp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_susp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:AES_SUSP7R)

For information about available fields see [`mod@aes_susp7r`]
module*/
pub type AES_SUSP7R = crate::Reg<aes_susp7r::AES_SUSP7Rrs>;
///AES suspend registers
pub mod aes_susp7r;
