#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    pub otg_fs_hcfg: OTG_FS_HCFG,
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    pub otg_fs_hfir: OTG_FS_HFIR,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    pub otg_fs_hfnum: OTG_FS_HFNUM,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    pub otg_fs_hptxsts: OTG_FS_HPTXSTS,
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    pub otg_fs_haint: OTG_FS_HAINT,
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    pub otg_fs_haintmsk: OTG_FS_HAINTMSK,
    _reserved6: [u8; 36usize],
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    pub otg_fs_hprt: OTG_FS_HPRT,
    _reserved7: [u8; 188usize],
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    pub otg_fs_hcchar0: OTG_FS_HCCHAR0,
    _reserved8: [u8; 4usize],
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    pub otg_fs_hcint0: OTG_FS_HCINT0,
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    pub otg_fs_hcintmsk0: OTG_FS_HCINTMSK0,
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    pub otg_fs_hctsiz0: OTG_FS_HCTSIZ0,
    _reserved11: [u8; 12usize],
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    pub otg_fs_hcchar1: OTG_FS_HCCHAR1,
    _reserved12: [u8; 4usize],
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    pub otg_fs_hcint1: OTG_FS_HCINT1,
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    pub otg_fs_hcintmsk1: OTG_FS_HCINTMSK1,
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    pub otg_fs_hctsiz1: OTG_FS_HCTSIZ1,
    _reserved15: [u8; 12usize],
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    pub otg_fs_hcchar2: OTG_FS_HCCHAR2,
    _reserved16: [u8; 4usize],
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    pub otg_fs_hcint2: OTG_FS_HCINT2,
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    pub otg_fs_hcintmsk2: OTG_FS_HCINTMSK2,
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    pub otg_fs_hctsiz2: OTG_FS_HCTSIZ2,
    _reserved19: [u8; 12usize],
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    pub otg_fs_hcchar3: OTG_FS_HCCHAR3,
    _reserved20: [u8; 4usize],
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    pub otg_fs_hcint3: OTG_FS_HCINT3,
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    pub otg_fs_hcintmsk3: OTG_FS_HCINTMSK3,
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    pub otg_fs_hctsiz3: OTG_FS_HCTSIZ3,
    _reserved23: [u8; 12usize],
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    pub otg_fs_hcchar4: OTG_FS_HCCHAR4,
    _reserved24: [u8; 4usize],
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    pub otg_fs_hcint4: OTG_FS_HCINT4,
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    pub otg_fs_hcintmsk4: OTG_FS_HCINTMSK4,
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    pub otg_fs_hctsiz4: OTG_FS_HCTSIZ4,
    _reserved27: [u8; 12usize],
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    pub otg_fs_hcchar5: OTG_FS_HCCHAR5,
    _reserved28: [u8; 4usize],
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    pub otg_fs_hcint5: OTG_FS_HCINT5,
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    pub otg_fs_hcintmsk5: OTG_FS_HCINTMSK5,
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    pub otg_fs_hctsiz5: OTG_FS_HCTSIZ5,
    _reserved31: [u8; 12usize],
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    pub otg_fs_hcchar6: OTG_FS_HCCHAR6,
    _reserved32: [u8; 4usize],
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    pub otg_fs_hcint6: OTG_FS_HCINT6,
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    pub otg_fs_hcintmsk6: OTG_FS_HCINTMSK6,
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    pub otg_fs_hctsiz6: OTG_FS_HCTSIZ6,
    _reserved35: [u8; 12usize],
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    pub otg_fs_hcchar7: OTG_FS_HCCHAR7,
    _reserved36: [u8; 4usize],
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    pub otg_fs_hcint7: OTG_FS_HCINT7,
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    pub otg_fs_hcintmsk7: OTG_FS_HCINTMSK7,
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    pub otg_fs_hctsiz7: OTG_FS_HCTSIZ7,
    #[doc = "0x1f4 - OTG_FS host channel-8 characteristics register"]
    pub otg_fs_hcchar8: OTG_FS_HCCHAR8,
    #[doc = "0x1f8 - OTG_FS host channel-8 interrupt register"]
    pub otg_fs_hcint8: OTG_FS_HCINT8,
    #[doc = "0x1fc - OTG_FS host channel-8 mask register"]
    pub otg_fs_hcintmsk8: OTG_FS_HCINTMSK8,
    #[doc = "0x200 - OTG_FS host channel-8 transfer size register"]
    pub otg_fs_hctsiz8: OTG_FS_HCTSIZ8,
    #[doc = "0x204 - OTG_FS host channel-9 characteristics register"]
    pub otg_fs_hcchar9: OTG_FS_HCCHAR9,
    #[doc = "0x208 - OTG_FS host channel-9 interrupt register"]
    pub otg_fs_hcint9: OTG_FS_HCINT9,
    #[doc = "0x20c - OTG_FS host channel-9 mask register"]
    pub otg_fs_hcintmsk9: OTG_FS_HCINTMSK9,
    #[doc = "0x210 - OTG_FS host channel-9 transfer size register"]
    pub otg_fs_hctsiz9: OTG_FS_HCTSIZ9,
    #[doc = "0x214 - OTG_FS host channel-10 characteristics register"]
    pub otg_fs_hcchar10: OTG_FS_HCCHAR10,
    #[doc = "0x218 - OTG_FS host channel-10 interrupt register"]
    pub otg_fs_hcint10: OTG_FS_HCINT10,
    #[doc = "0x21c - OTG_FS host channel-10 mask register"]
    pub otg_fs_hcintmsk10: OTG_FS_HCINTMSK10,
    #[doc = "0x220 - OTG_FS host channel-10 transfer size register"]
    pub otg_fs_hctsiz10: OTG_FS_HCTSIZ10,
    #[doc = "0x224 - OTG_FS host channel-11 characteristics register"]
    pub otg_fs_hcchar11: OTG_FS_HCCHAR11,
    #[doc = "0x228 - OTG_FS host channel-11 interrupt register"]
    pub otg_fs_hcint11: OTG_FS_HCINT11,
    #[doc = "0x22c - OTG_FS host channel-11 mask register"]
    pub otg_fs_hcintmsk11: OTG_FS_HCINTMSK11,
    #[doc = "0x230 - OTG_FS host channel-11 transfer size register"]
    pub otg_fs_hctsiz11: OTG_FS_HCTSIZ11,
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcfg](otg_fs_hcfg) module"]
pub type OTG_FS_HCFG = crate::Reg<u32, _OTG_FS_HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCFG;
#[doc = "`read()` method returns [otg_fs_hcfg::R](otg_fs_hcfg::R) reader structure"]
impl crate::Readable for OTG_FS_HCFG {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcfg::W](otg_fs_hcfg::W) writer structure"]
impl crate::Writable for OTG_FS_HCFG {}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod otg_fs_hcfg;
#[doc = "OTG_FS Host frame interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hfir](otg_fs_hfir) module"]
pub type OTG_FS_HFIR = crate::Reg<u32, _OTG_FS_HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HFIR;
#[doc = "`read()` method returns [otg_fs_hfir::R](otg_fs_hfir::R) reader structure"]
impl crate::Readable for OTG_FS_HFIR {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hfir::W](otg_fs_hfir::W) writer structure"]
impl crate::Writable for OTG_FS_HFIR {}
#[doc = "OTG_FS Host frame interval register"]
pub mod otg_fs_hfir;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hfnum](otg_fs_hfnum) module"]
pub type OTG_FS_HFNUM = crate::Reg<u32, _OTG_FS_HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HFNUM;
#[doc = "`read()` method returns [otg_fs_hfnum::R](otg_fs_hfnum::R) reader structure"]
impl crate::Readable for OTG_FS_HFNUM {}
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod otg_fs_hfnum;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hptxsts](otg_fs_hptxsts) module"]
pub type OTG_FS_HPTXSTS = crate::Reg<u32, _OTG_FS_HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HPTXSTS;
#[doc = "`read()` method returns [otg_fs_hptxsts::R](otg_fs_hptxsts::R) reader structure"]
impl crate::Readable for OTG_FS_HPTXSTS {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hptxsts::W](otg_fs_hptxsts::W) writer structure"]
impl crate::Writable for OTG_FS_HPTXSTS {}
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod otg_fs_hptxsts;
#[doc = "OTG_FS Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_haint](otg_fs_haint) module"]
pub type OTG_FS_HAINT = crate::Reg<u32, _OTG_FS_HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HAINT;
#[doc = "`read()` method returns [otg_fs_haint::R](otg_fs_haint::R) reader structure"]
impl crate::Readable for OTG_FS_HAINT {}
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod otg_fs_haint;
#[doc = "OTG_FS host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_haintmsk](otg_fs_haintmsk) module"]
pub type OTG_FS_HAINTMSK = crate::Reg<u32, _OTG_FS_HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HAINTMSK;
#[doc = "`read()` method returns [otg_fs_haintmsk::R](otg_fs_haintmsk::R) reader structure"]
impl crate::Readable for OTG_FS_HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_fs_haintmsk::W](otg_fs_haintmsk::W) writer structure"]
impl crate::Writable for OTG_FS_HAINTMSK {}
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod otg_fs_haintmsk;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hprt](otg_fs_hprt) module"]
pub type OTG_FS_HPRT = crate::Reg<u32, _OTG_FS_HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HPRT;
#[doc = "`read()` method returns [otg_fs_hprt::R](otg_fs_hprt::R) reader structure"]
impl crate::Readable for OTG_FS_HPRT {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hprt::W](otg_fs_hprt::W) writer structure"]
impl crate::Writable for OTG_FS_HPRT {}
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod otg_fs_hprt;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar0](otg_fs_hcchar0) module"]
pub type OTG_FS_HCCHAR0 = crate::Reg<u32, _OTG_FS_HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR0;
#[doc = "`read()` method returns [otg_fs_hcchar0::R](otg_fs_hcchar0::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar0::W](otg_fs_hcchar0::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR0 {}
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod otg_fs_hcchar0;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar1](otg_fs_hcchar1) module"]
pub type OTG_FS_HCCHAR1 = crate::Reg<u32, _OTG_FS_HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR1;
#[doc = "`read()` method returns [otg_fs_hcchar1::R](otg_fs_hcchar1::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar1::W](otg_fs_hcchar1::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR1 {}
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod otg_fs_hcchar1;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar2](otg_fs_hcchar2) module"]
pub type OTG_FS_HCCHAR2 = crate::Reg<u32, _OTG_FS_HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR2;
#[doc = "`read()` method returns [otg_fs_hcchar2::R](otg_fs_hcchar2::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar2::W](otg_fs_hcchar2::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR2 {}
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod otg_fs_hcchar2;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar3](otg_fs_hcchar3) module"]
pub type OTG_FS_HCCHAR3 = crate::Reg<u32, _OTG_FS_HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR3;
#[doc = "`read()` method returns [otg_fs_hcchar3::R](otg_fs_hcchar3::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar3::W](otg_fs_hcchar3::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR3 {}
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod otg_fs_hcchar3;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar4](otg_fs_hcchar4) module"]
pub type OTG_FS_HCCHAR4 = crate::Reg<u32, _OTG_FS_HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR4;
#[doc = "`read()` method returns [otg_fs_hcchar4::R](otg_fs_hcchar4::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar4::W](otg_fs_hcchar4::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR4 {}
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod otg_fs_hcchar4;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar5](otg_fs_hcchar5) module"]
pub type OTG_FS_HCCHAR5 = crate::Reg<u32, _OTG_FS_HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR5;
#[doc = "`read()` method returns [otg_fs_hcchar5::R](otg_fs_hcchar5::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar5::W](otg_fs_hcchar5::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR5 {}
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod otg_fs_hcchar5;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar6](otg_fs_hcchar6) module"]
pub type OTG_FS_HCCHAR6 = crate::Reg<u32, _OTG_FS_HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR6;
#[doc = "`read()` method returns [otg_fs_hcchar6::R](otg_fs_hcchar6::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR6 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar6::W](otg_fs_hcchar6::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR6 {}
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod otg_fs_hcchar6;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar7](otg_fs_hcchar7) module"]
pub type OTG_FS_HCCHAR7 = crate::Reg<u32, _OTG_FS_HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR7;
#[doc = "`read()` method returns [otg_fs_hcchar7::R](otg_fs_hcchar7::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR7 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar7::W](otg_fs_hcchar7::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR7 {}
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod otg_fs_hcchar7;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint0](otg_fs_hcint0) module"]
pub type OTG_FS_HCINT0 = crate::Reg<u32, _OTG_FS_HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT0;
#[doc = "`read()` method returns [otg_fs_hcint0::R](otg_fs_hcint0::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint0::W](otg_fs_hcint0::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT0 {}
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod otg_fs_hcint0;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint1](otg_fs_hcint1) module"]
pub type OTG_FS_HCINT1 = crate::Reg<u32, _OTG_FS_HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT1;
#[doc = "`read()` method returns [otg_fs_hcint1::R](otg_fs_hcint1::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint1::W](otg_fs_hcint1::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT1 {}
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod otg_fs_hcint1;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint2](otg_fs_hcint2) module"]
pub type OTG_FS_HCINT2 = crate::Reg<u32, _OTG_FS_HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT2;
#[doc = "`read()` method returns [otg_fs_hcint2::R](otg_fs_hcint2::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint2::W](otg_fs_hcint2::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT2 {}
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod otg_fs_hcint2;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint3](otg_fs_hcint3) module"]
pub type OTG_FS_HCINT3 = crate::Reg<u32, _OTG_FS_HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT3;
#[doc = "`read()` method returns [otg_fs_hcint3::R](otg_fs_hcint3::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint3::W](otg_fs_hcint3::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT3 {}
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod otg_fs_hcint3;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint4](otg_fs_hcint4) module"]
pub type OTG_FS_HCINT4 = crate::Reg<u32, _OTG_FS_HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT4;
#[doc = "`read()` method returns [otg_fs_hcint4::R](otg_fs_hcint4::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint4::W](otg_fs_hcint4::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT4 {}
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod otg_fs_hcint4;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint5](otg_fs_hcint5) module"]
pub type OTG_FS_HCINT5 = crate::Reg<u32, _OTG_FS_HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT5;
#[doc = "`read()` method returns [otg_fs_hcint5::R](otg_fs_hcint5::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint5::W](otg_fs_hcint5::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT5 {}
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod otg_fs_hcint5;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint6](otg_fs_hcint6) module"]
pub type OTG_FS_HCINT6 = crate::Reg<u32, _OTG_FS_HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT6;
#[doc = "`read()` method returns [otg_fs_hcint6::R](otg_fs_hcint6::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint6::W](otg_fs_hcint6::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT6 {}
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod otg_fs_hcint6;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint7](otg_fs_hcint7) module"]
pub type OTG_FS_HCINT7 = crate::Reg<u32, _OTG_FS_HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT7;
#[doc = "`read()` method returns [otg_fs_hcint7::R](otg_fs_hcint7::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint7::W](otg_fs_hcint7::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT7 {}
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod otg_fs_hcint7;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk0](otg_fs_hcintmsk0) module"]
pub type OTG_FS_HCINTMSK0 = crate::Reg<u32, _OTG_FS_HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK0;
#[doc = "`read()` method returns [otg_fs_hcintmsk0::R](otg_fs_hcintmsk0::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk0::W](otg_fs_hcintmsk0::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK0 {}
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod otg_fs_hcintmsk0;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk1](otg_fs_hcintmsk1) module"]
pub type OTG_FS_HCINTMSK1 = crate::Reg<u32, _OTG_FS_HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK1;
#[doc = "`read()` method returns [otg_fs_hcintmsk1::R](otg_fs_hcintmsk1::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk1::W](otg_fs_hcintmsk1::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK1 {}
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod otg_fs_hcintmsk1;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk2](otg_fs_hcintmsk2) module"]
pub type OTG_FS_HCINTMSK2 = crate::Reg<u32, _OTG_FS_HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK2;
#[doc = "`read()` method returns [otg_fs_hcintmsk2::R](otg_fs_hcintmsk2::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk2::W](otg_fs_hcintmsk2::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK2 {}
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod otg_fs_hcintmsk2;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk3](otg_fs_hcintmsk3) module"]
pub type OTG_FS_HCINTMSK3 = crate::Reg<u32, _OTG_FS_HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK3;
#[doc = "`read()` method returns [otg_fs_hcintmsk3::R](otg_fs_hcintmsk3::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk3::W](otg_fs_hcintmsk3::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK3 {}
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod otg_fs_hcintmsk3;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk4](otg_fs_hcintmsk4) module"]
pub type OTG_FS_HCINTMSK4 = crate::Reg<u32, _OTG_FS_HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK4;
#[doc = "`read()` method returns [otg_fs_hcintmsk4::R](otg_fs_hcintmsk4::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk4::W](otg_fs_hcintmsk4::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK4 {}
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod otg_fs_hcintmsk4;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk5](otg_fs_hcintmsk5) module"]
pub type OTG_FS_HCINTMSK5 = crate::Reg<u32, _OTG_FS_HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK5;
#[doc = "`read()` method returns [otg_fs_hcintmsk5::R](otg_fs_hcintmsk5::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk5::W](otg_fs_hcintmsk5::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK5 {}
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod otg_fs_hcintmsk5;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk6](otg_fs_hcintmsk6) module"]
pub type OTG_FS_HCINTMSK6 = crate::Reg<u32, _OTG_FS_HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK6;
#[doc = "`read()` method returns [otg_fs_hcintmsk6::R](otg_fs_hcintmsk6::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK6 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk6::W](otg_fs_hcintmsk6::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK6 {}
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod otg_fs_hcintmsk6;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk7](otg_fs_hcintmsk7) module"]
pub type OTG_FS_HCINTMSK7 = crate::Reg<u32, _OTG_FS_HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK7;
#[doc = "`read()` method returns [otg_fs_hcintmsk7::R](otg_fs_hcintmsk7::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK7 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk7::W](otg_fs_hcintmsk7::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK7 {}
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod otg_fs_hcintmsk7;
#[doc = "OTG_FS host channel-0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz0](otg_fs_hctsiz0) module"]
pub type OTG_FS_HCTSIZ0 = crate::Reg<u32, _OTG_FS_HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ0;
#[doc = "`read()` method returns [otg_fs_hctsiz0::R](otg_fs_hctsiz0::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz0::W](otg_fs_hctsiz0::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ0 {}
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod otg_fs_hctsiz0;
#[doc = "OTG_FS host channel-1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz1](otg_fs_hctsiz1) module"]
pub type OTG_FS_HCTSIZ1 = crate::Reg<u32, _OTG_FS_HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ1;
#[doc = "`read()` method returns [otg_fs_hctsiz1::R](otg_fs_hctsiz1::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz1::W](otg_fs_hctsiz1::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ1 {}
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod otg_fs_hctsiz1;
#[doc = "OTG_FS host channel-2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz2](otg_fs_hctsiz2) module"]
pub type OTG_FS_HCTSIZ2 = crate::Reg<u32, _OTG_FS_HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ2;
#[doc = "`read()` method returns [otg_fs_hctsiz2::R](otg_fs_hctsiz2::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz2::W](otg_fs_hctsiz2::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ2 {}
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod otg_fs_hctsiz2;
#[doc = "OTG_FS host channel-3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz3](otg_fs_hctsiz3) module"]
pub type OTG_FS_HCTSIZ3 = crate::Reg<u32, _OTG_FS_HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ3;
#[doc = "`read()` method returns [otg_fs_hctsiz3::R](otg_fs_hctsiz3::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz3::W](otg_fs_hctsiz3::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ3 {}
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod otg_fs_hctsiz3;
#[doc = "OTG_FS host channel-x transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz4](otg_fs_hctsiz4) module"]
pub type OTG_FS_HCTSIZ4 = crate::Reg<u32, _OTG_FS_HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ4;
#[doc = "`read()` method returns [otg_fs_hctsiz4::R](otg_fs_hctsiz4::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz4::W](otg_fs_hctsiz4::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ4 {}
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod otg_fs_hctsiz4;
#[doc = "OTG_FS host channel-5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz5](otg_fs_hctsiz5) module"]
pub type OTG_FS_HCTSIZ5 = crate::Reg<u32, _OTG_FS_HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ5;
#[doc = "`read()` method returns [otg_fs_hctsiz5::R](otg_fs_hctsiz5::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz5::W](otg_fs_hctsiz5::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ5 {}
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod otg_fs_hctsiz5;
#[doc = "OTG_FS host channel-6 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz6](otg_fs_hctsiz6) module"]
pub type OTG_FS_HCTSIZ6 = crate::Reg<u32, _OTG_FS_HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ6;
#[doc = "`read()` method returns [otg_fs_hctsiz6::R](otg_fs_hctsiz6::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz6::W](otg_fs_hctsiz6::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ6 {}
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod otg_fs_hctsiz6;
#[doc = "OTG_FS host channel-7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz7](otg_fs_hctsiz7) module"]
pub type OTG_FS_HCTSIZ7 = crate::Reg<u32, _OTG_FS_HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ7;
#[doc = "`read()` method returns [otg_fs_hctsiz7::R](otg_fs_hctsiz7::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz7::W](otg_fs_hctsiz7::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ7 {}
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod otg_fs_hctsiz7;
#[doc = "OTG_FS host channel-8 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar8](otg_fs_hcchar8) module"]
pub type OTG_FS_HCCHAR8 = crate::Reg<u32, _OTG_FS_HCCHAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR8;
#[doc = "`read()` method returns [otg_fs_hcchar8::R](otg_fs_hcchar8::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR8 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar8::W](otg_fs_hcchar8::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR8 {}
#[doc = "OTG_FS host channel-8 characteristics register"]
pub mod otg_fs_hcchar8;
#[doc = "OTG_FS host channel-8 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint8](otg_fs_hcint8) module"]
pub type OTG_FS_HCINT8 = crate::Reg<u32, _OTG_FS_HCINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT8;
#[doc = "`read()` method returns [otg_fs_hcint8::R](otg_fs_hcint8::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT8 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint8::W](otg_fs_hcint8::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT8 {}
#[doc = "OTG_FS host channel-8 interrupt register"]
pub mod otg_fs_hcint8;
#[doc = "OTG_FS host channel-8 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk8](otg_fs_hcintmsk8) module"]
pub type OTG_FS_HCINTMSK8 = crate::Reg<u32, _OTG_FS_HCINTMSK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK8;
#[doc = "`read()` method returns [otg_fs_hcintmsk8::R](otg_fs_hcintmsk8::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK8 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk8::W](otg_fs_hcintmsk8::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK8 {}
#[doc = "OTG_FS host channel-8 mask register"]
pub mod otg_fs_hcintmsk8;
#[doc = "OTG_FS host channel-8 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz8](otg_fs_hctsiz8) module"]
pub type OTG_FS_HCTSIZ8 = crate::Reg<u32, _OTG_FS_HCTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ8;
#[doc = "`read()` method returns [otg_fs_hctsiz8::R](otg_fs_hctsiz8::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ8 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz8::W](otg_fs_hctsiz8::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ8 {}
#[doc = "OTG_FS host channel-8 transfer size register"]
pub mod otg_fs_hctsiz8;
#[doc = "OTG_FS host channel-9 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar9](otg_fs_hcchar9) module"]
pub type OTG_FS_HCCHAR9 = crate::Reg<u32, _OTG_FS_HCCHAR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR9;
#[doc = "`read()` method returns [otg_fs_hcchar9::R](otg_fs_hcchar9::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR9 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar9::W](otg_fs_hcchar9::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR9 {}
#[doc = "OTG_FS host channel-9 characteristics register"]
pub mod otg_fs_hcchar9;
#[doc = "OTG_FS host channel-9 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint9](otg_fs_hcint9) module"]
pub type OTG_FS_HCINT9 = crate::Reg<u32, _OTG_FS_HCINT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT9;
#[doc = "`read()` method returns [otg_fs_hcint9::R](otg_fs_hcint9::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT9 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint9::W](otg_fs_hcint9::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT9 {}
#[doc = "OTG_FS host channel-9 interrupt register"]
pub mod otg_fs_hcint9;
#[doc = "OTG_FS host channel-9 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk9](otg_fs_hcintmsk9) module"]
pub type OTG_FS_HCINTMSK9 = crate::Reg<u32, _OTG_FS_HCINTMSK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK9;
#[doc = "`read()` method returns [otg_fs_hcintmsk9::R](otg_fs_hcintmsk9::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK9 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk9::W](otg_fs_hcintmsk9::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK9 {}
#[doc = "OTG_FS host channel-9 mask register"]
pub mod otg_fs_hcintmsk9;
#[doc = "OTG_FS host channel-9 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz9](otg_fs_hctsiz9) module"]
pub type OTG_FS_HCTSIZ9 = crate::Reg<u32, _OTG_FS_HCTSIZ9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ9;
#[doc = "`read()` method returns [otg_fs_hctsiz9::R](otg_fs_hctsiz9::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ9 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz9::W](otg_fs_hctsiz9::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ9 {}
#[doc = "OTG_FS host channel-9 transfer size register"]
pub mod otg_fs_hctsiz9;
#[doc = "OTG_FS host channel-10 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar10](otg_fs_hcchar10) module"]
pub type OTG_FS_HCCHAR10 = crate::Reg<u32, _OTG_FS_HCCHAR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR10;
#[doc = "`read()` method returns [otg_fs_hcchar10::R](otg_fs_hcchar10::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR10 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar10::W](otg_fs_hcchar10::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR10 {}
#[doc = "OTG_FS host channel-10 characteristics register"]
pub mod otg_fs_hcchar10;
#[doc = "OTG_FS host channel-10 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint10](otg_fs_hcint10) module"]
pub type OTG_FS_HCINT10 = crate::Reg<u32, _OTG_FS_HCINT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT10;
#[doc = "`read()` method returns [otg_fs_hcint10::R](otg_fs_hcint10::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT10 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint10::W](otg_fs_hcint10::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT10 {}
#[doc = "OTG_FS host channel-10 interrupt register"]
pub mod otg_fs_hcint10;
#[doc = "OTG_FS host channel-10 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk10](otg_fs_hcintmsk10) module"]
pub type OTG_FS_HCINTMSK10 = crate::Reg<u32, _OTG_FS_HCINTMSK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK10;
#[doc = "`read()` method returns [otg_fs_hcintmsk10::R](otg_fs_hcintmsk10::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK10 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk10::W](otg_fs_hcintmsk10::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK10 {}
#[doc = "OTG_FS host channel-10 mask register"]
pub mod otg_fs_hcintmsk10;
#[doc = "OTG_FS host channel-10 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz10](otg_fs_hctsiz10) module"]
pub type OTG_FS_HCTSIZ10 = crate::Reg<u32, _OTG_FS_HCTSIZ10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ10;
#[doc = "`read()` method returns [otg_fs_hctsiz10::R](otg_fs_hctsiz10::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ10 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz10::W](otg_fs_hctsiz10::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ10 {}
#[doc = "OTG_FS host channel-10 transfer size register"]
pub mod otg_fs_hctsiz10;
#[doc = "OTG_FS host channel-11 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcchar11](otg_fs_hcchar11) module"]
pub type OTG_FS_HCCHAR11 = crate::Reg<u32, _OTG_FS_HCCHAR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCCHAR11;
#[doc = "`read()` method returns [otg_fs_hcchar11::R](otg_fs_hcchar11::R) reader structure"]
impl crate::Readable for OTG_FS_HCCHAR11 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcchar11::W](otg_fs_hcchar11::W) writer structure"]
impl crate::Writable for OTG_FS_HCCHAR11 {}
#[doc = "OTG_FS host channel-11 characteristics register"]
pub mod otg_fs_hcchar11;
#[doc = "OTG_FS host channel-11 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcint11](otg_fs_hcint11) module"]
pub type OTG_FS_HCINT11 = crate::Reg<u32, _OTG_FS_HCINT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINT11;
#[doc = "`read()` method returns [otg_fs_hcint11::R](otg_fs_hcint11::R) reader structure"]
impl crate::Readable for OTG_FS_HCINT11 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcint11::W](otg_fs_hcint11::W) writer structure"]
impl crate::Writable for OTG_FS_HCINT11 {}
#[doc = "OTG_FS host channel-11 interrupt register"]
pub mod otg_fs_hcint11;
#[doc = "OTG_FS host channel-11 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hcintmsk11](otg_fs_hcintmsk11) module"]
pub type OTG_FS_HCINTMSK11 = crate::Reg<u32, _OTG_FS_HCINTMSK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCINTMSK11;
#[doc = "`read()` method returns [otg_fs_hcintmsk11::R](otg_fs_hcintmsk11::R) reader structure"]
impl crate::Readable for OTG_FS_HCINTMSK11 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hcintmsk11::W](otg_fs_hcintmsk11::W) writer structure"]
impl crate::Writable for OTG_FS_HCINTMSK11 {}
#[doc = "OTG_FS host channel-11 mask register"]
pub mod otg_fs_hcintmsk11;
#[doc = "OTG_FS host channel-11 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_hctsiz11](otg_fs_hctsiz11) module"]
pub type OTG_FS_HCTSIZ11 = crate::Reg<u32, _OTG_FS_HCTSIZ11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_FS_HCTSIZ11;
#[doc = "`read()` method returns [otg_fs_hctsiz11::R](otg_fs_hctsiz11::R) reader structure"]
impl crate::Readable for OTG_FS_HCTSIZ11 {}
#[doc = "`write(|w| ..)` method takes [otg_fs_hctsiz11::W](otg_fs_hctsiz11::W) writer structure"]
impl crate::Writable for OTG_FS_HCTSIZ11 {}
#[doc = "OTG_FS host channel-11 transfer size register"]
pub mod otg_fs_hctsiz11;
