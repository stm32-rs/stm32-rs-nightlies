#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dinr: DINR,
    doutr: DOUTR,
    keyr0: KEYR0,
    keyr1: KEYR1,
    keyr2: KEYR2,
    keyr3: KEYR3,
    ivr0: IVR0,
    ivr1: IVR1,
    ivr2: IVR2,
    ivr3: IVR3,
}
impl RegisterBlock {
    ///0x00 - AES_CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - AES_SR register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - AES_DINR register
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    ///0x0c - AES_DOUTR register
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    ///0x10 - AES_KEYRx register
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    ///0x14 - AES_KEYRx register
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    ///0x18 - AES_KEYRx register
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    ///0x1c - AES_KEYRx register
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    ///0x20 - AES_IVRx register
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    ///0x24 - AES_IVRx register
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    ///0x28 - AES_IVRx register
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    ///0x2c - AES_IVRx register
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
}
/**CR (rw) register accessor: AES_CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///AES_CR register
pub mod cr;
/**SR (r) register accessor: AES_SR register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///AES_SR register
pub mod sr;
/**DINR (rw) register accessor: AES_DINR register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:DINR)

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///AES_DINR register
pub mod dinr;
/**DOUTR (r) register accessor: AES_DOUTR register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:DOUTR)

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///AES_DOUTR register
pub mod doutr;
/**KEYR0 (rw) register accessor: AES_KEYRx register

You can [`read`](crate::Reg::read) this register and get [`keyr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:KEYR0)

For information about available fields see [`mod@keyr0`] module*/
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
///AES_KEYRx register
pub mod keyr0;
/**KEYR1 (rw) register accessor: AES_KEYRx register

You can [`read`](crate::Reg::read) this register and get [`keyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:KEYR1)

For information about available fields see [`mod@keyr1`] module*/
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
///AES_KEYRx register
pub mod keyr1;
/**KEYR2 (rw) register accessor: AES_KEYRx register

You can [`read`](crate::Reg::read) this register and get [`keyr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:KEYR2)

For information about available fields see [`mod@keyr2`] module*/
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
///AES_KEYRx register
pub mod keyr2;
/**KEYR3 (rw) register accessor: AES_KEYRx register

You can [`read`](crate::Reg::read) this register and get [`keyr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:KEYR3)

For information about available fields see [`mod@keyr3`] module*/
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
///AES_KEYRx register
pub mod keyr3;
/**IVR0 (rw) register accessor: AES_IVRx register

You can [`read`](crate::Reg::read) this register and get [`ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:IVR0)

For information about available fields see [`mod@ivr0`] module*/
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
///AES_IVRx register
pub mod ivr0;
/**IVR1 (rw) register accessor: AES_IVRx register

You can [`read`](crate::Reg::read) this register and get [`ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:IVR1)

For information about available fields see [`mod@ivr1`] module*/
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
///AES_IVRx register
pub mod ivr1;
/**IVR2 (rw) register accessor: AES_IVRx register

You can [`read`](crate::Reg::read) this register and get [`ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:IVR2)

For information about available fields see [`mod@ivr2`] module*/
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
///AES_IVRx register
pub mod ivr2;
/**IVR3 (rw) register accessor: AES_IVRx register

You can [`read`](crate::Reg::read) this register and get [`ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#AES:IVR3)

For information about available fields see [`mod@ivr3`] module*/
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
///AES_IVRx register
pub mod ivr3;
