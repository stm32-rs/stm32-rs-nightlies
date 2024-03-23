#[doc = "Register `LTDC_CDSR` reader"]
pub type R = crate::R<LTDC_CDSRrs>;
#[doc = "Field `VDES` reader - vertical data enable display status"]
pub type VDES_R = crate::BitReader;
#[doc = "Field `HDES` reader - horizontal data enable display status"]
pub type HDES_R = crate::BitReader;
#[doc = "Field `VSYNCS` reader - vertical synchronization display status"]
pub type VSYNCS_R = crate::BitReader;
#[doc = "Field `HSYNCS` reader - horizontal synchronization display status"]
pub type HSYNCS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - vertical data enable display status"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - horizontal data enable display status"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - vertical synchronization display status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - horizontal synchronization display status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "LTDC current display status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_cdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_CDSRrs;
impl crate::RegisterSpec for LTDC_CDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_cdsr::R`](R) reader structure"]
impl crate::Readable for LTDC_CDSRrs {}
#[doc = "`reset()` method sets LTDC_CDSR to value 0x0f"]
impl crate::Resettable for LTDC_CDSRrs {
    const RESET_VALUE: u32 = 0x0f;
}
