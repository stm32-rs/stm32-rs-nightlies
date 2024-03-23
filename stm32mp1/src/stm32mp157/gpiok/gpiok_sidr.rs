#[doc = "Register `GPIOK_SIDR` reader"]
pub type R = crate::R<GPIOK_SIDRrs>;
#[doc = "Field `SIDR` reader - SIDR"]
pub type SIDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SIDR"]
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new(self.bits)
    }
}
#[doc = "GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiok_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOK_SIDRrs;
impl crate::RegisterSpec for GPIOK_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiok_sidr::R`](R) reader structure"]
impl crate::Readable for GPIOK_SIDRrs {}
#[doc = "`reset()` method sets GPIOK_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for GPIOK_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
