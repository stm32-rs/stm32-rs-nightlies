#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub otg_hs_hcfg: OTG_HS_HCFG,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub otg_hs_hfir: OTG_HS_HFIR,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub otg_hs_hfnum: OTG_HS_HFNUM,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub otg_hs_hptxsts: OTG_HS_HPTXSTS,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub otg_hs_haint: OTG_HS_HAINT,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub otg_hs_haintmsk: OTG_HS_HAINTMSK,
    _reserved6: [u8; 36usize],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub otg_hs_hprt: OTG_HS_HPRT,
    _reserved7: [u8; 188usize],
    #[doc = "0x100 - OTG_HS host channel-0 characteristics register"]
    pub otg_hs_hcchar0: OTG_HS_HCCHAR0,
    #[doc = "0x104 - OTG_HS host channel-0 split control register"]
    pub otg_hs_hcsplt0: OTG_HS_HCSPLT0,
    #[doc = "0x108 - OTG_HS host channel-11 interrupt register"]
    pub otg_hs_hcint0: OTG_HS_HCINT0,
    #[doc = "0x10c - OTG_HS host channel-11 interrupt mask register"]
    pub otg_hs_hcintmsk0: OTG_HS_HCINTMSK0,
    #[doc = "0x110 - OTG_HS host channel-11 transfer size register"]
    pub otg_hs_hctsiz0: OTG_HS_HCTSIZ0,
    #[doc = "0x114 - OTG_HS host channel-0 DMA address register"]
    pub otg_hs_hcdma0: OTG_HS_HCDMA0,
    _reserved13: [u8; 8usize],
    #[doc = "0x120 - OTG_HS host channel-1 characteristics register"]
    pub otg_hs_hcchar1: OTG_HS_HCCHAR1,
    #[doc = "0x124 - OTG_HS host channel-1 split control register"]
    pub otg_hs_hcsplt1: OTG_HS_HCSPLT1,
    #[doc = "0x128 - OTG_HS host channel-1 interrupt register"]
    pub otg_hs_hcint1: OTG_HS_HCINT1,
    #[doc = "0x12c - OTG_HS host channel-1 interrupt mask register"]
    pub otg_hs_hcintmsk1: OTG_HS_HCINTMSK1,
    #[doc = "0x130 - OTG_HS host channel-1 transfer size register"]
    pub otg_hs_hctsiz1: OTG_HS_HCTSIZ1,
    #[doc = "0x134 - OTG_HS host channel-1 DMA address register"]
    pub otg_hs_hcdma1: OTG_HS_HCDMA1,
    _reserved19: [u8; 8usize],
    #[doc = "0x140 - OTG_HS host channel-2 characteristics register"]
    pub otg_hs_hcchar2: OTG_HS_HCCHAR2,
    #[doc = "0x144 - OTG_HS host channel-2 split control register"]
    pub otg_hs_hcsplt2: OTG_HS_HCSPLT2,
    #[doc = "0x148 - OTG_HS host channel-2 interrupt register"]
    pub otg_hs_hcint2: OTG_HS_HCINT2,
    #[doc = "0x14c - OTG_HS host channel-2 interrupt mask register"]
    pub otg_hs_hcintmsk2: OTG_HS_HCINTMSK2,
    #[doc = "0x150 - OTG_HS host channel-2 transfer size register"]
    pub otg_hs_hctsiz2: OTG_HS_HCTSIZ2,
    #[doc = "0x154 - OTG_HS host channel-2 DMA address register"]
    pub otg_hs_hcdma2: OTG_HS_HCDMA2,
    _reserved25: [u8; 8usize],
    #[doc = "0x160 - OTG_HS host channel-3 characteristics register"]
    pub otg_hs_hcchar3: OTG_HS_HCCHAR3,
    #[doc = "0x164 - OTG_HS host channel-3 split control register"]
    pub otg_hs_hcsplt3: OTG_HS_HCSPLT3,
    #[doc = "0x168 - OTG_HS host channel-3 interrupt register"]
    pub otg_hs_hcint3: OTG_HS_HCINT3,
    #[doc = "0x16c - OTG_HS host channel-3 interrupt mask register"]
    pub otg_hs_hcintmsk3: OTG_HS_HCINTMSK3,
    #[doc = "0x170 - OTG_HS host channel-3 transfer size register"]
    pub otg_hs_hctsiz3: OTG_HS_HCTSIZ3,
    #[doc = "0x174 - OTG_HS host channel-3 DMA address register"]
    pub otg_hs_hcdma3: OTG_HS_HCDMA3,
    _reserved31: [u8; 8usize],
    #[doc = "0x180 - OTG_HS host channel-4 characteristics register"]
    pub otg_hs_hcchar4: OTG_HS_HCCHAR4,
    #[doc = "0x184 - OTG_HS host channel-4 split control register"]
    pub otg_hs_hcsplt4: OTG_HS_HCSPLT4,
    #[doc = "0x188 - OTG_HS host channel-4 interrupt register"]
    pub otg_hs_hcint4: OTG_HS_HCINT4,
    #[doc = "0x18c - OTG_HS host channel-4 interrupt mask register"]
    pub otg_hs_hcintmsk4: OTG_HS_HCINTMSK4,
    #[doc = "0x190 - OTG_HS host channel-4 transfer size register"]
    pub otg_hs_hctsiz4: OTG_HS_HCTSIZ4,
    #[doc = "0x194 - OTG_HS host channel-4 DMA address register"]
    pub otg_hs_hcdma4: OTG_HS_HCDMA4,
    _reserved37: [u8; 8usize],
    #[doc = "0x1a0 - OTG_HS host channel-5 characteristics register"]
    pub otg_hs_hcchar5: OTG_HS_HCCHAR5,
    #[doc = "0x1a4 - OTG_HS host channel-5 split control register"]
    pub otg_hs_hcsplt5: OTG_HS_HCSPLT5,
    #[doc = "0x1a8 - OTG_HS host channel-5 interrupt register"]
    pub otg_hs_hcint5: OTG_HS_HCINT5,
    #[doc = "0x1ac - OTG_HS host channel-5 interrupt mask register"]
    pub otg_hs_hcintmsk5: OTG_HS_HCINTMSK5,
    #[doc = "0x1b0 - OTG_HS host channel-5 transfer size register"]
    pub otg_hs_hctsiz5: OTG_HS_HCTSIZ5,
    #[doc = "0x1b4 - OTG_HS host channel-5 DMA address register"]
    pub otg_hs_hcdma5: OTG_HS_HCDMA5,
    _reserved43: [u8; 8usize],
    #[doc = "0x1c0 - OTG_HS host channel-6 characteristics register"]
    pub otg_hs_hcchar6: OTG_HS_HCCHAR6,
    #[doc = "0x1c4 - OTG_HS host channel-6 split control register"]
    pub otg_hs_hcsplt6: OTG_HS_HCSPLT6,
    #[doc = "0x1c8 - OTG_HS host channel-6 interrupt register"]
    pub otg_hs_hcint6: OTG_HS_HCINT6,
    #[doc = "0x1cc - OTG_HS host channel-6 interrupt mask register"]
    pub otg_hs_hcintmsk6: OTG_HS_HCINTMSK6,
    #[doc = "0x1d0 - OTG_HS host channel-6 transfer size register"]
    pub otg_hs_hctsiz6: OTG_HS_HCTSIZ6,
    #[doc = "0x1d4 - OTG_HS host channel-6 DMA address register"]
    pub otg_hs_hcdma6: OTG_HS_HCDMA6,
    _reserved49: [u8; 8usize],
    #[doc = "0x1e0 - OTG_HS host channel-7 characteristics register"]
    pub otg_hs_hcchar7: OTG_HS_HCCHAR7,
    #[doc = "0x1e4 - OTG_HS host channel-7 split control register"]
    pub otg_hs_hcsplt7: OTG_HS_HCSPLT7,
    #[doc = "0x1e8 - OTG_HS host channel-7 interrupt register"]
    pub otg_hs_hcint7: OTG_HS_HCINT7,
    #[doc = "0x1ec - OTG_HS host channel-7 interrupt mask register"]
    pub otg_hs_hcintmsk7: OTG_HS_HCINTMSK7,
    #[doc = "0x1f0 - OTG_HS host channel-7 transfer size register"]
    pub otg_hs_hctsiz7: OTG_HS_HCTSIZ7,
    #[doc = "0x1f4 - OTG_HS host channel-7 DMA address register"]
    pub otg_hs_hcdma7: OTG_HS_HCDMA7,
    _reserved55: [u8; 8usize],
    #[doc = "0x200 - OTG_HS host channel-8 characteristics register"]
    pub otg_hs_hcchar8: OTG_HS_HCCHAR8,
    #[doc = "0x204 - OTG_HS host channel-8 split control register"]
    pub otg_hs_hcsplt8: OTG_HS_HCSPLT8,
    #[doc = "0x208 - OTG_HS host channel-8 interrupt register"]
    pub otg_hs_hcint8: OTG_HS_HCINT8,
    #[doc = "0x20c - OTG_HS host channel-8 interrupt mask register"]
    pub otg_hs_hcintmsk8: OTG_HS_HCINTMSK8,
    #[doc = "0x210 - OTG_HS host channel-8 transfer size register"]
    pub otg_hs_hctsiz8: OTG_HS_HCTSIZ8,
    #[doc = "0x214 - OTG_HS host channel-8 DMA address register"]
    pub otg_hs_hcdma8: OTG_HS_HCDMA8,
    _reserved61: [u8; 8usize],
    #[doc = "0x220 - OTG_HS host channel-9 characteristics register"]
    pub otg_hs_hcchar9: OTG_HS_HCCHAR9,
    #[doc = "0x224 - OTG_HS host channel-9 split control register"]
    pub otg_hs_hcsplt9: OTG_HS_HCSPLT9,
    #[doc = "0x228 - OTG_HS host channel-9 interrupt register"]
    pub otg_hs_hcint9: OTG_HS_HCINT9,
    #[doc = "0x22c - OTG_HS host channel-9 interrupt mask register"]
    pub otg_hs_hcintmsk9: OTG_HS_HCINTMSK9,
    #[doc = "0x230 - OTG_HS host channel-9 transfer size register"]
    pub otg_hs_hctsiz9: OTG_HS_HCTSIZ9,
    #[doc = "0x234 - OTG_HS host channel-9 DMA address register"]
    pub otg_hs_hcdma9: OTG_HS_HCDMA9,
    _reserved67: [u8; 8usize],
    #[doc = "0x240 - OTG_HS host channel-10 characteristics register"]
    pub otg_hs_hcchar10: OTG_HS_HCCHAR10,
    #[doc = "0x244 - OTG_HS host channel-10 split control register"]
    pub otg_hs_hcsplt10: OTG_HS_HCSPLT10,
    #[doc = "0x248 - OTG_HS host channel-10 interrupt register"]
    pub otg_hs_hcint10: OTG_HS_HCINT10,
    #[doc = "0x24c - OTG_HS host channel-10 interrupt mask register"]
    pub otg_hs_hcintmsk10: OTG_HS_HCINTMSK10,
    #[doc = "0x250 - OTG_HS host channel-10 transfer size register"]
    pub otg_hs_hctsiz10: OTG_HS_HCTSIZ10,
    #[doc = "0x254 - OTG_HS host channel-10 DMA address register"]
    pub otg_hs_hcdma10: OTG_HS_HCDMA10,
    _reserved73: [u8; 8usize],
    #[doc = "0x260 - OTG_HS host channel-11 characteristics register"]
    pub otg_hs_hcchar11: OTG_HS_HCCHAR11,
    #[doc = "0x264 - OTG_HS host channel-11 split control register"]
    pub otg_hs_hcsplt11: OTG_HS_HCSPLT11,
    #[doc = "0x268 - OTG_HS host channel-11 interrupt register"]
    pub otg_hs_hcint11: OTG_HS_HCINT11,
    #[doc = "0x26c - OTG_HS host channel-11 interrupt mask register"]
    pub otg_hs_hcintmsk11: OTG_HS_HCINTMSK11,
    #[doc = "0x270 - OTG_HS host channel-11 transfer size register"]
    pub otg_hs_hctsiz11: OTG_HS_HCTSIZ11,
    #[doc = "0x274 - OTG_HS host channel-11 DMA address register"]
    pub otg_hs_hcdma11: OTG_HS_HCDMA11,
}
#[doc = "OTG_HS host configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcfg](otg_hs_hcfg) module"]
pub type OTG_HS_HCFG = crate::Reg<u32, _OTG_HS_HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCFG;
#[doc = "`read()` method returns [otg_hs_hcfg::R](otg_hs_hcfg::R) reader structure"]
impl crate::Readable for OTG_HS_HCFG {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcfg::W](otg_hs_hcfg::W) writer structure"]
impl crate::Writable for OTG_HS_HCFG {}
#[doc = "OTG_HS host configuration register"]
pub mod otg_hs_hcfg;
#[doc = "OTG_HS Host frame interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hfir](otg_hs_hfir) module"]
pub type OTG_HS_HFIR = crate::Reg<u32, _OTG_HS_HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HFIR;
#[doc = "`read()` method returns [otg_hs_hfir::R](otg_hs_hfir::R) reader structure"]
impl crate::Readable for OTG_HS_HFIR {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hfir::W](otg_hs_hfir::W) writer structure"]
impl crate::Writable for OTG_HS_HFIR {}
#[doc = "OTG_HS Host frame interval register"]
pub mod otg_hs_hfir;
#[doc = "OTG_HS host frame number/frame time remaining register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hfnum](otg_hs_hfnum) module"]
pub type OTG_HS_HFNUM = crate::Reg<u32, _OTG_HS_HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HFNUM;
#[doc = "`read()` method returns [otg_hs_hfnum::R](otg_hs_hfnum::R) reader structure"]
impl crate::Readable for OTG_HS_HFNUM {}
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod otg_hs_hfnum;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hptxsts](otg_hs_hptxsts) module"]
pub type OTG_HS_HPTXSTS = crate::Reg<u32, _OTG_HS_HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HPTXSTS;
#[doc = "`read()` method returns [otg_hs_hptxsts::R](otg_hs_hptxsts::R) reader structure"]
impl crate::Readable for OTG_HS_HPTXSTS {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hptxsts::W](otg_hs_hptxsts::W) writer structure"]
impl crate::Writable for OTG_HS_HPTXSTS {}
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod otg_hs_hptxsts;
#[doc = "OTG_HS Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_haint](otg_hs_haint) module"]
pub type OTG_HS_HAINT = crate::Reg<u32, _OTG_HS_HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HAINT;
#[doc = "`read()` method returns [otg_hs_haint::R](otg_hs_haint::R) reader structure"]
impl crate::Readable for OTG_HS_HAINT {}
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod otg_hs_haint;
#[doc = "OTG_HS host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_haintmsk](otg_hs_haintmsk) module"]
pub type OTG_HS_HAINTMSK = crate::Reg<u32, _OTG_HS_HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HAINTMSK;
#[doc = "`read()` method returns [otg_hs_haintmsk::R](otg_hs_haintmsk::R) reader structure"]
impl crate::Readable for OTG_HS_HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_hs_haintmsk::W](otg_hs_haintmsk::W) writer structure"]
impl crate::Writable for OTG_HS_HAINTMSK {}
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod otg_hs_haintmsk;
#[doc = "OTG_HS host port control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hprt](otg_hs_hprt) module"]
pub type OTG_HS_HPRT = crate::Reg<u32, _OTG_HS_HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HPRT;
#[doc = "`read()` method returns [otg_hs_hprt::R](otg_hs_hprt::R) reader structure"]
impl crate::Readable for OTG_HS_HPRT {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hprt::W](otg_hs_hprt::W) writer structure"]
impl crate::Writable for OTG_HS_HPRT {}
#[doc = "OTG_HS host port control and status register"]
pub mod otg_hs_hprt;
#[doc = "OTG_HS host channel-0 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar0](otg_hs_hcchar0) module"]
pub type OTG_HS_HCCHAR0 = crate::Reg<u32, _OTG_HS_HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR0;
#[doc = "`read()` method returns [otg_hs_hcchar0::R](otg_hs_hcchar0::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar0::W](otg_hs_hcchar0::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR0 {}
#[doc = "OTG_HS host channel-0 characteristics register"]
pub mod otg_hs_hcchar0;
#[doc = "OTG_HS host channel-1 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar1](otg_hs_hcchar1) module"]
pub type OTG_HS_HCCHAR1 = crate::Reg<u32, _OTG_HS_HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR1;
#[doc = "`read()` method returns [otg_hs_hcchar1::R](otg_hs_hcchar1::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar1::W](otg_hs_hcchar1::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR1 {}
#[doc = "OTG_HS host channel-1 characteristics register"]
pub mod otg_hs_hcchar1;
#[doc = "OTG_HS host channel-2 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar2](otg_hs_hcchar2) module"]
pub type OTG_HS_HCCHAR2 = crate::Reg<u32, _OTG_HS_HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR2;
#[doc = "`read()` method returns [otg_hs_hcchar2::R](otg_hs_hcchar2::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar2::W](otg_hs_hcchar2::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR2 {}
#[doc = "OTG_HS host channel-2 characteristics register"]
pub mod otg_hs_hcchar2;
#[doc = "OTG_HS host channel-3 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar3](otg_hs_hcchar3) module"]
pub type OTG_HS_HCCHAR3 = crate::Reg<u32, _OTG_HS_HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR3;
#[doc = "`read()` method returns [otg_hs_hcchar3::R](otg_hs_hcchar3::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar3::W](otg_hs_hcchar3::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR3 {}
#[doc = "OTG_HS host channel-3 characteristics register"]
pub mod otg_hs_hcchar3;
#[doc = "OTG_HS host channel-4 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar4](otg_hs_hcchar4) module"]
pub type OTG_HS_HCCHAR4 = crate::Reg<u32, _OTG_HS_HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR4;
#[doc = "`read()` method returns [otg_hs_hcchar4::R](otg_hs_hcchar4::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar4::W](otg_hs_hcchar4::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR4 {}
#[doc = "OTG_HS host channel-4 characteristics register"]
pub mod otg_hs_hcchar4;
#[doc = "OTG_HS host channel-5 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar5](otg_hs_hcchar5) module"]
pub type OTG_HS_HCCHAR5 = crate::Reg<u32, _OTG_HS_HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR5;
#[doc = "`read()` method returns [otg_hs_hcchar5::R](otg_hs_hcchar5::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar5::W](otg_hs_hcchar5::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR5 {}
#[doc = "OTG_HS host channel-5 characteristics register"]
pub mod otg_hs_hcchar5;
#[doc = "OTG_HS host channel-6 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar6](otg_hs_hcchar6) module"]
pub type OTG_HS_HCCHAR6 = crate::Reg<u32, _OTG_HS_HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR6;
#[doc = "`read()` method returns [otg_hs_hcchar6::R](otg_hs_hcchar6::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar6::W](otg_hs_hcchar6::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR6 {}
#[doc = "OTG_HS host channel-6 characteristics register"]
pub mod otg_hs_hcchar6;
#[doc = "OTG_HS host channel-7 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar7](otg_hs_hcchar7) module"]
pub type OTG_HS_HCCHAR7 = crate::Reg<u32, _OTG_HS_HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR7;
#[doc = "`read()` method returns [otg_hs_hcchar7::R](otg_hs_hcchar7::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar7::W](otg_hs_hcchar7::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR7 {}
#[doc = "OTG_HS host channel-7 characteristics register"]
pub mod otg_hs_hcchar7;
#[doc = "OTG_HS host channel-8 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar8](otg_hs_hcchar8) module"]
pub type OTG_HS_HCCHAR8 = crate::Reg<u32, _OTG_HS_HCCHAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR8;
#[doc = "`read()` method returns [otg_hs_hcchar8::R](otg_hs_hcchar8::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar8::W](otg_hs_hcchar8::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR8 {}
#[doc = "OTG_HS host channel-8 characteristics register"]
pub mod otg_hs_hcchar8;
#[doc = "OTG_HS host channel-9 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar9](otg_hs_hcchar9) module"]
pub type OTG_HS_HCCHAR9 = crate::Reg<u32, _OTG_HS_HCCHAR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR9;
#[doc = "`read()` method returns [otg_hs_hcchar9::R](otg_hs_hcchar9::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar9::W](otg_hs_hcchar9::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR9 {}
#[doc = "OTG_HS host channel-9 characteristics register"]
pub mod otg_hs_hcchar9;
#[doc = "OTG_HS host channel-10 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar10](otg_hs_hcchar10) module"]
pub type OTG_HS_HCCHAR10 = crate::Reg<u32, _OTG_HS_HCCHAR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR10;
#[doc = "`read()` method returns [otg_hs_hcchar10::R](otg_hs_hcchar10::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar10::W](otg_hs_hcchar10::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR10 {}
#[doc = "OTG_HS host channel-10 characteristics register"]
pub mod otg_hs_hcchar10;
#[doc = "OTG_HS host channel-11 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcchar11](otg_hs_hcchar11) module"]
pub type OTG_HS_HCCHAR11 = crate::Reg<u32, _OTG_HS_HCCHAR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCCHAR11;
#[doc = "`read()` method returns [otg_hs_hcchar11::R](otg_hs_hcchar11::R) reader structure"]
impl crate::Readable for OTG_HS_HCCHAR11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcchar11::W](otg_hs_hcchar11::W) writer structure"]
impl crate::Writable for OTG_HS_HCCHAR11 {}
#[doc = "OTG_HS host channel-11 characteristics register"]
pub mod otg_hs_hcchar11;
#[doc = "OTG_HS host channel-0 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt0](otg_hs_hcsplt0) module"]
pub type OTG_HS_HCSPLT0 = crate::Reg<u32, _OTG_HS_HCSPLT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT0;
#[doc = "`read()` method returns [otg_hs_hcsplt0::R](otg_hs_hcsplt0::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt0::W](otg_hs_hcsplt0::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT0 {}
#[doc = "OTG_HS host channel-0 split control register"]
pub mod otg_hs_hcsplt0;
#[doc = "OTG_HS host channel-1 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt1](otg_hs_hcsplt1) module"]
pub type OTG_HS_HCSPLT1 = crate::Reg<u32, _OTG_HS_HCSPLT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT1;
#[doc = "`read()` method returns [otg_hs_hcsplt1::R](otg_hs_hcsplt1::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt1::W](otg_hs_hcsplt1::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT1 {}
#[doc = "OTG_HS host channel-1 split control register"]
pub mod otg_hs_hcsplt1;
#[doc = "OTG_HS host channel-2 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt2](otg_hs_hcsplt2) module"]
pub type OTG_HS_HCSPLT2 = crate::Reg<u32, _OTG_HS_HCSPLT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT2;
#[doc = "`read()` method returns [otg_hs_hcsplt2::R](otg_hs_hcsplt2::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt2::W](otg_hs_hcsplt2::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT2 {}
#[doc = "OTG_HS host channel-2 split control register"]
pub mod otg_hs_hcsplt2;
#[doc = "OTG_HS host channel-3 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt3](otg_hs_hcsplt3) module"]
pub type OTG_HS_HCSPLT3 = crate::Reg<u32, _OTG_HS_HCSPLT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT3;
#[doc = "`read()` method returns [otg_hs_hcsplt3::R](otg_hs_hcsplt3::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt3::W](otg_hs_hcsplt3::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT3 {}
#[doc = "OTG_HS host channel-3 split control register"]
pub mod otg_hs_hcsplt3;
#[doc = "OTG_HS host channel-4 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt4](otg_hs_hcsplt4) module"]
pub type OTG_HS_HCSPLT4 = crate::Reg<u32, _OTG_HS_HCSPLT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT4;
#[doc = "`read()` method returns [otg_hs_hcsplt4::R](otg_hs_hcsplt4::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt4::W](otg_hs_hcsplt4::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT4 {}
#[doc = "OTG_HS host channel-4 split control register"]
pub mod otg_hs_hcsplt4;
#[doc = "OTG_HS host channel-5 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt5](otg_hs_hcsplt5) module"]
pub type OTG_HS_HCSPLT5 = crate::Reg<u32, _OTG_HS_HCSPLT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT5;
#[doc = "`read()` method returns [otg_hs_hcsplt5::R](otg_hs_hcsplt5::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt5::W](otg_hs_hcsplt5::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT5 {}
#[doc = "OTG_HS host channel-5 split control register"]
pub mod otg_hs_hcsplt5;
#[doc = "OTG_HS host channel-6 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt6](otg_hs_hcsplt6) module"]
pub type OTG_HS_HCSPLT6 = crate::Reg<u32, _OTG_HS_HCSPLT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT6;
#[doc = "`read()` method returns [otg_hs_hcsplt6::R](otg_hs_hcsplt6::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt6::W](otg_hs_hcsplt6::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT6 {}
#[doc = "OTG_HS host channel-6 split control register"]
pub mod otg_hs_hcsplt6;
#[doc = "OTG_HS host channel-7 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt7](otg_hs_hcsplt7) module"]
pub type OTG_HS_HCSPLT7 = crate::Reg<u32, _OTG_HS_HCSPLT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT7;
#[doc = "`read()` method returns [otg_hs_hcsplt7::R](otg_hs_hcsplt7::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt7::W](otg_hs_hcsplt7::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT7 {}
#[doc = "OTG_HS host channel-7 split control register"]
pub mod otg_hs_hcsplt7;
#[doc = "OTG_HS host channel-8 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt8](otg_hs_hcsplt8) module"]
pub type OTG_HS_HCSPLT8 = crate::Reg<u32, _OTG_HS_HCSPLT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT8;
#[doc = "`read()` method returns [otg_hs_hcsplt8::R](otg_hs_hcsplt8::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt8::W](otg_hs_hcsplt8::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT8 {}
#[doc = "OTG_HS host channel-8 split control register"]
pub mod otg_hs_hcsplt8;
#[doc = "OTG_HS host channel-9 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt9](otg_hs_hcsplt9) module"]
pub type OTG_HS_HCSPLT9 = crate::Reg<u32, _OTG_HS_HCSPLT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT9;
#[doc = "`read()` method returns [otg_hs_hcsplt9::R](otg_hs_hcsplt9::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt9::W](otg_hs_hcsplt9::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT9 {}
#[doc = "OTG_HS host channel-9 split control register"]
pub mod otg_hs_hcsplt9;
#[doc = "OTG_HS host channel-10 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt10](otg_hs_hcsplt10) module"]
pub type OTG_HS_HCSPLT10 = crate::Reg<u32, _OTG_HS_HCSPLT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT10;
#[doc = "`read()` method returns [otg_hs_hcsplt10::R](otg_hs_hcsplt10::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt10::W](otg_hs_hcsplt10::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT10 {}
#[doc = "OTG_HS host channel-10 split control register"]
pub mod otg_hs_hcsplt10;
#[doc = "OTG_HS host channel-11 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcsplt11](otg_hs_hcsplt11) module"]
pub type OTG_HS_HCSPLT11 = crate::Reg<u32, _OTG_HS_HCSPLT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCSPLT11;
#[doc = "`read()` method returns [otg_hs_hcsplt11::R](otg_hs_hcsplt11::R) reader structure"]
impl crate::Readable for OTG_HS_HCSPLT11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcsplt11::W](otg_hs_hcsplt11::W) writer structure"]
impl crate::Writable for OTG_HS_HCSPLT11 {}
#[doc = "OTG_HS host channel-11 split control register"]
pub mod otg_hs_hcsplt11;
#[doc = "OTG_HS host channel-11 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint0](otg_hs_hcint0) module"]
pub type OTG_HS_HCINT0 = crate::Reg<u32, _OTG_HS_HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT0;
#[doc = "`read()` method returns [otg_hs_hcint0::R](otg_hs_hcint0::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint0::W](otg_hs_hcint0::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT0 {}
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod otg_hs_hcint0;
#[doc = "OTG_HS host channel-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint1](otg_hs_hcint1) module"]
pub type OTG_HS_HCINT1 = crate::Reg<u32, _OTG_HS_HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT1;
#[doc = "`read()` method returns [otg_hs_hcint1::R](otg_hs_hcint1::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint1::W](otg_hs_hcint1::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT1 {}
#[doc = "OTG_HS host channel-1 interrupt register"]
pub mod otg_hs_hcint1;
#[doc = "OTG_HS host channel-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint2](otg_hs_hcint2) module"]
pub type OTG_HS_HCINT2 = crate::Reg<u32, _OTG_HS_HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT2;
#[doc = "`read()` method returns [otg_hs_hcint2::R](otg_hs_hcint2::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint2::W](otg_hs_hcint2::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT2 {}
#[doc = "OTG_HS host channel-2 interrupt register"]
pub mod otg_hs_hcint2;
#[doc = "OTG_HS host channel-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint3](otg_hs_hcint3) module"]
pub type OTG_HS_HCINT3 = crate::Reg<u32, _OTG_HS_HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT3;
#[doc = "`read()` method returns [otg_hs_hcint3::R](otg_hs_hcint3::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint3::W](otg_hs_hcint3::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT3 {}
#[doc = "OTG_HS host channel-3 interrupt register"]
pub mod otg_hs_hcint3;
#[doc = "OTG_HS host channel-4 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint4](otg_hs_hcint4) module"]
pub type OTG_HS_HCINT4 = crate::Reg<u32, _OTG_HS_HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT4;
#[doc = "`read()` method returns [otg_hs_hcint4::R](otg_hs_hcint4::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint4::W](otg_hs_hcint4::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT4 {}
#[doc = "OTG_HS host channel-4 interrupt register"]
pub mod otg_hs_hcint4;
#[doc = "OTG_HS host channel-5 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint5](otg_hs_hcint5) module"]
pub type OTG_HS_HCINT5 = crate::Reg<u32, _OTG_HS_HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT5;
#[doc = "`read()` method returns [otg_hs_hcint5::R](otg_hs_hcint5::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint5::W](otg_hs_hcint5::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT5 {}
#[doc = "OTG_HS host channel-5 interrupt register"]
pub mod otg_hs_hcint5;
#[doc = "OTG_HS host channel-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint6](otg_hs_hcint6) module"]
pub type OTG_HS_HCINT6 = crate::Reg<u32, _OTG_HS_HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT6;
#[doc = "`read()` method returns [otg_hs_hcint6::R](otg_hs_hcint6::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint6::W](otg_hs_hcint6::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT6 {}
#[doc = "OTG_HS host channel-6 interrupt register"]
pub mod otg_hs_hcint6;
#[doc = "OTG_HS host channel-7 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint7](otg_hs_hcint7) module"]
pub type OTG_HS_HCINT7 = crate::Reg<u32, _OTG_HS_HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT7;
#[doc = "`read()` method returns [otg_hs_hcint7::R](otg_hs_hcint7::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint7::W](otg_hs_hcint7::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT7 {}
#[doc = "OTG_HS host channel-7 interrupt register"]
pub mod otg_hs_hcint7;
#[doc = "OTG_HS host channel-8 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint8](otg_hs_hcint8) module"]
pub type OTG_HS_HCINT8 = crate::Reg<u32, _OTG_HS_HCINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT8;
#[doc = "`read()` method returns [otg_hs_hcint8::R](otg_hs_hcint8::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint8::W](otg_hs_hcint8::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT8 {}
#[doc = "OTG_HS host channel-8 interrupt register"]
pub mod otg_hs_hcint8;
#[doc = "OTG_HS host channel-9 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint9](otg_hs_hcint9) module"]
pub type OTG_HS_HCINT9 = crate::Reg<u32, _OTG_HS_HCINT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT9;
#[doc = "`read()` method returns [otg_hs_hcint9::R](otg_hs_hcint9::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint9::W](otg_hs_hcint9::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT9 {}
#[doc = "OTG_HS host channel-9 interrupt register"]
pub mod otg_hs_hcint9;
#[doc = "OTG_HS host channel-10 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint10](otg_hs_hcint10) module"]
pub type OTG_HS_HCINT10 = crate::Reg<u32, _OTG_HS_HCINT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT10;
#[doc = "`read()` method returns [otg_hs_hcint10::R](otg_hs_hcint10::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint10::W](otg_hs_hcint10::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT10 {}
#[doc = "OTG_HS host channel-10 interrupt register"]
pub mod otg_hs_hcint10;
#[doc = "OTG_HS host channel-11 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcint11](otg_hs_hcint11) module"]
pub type OTG_HS_HCINT11 = crate::Reg<u32, _OTG_HS_HCINT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINT11;
#[doc = "`read()` method returns [otg_hs_hcint11::R](otg_hs_hcint11::R) reader structure"]
impl crate::Readable for OTG_HS_HCINT11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcint11::W](otg_hs_hcint11::W) writer structure"]
impl crate::Writable for OTG_HS_HCINT11 {}
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod otg_hs_hcint11;
#[doc = "OTG_HS host channel-11 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk0](otg_hs_hcintmsk0) module"]
pub type OTG_HS_HCINTMSK0 = crate::Reg<u32, _OTG_HS_HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK0;
#[doc = "`read()` method returns [otg_hs_hcintmsk0::R](otg_hs_hcintmsk0::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk0::W](otg_hs_hcintmsk0::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK0 {}
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod otg_hs_hcintmsk0;
#[doc = "OTG_HS host channel-1 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk1](otg_hs_hcintmsk1) module"]
pub type OTG_HS_HCINTMSK1 = crate::Reg<u32, _OTG_HS_HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK1;
#[doc = "`read()` method returns [otg_hs_hcintmsk1::R](otg_hs_hcintmsk1::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk1::W](otg_hs_hcintmsk1::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK1 {}
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub mod otg_hs_hcintmsk1;
#[doc = "OTG_HS host channel-2 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk2](otg_hs_hcintmsk2) module"]
pub type OTG_HS_HCINTMSK2 = crate::Reg<u32, _OTG_HS_HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK2;
#[doc = "`read()` method returns [otg_hs_hcintmsk2::R](otg_hs_hcintmsk2::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk2::W](otg_hs_hcintmsk2::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK2 {}
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub mod otg_hs_hcintmsk2;
#[doc = "OTG_HS host channel-3 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk3](otg_hs_hcintmsk3) module"]
pub type OTG_HS_HCINTMSK3 = crate::Reg<u32, _OTG_HS_HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK3;
#[doc = "`read()` method returns [otg_hs_hcintmsk3::R](otg_hs_hcintmsk3::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk3::W](otg_hs_hcintmsk3::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK3 {}
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub mod otg_hs_hcintmsk3;
#[doc = "OTG_HS host channel-4 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk4](otg_hs_hcintmsk4) module"]
pub type OTG_HS_HCINTMSK4 = crate::Reg<u32, _OTG_HS_HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK4;
#[doc = "`read()` method returns [otg_hs_hcintmsk4::R](otg_hs_hcintmsk4::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk4::W](otg_hs_hcintmsk4::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK4 {}
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub mod otg_hs_hcintmsk4;
#[doc = "OTG_HS host channel-5 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk5](otg_hs_hcintmsk5) module"]
pub type OTG_HS_HCINTMSK5 = crate::Reg<u32, _OTG_HS_HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK5;
#[doc = "`read()` method returns [otg_hs_hcintmsk5::R](otg_hs_hcintmsk5::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk5::W](otg_hs_hcintmsk5::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK5 {}
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub mod otg_hs_hcintmsk5;
#[doc = "OTG_HS host channel-6 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk6](otg_hs_hcintmsk6) module"]
pub type OTG_HS_HCINTMSK6 = crate::Reg<u32, _OTG_HS_HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK6;
#[doc = "`read()` method returns [otg_hs_hcintmsk6::R](otg_hs_hcintmsk6::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk6::W](otg_hs_hcintmsk6::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK6 {}
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub mod otg_hs_hcintmsk6;
#[doc = "OTG_HS host channel-7 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk7](otg_hs_hcintmsk7) module"]
pub type OTG_HS_HCINTMSK7 = crate::Reg<u32, _OTG_HS_HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK7;
#[doc = "`read()` method returns [otg_hs_hcintmsk7::R](otg_hs_hcintmsk7::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk7::W](otg_hs_hcintmsk7::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK7 {}
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub mod otg_hs_hcintmsk7;
#[doc = "OTG_HS host channel-8 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk8](otg_hs_hcintmsk8) module"]
pub type OTG_HS_HCINTMSK8 = crate::Reg<u32, _OTG_HS_HCINTMSK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK8;
#[doc = "`read()` method returns [otg_hs_hcintmsk8::R](otg_hs_hcintmsk8::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk8::W](otg_hs_hcintmsk8::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK8 {}
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub mod otg_hs_hcintmsk8;
#[doc = "OTG_HS host channel-9 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk9](otg_hs_hcintmsk9) module"]
pub type OTG_HS_HCINTMSK9 = crate::Reg<u32, _OTG_HS_HCINTMSK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK9;
#[doc = "`read()` method returns [otg_hs_hcintmsk9::R](otg_hs_hcintmsk9::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk9::W](otg_hs_hcintmsk9::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK9 {}
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub mod otg_hs_hcintmsk9;
#[doc = "OTG_HS host channel-10 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk10](otg_hs_hcintmsk10) module"]
pub type OTG_HS_HCINTMSK10 = crate::Reg<u32, _OTG_HS_HCINTMSK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK10;
#[doc = "`read()` method returns [otg_hs_hcintmsk10::R](otg_hs_hcintmsk10::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk10::W](otg_hs_hcintmsk10::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK10 {}
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub mod otg_hs_hcintmsk10;
#[doc = "OTG_HS host channel-11 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcintmsk11](otg_hs_hcintmsk11) module"]
pub type OTG_HS_HCINTMSK11 = crate::Reg<u32, _OTG_HS_HCINTMSK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCINTMSK11;
#[doc = "`read()` method returns [otg_hs_hcintmsk11::R](otg_hs_hcintmsk11::R) reader structure"]
impl crate::Readable for OTG_HS_HCINTMSK11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcintmsk11::W](otg_hs_hcintmsk11::W) writer structure"]
impl crate::Writable for OTG_HS_HCINTMSK11 {}
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod otg_hs_hcintmsk11;
#[doc = "OTG_HS host channel-11 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz0](otg_hs_hctsiz0) module"]
pub type OTG_HS_HCTSIZ0 = crate::Reg<u32, _OTG_HS_HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ0;
#[doc = "`read()` method returns [otg_hs_hctsiz0::R](otg_hs_hctsiz0::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz0::W](otg_hs_hctsiz0::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ0 {}
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod otg_hs_hctsiz0;
#[doc = "OTG_HS host channel-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz1](otg_hs_hctsiz1) module"]
pub type OTG_HS_HCTSIZ1 = crate::Reg<u32, _OTG_HS_HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ1;
#[doc = "`read()` method returns [otg_hs_hctsiz1::R](otg_hs_hctsiz1::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz1::W](otg_hs_hctsiz1::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ1 {}
#[doc = "OTG_HS host channel-1 transfer size register"]
pub mod otg_hs_hctsiz1;
#[doc = "OTG_HS host channel-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz2](otg_hs_hctsiz2) module"]
pub type OTG_HS_HCTSIZ2 = crate::Reg<u32, _OTG_HS_HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ2;
#[doc = "`read()` method returns [otg_hs_hctsiz2::R](otg_hs_hctsiz2::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz2::W](otg_hs_hctsiz2::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ2 {}
#[doc = "OTG_HS host channel-2 transfer size register"]
pub mod otg_hs_hctsiz2;
#[doc = "OTG_HS host channel-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz3](otg_hs_hctsiz3) module"]
pub type OTG_HS_HCTSIZ3 = crate::Reg<u32, _OTG_HS_HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ3;
#[doc = "`read()` method returns [otg_hs_hctsiz3::R](otg_hs_hctsiz3::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz3::W](otg_hs_hctsiz3::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ3 {}
#[doc = "OTG_HS host channel-3 transfer size register"]
pub mod otg_hs_hctsiz3;
#[doc = "OTG_HS host channel-4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz4](otg_hs_hctsiz4) module"]
pub type OTG_HS_HCTSIZ4 = crate::Reg<u32, _OTG_HS_HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ4;
#[doc = "`read()` method returns [otg_hs_hctsiz4::R](otg_hs_hctsiz4::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz4::W](otg_hs_hctsiz4::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ4 {}
#[doc = "OTG_HS host channel-4 transfer size register"]
pub mod otg_hs_hctsiz4;
#[doc = "OTG_HS host channel-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz5](otg_hs_hctsiz5) module"]
pub type OTG_HS_HCTSIZ5 = crate::Reg<u32, _OTG_HS_HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ5;
#[doc = "`read()` method returns [otg_hs_hctsiz5::R](otg_hs_hctsiz5::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz5::W](otg_hs_hctsiz5::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ5 {}
#[doc = "OTG_HS host channel-5 transfer size register"]
pub mod otg_hs_hctsiz5;
#[doc = "OTG_HS host channel-6 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz6](otg_hs_hctsiz6) module"]
pub type OTG_HS_HCTSIZ6 = crate::Reg<u32, _OTG_HS_HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ6;
#[doc = "`read()` method returns [otg_hs_hctsiz6::R](otg_hs_hctsiz6::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz6::W](otg_hs_hctsiz6::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ6 {}
#[doc = "OTG_HS host channel-6 transfer size register"]
pub mod otg_hs_hctsiz6;
#[doc = "OTG_HS host channel-7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz7](otg_hs_hctsiz7) module"]
pub type OTG_HS_HCTSIZ7 = crate::Reg<u32, _OTG_HS_HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ7;
#[doc = "`read()` method returns [otg_hs_hctsiz7::R](otg_hs_hctsiz7::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz7::W](otg_hs_hctsiz7::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ7 {}
#[doc = "OTG_HS host channel-7 transfer size register"]
pub mod otg_hs_hctsiz7;
#[doc = "OTG_HS host channel-8 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz8](otg_hs_hctsiz8) module"]
pub type OTG_HS_HCTSIZ8 = crate::Reg<u32, _OTG_HS_HCTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ8;
#[doc = "`read()` method returns [otg_hs_hctsiz8::R](otg_hs_hctsiz8::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz8::W](otg_hs_hctsiz8::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ8 {}
#[doc = "OTG_HS host channel-8 transfer size register"]
pub mod otg_hs_hctsiz8;
#[doc = "OTG_HS host channel-9 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz9](otg_hs_hctsiz9) module"]
pub type OTG_HS_HCTSIZ9 = crate::Reg<u32, _OTG_HS_HCTSIZ9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ9;
#[doc = "`read()` method returns [otg_hs_hctsiz9::R](otg_hs_hctsiz9::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz9::W](otg_hs_hctsiz9::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ9 {}
#[doc = "OTG_HS host channel-9 transfer size register"]
pub mod otg_hs_hctsiz9;
#[doc = "OTG_HS host channel-10 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz10](otg_hs_hctsiz10) module"]
pub type OTG_HS_HCTSIZ10 = crate::Reg<u32, _OTG_HS_HCTSIZ10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ10;
#[doc = "`read()` method returns [otg_hs_hctsiz10::R](otg_hs_hctsiz10::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz10::W](otg_hs_hctsiz10::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ10 {}
#[doc = "OTG_HS host channel-10 transfer size register"]
pub mod otg_hs_hctsiz10;
#[doc = "OTG_HS host channel-11 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hctsiz11](otg_hs_hctsiz11) module"]
pub type OTG_HS_HCTSIZ11 = crate::Reg<u32, _OTG_HS_HCTSIZ11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCTSIZ11;
#[doc = "`read()` method returns [otg_hs_hctsiz11::R](otg_hs_hctsiz11::R) reader structure"]
impl crate::Readable for OTG_HS_HCTSIZ11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hctsiz11::W](otg_hs_hctsiz11::W) writer structure"]
impl crate::Writable for OTG_HS_HCTSIZ11 {}
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod otg_hs_hctsiz11;
#[doc = "OTG_HS host channel-0 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma0](otg_hs_hcdma0) module"]
pub type OTG_HS_HCDMA0 = crate::Reg<u32, _OTG_HS_HCDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA0;
#[doc = "`read()` method returns [otg_hs_hcdma0::R](otg_hs_hcdma0::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA0 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma0::W](otg_hs_hcdma0::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA0 {}
#[doc = "OTG_HS host channel-0 DMA address register"]
pub mod otg_hs_hcdma0;
#[doc = "OTG_HS host channel-1 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma1](otg_hs_hcdma1) module"]
pub type OTG_HS_HCDMA1 = crate::Reg<u32, _OTG_HS_HCDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA1;
#[doc = "`read()` method returns [otg_hs_hcdma1::R](otg_hs_hcdma1::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma1::W](otg_hs_hcdma1::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA1 {}
#[doc = "OTG_HS host channel-1 DMA address register"]
pub mod otg_hs_hcdma1;
#[doc = "OTG_HS host channel-2 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma2](otg_hs_hcdma2) module"]
pub type OTG_HS_HCDMA2 = crate::Reg<u32, _OTG_HS_HCDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA2;
#[doc = "`read()` method returns [otg_hs_hcdma2::R](otg_hs_hcdma2::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA2 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma2::W](otg_hs_hcdma2::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA2 {}
#[doc = "OTG_HS host channel-2 DMA address register"]
pub mod otg_hs_hcdma2;
#[doc = "OTG_HS host channel-3 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma3](otg_hs_hcdma3) module"]
pub type OTG_HS_HCDMA3 = crate::Reg<u32, _OTG_HS_HCDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA3;
#[doc = "`read()` method returns [otg_hs_hcdma3::R](otg_hs_hcdma3::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA3 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma3::W](otg_hs_hcdma3::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA3 {}
#[doc = "OTG_HS host channel-3 DMA address register"]
pub mod otg_hs_hcdma3;
#[doc = "OTG_HS host channel-4 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma4](otg_hs_hcdma4) module"]
pub type OTG_HS_HCDMA4 = crate::Reg<u32, _OTG_HS_HCDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA4;
#[doc = "`read()` method returns [otg_hs_hcdma4::R](otg_hs_hcdma4::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA4 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma4::W](otg_hs_hcdma4::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA4 {}
#[doc = "OTG_HS host channel-4 DMA address register"]
pub mod otg_hs_hcdma4;
#[doc = "OTG_HS host channel-5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma5](otg_hs_hcdma5) module"]
pub type OTG_HS_HCDMA5 = crate::Reg<u32, _OTG_HS_HCDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA5;
#[doc = "`read()` method returns [otg_hs_hcdma5::R](otg_hs_hcdma5::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA5 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma5::W](otg_hs_hcdma5::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA5 {}
#[doc = "OTG_HS host channel-5 DMA address register"]
pub mod otg_hs_hcdma5;
#[doc = "OTG_HS host channel-6 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma6](otg_hs_hcdma6) module"]
pub type OTG_HS_HCDMA6 = crate::Reg<u32, _OTG_HS_HCDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA6;
#[doc = "`read()` method returns [otg_hs_hcdma6::R](otg_hs_hcdma6::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA6 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma6::W](otg_hs_hcdma6::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA6 {}
#[doc = "OTG_HS host channel-6 DMA address register"]
pub mod otg_hs_hcdma6;
#[doc = "OTG_HS host channel-7 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma7](otg_hs_hcdma7) module"]
pub type OTG_HS_HCDMA7 = crate::Reg<u32, _OTG_HS_HCDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA7;
#[doc = "`read()` method returns [otg_hs_hcdma7::R](otg_hs_hcdma7::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA7 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma7::W](otg_hs_hcdma7::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA7 {}
#[doc = "OTG_HS host channel-7 DMA address register"]
pub mod otg_hs_hcdma7;
#[doc = "OTG_HS host channel-8 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma8](otg_hs_hcdma8) module"]
pub type OTG_HS_HCDMA8 = crate::Reg<u32, _OTG_HS_HCDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA8;
#[doc = "`read()` method returns [otg_hs_hcdma8::R](otg_hs_hcdma8::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA8 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma8::W](otg_hs_hcdma8::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA8 {}
#[doc = "OTG_HS host channel-8 DMA address register"]
pub mod otg_hs_hcdma8;
#[doc = "OTG_HS host channel-9 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma9](otg_hs_hcdma9) module"]
pub type OTG_HS_HCDMA9 = crate::Reg<u32, _OTG_HS_HCDMA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA9;
#[doc = "`read()` method returns [otg_hs_hcdma9::R](otg_hs_hcdma9::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA9 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma9::W](otg_hs_hcdma9::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA9 {}
#[doc = "OTG_HS host channel-9 DMA address register"]
pub mod otg_hs_hcdma9;
#[doc = "OTG_HS host channel-10 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma10](otg_hs_hcdma10) module"]
pub type OTG_HS_HCDMA10 = crate::Reg<u32, _OTG_HS_HCDMA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA10;
#[doc = "`read()` method returns [otg_hs_hcdma10::R](otg_hs_hcdma10::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA10 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma10::W](otg_hs_hcdma10::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA10 {}
#[doc = "OTG_HS host channel-10 DMA address register"]
pub mod otg_hs_hcdma10;
#[doc = "OTG_HS host channel-11 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hcdma11](otg_hs_hcdma11) module"]
pub type OTG_HS_HCDMA11 = crate::Reg<u32, _OTG_HS_HCDMA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_HCDMA11;
#[doc = "`read()` method returns [otg_hs_hcdma11::R](otg_hs_hcdma11::R) reader structure"]
impl crate::Readable for OTG_HS_HCDMA11 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_hcdma11::W](otg_hs_hcdma11::W) writer structure"]
impl crate::Writable for OTG_HS_HCDMA11 {}
#[doc = "OTG_HS host channel-11 DMA address register"]
pub mod otg_hs_hcdma11;
