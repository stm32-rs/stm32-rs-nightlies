#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    acr: ACR,
    pecr: PECR,
    pdkeyr: PDKEYR,
    pekeyr: PEKEYR,
    prgkeyr: PRGKEYR,
    optkeyr: OPTKEYR,
    sr: SR,
    obr: OBR,
    wrpr1: WRPR1,
    _reserved9: [u8; 0x5c],
    wrpr2: WRPR2,
    wrpr3: WRPR3,
}
impl RegisterBlock {
    ///0x00 - Access control register
    #[inline(always)]
    pub const fn acr(&self) -> &ACR {
        &self.acr
    }
    ///0x04 - Program/erase control register
    #[inline(always)]
    pub const fn pecr(&self) -> &PECR {
        &self.pecr
    }
    ///0x08 - Power down key register
    #[inline(always)]
    pub const fn pdkeyr(&self) -> &PDKEYR {
        &self.pdkeyr
    }
    ///0x0c - Program/erase key register
    #[inline(always)]
    pub const fn pekeyr(&self) -> &PEKEYR {
        &self.pekeyr
    }
    ///0x10 - Program memory key register
    #[inline(always)]
    pub const fn prgkeyr(&self) -> &PRGKEYR {
        &self.prgkeyr
    }
    ///0x14 - Option byte key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        &self.optkeyr
    }
    ///0x18 - Status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x1c - Option byte register
    #[inline(always)]
    pub const fn obr(&self) -> &OBR {
        &self.obr
    }
    ///0x20 - Write protection register
    #[inline(always)]
    pub const fn wrpr1(&self) -> &WRPR1 {
        &self.wrpr1
    }
    ///0x80 - Write protection register
    #[inline(always)]
    pub const fn wrpr2(&self) -> &WRPR2 {
        &self.wrpr2
    }
    ///0x84 - Write protection register
    #[inline(always)]
    pub const fn wrpr3(&self) -> &WRPR3 {
        &self.wrpr3
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
/**PECR (rw) register accessor: Program/erase control register

You can [`read`](crate::Reg::read) this register and get [`pecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:PECR)

For information about available fields see [`mod@pecr`] module*/
pub type PECR = crate::Reg<pecr::PECRrs>;
///Program/erase control register
pub mod pecr;
/**PDKEYR (w) register accessor: Power down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:PDKEYR)

For information about available fields see [`mod@pdkeyr`] module*/
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
///Power down key register
pub mod pdkeyr;
/**PEKEYR (w) register accessor: Program/erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pekeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:PEKEYR)

For information about available fields see [`mod@pekeyr`] module*/
pub type PEKEYR = crate::Reg<pekeyr::PEKEYRrs>;
///Program/erase key register
pub mod pekeyr;
/**PRGKEYR (w) register accessor: Program memory key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:PRGKEYR)

For information about available fields see [`mod@prgkeyr`] module*/
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYRrs>;
///Program memory key register
pub mod prgkeyr;
/**OPTKEYR (w) register accessor: Option byte key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///Option byte key register
pub mod optkeyr;
/**SR (rw) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**OBR (r) register accessor: Option byte register

You can [`read`](crate::Reg::read) this register and get [`obr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:OBR)

For information about available fields see [`mod@obr`] module*/
pub type OBR = crate::Reg<obr::OBRrs>;
///Option byte register
pub mod obr;
/**WRPR1 (rw) register accessor: Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:WRPR1)

For information about available fields see [`mod@wrpr1`] module*/
pub type WRPR1 = crate::Reg<wrpr1::WRPR1rs>;
///Write protection register
pub mod wrpr1;
/**WRPR2 (rw) register accessor: Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:WRPR2)

For information about available fields see [`mod@wrpr2`] module*/
pub type WRPR2 = crate::Reg<wrpr2::WRPR2rs>;
///Write protection register
pub mod wrpr2;
/**WRPR3 (rw) register accessor: Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrpr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#Flash:WRPR3)

For information about available fields see [`mod@wrpr3`] module*/
pub type WRPR3 = crate::Reg<wrpr3::WRPR3rs>;
///Write protection register
pub mod wrpr3;
