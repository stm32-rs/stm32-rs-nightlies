#[doc = "Register `LTDC_BCCR` reader"]
pub type R = crate::R<LTDC_BCCRrs>;
#[doc = "Register `LTDC_BCCR` writer"]
pub type W = crate::W<LTDC_BCCRrs>;
#[doc = "Field `BCBLUE` reader - BCBLUE"]
pub type BCBLUE_R = crate::FieldReader;
#[doc = "Field `BCBLUE` writer - BCBLUE"]
pub type BCBLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BCGREEN` reader - BCGREEN"]
pub type BCGREEN_R = crate::FieldReader;
#[doc = "Field `BCGREEN` writer - BCGREEN"]
pub type BCGREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BCRED` reader - BCRED"]
pub type BCRED_R = crate::FieldReader;
#[doc = "Field `BCRED` writer - BCRED"]
pub type BCRED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BCBLUE"]
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BCGREEN"]
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BCRED"]
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCBLUE"]
    #[inline(always)]
    #[must_use]
    pub fn bcblue(&mut self) -> BCBLUE_W<LTDC_BCCRrs> {
        BCBLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - BCGREEN"]
    #[inline(always)]
    #[must_use]
    pub fn bcgreen(&mut self) -> BCGREEN_W<LTDC_BCCRrs> {
        BCGREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - BCRED"]
    #[inline(always)]
    #[must_use]
    pub fn bcred(&mut self) -> BCRED_W<LTDC_BCCRrs> {
        BCRED_W::new(self, 16)
    }
}
#[doc = "This register defines the background color (RGB888).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_bccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_bccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_BCCRrs;
impl crate::RegisterSpec for LTDC_BCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_bccr::R`](R) reader structure"]
impl crate::Readable for LTDC_BCCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_bccr::W`](W) writer structure"]
impl crate::Writable for LTDC_BCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_BCCR to value 0"]
impl crate::Resettable for LTDC_BCCRrs {
    const RESET_VALUE: u32 = 0;
}
