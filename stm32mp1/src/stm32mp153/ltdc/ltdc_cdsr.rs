#[doc = "Register `LTDC_CDSR` reader"]
pub type R = crate::R<LTDC_CDSRrs>;
#[doc = "Field `VDES` reader - VDES"]
pub type VDES_R = crate::BitReader;
#[doc = "Field `HDES` reader - HDES"]
pub type HDES_R = crate::BitReader;
#[doc = "Field `VSYNCS` reader - VSYNCS"]
pub type VSYNCS_R = crate::BitReader;
#[doc = "Field `HSYNCS` reader - HSYNCS"]
pub type HSYNCS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - VDES"]
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HDES"]
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VSYNCS"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSYNCS"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "This register returns the status of the current display phase which is controlled by the HSYNC, VSYNC, and horizontal/vertical DE signals. Example: if the current display phase is the vertical synchronization, the VSYNCS bit is set (active high). If the current display phase is the horizontal synchronization, the HSYNCS bit is active high.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_cdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
