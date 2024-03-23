#[doc = "Register `EXTICR1` reader"]
pub type R = crate::R<EXTICR1rs>;
#[doc = "Register `EXTICR1` writer"]
pub type W = crate::W<EXTICR1rs>;
#[doc = "Field `EXTI0_7` reader - EXTIm GPIO port selection"]
pub type EXTI0_7_R = crate::FieldReader;
#[doc = "Field `EXTI0_7` writer - EXTIm GPIO port selection"]
pub type EXTI0_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI8_15` reader - EXTIm+1 GPIO port selection"]
pub type EXTI8_15_R = crate::FieldReader;
#[doc = "Field `EXTI8_15` writer - EXTIm+1 GPIO port selection"]
pub type EXTI8_15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI16_23` reader - EXTIm+2 GPIO port selection"]
pub type EXTI16_23_R = crate::FieldReader;
#[doc = "Field `EXTI16_23` writer - EXTIm+2 GPIO port selection"]
pub type EXTI16_23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI24_31` reader - EXTIm+3 GPIO port selection"]
pub type EXTI24_31_R = crate::FieldReader;
#[doc = "Field `EXTI24_31` writer - EXTIm+3 GPIO port selection"]
pub type EXTI24_31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<EXTICR1rs> {
        EXTI0_7_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<EXTICR1rs> {
        EXTI8_15_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<EXTICR1rs> {
        EXTI16_23_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<EXTICR1rs> {
        EXTI24_31_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR1rs;
impl crate::RegisterSpec for EXTICR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr1::R`](R) reader structure"]
impl crate::Readable for EXTICR1rs {}
#[doc = "`write(|w| ..)` method takes [`exticr1::W`](W) writer structure"]
impl crate::Writable for EXTICR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1rs {
    const RESET_VALUE: u32 = 0;
}
