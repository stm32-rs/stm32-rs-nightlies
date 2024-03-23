#[doc = "Register `EXTI_EXTICR4` reader"]
pub type R = crate::R<EXTI_EXTICR4rs>;
#[doc = "Register `EXTI_EXTICR4` writer"]
pub type W = crate::W<EXTI_EXTICR4rs>;
#[doc = "Field `EXTI12` reader - EXTIm GPIO port selection"]
pub type EXTI12_R = crate::FieldReader;
#[doc = "Field `EXTI12` writer - EXTIm GPIO port selection"]
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI13` reader - EXTIm+1 GPIO port selection"]
pub type EXTI13_R = crate::FieldReader;
#[doc = "Field `EXTI13` writer - EXTIm+1 GPIO port selection"]
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI14` reader - EXTIm+2 GPIO port selection"]
pub type EXTI14_R = crate::FieldReader;
#[doc = "Field `EXTI14` writer - EXTIm+2 GPIO port selection"]
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI15` reader - EXTIm+3 GPIO port selection"]
pub type EXTI15_R = crate::FieldReader;
#[doc = "Field `EXTI15` writer - EXTIm+3 GPIO port selection"]
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<EXTI_EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<EXTI_EXTICR4rs> {
        EXTI13_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<EXTI_EXTICR4rs> {
        EXTI14_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<EXTI_EXTICR4rs> {
        EXTI15_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_EXTICR4rs;
impl crate::RegisterSpec for EXTI_EXTICR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr4::R`](R) reader structure"]
impl crate::Readable for EXTI_EXTICR4rs {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr4::W`](W) writer structure"]
impl crate::Writable for EXTI_EXTICR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR4 to value 0"]
impl crate::Resettable for EXTI_EXTICR4rs {
    const RESET_VALUE: u32 = 0;
}
