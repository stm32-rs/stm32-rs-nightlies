#[doc = "Register `K2RR` writer"]
pub type W = crate::W<K2RRrs>;
#[doc = "Field `b` writer - b64"]
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - b64"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<K2RRrs> {
        B_W::new(self, 0)
    }
}
#[doc = "key registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`k2rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct K2RRrs;
impl crate::RegisterSpec for K2RRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`k2rr::W`](W) writer structure"]
impl crate::Writable for K2RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets K2RR to value 0"]
impl crate::Resettable for K2RRrs {
    const RESET_VALUE: u32 = 0;
}
