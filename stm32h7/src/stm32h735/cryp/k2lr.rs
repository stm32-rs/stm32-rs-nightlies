#[doc = "Register `K2LR` writer"]
pub type W = crate::W<K2LRrs>;
#[doc = "Field `k` writer - k96"]
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `b121` writer - b121"]
pub type B121_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:30 - k96"]
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<K2LRrs> {
        K_W::new(self, 0)
    }
    #[doc = "Bit 25 - b121"]
    #[inline(always)]
    #[must_use]
    pub fn b121(&mut self) -> B121_W<K2LRrs> {
        B121_W::new(self, 25)
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
