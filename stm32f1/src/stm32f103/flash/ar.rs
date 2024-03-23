#[doc = "Register `AR` writer"]
pub type W = crate::W<ARrs>;
#[doc = "Field `FAR` writer - Flash Address"]
pub type FAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash Address"]
    #[inline(always)]
    #[must_use]
    pub fn far(&mut self) -> FAR_W<ARrs> {
        FAR_W::new(self, 0)
    }
}
#[doc = "Flash address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARrs;
impl crate::RegisterSpec for ARrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ar::W`](W) writer structure"]
impl crate::Writable for ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for ARrs {
    const RESET_VALUE: u32 = 0;
}
