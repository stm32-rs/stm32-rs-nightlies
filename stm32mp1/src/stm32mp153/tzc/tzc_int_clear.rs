#[doc = "Register `TZC_INT_CLEAR` reader"]
pub type R = crate::R<TZC_INT_CLEARrs>;
#[doc = "Register `TZC_INT_CLEAR` writer"]
pub type W = crate::W<TZC_INT_CLEARrs>;
#[doc = "Field `CLEAR` writer - CLEAR"]
pub type CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bits 0:1 - CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<TZC_INT_CLEARrs> {
        CLEAR_W::new(self, 0)
    }
}
#[doc = "Interrupt clear for each filter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_int_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_int_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_INT_CLEARrs;
impl crate::RegisterSpec for TZC_INT_CLEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_int_clear::R`](R) reader structure"]
impl crate::Readable for TZC_INT_CLEARrs {}
#[doc = "`write(|w| ..)` method takes [`tzc_int_clear::W`](W) writer structure"]
impl crate::Writable for TZC_INT_CLEARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_INT_CLEAR to value 0"]
impl crate::Resettable for TZC_INT_CLEARrs {
    const RESET_VALUE: u32 = 0;
}
