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
    aes_suspr0: AES_SUSPR0,
    aes_suspr1: AES_SUSPR1,
    aes_suspr2: AES_SUSPR2,
    aes_suspr3: AES_SUSPR3,
    aes_suspr4: AES_SUSPR4,
    aes_suspr5: AES_SUSPR5,
    aes_suspr6: AES_SUSPR6,
    aes_suspr7: AES_SUSPR7,
    _reserved24: [u8; 0x02a0],
    aes_ier: AES_IER,
    aes_isr: AES_ISR,
    aes_icr: AES_ICR,
}
impl RegisterBlock {
    ///0x00 - AES control register
    #[inline(always)]
    pub const fn aes_cr(&self) -> &AES_CR {
        &self.aes_cr
    }
    ///0x04 - AES status register
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
    pub const fn aes_suspr0(&self) -> &AES_SUSPR0 {
        &self.aes_suspr0
    }
    ///0x44 - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr1(&self) -> &AES_SUSPR1 {
        &self.aes_suspr1
    }
    ///0x48 - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr2(&self) -> &AES_SUSPR2 {
        &self.aes_suspr2
    }
    ///0x4c - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr3(&self) -> &AES_SUSPR3 {
        &self.aes_suspr3
    }
    ///0x50 - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr4(&self) -> &AES_SUSPR4 {
        &self.aes_suspr4
    }
    ///0x54 - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr5(&self) -> &AES_SUSPR5 {
        &self.aes_suspr5
    }
    ///0x58 - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr6(&self) -> &AES_SUSPR6 {
        &self.aes_suspr6
    }
    ///0x5c - AES suspend registers
    #[inline(always)]
    pub const fn aes_suspr7(&self) -> &AES_SUSPR7 {
        &self.aes_suspr7
    }
    ///0x300 - AES interrupt enable register
    #[inline(always)]
    pub const fn aes_ier(&self) -> &AES_IER {
        &self.aes_ier
    }
    ///0x304 - AES interrupt status register
    #[inline(always)]
    pub const fn aes_isr(&self) -> &AES_ISR {
        &self.aes_isr
    }
    ///0x308 - AES interrupt clear register
    #[inline(always)]
    pub const fn aes_icr(&self) -> &AES_ICR {
        &self.aes_icr
    }
}
/**AES_CR (rw) register accessor: AES control register

You can [`read`](crate::Reg::read) this register and get [`aes_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_CR)

For information about available fields see [`mod@aes_cr`]
module*/
pub type AES_CR = crate::Reg<aes_cr::AES_CRrs>;
///AES control register
pub mod aes_cr;
/**AES_SR (r) register accessor: AES status register

You can [`read`](crate::Reg::read) this register and get [`aes_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SR)

For information about available fields see [`mod@aes_sr`]
module*/
pub type AES_SR = crate::Reg<aes_sr::AES_SRrs>;
///AES status register
pub mod aes_sr;
/**AES_DINR (w) register accessor: AES data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_dinr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_DINR)

For information about available fields see [`mod@aes_dinr`]
module*/
pub type AES_DINR = crate::Reg<aes_dinr::AES_DINRrs>;
///AES data input register
pub mod aes_dinr;
/**AES_DOUTR (r) register accessor: AES data output register

You can [`read`](crate::Reg::read) this register and get [`aes_doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_DOUTR)

For information about available fields see [`mod@aes_doutr`]
module*/
pub type AES_DOUTR = crate::Reg<aes_doutr::AES_DOUTRrs>;
///AES data output register
pub mod aes_doutr;
/**AES_KEYR0 (w) register accessor: AES key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR0)

For information about available fields see [`mod@aes_keyr0`]
module*/
pub type AES_KEYR0 = crate::Reg<aes_keyr0::AES_KEYR0rs>;
///AES key register 0
pub mod aes_keyr0;
/**AES_KEYR1 (w) register accessor: AES key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR1)

For information about available fields see [`mod@aes_keyr1`]
module*/
pub type AES_KEYR1 = crate::Reg<aes_keyr1::AES_KEYR1rs>;
///AES key register 1
pub mod aes_keyr1;
/**AES_KEYR2 (w) register accessor: AES key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR2)

For information about available fields see [`mod@aes_keyr2`]
module*/
pub type AES_KEYR2 = crate::Reg<aes_keyr2::AES_KEYR2rs>;
///AES key register 2
pub mod aes_keyr2;
/**AES_KEYR3 (w) register accessor: AES key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR3)

For information about available fields see [`mod@aes_keyr3`]
module*/
pub type AES_KEYR3 = crate::Reg<aes_keyr3::AES_KEYR3rs>;
///AES key register 3
pub mod aes_keyr3;
/**AES_IVR0 (rw) register accessor: AES initialization vector register 0

You can [`read`](crate::Reg::read) this register and get [`aes_ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_IVR0)

For information about available fields see [`mod@aes_ivr0`]
module*/
pub type AES_IVR0 = crate::Reg<aes_ivr0::AES_IVR0rs>;
///AES initialization vector register 0
pub mod aes_ivr0;
/**AES_IVR1 (rw) register accessor: AES initialization vector register 1

You can [`read`](crate::Reg::read) this register and get [`aes_ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_IVR1)

For information about available fields see [`mod@aes_ivr1`]
module*/
pub type AES_IVR1 = crate::Reg<aes_ivr1::AES_IVR1rs>;
///AES initialization vector register 1
pub mod aes_ivr1;
/**AES_IVR2 (rw) register accessor: AES initialization vector register 2

You can [`read`](crate::Reg::read) this register and get [`aes_ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_IVR2)

For information about available fields see [`mod@aes_ivr2`]
module*/
pub type AES_IVR2 = crate::Reg<aes_ivr2::AES_IVR2rs>;
///AES initialization vector register 2
pub mod aes_ivr2;
/**AES_IVR3 (rw) register accessor: AES initialization vector register 3

You can [`read`](crate::Reg::read) this register and get [`aes_ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_IVR3)

For information about available fields see [`mod@aes_ivr3`]
module*/
pub type AES_IVR3 = crate::Reg<aes_ivr3::AES_IVR3rs>;
///AES initialization vector register 3
pub mod aes_ivr3;
/**AES_KEYR4 (w) register accessor: AES key register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR4)

For information about available fields see [`mod@aes_keyr4`]
module*/
pub type AES_KEYR4 = crate::Reg<aes_keyr4::AES_KEYR4rs>;
///AES key register 4
pub mod aes_keyr4;
/**AES_KEYR5 (w) register accessor: AES key register 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR5)

For information about available fields see [`mod@aes_keyr5`]
module*/
pub type AES_KEYR5 = crate::Reg<aes_keyr5::AES_KEYR5rs>;
///AES key register 5
pub mod aes_keyr5;
/**AES_KEYR6 (w) register accessor: AES key register 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR6)

For information about available fields see [`mod@aes_keyr6`]
module*/
pub type AES_KEYR6 = crate::Reg<aes_keyr6::AES_KEYR6rs>;
///AES key register 6
pub mod aes_keyr6;
/**AES_KEYR7 (w) register accessor: AES key register 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_keyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_KEYR7)

For information about available fields see [`mod@aes_keyr7`]
module*/
pub type AES_KEYR7 = crate::Reg<aes_keyr7::AES_KEYR7rs>;
///AES key register 7
pub mod aes_keyr7;
/**AES_SUSPR0 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR0)

For information about available fields see [`mod@aes_suspr0`]
module*/
pub type AES_SUSPR0 = crate::Reg<aes_suspr0::AES_SUSPR0rs>;
///AES suspend registers
pub mod aes_suspr0;
/**AES_SUSPR1 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR1)

For information about available fields see [`mod@aes_suspr1`]
module*/
pub type AES_SUSPR1 = crate::Reg<aes_suspr1::AES_SUSPR1rs>;
///AES suspend registers
pub mod aes_suspr1;
/**AES_SUSPR2 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR2)

For information about available fields see [`mod@aes_suspr2`]
module*/
pub type AES_SUSPR2 = crate::Reg<aes_suspr2::AES_SUSPR2rs>;
///AES suspend registers
pub mod aes_suspr2;
/**AES_SUSPR3 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR3)

For information about available fields see [`mod@aes_suspr3`]
module*/
pub type AES_SUSPR3 = crate::Reg<aes_suspr3::AES_SUSPR3rs>;
///AES suspend registers
pub mod aes_suspr3;
/**AES_SUSPR4 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR4)

For information about available fields see [`mod@aes_suspr4`]
module*/
pub type AES_SUSPR4 = crate::Reg<aes_suspr4::AES_SUSPR4rs>;
///AES suspend registers
pub mod aes_suspr4;
/**AES_SUSPR5 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR5)

For information about available fields see [`mod@aes_suspr5`]
module*/
pub type AES_SUSPR5 = crate::Reg<aes_suspr5::AES_SUSPR5rs>;
///AES suspend registers
pub mod aes_suspr5;
/**AES_SUSPR6 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR6)

For information about available fields see [`mod@aes_suspr6`]
module*/
pub type AES_SUSPR6 = crate::Reg<aes_suspr6::AES_SUSPR6rs>;
///AES suspend registers
pub mod aes_suspr6;
/**AES_SUSPR7 (rw) register accessor: AES suspend registers

You can [`read`](crate::Reg::read) this register and get [`aes_suspr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_suspr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_SUSPR7)

For information about available fields see [`mod@aes_suspr7`]
module*/
pub type AES_SUSPR7 = crate::Reg<aes_suspr7::AES_SUSPR7rs>;
///AES suspend registers
pub mod aes_suspr7;
/**AES_IER (rw) register accessor: AES interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`aes_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_IER)

For information about available fields see [`mod@aes_ier`]
module*/
pub type AES_IER = crate::Reg<aes_ier::AES_IERrs>;
///AES interrupt enable register
pub mod aes_ier;
/**AES_ISR (r) register accessor: AES interrupt status register

You can [`read`](crate::Reg::read) this register and get [`aes_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_ISR)

For information about available fields see [`mod@aes_isr`]
module*/
pub type AES_ISR = crate::Reg<aes_isr::AES_ISRrs>;
///AES interrupt status register
pub mod aes_isr;
/**AES_ICR (w) register accessor: AES interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#AES:AES_ICR)

For information about available fields see [`mod@aes_icr`]
module*/
pub type AES_ICR = crate::Reg<aes_icr::AES_ICRrs>;
///AES interrupt clear register
pub mod aes_icr;
