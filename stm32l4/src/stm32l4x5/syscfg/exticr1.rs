#[doc = "Register `EXTICR1` reader"]
pub type R = crate::R<EXTICR1rs>;
#[doc = "Register `EXTICR1` writer"]
pub type W = crate::W<EXTICR1rs>;
#[doc = "Field `EXTI0` reader - EXTI 0 configuration bits"]
pub type EXTI0_R = crate::FieldReader;
#[doc = "Field `EXTI0` writer - EXTI 0 configuration bits"]
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTI1` reader - EXTI 1 configuration bits"]
pub type EXTI1_R = crate::FieldReader;
#[doc = "Field `EXTI1` writer - EXTI 1 configuration bits"]
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTI2` reader - EXTI 2 configuration bits"]
pub type EXTI2_R = crate::FieldReader;
#[doc = "Field `EXTI2` writer - EXTI 2 configuration bits"]
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTI3` reader - EXTI 3 configuration bits"]
pub type EXTI3_R = crate::FieldReader;
#[doc = "Field `EXTI3` writer - EXTI 3 configuration bits"]
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - EXTI 0 configuration bits"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - EXTI 1 configuration bits"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - EXTI 2 configuration bits"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - EXTI 3 configuration bits"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EXTI 0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - EXTI 1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<EXTICR1rs> {
        EXTI1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - EXTI 2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<EXTICR1rs> {
        EXTI2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - EXTI 3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<EXTICR1rs> {
        EXTI3_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
