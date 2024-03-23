#[doc = "Register `GPIOG_SIDR` reader"]
pub type R = crate::R<GPIOG_SIDRrs>;
#[doc = "Field `SIDR` reader - SIDR"]
pub type SIDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SIDR"]
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new(self.bits)
    }
}
#[doc = "GPIO size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiog_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOG_SIDRrs;
impl crate::RegisterSpec for GPIOG_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiog_sidr::R`](R) reader structure"]
impl crate::Readable for GPIOG_SIDRrs {}
#[doc = "`reset()` method sets GPIOG_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for GPIOG_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
