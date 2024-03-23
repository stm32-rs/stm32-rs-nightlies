#[doc = "Register `TIM13_TISEL` reader"]
pub type R = crate::R<TIM13_TISELrs>;
#[doc = "Register `TIM13_TISEL` writer"]
pub type W = crate::W<TIM13_TISELrs>;
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM13_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
}
#[doc = "TIM13 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim13_tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim13_tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM13_TISELrs;
impl crate::RegisterSpec for TIM13_TISELrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tim13_tisel::R`](R) reader structure"]
impl crate::Readable for TIM13_TISELrs {}
#[doc = "`write(|w| ..)` method takes [`tim13_tisel::W`](W) writer structure"]
impl crate::Writable for TIM13_TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM13_TISEL to value 0"]
impl crate::Resettable for TIM13_TISELrs {
    const RESET_VALUE: u16 = 0;
}
