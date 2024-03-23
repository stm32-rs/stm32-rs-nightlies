#[doc = "Register `CDSR` reader"]
pub type R = crate::R<CDSRrs>;
#[doc = "Field `VDES` reader - Vertical Data Enable display Status"]
pub type VDES_R = crate::BitReader;
#[doc = "Field `HDES` reader - Horizontal Data Enable display Status"]
pub type HDES_R = crate::BitReader;
#[doc = "Field `VSYNCS` reader - Vertical Synchronization display Status"]
pub type VSYNCS_R = crate::BitReader;
#[doc = "Field `HSYNCS` reader - Horizontal Synchronization display Status"]
pub type HSYNCS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LTDC Current Display Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDSRrs;
impl crate::RegisterSpec for CDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdsr::R`](R) reader structure"]
impl crate::Readable for CDSRrs {}
#[doc = "`reset()` method sets CDSR to value 0x0f"]
impl crate::Resettable for CDSRrs {
    const RESET_VALUE: u32 = 0x0f;
}
