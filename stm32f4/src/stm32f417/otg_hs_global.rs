#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    otg_hs_gotgctl: OTG_HS_GOTGCTL,
    otg_hs_gotgint: OTG_HS_GOTGINT,
    otg_hs_gahbcfg: OTG_HS_GAHBCFG,
    otg_hs_gusbcfg: OTG_HS_GUSBCFG,
    otg_hs_grstctl: OTG_HS_GRSTCTL,
    otg_hs_gintsts: OTG_HS_GINTSTS,
    otg_hs_gintmsk: OTG_HS_GINTMSK,
    _reserved_7_otg_hs_grxstsr: [u8; 0x04],
    _reserved_8_otg_hs_grxstsp: [u8; 0x04],
    otg_hs_grxfsiz: OTG_HS_GRXFSIZ,
    _reserved_10_otg_hs: [u8; 0x04],
    otg_hs_gnptxsts: OTG_HS_GNPTXSTS,
    _reserved12: [u8; 0x08],
    otg_hs_gccfg: OTG_HS_GCCFG,
    otg_hs_cid: OTG_HS_CID,
    _reserved14: [u8; 0xc0],
    otg_hs_hptxfsiz: OTG_HS_HPTXFSIZ,
    otg_hs_dieptxf1: OTG_HS_DIEPTXF1,
    otg_hs_dieptxf2: OTG_HS_DIEPTXF2,
    _reserved17: [u8; 0x10],
    otg_hs_dieptxf3: OTG_HS_DIEPTXF3,
    otg_hs_dieptxf4: OTG_HS_DIEPTXF4,
    otg_hs_dieptxf5: OTG_HS_DIEPTXF5,
    otg_hs_dieptxf6: OTG_HS_DIEPTXF6,
    otg_hs_dieptxf7: OTG_HS_DIEPTXF7,
}
impl RegisterBlock {
    ///0x00 - OTG_HS control and status register
    #[inline(always)]
    pub const fn otg_hs_gotgctl(&self) -> &OTG_HS_GOTGCTL {
        &self.otg_hs_gotgctl
    }
    ///0x04 - OTG_HS interrupt register
    #[inline(always)]
    pub const fn otg_hs_gotgint(&self) -> &OTG_HS_GOTGINT {
        &self.otg_hs_gotgint
    }
    ///0x08 - OTG_HS AHB configuration register
    #[inline(always)]
    pub const fn otg_hs_gahbcfg(&self) -> &OTG_HS_GAHBCFG {
        &self.otg_hs_gahbcfg
    }
    ///0x0c - OTG_HS USB configuration register
    #[inline(always)]
    pub const fn otg_hs_gusbcfg(&self) -> &OTG_HS_GUSBCFG {
        &self.otg_hs_gusbcfg
    }
    ///0x10 - OTG_HS reset register
    #[inline(always)]
    pub const fn otg_hs_grstctl(&self) -> &OTG_HS_GRSTCTL {
        &self.otg_hs_grstctl
    }
    ///0x14 - OTG_HS core interrupt register
    #[inline(always)]
    pub const fn otg_hs_gintsts(&self) -> &OTG_HS_GINTSTS {
        &self.otg_hs_gintsts
    }
    ///0x18 - OTG_HS interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_gintmsk(&self) -> &OTG_HS_GINTMSK {
        &self.otg_hs_gintmsk
    }
    ///0x1c - OTG_HS Receive status debug read register (peripheral mode mode)
    #[inline(always)]
    pub const fn otg_hs_grxstsr_peripheral(&self) -> &OTG_HS_GRXSTSR_PERIPHERAL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x1c - OTG_HS Receive status debug read register (host mode)
    #[inline(always)]
    pub const fn otg_hs_grxstsr_host(&self) -> &OTG_HS_GRXSTSR_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    ///0x20 - OTG_HS status read and pop register (peripheral mode)
    #[inline(always)]
    pub const fn otg_hs_grxstsp_peripheral(&self) -> &OTG_HS_GRXSTSP_PERIPHERAL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x20 - OTG_HS status read and pop register (host mode)
    #[inline(always)]
    pub const fn otg_hs_grxstsp_host(&self) -> &OTG_HS_GRXSTSP_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    ///0x24 - OTG_HS Receive FIFO size register
    #[inline(always)]
    pub const fn otg_hs_grxfsiz(&self) -> &OTG_HS_GRXFSIZ {
        &self.otg_hs_grxfsiz
    }
    ///0x28 - Endpoint 0 transmit FIFO size (peripheral mode)
    #[inline(always)]
    pub const fn otg_hs_tx0fsiz_peripheral(&self) -> &OTG_HS_TX0FSIZ_PERIPHERAL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)
    #[inline(always)]
    pub const fn otg_hs_gnptxfsiz_host(&self) -> &OTG_HS_GNPTXFSIZ_HOST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).cast() }
    }
    ///0x2c - OTG_HS nonperiodic transmit FIFO/queue status register
    #[inline(always)]
    pub const fn otg_hs_gnptxsts(&self) -> &OTG_HS_GNPTXSTS {
        &self.otg_hs_gnptxsts
    }
    ///0x38 - OTG_HS general core configuration register
    #[inline(always)]
    pub const fn otg_hs_gccfg(&self) -> &OTG_HS_GCCFG {
        &self.otg_hs_gccfg
    }
    ///0x3c - OTG_HS core ID register
    #[inline(always)]
    pub const fn otg_hs_cid(&self) -> &OTG_HS_CID {
        &self.otg_hs_cid
    }
    ///0x100 - OTG_HS Host periodic transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_hptxfsiz(&self) -> &OTG_HS_HPTXFSIZ {
        &self.otg_hs_hptxfsiz
    }
    ///0x104 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf1(&self) -> &OTG_HS_DIEPTXF1 {
        &self.otg_hs_dieptxf1
    }
    ///0x108 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf2(&self) -> &OTG_HS_DIEPTXF2 {
        &self.otg_hs_dieptxf2
    }
    ///0x11c - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf3(&self) -> &OTG_HS_DIEPTXF3 {
        &self.otg_hs_dieptxf3
    }
    ///0x120 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf4(&self) -> &OTG_HS_DIEPTXF4 {
        &self.otg_hs_dieptxf4
    }
    ///0x124 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf5(&self) -> &OTG_HS_DIEPTXF5 {
        &self.otg_hs_dieptxf5
    }
    ///0x128 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf6(&self) -> &OTG_HS_DIEPTXF6 {
        &self.otg_hs_dieptxf6
    }
    ///0x12c - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub const fn otg_hs_dieptxf7(&self) -> &OTG_HS_DIEPTXF7 {
        &self.otg_hs_dieptxf7
    }
}
/**OTG_HS_GOTGCTL (rw) register accessor: OTG_HS control and status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GOTGCTL)

For information about available fields see [`mod@otg_hs_gotgctl`] module*/
pub type OTG_HS_GOTGCTL = crate::Reg<otg_hs_gotgctl::OTG_HS_GOTGCTLrs>;
///OTG_HS control and status register
pub mod otg_hs_gotgctl;
/**OTG_HS_GOTGINT (rw) register accessor: OTG_HS interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GOTGINT)

For information about available fields see [`mod@otg_hs_gotgint`] module*/
pub type OTG_HS_GOTGINT = crate::Reg<otg_hs_gotgint::OTG_HS_GOTGINTrs>;
///OTG_HS interrupt register
pub mod otg_hs_gotgint;
/**OTG_HS_GAHBCFG (rw) register accessor: OTG_HS AHB configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GAHBCFG)

For information about available fields see [`mod@otg_hs_gahbcfg`] module*/
pub type OTG_HS_GAHBCFG = crate::Reg<otg_hs_gahbcfg::OTG_HS_GAHBCFGrs>;
///OTG_HS AHB configuration register
pub mod otg_hs_gahbcfg;
/**OTG_HS_GUSBCFG (rw) register accessor: OTG_HS USB configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GUSBCFG)

For information about available fields see [`mod@otg_hs_gusbcfg`] module*/
pub type OTG_HS_GUSBCFG = crate::Reg<otg_hs_gusbcfg::OTG_HS_GUSBCFGrs>;
///OTG_HS USB configuration register
pub mod otg_hs_gusbcfg;
/**OTG_HS_GRSTCTL (rw) register accessor: OTG_HS reset register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GRSTCTL)

For information about available fields see [`mod@otg_hs_grstctl`] module*/
pub type OTG_HS_GRSTCTL = crate::Reg<otg_hs_grstctl::OTG_HS_GRSTCTLrs>;
///OTG_HS reset register
pub mod otg_hs_grstctl;
/**OTG_HS_GINTSTS (rw) register accessor: OTG_HS core interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GINTSTS)

For information about available fields see [`mod@otg_hs_gintsts`] module*/
pub type OTG_HS_GINTSTS = crate::Reg<otg_hs_gintsts::OTG_HS_GINTSTSrs>;
///OTG_HS core interrupt register
pub mod otg_hs_gintsts;
/**OTG_HS_GINTMSK (rw) register accessor: OTG_HS interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GINTMSK)

For information about available fields see [`mod@otg_hs_gintmsk`] module*/
pub type OTG_HS_GINTMSK = crate::Reg<otg_hs_gintmsk::OTG_HS_GINTMSKrs>;
///OTG_HS interrupt mask register
pub mod otg_hs_gintmsk;
/**OTG_HS_GRXSTSR_Host (r) register accessor: OTG_HS Receive status debug read register (host mode)

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GRXSTSR_Host)

For information about available fields see [`mod@otg_hs_grxstsr_host`] module*/
#[doc(alias = "OTG_HS_GRXSTSR_Host")]
pub type OTG_HS_GRXSTSR_HOST = crate::Reg<otg_hs_grxstsr_host::OTG_HS_GRXSTSR_HOSTrs>;
///OTG_HS Receive status debug read register (host mode)
pub mod otg_hs_grxstsr_host;
/**OTG_HS_GRXSTSP_Host (r) register accessor: OTG_HS status read and pop register (host mode)

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grxstsp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GRXSTSP_Host)

For information about available fields see [`mod@otg_hs_grxstsp_host`] module*/
#[doc(alias = "OTG_HS_GRXSTSP_Host")]
pub type OTG_HS_GRXSTSP_HOST = crate::Reg<otg_hs_grxstsp_host::OTG_HS_GRXSTSP_HOSTrs>;
///OTG_HS status read and pop register (host mode)
pub mod otg_hs_grxstsp_host;
/**OTG_HS_GRXFSIZ (rw) register accessor: OTG_HS Receive FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GRXFSIZ)

For information about available fields see [`mod@otg_hs_grxfsiz`] module*/
pub type OTG_HS_GRXFSIZ = crate::Reg<otg_hs_grxfsiz::OTG_HS_GRXFSIZrs>;
///OTG_HS Receive FIFO size register
pub mod otg_hs_grxfsiz;
/**OTG_HS_GNPTXFSIZ_Host (rw) register accessor: OTG_HS nonperiodic transmit FIFO size register (host mode)

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gnptxfsiz_host::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gnptxfsiz_host::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GNPTXFSIZ_Host)

For information about available fields see [`mod@otg_hs_gnptxfsiz_host`] module*/
#[doc(alias = "OTG_HS_GNPTXFSIZ_Host")]
pub type OTG_HS_GNPTXFSIZ_HOST = crate::Reg<otg_hs_gnptxfsiz_host::OTG_HS_GNPTXFSIZ_HOSTrs>;
///OTG_HS nonperiodic transmit FIFO size register (host mode)
pub mod otg_hs_gnptxfsiz_host;
/**OTG_HS_TX0FSIZ_Peripheral (rw) register accessor: Endpoint 0 transmit FIFO size (peripheral mode)

You can [`read`](crate::Reg::read) this register and get [`otg_hs_tx0fsiz_peripheral::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_tx0fsiz_peripheral::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_TX0FSIZ_Peripheral)

For information about available fields see [`mod@otg_hs_tx0fsiz_peripheral`] module*/
#[doc(alias = "OTG_HS_TX0FSIZ_Peripheral")]
pub type OTG_HS_TX0FSIZ_PERIPHERAL =
    crate::Reg<otg_hs_tx0fsiz_peripheral::OTG_HS_TX0FSIZ_PERIPHERALrs>;
///Endpoint 0 transmit FIFO size (peripheral mode)
pub mod otg_hs_tx0fsiz_peripheral;
/**OTG_HS_GNPTXSTS (r) register accessor: OTG_HS nonperiodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GNPTXSTS)

For information about available fields see [`mod@otg_hs_gnptxsts`] module*/
pub type OTG_HS_GNPTXSTS = crate::Reg<otg_hs_gnptxsts::OTG_HS_GNPTXSTSrs>;
///OTG_HS nonperiodic transmit FIFO/queue status register
pub mod otg_hs_gnptxsts;
/**OTG_HS_GCCFG (rw) register accessor: OTG_HS general core configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GCCFG)

For information about available fields see [`mod@otg_hs_gccfg`] module*/
pub type OTG_HS_GCCFG = crate::Reg<otg_hs_gccfg::OTG_HS_GCCFGrs>;
///OTG_HS general core configuration register
pub mod otg_hs_gccfg;
/**OTG_HS_CID (rw) register accessor: OTG_HS core ID register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_CID)

For information about available fields see [`mod@otg_hs_cid`] module*/
pub type OTG_HS_CID = crate::Reg<otg_hs_cid::OTG_HS_CIDrs>;
///OTG_HS core ID register
pub mod otg_hs_cid;
/**OTG_HS_HPTXFSIZ (rw) register accessor: OTG_HS Host periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_HPTXFSIZ)

For information about available fields see [`mod@otg_hs_hptxfsiz`] module*/
pub type OTG_HS_HPTXFSIZ = crate::Reg<otg_hs_hptxfsiz::OTG_HS_HPTXFSIZrs>;
///OTG_HS Host periodic transmit FIFO size register
pub mod otg_hs_hptxfsiz;
/**OTG_HS_DIEPTXF1 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF1)

For information about available fields see [`mod@otg_hs_dieptxf1`] module*/
pub type OTG_HS_DIEPTXF1 = crate::Reg<otg_hs_dieptxf1::OTG_HS_DIEPTXF1rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf1;
/**OTG_HS_DIEPTXF2 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF2)

For information about available fields see [`mod@otg_hs_dieptxf2`] module*/
pub type OTG_HS_DIEPTXF2 = crate::Reg<otg_hs_dieptxf2::OTG_HS_DIEPTXF2rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf2;
/**OTG_HS_DIEPTXF3 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF3)

For information about available fields see [`mod@otg_hs_dieptxf3`] module*/
pub type OTG_HS_DIEPTXF3 = crate::Reg<otg_hs_dieptxf3::OTG_HS_DIEPTXF3rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf3;
/**OTG_HS_DIEPTXF4 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF4)

For information about available fields see [`mod@otg_hs_dieptxf4`] module*/
pub type OTG_HS_DIEPTXF4 = crate::Reg<otg_hs_dieptxf4::OTG_HS_DIEPTXF4rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf4;
/**OTG_HS_DIEPTXF5 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF5)

For information about available fields see [`mod@otg_hs_dieptxf5`] module*/
pub type OTG_HS_DIEPTXF5 = crate::Reg<otg_hs_dieptxf5::OTG_HS_DIEPTXF5rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf5;
/**OTG_HS_DIEPTXF6 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF6)

For information about available fields see [`mod@otg_hs_dieptxf6`] module*/
pub type OTG_HS_DIEPTXF6 = crate::Reg<otg_hs_dieptxf6::OTG_HS_DIEPTXF6rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf6;
/**OTG_HS_DIEPTXF7 (rw) register accessor: OTG_HS device IN endpoint transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptxf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptxf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_DIEPTXF7)

For information about available fields see [`mod@otg_hs_dieptxf7`] module*/
pub type OTG_HS_DIEPTXF7 = crate::Reg<otg_hs_dieptxf7::OTG_HS_DIEPTXF7rs>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod otg_hs_dieptxf7;
/**OTG_HS_GRXSTSR_Peripheral (r) register accessor: OTG_HS Receive status debug read register (peripheral mode mode)

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grxstsr_peripheral::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GRXSTSR_Peripheral)

For information about available fields see [`mod@otg_hs_grxstsr_peripheral`] module*/
#[doc(alias = "OTG_HS_GRXSTSR_Peripheral")]
pub type OTG_HS_GRXSTSR_PERIPHERAL =
    crate::Reg<otg_hs_grxstsr_peripheral::OTG_HS_GRXSTSR_PERIPHERALrs>;
///OTG_HS Receive status debug read register (peripheral mode mode)
pub mod otg_hs_grxstsr_peripheral;
/**OTG_HS_GRXSTSP_Peripheral (r) register accessor: OTG_HS status read and pop register (peripheral mode)

You can [`read`](crate::Reg::read) this register and get [`otg_hs_grxstsp_peripheral::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_GRXSTSP_Peripheral)

For information about available fields see [`mod@otg_hs_grxstsp_peripheral`] module*/
#[doc(alias = "OTG_HS_GRXSTSP_Peripheral")]
pub type OTG_HS_GRXSTSP_PERIPHERAL =
    crate::Reg<otg_hs_grxstsp_peripheral::OTG_HS_GRXSTSP_PERIPHERALrs>;
///OTG_HS status read and pop register (peripheral mode)
pub mod otg_hs_grxstsp_peripheral;
