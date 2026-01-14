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
    keyr4: KEYR4,
    keyr5: KEYR5,
    keyr6: KEYR6,
    keyr7: KEYR7,
    susp0r: SUSP0R,
    susp1r: SUSP1R,
    susp2r: SUSP2R,
    susp3r: SUSP3R,
    susp4r: SUSP4R,
    susp5r: SUSP5R,
    susp6r: SUSP6R,
    susp7r: SUSP7R,
}
impl RegisterBlock {
    ///0x00 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - data input register
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    ///0x0c - data output register
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    ///0x10 - key register
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    ///0x14 - key register
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    ///0x18 - key register
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    ///0x1c - key register
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    ///0x20 - initialization vector register
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    ///0x24 - initialization vector register
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    ///0x28 - initialization vector register
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    ///0x2c - initialization vector register
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
    ///0x30 - key registers
    #[inline(always)]
    pub const fn keyr4(&self) -> &KEYR4 {
        &self.keyr4
    }
    ///0x34 - key registers
    #[inline(always)]
    pub const fn keyr5(&self) -> &KEYR5 {
        &self.keyr5
    }
    ///0x38 - key registers
    #[inline(always)]
    pub const fn keyr6(&self) -> &KEYR6 {
        &self.keyr6
    }
    ///0x3c - key registers
    #[inline(always)]
    pub const fn keyr7(&self) -> &KEYR7 {
        &self.keyr7
    }
    ///0x40 - Suspend registers
    #[inline(always)]
    pub const fn susp0r(&self) -> &SUSP0R {
        &self.susp0r
    }
    ///0x44 - Suspend registers
    #[inline(always)]
    pub const fn susp1r(&self) -> &SUSP1R {
        &self.susp1r
    }
    ///0x48 - Suspend registers
    #[inline(always)]
    pub const fn susp2r(&self) -> &SUSP2R {
        &self.susp2r
    }
    ///0x4c - Suspend registers
    #[inline(always)]
    pub const fn susp3r(&self) -> &SUSP3R {
        &self.susp3r
    }
    ///0x50 - Suspend registers
    #[inline(always)]
    pub const fn susp4r(&self) -> &SUSP4R {
        &self.susp4r
    }
    ///0x54 - Suspend registers
    #[inline(always)]
    pub const fn susp5r(&self) -> &SUSP5R {
        &self.susp5r
    }
    ///0x58 - Suspend registers
    #[inline(always)]
    pub const fn susp6r(&self) -> &SUSP6R {
        &self.susp6r
    }
    ///0x5c - Suspend registers
    #[inline(always)]
    pub const fn susp7r(&self) -> &SUSP7R {
        &self.susp7r
    }
}
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**SR (r) register accessor: status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///status register
pub mod sr;
/**DINR (rw) register accessor: data input register

You can [`read`](crate::Reg::read) this register and get [`dinr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:DINR)

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///data input register
pub mod dinr;
/**DOUTR (r) register accessor: data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:DOUTR)

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///data output register
pub mod doutr;
/**KEYR0 (rw) register accessor: key register

You can [`read`](crate::Reg::read) this register and get [`keyr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR0)

For information about available fields see [`mod@keyr0`] module*/
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
///key register
pub mod keyr0;
/**KEYR1 (rw) register accessor: key register

You can [`read`](crate::Reg::read) this register and get [`keyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR1)

For information about available fields see [`mod@keyr1`] module*/
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
///key register
pub mod keyr1;
/**KEYR2 (r) register accessor: key register

You can [`read`](crate::Reg::read) this register and get [`keyr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR2)

For information about available fields see [`mod@keyr2`] module*/
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
///key register
pub mod keyr2;
/**KEYR3 (r) register accessor: key register

You can [`read`](crate::Reg::read) this register and get [`keyr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR3)

For information about available fields see [`mod@keyr3`] module*/
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
///key register
pub mod keyr3;
/**IVR0 (rw) register accessor: initialization vector register

You can [`read`](crate::Reg::read) this register and get [`ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:IVR0)

For information about available fields see [`mod@ivr0`] module*/
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
///initialization vector register
pub mod ivr0;
/**IVR1 (rw) register accessor: initialization vector register

You can [`read`](crate::Reg::read) this register and get [`ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:IVR1)

For information about available fields see [`mod@ivr1`] module*/
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
///initialization vector register
pub mod ivr1;
/**IVR2 (rw) register accessor: initialization vector register

You can [`read`](crate::Reg::read) this register and get [`ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:IVR2)

For information about available fields see [`mod@ivr2`] module*/
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
///initialization vector register
pub mod ivr2;
/**IVR3 (rw) register accessor: initialization vector register

You can [`read`](crate::Reg::read) this register and get [`ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:IVR3)

For information about available fields see [`mod@ivr3`] module*/
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
///initialization vector register
pub mod ivr3;
/**KEYR4 (rw) register accessor: key registers

You can [`read`](crate::Reg::read) this register and get [`keyr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR4)

For information about available fields see [`mod@keyr4`] module*/
pub type KEYR4 = crate::Reg<keyr4::KEYR4rs>;
///key registers
pub mod keyr4;
/**KEYR5 (rw) register accessor: key registers

You can [`read`](crate::Reg::read) this register and get [`keyr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR5)

For information about available fields see [`mod@keyr5`] module*/
pub type KEYR5 = crate::Reg<keyr5::KEYR5rs>;
///key registers
pub mod keyr5;
/**KEYR6 (rw) register accessor: key registers

You can [`read`](crate::Reg::read) this register and get [`keyr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR6)

For information about available fields see [`mod@keyr6`] module*/
pub type KEYR6 = crate::Reg<keyr6::KEYR6rs>;
///key registers
pub mod keyr6;
/**KEYR7 (rw) register accessor: key registers

You can [`read`](crate::Reg::read) this register and get [`keyr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR7)

For information about available fields see [`mod@keyr7`] module*/
pub type KEYR7 = crate::Reg<keyr7::KEYR7rs>;
///key registers
pub mod keyr7;
/**SUSP0R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP0R)

For information about available fields see [`mod@susp0r`] module*/
pub type SUSP0R = crate::Reg<susp0r::SUSP0Rrs>;
///Suspend registers
pub mod susp0r;
/**SUSP1R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP1R)

For information about available fields see [`mod@susp1r`] module*/
pub type SUSP1R = crate::Reg<susp1r::SUSP1Rrs>;
///Suspend registers
pub mod susp1r;
/**SUSP2R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP2R)

For information about available fields see [`mod@susp2r`] module*/
pub type SUSP2R = crate::Reg<susp2r::SUSP2Rrs>;
///Suspend registers
pub mod susp2r;
/**SUSP3R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP3R)

For information about available fields see [`mod@susp3r`] module*/
pub type SUSP3R = crate::Reg<susp3r::SUSP3Rrs>;
///Suspend registers
pub mod susp3r;
/**SUSP4R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP4R)

For information about available fields see [`mod@susp4r`] module*/
pub type SUSP4R = crate::Reg<susp4r::SUSP4Rrs>;
///Suspend registers
pub mod susp4r;
/**SUSP5R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP5R)

For information about available fields see [`mod@susp5r`] module*/
pub type SUSP5R = crate::Reg<susp5r::SUSP5Rrs>;
///Suspend registers
pub mod susp5r;
/**SUSP6R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP6R)

For information about available fields see [`mod@susp6r`] module*/
pub type SUSP6R = crate::Reg<susp6r::SUSP6Rrs>;
///Suspend registers
pub mod susp6r;
/**SUSP7R (rw) register accessor: Suspend registers

You can [`read`](crate::Reg::read) this register and get [`susp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`susp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:SUSP7R)

For information about available fields see [`mod@susp7r`] module*/
pub type SUSP7R = crate::Reg<susp7r::SUSP7Rrs>;
///Suspend registers
pub mod susp7r;
