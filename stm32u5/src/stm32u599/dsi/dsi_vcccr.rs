#[doc = "Register `DSI_VCCCR` reader"]
pub type R = crate::R<DSI_VCCCRrs>;
#[doc = "Field `NUMC` reader - Number of chunks This field returns the number of chunks being transmitted during a line period."]
pub type NUMC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Number of chunks This field returns the number of chunks being transmitted during a line period."]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vcccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VCCCRrs;
impl crate::RegisterSpec for DSI_VCCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vcccr::R`](R) reader structure"]
impl crate::Readable for DSI_VCCCRrs {}
#[doc = "`reset()` method sets DSI_VCCCR to value 0"]
impl crate::Resettable for DSI_VCCCRrs {
    const RESET_VALUE: u32 = 0;
}
