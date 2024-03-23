#[doc = "Register `HWCFGR` reader"]
pub type R = crate::R<HWCFGRrs>;
#[doc = "Field `TECHNO` reader - TECHNO"]
pub type TECHNO_R = crate::FieldReader;
#[doc = "Field `FIFOSIZE` reader - FIFOSIZE"]
pub type FIFOSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - TECHNO"]
    #[inline(always)]
    pub fn techno(&self) -> TECHNO_R {
        TECHNO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - FIFOSIZE"]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
#[doc = "DSI Host hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGRrs;
impl crate::RegisterSpec for HWCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr::R`](R) reader structure"]
impl crate::Readable for HWCFGRrs {}
#[doc = "`reset()` method sets HWCFGR to value 0x5a01"]
impl crate::Resettable for HWCFGRrs {
    const RESET_VALUE: u32 = 0x5a01;
}
