#[doc = "Register `PWR_SID` reader"]
pub type R = crate::R<PWR_SIDrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "PWR size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_sid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_SIDrs;
impl crate::RegisterSpec for PWR_SIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_sid::R`](R) reader structure"]
impl crate::Readable for PWR_SIDrs {}
#[doc = "`reset()` method sets PWR_SID to value 0xa3c5_dd01"]
impl crate::Resettable for PWR_SIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
