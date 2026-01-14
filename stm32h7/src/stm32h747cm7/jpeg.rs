#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    confr0: CONFR0,
    confr1: CONFR1,
    confr2: CONFR2,
    confr3: CONFR3,
    confrn1: CONFRN1,
    confrn2: CONFRN2,
    confrn3: CONFRN3,
    confrn4: CONFRN4,
    _reserved8: [u8; 0x10],
    cr: CR,
    sr: SR,
    cfr: CFR,
    _reserved11: [u8; 0x04],
    dir: DIR,
    dor: DOR,
}
impl RegisterBlock {
    ///0x00 - JPEG codec control register
    #[inline(always)]
    pub const fn confr0(&self) -> &CONFR0 {
        &self.confr0
    }
    ///0x04 - JPEG codec configuration register 1
    #[inline(always)]
    pub const fn confr1(&self) -> &CONFR1 {
        &self.confr1
    }
    ///0x08 - JPEG codec configuration register 2
    #[inline(always)]
    pub const fn confr2(&self) -> &CONFR2 {
        &self.confr2
    }
    ///0x0c - JPEG codec configuration register 3
    #[inline(always)]
    pub const fn confr3(&self) -> &CONFR3 {
        &self.confr3
    }
    ///0x10 - JPEG codec configuration register 4-7
    #[inline(always)]
    pub const fn confrn1(&self) -> &CONFRN1 {
        &self.confrn1
    }
    ///0x14 - JPEG codec configuration register 4-7
    #[inline(always)]
    pub const fn confrn2(&self) -> &CONFRN2 {
        &self.confrn2
    }
    ///0x18 - JPEG codec configuration register 4-7
    #[inline(always)]
    pub const fn confrn3(&self) -> &CONFRN3 {
        &self.confrn3
    }
    ///0x1c - JPEG codec configuration register 4-7
    #[inline(always)]
    pub const fn confrn4(&self) -> &CONFRN4 {
        &self.confrn4
    }
    ///0x30 - JPEG control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x34 - JPEG status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x38 - JPEG clear flag register
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x40 - JPEG data input register
    #[inline(always)]
    pub const fn dir(&self) -> &DIR {
        &self.dir
    }
    ///0x44 - JPEG data output register
    #[inline(always)]
    pub const fn dor(&self) -> &DOR {
        &self.dor
    }
}
/**CONFR0 (w) register accessor: JPEG codec control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFR0)

For information about available fields see [`mod@confr0`] module*/
pub type CONFR0 = crate::Reg<confr0::CONFR0rs>;
///JPEG codec control register
pub mod confr0;
/**CONFR1 (rw) register accessor: JPEG codec configuration register 1

You can [`read`](crate::Reg::read) this register and get [`confr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFR1)

For information about available fields see [`mod@confr1`] module*/
pub type CONFR1 = crate::Reg<confr1::CONFR1rs>;
///JPEG codec configuration register 1
pub mod confr1;
/**CONFR2 (rw) register accessor: JPEG codec configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFR2)

For information about available fields see [`mod@confr2`] module*/
pub type CONFR2 = crate::Reg<confr2::CONFR2rs>;
///JPEG codec configuration register 2
pub mod confr2;
/**CONFR3 (rw) register accessor: JPEG codec configuration register 3

You can [`read`](crate::Reg::read) this register and get [`confr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFR3)

For information about available fields see [`mod@confr3`] module*/
pub type CONFR3 = crate::Reg<confr3::CONFR3rs>;
///JPEG codec configuration register 3
pub mod confr3;
/**CONFRN1 (rw) register accessor: JPEG codec configuration register 4-7

You can [`read`](crate::Reg::read) this register and get [`confrn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFRN1)

For information about available fields see [`mod@confrn1`] module*/
pub type CONFRN1 = crate::Reg<confrn1::CONFRN1rs>;
///JPEG codec configuration register 4-7
pub mod confrn1;
/**CONFRN2 (rw) register accessor: JPEG codec configuration register 4-7

You can [`read`](crate::Reg::read) this register and get [`confrn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFRN2)

For information about available fields see [`mod@confrn2`] module*/
pub type CONFRN2 = crate::Reg<confrn2::CONFRN2rs>;
///JPEG codec configuration register 4-7
pub mod confrn2;
/**CONFRN3 (rw) register accessor: JPEG codec configuration register 4-7

You can [`read`](crate::Reg::read) this register and get [`confrn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFRN3)

For information about available fields see [`mod@confrn3`] module*/
pub type CONFRN3 = crate::Reg<confrn3::CONFRN3rs>;
///JPEG codec configuration register 4-7
pub mod confrn3;
/**CONFRN4 (rw) register accessor: JPEG codec configuration register 4-7

You can [`read`](crate::Reg::read) this register and get [`confrn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CONFRN4)

For information about available fields see [`mod@confrn4`] module*/
pub type CONFRN4 = crate::Reg<confrn4::CONFRN4rs>;
///JPEG codec configuration register 4-7
pub mod confrn4;
/**CR (rw) register accessor: JPEG control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///JPEG control register
pub mod cr;
/**SR (r) register accessor: JPEG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///JPEG status register
pub mod sr;
/**CFR (rw) register accessor: JPEG clear flag register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///JPEG clear flag register
pub mod cfr;
/**DIR (w) register accessor: JPEG data input register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:DIR)

For information about available fields see [`mod@dir`] module*/
pub type DIR = crate::Reg<dir::DIRrs>;
///JPEG data input register
pub mod dir;
/**DOR (r) register accessor: JPEG data output register

You can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#JPEG:DOR)

For information about available fields see [`mod@dor`] module*/
pub type DOR = crate::Reg<dor::DORrs>;
///JPEG data output register
pub mod dor;
