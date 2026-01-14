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
    optr: OPTR,
    wrprot1: WRPROT1,
    _reserved9: [u8; 0x5c],
    wrprot2: WRPROT2,
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
    pub const fn optr(&self) -> &OPTR {
        &self.optr
    }
    ///0x20 - Write protection register
    #[inline(always)]
    pub const fn wrprot1(&self) -> &WRPROT1 {
        &self.wrprot1
    }
    ///0x80 - Write protection register
    #[inline(always)]
    pub const fn wrprot2(&self) -> &WRPROT2 {
        &self.wrprot2
    }
}
/**ACR (rw) register accessor: Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:ACR)

For information about available fields see [`mod@acr`] module*/
pub type ACR = crate::Reg<acr::ACRrs>;
///Access control register
pub mod acr;
/**PECR (rw) register accessor: Program/erase control register

You can [`read`](crate::Reg::read) this register and get [`pecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:PECR)

For information about available fields see [`mod@pecr`] module*/
pub type PECR = crate::Reg<pecr::PECRrs>;
///Program/erase control register
pub mod pecr;
/**PDKEYR (w) register accessor: Power down key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:PDKEYR)

For information about available fields see [`mod@pdkeyr`] module*/
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYRrs>;
///Power down key register
pub mod pdkeyr;
/**PEKEYR (w) register accessor: Program/erase key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pekeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:PEKEYR)

For information about available fields see [`mod@pekeyr`] module*/
pub type PEKEYR = crate::Reg<pekeyr::PEKEYRrs>;
///Program/erase key register
pub mod pekeyr;
/**PRGKEYR (w) register accessor: Program memory key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:PRGKEYR)

For information about available fields see [`mod@prgkeyr`] module*/
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYRrs>;
///Program memory key register
pub mod prgkeyr;
/**OPTKEYR (w) register accessor: Option byte key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:OPTKEYR)

For information about available fields see [`mod@optkeyr`] module*/
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYRrs>;
///Option byte key register
pub mod optkeyr;
/**SR (rw) register accessor: Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///Status register
pub mod sr;
/**OPTR (r) register accessor: Option byte register

You can [`read`](crate::Reg::read) this register and get [`optr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:OPTR)

For information about available fields see [`mod@optr`] module*/
pub type OPTR = crate::Reg<optr::OPTRrs>;
///Option byte register
pub mod optr;
/**WRPROT1 (r) register accessor: Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrprot1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:WRPROT1)

For information about available fields see [`mod@wrprot1`] module*/
pub type WRPROT1 = crate::Reg<wrprot1::WRPROT1rs>;
///Write protection register
pub mod wrprot1;
/**WRPROT2 (r) register accessor: Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrprot2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:WRPROT2)

For information about available fields see [`mod@wrprot2`] module*/
pub type WRPROT2 = crate::Reg<wrprot2::WRPROT2rs>;
///Write protection register
pub mod wrprot2;
