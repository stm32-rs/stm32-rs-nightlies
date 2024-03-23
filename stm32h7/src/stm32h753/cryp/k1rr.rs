#[doc = "Register `K1RR` writer"]
pub type W = crate::W<K1RRrs>;
#[doc = "Field `K1` writer - K128"]
pub type K1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - K128"]
    #[inline(always)]
    #[must_use]
    pub fn k1(&mut self) -> K1_W<K1RRrs> {
        K1_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k1rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct K1RRrs;
impl crate::RegisterSpec for K1RRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`k1rr::W`](W) writer structure"]
impl crate::Writable for K1RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets K1RR to value 0"]
impl crate::Resettable for K1RRrs {
    const RESET_VALUE: u32 = 0;
}
