#[doc = "Register `K0LR` writer"]
pub type W = crate::W<K0LRrs>;
#[doc = "Field `k2` writer - k224"]
pub type K2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - k224"]
    #[inline(always)]
    #[must_use]
    pub fn k2(&mut self) -> K2_W<K0LRrs> {
        K2_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k0lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct K0LRrs;
impl crate::RegisterSpec for K0LRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`k0lr::W`](W) writer structure"]
impl crate::Writable for K0LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets K0LR to value 0"]
impl crate::Resettable for K0LRrs {
    const RESET_VALUE: u32 = 0;
}
