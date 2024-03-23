#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Field `PVU` reader - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Prescaler value can be updated only when PVU bit is reset."]
pub type PVU_R = crate::BitReader;
#[doc = "Field `RVU` reader - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Reload value can be updated only when RVU bit is reset."]
pub type RVU_R = crate::BitReader;
#[doc = "Field `WVU` reader - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Window value can be updated only when WVU bit is reset."]
pub type WVU_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Prescaler value can be updated only when PVU bit is reset."]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Reload value can be updated only when RVU bit is reset."]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to five LSI cycles). Window value can be updated only when WVU bit is reset."]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
