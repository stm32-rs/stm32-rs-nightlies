#[doc = "Register `TIM15_TISEL` reader"]
pub type R = crate::R<TIM15_TISELrs>;
#[doc = "Register `TIM15_TISEL` writer"]
pub type W = crate::W<TIM15_TISELrs>;
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI2SEL` reader - TI2SEL"]
pub type TI2SEL_R = crate::FieldReader;
#[doc = "Field `TI2SEL` writer - TI2SEL"]
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM15_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TIM15_TISELrs> {
        TI2SEL_W::new(self, 8)
    }
}
#[doc = "TIM15 input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim15_tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim15_tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM15_TISELrs;
impl crate::RegisterSpec for TIM15_TISELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim15_tisel::R`](R) reader structure"]
impl crate::Readable for TIM15_TISELrs {}
#[doc = "`write(|w| ..)` method takes [`tim15_tisel::W`](W) writer structure"]
impl crate::Writable for TIM15_TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM15_TISEL to value 0"]
impl crate::Resettable for TIM15_TISELrs {
    const RESET_VALUE: u32 = 0;
}
