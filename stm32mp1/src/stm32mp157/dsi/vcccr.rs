#[doc = "Register `VCCCR` reader"]
pub type R = crate::R<VCCCRrs>;
#[doc = "Field `NUMC` reader - NUMC"]
pub type NUMC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video chunks current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vcccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VCCCRrs;
impl crate::RegisterSpec for VCCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vcccr::R`](R) reader structure"]
impl crate::Readable for VCCCRrs {}
#[doc = "`reset()` method sets VCCCR to value 0"]
impl crate::Resettable for VCCCRrs {
    const RESET_VALUE: u32 = 0;
}
