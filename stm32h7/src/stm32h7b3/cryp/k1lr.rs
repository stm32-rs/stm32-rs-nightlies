#[doc = "Register `K1LR` writer"]
pub type W = crate::W<K1LRrs>;
#[doc = "Field `b1` writer - b160"]
pub type B1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - b160"]
    #[inline(always)]
    #[must_use]
    pub fn b1(&mut self) -> B1_W<K1LRrs> {
        B1_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k1lr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct K1LRrs;
impl crate::RegisterSpec for K1LRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`k1lr::W`](W) writer structure"]
impl crate::Writable for K1LRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets K1LR to value 0"]
impl crate::Resettable for K1LRrs {
    const RESET_VALUE: u32 = 0;
}
