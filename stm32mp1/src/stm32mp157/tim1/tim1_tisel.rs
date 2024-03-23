#[doc = "Register `TIM1_TISEL` reader"]
pub type R = crate::R<TIM1_TISELrs>;
#[doc = "Register `TIM1_TISEL` writer"]
pub type W = crate::W<TIM1_TISELrs>;
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI2SEL` reader - TI2SEL"]
pub type TI2SEL_R = crate::FieldReader;
#[doc = "Field `TI2SEL` writer - TI2SEL"]
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI3SEL` reader - TI3SEL"]
pub type TI3SEL_R = crate::FieldReader;
#[doc = "Field `TI3SEL` writer - TI3SEL"]
pub type TI3SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI4SEL` reader - TI4SEL"]
pub type TI4SEL_R = crate::FieldReader;
#[doc = "Field `TI4SEL` writer - TI4SEL"]
pub type TI4SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bits 16:19 - TI3SEL"]
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - TI4SEL"]
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM1_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TIM1_TISELrs> {
        TI2SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - TI3SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> TI3SEL_W<TIM1_TISELrs> {
        TI3SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - TI4SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> TI4SEL_W<TIM1_TISELrs> {
        TI4SEL_W::new(self, 24)
    }
}
#[doc = "TIM1 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_TISELrs;
impl crate::RegisterSpec for TIM1_TISELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_tisel::R`](R) reader structure"]
impl crate::Readable for TIM1_TISELrs {}
#[doc = "`write(|w| ..)` method takes [`tim1_tisel::W`](W) writer structure"]
impl crate::Writable for TIM1_TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_TISEL to value 0"]
impl crate::Resettable for TIM1_TISELrs {
    const RESET_VALUE: u32 = 0;
}
