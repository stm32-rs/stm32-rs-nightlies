#[doc = "Register `LVEKEYR` writer"]
pub type W = crate::W<LVEKEYRrs>;
#[doc = "Field `LVEKEYR` writer - LVEKEYR"]
pub type LVEKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - LVEKEYR"]
    #[inline(always)]
    #[must_use]
    pub fn lvekeyr(&mut self) -> LVEKEYR_W<LVEKEYRrs> {
        LVEKEYR_W::new(self, 0)
    }
}
#[doc = "Flash low voltage key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvekeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVEKEYRrs;
impl crate::RegisterSpec for LVEKEYRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lvekeyr::W`](W) writer structure"]
impl crate::Writable for LVEKEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LVEKEYR to value 0"]
impl crate::Resettable for LVEKEYRrs {
    const RESET_VALUE: u32 = 0;
}
