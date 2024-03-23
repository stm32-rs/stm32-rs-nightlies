#[doc = "Register `PECR` reader"]
pub type R = crate::R<PECRrs>;
#[doc = "Field `PEC` reader - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
pub type PEC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C PEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PECRrs;
impl crate::RegisterSpec for PECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PECRrs {}
#[doc = "`reset()` method sets PECR to value 0"]
impl crate::Resettable for PECRrs {
    const RESET_VALUE: u32 = 0;
}
