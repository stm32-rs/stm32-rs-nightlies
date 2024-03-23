#[doc = "Register `VERR` reader"]
pub type R = crate::R<VERRrs>;
#[doc = "Field `MINREV` reader - Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2"]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major revision These bits returns the ADC IP major revision"]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Minor revision These bits returns the ADC IP minor revision 0002: Major revision = X.2"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major revision These bits returns the ADC IP major revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "ADC version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERRrs;
impl crate::RegisterSpec for VERRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verr::R`](R) reader structure"]
impl crate::Readable for VERRrs {}
#[doc = "`reset()` method sets VERR to value 0x12"]
impl crate::Resettable for VERRrs {
    const RESET_VALUE: u32 = 0x12;
}
