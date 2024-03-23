#[doc = "Register `K2LR` writer"]
pub type W = crate::W<K2LRrs>;
#[doc = "Field `b` writer - b96"]
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - b96"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<K2LRrs> {
        B_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k2lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct K2LRrs;
impl crate::RegisterSpec for K2LRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`k2lr::W`](W) writer structure"]
impl crate::Writable for K2LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets K2LR to value 0"]
impl crate::Resettable for K2LRrs {
    const RESET_VALUE: u32 = 0;
}
