#[doc = "Register `RCC_SIDR` reader"]
pub type R = crate::R<RCC_SIDRrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "This register gives the decoding space, which is for the RCC of 4 kB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_SIDRrs;
impl crate::RegisterSpec for RCC_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_sidr::R`](R) reader structure"]
impl crate::Readable for RCC_SIDRrs {}
#[doc = "`reset()` method sets RCC_SIDR to value 0xa3c5_dd04"]
impl crate::Resettable for RCC_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd04;
}
