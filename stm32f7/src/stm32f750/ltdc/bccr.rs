#[doc = "Register `BCCR` reader"]
pub type R = crate::R<BCCRrs>;
#[doc = "Register `BCCR` writer"]
pub type W = crate::W<BCCRrs>;
#[doc = "Field `BCBLUE` reader - Background color blue value"]
pub type BCBLUE_R = crate::FieldReader;
#[doc = "Field `BCBLUE` writer - Background color blue value"]
pub type BCBLUE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `BCGREEN` reader - Background color green value"]
pub type BCGREEN_R = crate::FieldReader;
#[doc = "Field `BCGREEN` writer - Background color green value"]
pub type BCGREEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `BCRED` reader - Background color red value"]
pub type BCRED_R = crate::FieldReader;
#[doc = "Field `BCRED` writer - Background color red value"]
pub type BCRED_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Background color blue value"]
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background color green value"]
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background color red value"]
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background color blue value"]
    #[inline(always)]
    #[must_use]
    pub fn bcblue(&mut self) -> BCBLUE_W<BCCRrs> {
        BCBLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Background color green value"]
    #[inline(always)]
    #[must_use]
    pub fn bcgreen(&mut self) -> BCGREEN_W<BCCRrs> {
        BCGREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Background color red value"]
    #[inline(always)]
    #[must_use]
    pub fn bcred(&mut self) -> BCRED_W<BCCRrs> {
        BCRED_W::new(self, 16)
    }
}
#[doc = "Background Color Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCCRrs;
impl crate::RegisterSpec for BCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bccr::R`](R) reader structure"]
impl crate::Readable for BCCRrs {}
#[doc = "`write(|w| ..)` method takes [`bccr::W`](W) writer structure"]
impl crate::Writable for BCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCCR to value 0"]
impl crate::Resettable for BCCRrs {
    const RESET_VALUE: u32 = 0;
}
