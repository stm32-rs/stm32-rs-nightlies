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
    suspr0: SUSPR0,
    suspr1: SUSPR1,
    suspr2: SUSPR2,
    suspr3: SUSPR3,
    suspr4: SUSPR4,
    suspr5: SUSPR5,
    suspr6: SUSPR6,
    suspr7: SUSPR7,
    _reserved24: [u8; 0x02a0],
    ier: IER,
    isr: ISR,
    icr: ICR,
}
impl RegisterBlock {
    ///0x00 - SAES control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - SAES status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x08 - SAES data input register
    #[inline(always)]
    pub const fn dinr(&self) -> &DINR {
        &self.dinr
    }
    ///0x0c - SAES data output register
    #[inline(always)]
    pub const fn doutr(&self) -> &DOUTR {
        &self.doutr
    }
    ///0x10 - SAES key register 0
    #[inline(always)]
    pub const fn keyr0(&self) -> &KEYR0 {
        &self.keyr0
    }
    ///0x14 - SAES key register 1
    #[inline(always)]
    pub const fn keyr1(&self) -> &KEYR1 {
        &self.keyr1
    }
    ///0x18 - SAES key register 2
    #[inline(always)]
    pub const fn keyr2(&self) -> &KEYR2 {
        &self.keyr2
    }
    ///0x1c - SAES key register 3
    #[inline(always)]
    pub const fn keyr3(&self) -> &KEYR3 {
        &self.keyr3
    }
    ///0x20 - SAES initialization vector register 0
    #[inline(always)]
    pub const fn ivr0(&self) -> &IVR0 {
        &self.ivr0
    }
    ///0x24 - SAES initialization vector register 1
    #[inline(always)]
    pub const fn ivr1(&self) -> &IVR1 {
        &self.ivr1
    }
    ///0x28 - SAES initialization vector register 2
    #[inline(always)]
    pub const fn ivr2(&self) -> &IVR2 {
        &self.ivr2
    }
    ///0x2c - SAES initialization vector register 3
    #[inline(always)]
    pub const fn ivr3(&self) -> &IVR3 {
        &self.ivr3
    }
    ///0x30 - SAES key register 4
    #[inline(always)]
    pub const fn keyr4(&self) -> &KEYR4 {
        &self.keyr4
    }
    ///0x34 - SAES key register 5
    #[inline(always)]
    pub const fn keyr5(&self) -> &KEYR5 {
        &self.keyr5
    }
    ///0x38 - SAES key register 6
    #[inline(always)]
    pub const fn keyr6(&self) -> &KEYR6 {
        &self.keyr6
    }
    ///0x3c - SAES key register 7
    #[inline(always)]
    pub const fn keyr7(&self) -> &KEYR7 {
        &self.keyr7
    }
    ///0x40 - SAES suspend registers
    #[inline(always)]
    pub const fn suspr0(&self) -> &SUSPR0 {
        &self.suspr0
    }
    ///0x44 - SAES suspend registers
    #[inline(always)]
    pub const fn suspr1(&self) -> &SUSPR1 {
        &self.suspr1
    }
    ///0x48 - SAES suspend registers
    #[inline(always)]
    pub const fn suspr2(&self) -> &SUSPR2 {
        &self.suspr2
    }
    ///0x4c - SAES suspend registers
    #[inline(always)]
    pub const fn suspr3(&self) -> &SUSPR3 {
        &self.suspr3
    }
    ///0x50 - SAES suspend registers
    #[inline(always)]
    pub const fn suspr4(&self) -> &SUSPR4 {
        &self.suspr4
    }
    ///0x54 - SAES suspend registers
    #[inline(always)]
    pub const fn suspr5(&self) -> &SUSPR5 {
        &self.suspr5
    }
    ///0x58 - SAES suspend registers
    #[inline(always)]
    pub const fn suspr6(&self) -> &SUSPR6 {
        &self.suspr6
    }
    ///0x5c - SAES suspend registers
    #[inline(always)]
    pub const fn suspr7(&self) -> &SUSPR7 {
        &self.suspr7
    }
    ///0x300 - SAES interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x304 - SAES interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x308 - SAES interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
}
/**CR (rw) register accessor: SAES control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///SAES control register
pub mod cr;
/**SR (r) register accessor: SAES status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///SAES status register
pub mod sr;
/**DINR (w) register accessor: SAES data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dinr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:DINR)

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///SAES data input register
pub mod dinr;
/**DOUTR (r) register accessor: SAES data output register

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:DOUTR)

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///SAES data output register
pub mod doutr;
/**KEYR0 (w) register accessor: SAES key register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR0)

For information about available fields see [`mod@keyr0`] module*/
pub type KEYR0 = crate::Reg<keyr0::KEYR0rs>;
///SAES key register 0
pub mod keyr0;
/**KEYR1 (w) register accessor: SAES key register 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR1)

For information about available fields see [`mod@keyr1`] module*/
pub type KEYR1 = crate::Reg<keyr1::KEYR1rs>;
///SAES key register 1
pub mod keyr1;
/**KEYR2 (w) register accessor: SAES key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR2)

For information about available fields see [`mod@keyr2`] module*/
pub type KEYR2 = crate::Reg<keyr2::KEYR2rs>;
///SAES key register 2
pub mod keyr2;
/**KEYR3 (w) register accessor: SAES key register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR3)

For information about available fields see [`mod@keyr3`] module*/
pub type KEYR3 = crate::Reg<keyr3::KEYR3rs>;
///SAES key register 3
pub mod keyr3;
/**IVR0 (rw) register accessor: SAES initialization vector register 0

You can [`read`](crate::Reg::read) this register and get [`ivr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:IVR0)

For information about available fields see [`mod@ivr0`] module*/
pub type IVR0 = crate::Reg<ivr0::IVR0rs>;
///SAES initialization vector register 0
pub mod ivr0;
/**IVR1 (rw) register accessor: SAES initialization vector register 1

You can [`read`](crate::Reg::read) this register and get [`ivr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:IVR1)

For information about available fields see [`mod@ivr1`] module*/
pub type IVR1 = crate::Reg<ivr1::IVR1rs>;
///SAES initialization vector register 1
pub mod ivr1;
/**IVR2 (rw) register accessor: SAES initialization vector register 2

You can [`read`](crate::Reg::read) this register and get [`ivr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:IVR2)

For information about available fields see [`mod@ivr2`] module*/
pub type IVR2 = crate::Reg<ivr2::IVR2rs>;
///SAES initialization vector register 2
pub mod ivr2;
/**IVR3 (rw) register accessor: SAES initialization vector register 3

You can [`read`](crate::Reg::read) this register and get [`ivr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:IVR3)

For information about available fields see [`mod@ivr3`] module*/
pub type IVR3 = crate::Reg<ivr3::IVR3rs>;
///SAES initialization vector register 3
pub mod ivr3;
/**KEYR4 (w) register accessor: SAES key register 4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR4)

For information about available fields see [`mod@keyr4`] module*/
pub type KEYR4 = crate::Reg<keyr4::KEYR4rs>;
///SAES key register 4
pub mod keyr4;
/**KEYR5 (w) register accessor: SAES key register 5

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR5)

For information about available fields see [`mod@keyr5`] module*/
pub type KEYR5 = crate::Reg<keyr5::KEYR5rs>;
///SAES key register 5
pub mod keyr5;
/**KEYR6 (w) register accessor: SAES key register 6

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR6)

For information about available fields see [`mod@keyr6`] module*/
pub type KEYR6 = crate::Reg<keyr6::KEYR6rs>;
///SAES key register 6
pub mod keyr6;
/**KEYR7 (w) register accessor: SAES key register 7

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:KEYR7)

For information about available fields see [`mod@keyr7`] module*/
pub type KEYR7 = crate::Reg<keyr7::KEYR7rs>;
///SAES key register 7
pub mod keyr7;
/**SUSPR0 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR0)

For information about available fields see [`mod@suspr0`] module*/
pub type SUSPR0 = crate::Reg<suspr0::SUSPR0rs>;
///SAES suspend registers
pub mod suspr0;
/**SUSPR1 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR1)

For information about available fields see [`mod@suspr1`] module*/
pub type SUSPR1 = crate::Reg<suspr1::SUSPR1rs>;
///SAES suspend registers
pub mod suspr1;
/**SUSPR2 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR2)

For information about available fields see [`mod@suspr2`] module*/
pub type SUSPR2 = crate::Reg<suspr2::SUSPR2rs>;
///SAES suspend registers
pub mod suspr2;
/**SUSPR3 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR3)

For information about available fields see [`mod@suspr3`] module*/
pub type SUSPR3 = crate::Reg<suspr3::SUSPR3rs>;
///SAES suspend registers
pub mod suspr3;
/**SUSPR4 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR4)

For information about available fields see [`mod@suspr4`] module*/
pub type SUSPR4 = crate::Reg<suspr4::SUSPR4rs>;
///SAES suspend registers
pub mod suspr4;
/**SUSPR5 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR5)

For information about available fields see [`mod@suspr5`] module*/
pub type SUSPR5 = crate::Reg<suspr5::SUSPR5rs>;
///SAES suspend registers
pub mod suspr5;
/**SUSPR6 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR6)

For information about available fields see [`mod@suspr6`] module*/
pub type SUSPR6 = crate::Reg<suspr6::SUSPR6rs>;
///SAES suspend registers
pub mod suspr6;
/**SUSPR7 (rw) register accessor: SAES suspend registers

You can [`read`](crate::Reg::read) this register and get [`suspr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`suspr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:SUSPR7)

For information about available fields see [`mod@suspr7`] module*/
pub type SUSPR7 = crate::Reg<suspr7::SUSPR7rs>;
///SAES suspend registers
pub mod suspr7;
/**IER (rw) register accessor: SAES interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///SAES interrupt enable register
pub mod ier;
/**ISR (r) register accessor: SAES interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///SAES interrupt status register
pub mod isr;
/**ICR (w) register accessor: SAES interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SAES:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///SAES interrupt clear register
pub mod icr;
