#[doc = "Register `SSR` reader"]
pub type R = crate::R<SSRrs>;
#[doc = "Field `SS` reader - Synchronous binary counter"]
pub type SS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Synchronous binary counter"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
#[doc = "Sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSRrs;
impl crate::RegisterSpec for SSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssr::R`](R) reader structure"]
impl crate::Readable for SSRrs {}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SSRrs {
    const RESET_VALUE: u32 = 0;
}
