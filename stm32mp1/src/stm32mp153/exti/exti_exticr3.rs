#[doc = "Register `EXTI_EXTICR3` reader"]
pub type R = crate::R<EXTI_EXTICR3rs>;
#[doc = "Register `EXTI_EXTICR3` writer"]
pub type W = crate::W<EXTI_EXTICR3rs>;
#[doc = "Field `EXTI8` reader - EXTI8"]
pub type EXTI8_R = crate::FieldReader;
#[doc = "Field `EXTI8` writer - EXTI8"]
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI9` reader - EXTI9"]
pub type EXTI9_R = crate::FieldReader;
#[doc = "Field `EXTI9` writer - EXTI9"]
pub type EXTI9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI10` reader - EXTI10"]
pub type EXTI10_R = crate::FieldReader;
#[doc = "Field `EXTI10` writer - EXTI10"]
pub type EXTI10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXTI11` reader - EXTI11"]
pub type EXTI11_R = crate::FieldReader;
#[doc = "Field `EXTI11` writer - EXTI11"]
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI8"]
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<EXTI_EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI9"]
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<EXTI_EXTICR3rs> {
        EXTI9_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI10"]
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<EXTI_EXTICR3rs> {
        EXTI10_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI11"]
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<EXTI_EXTICR3rs> {
        EXTI11_W::new(self, 24)
    }
}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_exticr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_exticr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_EXTICR3rs;
impl crate::RegisterSpec for EXTI_EXTICR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_exticr3::R`](R) reader structure"]
impl crate::Readable for EXTI_EXTICR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_exticr3::W`](W) writer structure"]
impl crate::Writable for EXTI_EXTICR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_EXTICR3 to value 0"]
impl crate::Resettable for EXTI_EXTICR3rs {
    const RESET_VALUE: u32 = 0;
}
