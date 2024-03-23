#[doc = "Register `K0RR` writer"]
pub type W = crate::W<K0RRrs>;
#[doc = "Field `k` writer - k192"]
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - k192"]
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<K0RRrs> {
        K_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k0rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct K0RRrs;
impl crate::RegisterSpec for K0RRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`k0rr::W`](W) writer structure"]
impl crate::Writable for K0RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets K0RR to value 0"]
impl crate::Resettable for K0RRrs {
    const RESET_VALUE: u32 = 0;
}
