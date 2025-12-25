#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dmabmr: DMABMR,
    dmatpdr: DMATPDR,
    dmarpdr: DMARPDR,
    dmardlar: DMARDLAR,
    dmatdlar: DMATDLAR,
    dmasr: DMASR,
    dmaomr: DMAOMR,
    dmaier: DMAIER,
    dmamfbocr: DMAMFBOCR,
    dmarswtr: DMARSWTR,
    _reserved10: [u8; 0x20],
    dmachtdr: DMACHTDR,
    dmachrdr: DMACHRDR,
    dmachtbar: DMACHTBAR,
    dmachrbar: DMACHRBAR,
}
impl RegisterBlock {
    ///0x00 - Ethernet DMA bus mode register
    #[inline(always)]
    pub const fn dmabmr(&self) -> &DMABMR {
        &self.dmabmr
    }
    ///0x04 - Ethernet DMA transmit poll demand register
    #[inline(always)]
    pub const fn dmatpdr(&self) -> &DMATPDR {
        &self.dmatpdr
    }
    ///0x08 - EHERNET DMA receive poll demand register
    #[inline(always)]
    pub const fn dmarpdr(&self) -> &DMARPDR {
        &self.dmarpdr
    }
    ///0x0c - Ethernet DMA receive descriptor list address register
    #[inline(always)]
    pub const fn dmardlar(&self) -> &DMARDLAR {
        &self.dmardlar
    }
    ///0x10 - Ethernet DMA transmit descriptor list address register
    #[inline(always)]
    pub const fn dmatdlar(&self) -> &DMATDLAR {
        &self.dmatdlar
    }
    ///0x14 - Ethernet DMA status register
    #[inline(always)]
    pub const fn dmasr(&self) -> &DMASR {
        &self.dmasr
    }
    ///0x18 - Ethernet DMA operation mode register
    #[inline(always)]
    pub const fn dmaomr(&self) -> &DMAOMR {
        &self.dmaomr
    }
    ///0x1c - Ethernet DMA interrupt enable register
    #[inline(always)]
    pub const fn dmaier(&self) -> &DMAIER {
        &self.dmaier
    }
    ///0x20 - Ethernet DMA missed frame and buffer overflow counter register
    #[inline(always)]
    pub const fn dmamfbocr(&self) -> &DMAMFBOCR {
        &self.dmamfbocr
    }
    ///0x24 - Ethernet DMA receive status watchdog timer register
    #[inline(always)]
    pub const fn dmarswtr(&self) -> &DMARSWTR {
        &self.dmarswtr
    }
    ///0x48 - Ethernet DMA current host transmit descriptor register
    #[inline(always)]
    pub const fn dmachtdr(&self) -> &DMACHTDR {
        &self.dmachtdr
    }
    ///0x4c - Ethernet DMA current host receive descriptor register
    #[inline(always)]
    pub const fn dmachrdr(&self) -> &DMACHRDR {
        &self.dmachrdr
    }
    ///0x50 - Ethernet DMA current host transmit buffer address register
    #[inline(always)]
    pub const fn dmachtbar(&self) -> &DMACHTBAR {
        &self.dmachtbar
    }
    ///0x54 - Ethernet DMA current host receive buffer address register
    #[inline(always)]
    pub const fn dmachrbar(&self) -> &DMACHRBAR {
        &self.dmachrbar
    }
}
/**DMABMR (rw) register accessor: Ethernet DMA bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmabmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMABMR)

For information about available fields see [`mod@dmabmr`] module*/
pub type DMABMR = crate::Reg<dmabmr::DMABMRrs>;
///Ethernet DMA bus mode register
pub mod dmabmr;
/**DMATPDR (rw) register accessor: Ethernet DMA transmit poll demand register

You can [`read`](crate::Reg::read) this register and get [`dmatpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMATPDR)

For information about available fields see [`mod@dmatpdr`] module*/
pub type DMATPDR = crate::Reg<dmatpdr::DMATPDRrs>;
///Ethernet DMA transmit poll demand register
pub mod dmatpdr;
/**DMARPDR (rw) register accessor: EHERNET DMA receive poll demand register

You can [`read`](crate::Reg::read) this register and get [`dmarpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMARPDR)

For information about available fields see [`mod@dmarpdr`] module*/
pub type DMARPDR = crate::Reg<dmarpdr::DMARPDRrs>;
///EHERNET DMA receive poll demand register
pub mod dmarpdr;
/**DMARDLAR (rw) register accessor: Ethernet DMA receive descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmardlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmardlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMARDLAR)

For information about available fields see [`mod@dmardlar`] module*/
pub type DMARDLAR = crate::Reg<dmardlar::DMARDLARrs>;
///Ethernet DMA receive descriptor list address register
pub mod dmardlar;
/**DMATDLAR (rw) register accessor: Ethernet DMA transmit descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmatdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMATDLAR)

For information about available fields see [`mod@dmatdlar`] module*/
pub type DMATDLAR = crate::Reg<dmatdlar::DMATDLARrs>;
///Ethernet DMA transmit descriptor list address register
pub mod dmatdlar;
/**DMASR (rw) register accessor: Ethernet DMA status register

You can [`read`](crate::Reg::read) this register and get [`dmasr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMASR)

For information about available fields see [`mod@dmasr`] module*/
pub type DMASR = crate::Reg<dmasr::DMASRrs>;
///Ethernet DMA status register
pub mod dmasr;
/**DMAOMR (rw) register accessor: Ethernet DMA operation mode register

You can [`read`](crate::Reg::read) this register and get [`dmaomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMAOMR)

For information about available fields see [`mod@dmaomr`] module*/
pub type DMAOMR = crate::Reg<dmaomr::DMAOMRrs>;
///Ethernet DMA operation mode register
pub mod dmaomr;
/**DMAIER (rw) register accessor: Ethernet DMA interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmaier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMAIER)

For information about available fields see [`mod@dmaier`] module*/
pub type DMAIER = crate::Reg<dmaier::DMAIERrs>;
///Ethernet DMA interrupt enable register
pub mod dmaier;
/**DMAMFBOCR (rw) register accessor: Ethernet DMA missed frame and buffer overflow counter register

You can [`read`](crate::Reg::read) this register and get [`dmamfbocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamfbocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMAMFBOCR)

For information about available fields see [`mod@dmamfbocr`] module*/
pub type DMAMFBOCR = crate::Reg<dmamfbocr::DMAMFBOCRrs>;
///Ethernet DMA missed frame and buffer overflow counter register
pub mod dmamfbocr;
/**DMARSWTR (rw) register accessor: Ethernet DMA receive status watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmarswtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarswtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMARSWTR)

For information about available fields see [`mod@dmarswtr`] module*/
pub type DMARSWTR = crate::Reg<dmarswtr::DMARSWTRrs>;
///Ethernet DMA receive status watchdog timer register
pub mod dmarswtr;
/**DMACHTDR (r) register accessor: Ethernet DMA current host transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmachtdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMACHTDR)

For information about available fields see [`mod@dmachtdr`] module*/
pub type DMACHTDR = crate::Reg<dmachtdr::DMACHTDRrs>;
///Ethernet DMA current host transmit descriptor register
pub mod dmachtdr;
/**DMACHRDR (r) register accessor: Ethernet DMA current host receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmachrdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMACHRDR)

For information about available fields see [`mod@dmachrdr`] module*/
pub type DMACHRDR = crate::Reg<dmachrdr::DMACHRDRrs>;
///Ethernet DMA current host receive descriptor register
pub mod dmachrdr;
/**DMACHTBAR (r) register accessor: Ethernet DMA current host transmit buffer address register

You can [`read`](crate::Reg::read) this register and get [`dmachtbar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMACHTBAR)

For information about available fields see [`mod@dmachtbar`] module*/
pub type DMACHTBAR = crate::Reg<dmachtbar::DMACHTBARrs>;
///Ethernet DMA current host transmit buffer address register
pub mod dmachtbar;
/**DMACHRBAR (r) register accessor: Ethernet DMA current host receive buffer address register

You can [`read`](crate::Reg::read) this register and get [`dmachrbar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#Ethernet_DMA:DMACHRBAR)

For information about available fields see [`mod@dmachrbar`] module*/
pub type DMACHRBAR = crate::Reg<dmachrbar::DMACHRBARrs>;
///Ethernet DMA current host receive buffer address register
pub mod dmachrbar;
