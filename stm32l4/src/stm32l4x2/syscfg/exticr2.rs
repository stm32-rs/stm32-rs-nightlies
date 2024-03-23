#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2rs>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2rs>;
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub type EXTI4_R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub type EXTI5_R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub type EXTI6_R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub type EXTI7_R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EXTI 4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - EXTI 5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2rs> {
        EXTI5_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - EXTI 6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2rs> {
        EXTI6_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - EXTI 7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTICR2rs> {
        EXTI7_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for EXTICR2rs {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
