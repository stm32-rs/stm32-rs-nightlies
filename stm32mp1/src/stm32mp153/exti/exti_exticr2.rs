#[doc = "Register `EXTI_EXTICR2` reader"]
pub type R = crate::R<EXTI_EXTICR2rs>;
#[doc = "Register `EXTI_EXTICR2` writer"]
pub type W = crate::W<EXTI_EXTICR2rs>;
#[doc = "Field `EXTI4` reader - EXTI4"]
pub type EXTI4_R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI4"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI5` reader - EXTI5"]
pub type EXTI5_R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI5"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI6` reader - EXTI6"]
pub type EXTI6_R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI6"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI7` reader - EXTI7"]
pub type EXTI7_R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI7"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTI_EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI5"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTI_EXTICR2rs> {
        EXTI5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI6"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTI_EXTICR2rs> {
        EXTI6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI7"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTI_EXTICR2rs> {
        EXTI7_W::new(self, 24)
    }
}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_EXTICR2rs;
impl crate::RegisterSpec for EXTI_EXTICR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr2::R`](R) reader structure"]
impl crate::Readable for EXTI_EXTICR2rs {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr2::W`](W) writer structure"]
impl crate::Writable for EXTI_EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR2 to value 0"]
impl crate::Resettable for EXTI_EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
