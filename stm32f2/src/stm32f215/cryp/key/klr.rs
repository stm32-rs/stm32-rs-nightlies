#[doc = "Register `KLR` writer"]
pub type W = crate::W<KLRrs>;
#[doc = "Field `b2` writer - b224"]
pub type B2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - b224"]
    #[inline(always)]
    #[must_use]
    pub fn b2(&mut self) -> B2_W<KLRrs> {
        B2_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`klr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KLRrs;
impl crate::RegisterSpec for KLRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`klr::W`](W) writer structure"]
impl crate::Writable for KLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KLR to value 0"]
impl crate::Resettable for KLRrs {
    const RESET_VALUE: u32 = 0;
}
