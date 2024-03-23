#[doc = "Register `TIM8_ARR` reader"]
pub type R = crate::R<TIM8_ARRrs>;
#[doc = "Register `TIM8_ARR` writer"]
pub type W = crate::W<TIM8_ARRrs>;
#[doc = "Field `ARR` reader - ARR"]
pub type ARR_R = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - ARR"]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<TIM8_ARRrs> {
        ARR_W::new(self, 0)
    }
}
#[doc = "TIM8 auto-reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim8_arr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim8_arr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM8_ARRrs;
impl crate::RegisterSpec for TIM8_ARRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim8_arr::R`](R) reader structure"]
impl crate::Readable for TIM8_ARRrs {}
#[doc = "`write(|w| ..)` method takes [`tim8_arr::W`](W) writer structure"]
impl crate::Writable for TIM8_ARRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM8_ARR to value 0xffff"]
impl crate::Resettable for TIM8_ARRrs {
    const RESET_VALUE: u16 = 0xffff;
}
