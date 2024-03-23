#[doc = "Register `LTDC_SSCR` reader"]
pub type R = crate::R<LTDC_SSCRrs>;
#[doc = "Register `LTDC_SSCR` writer"]
pub type W = crate::W<LTDC_SSCRrs>;
#[doc = "Field `VSH` reader - VSH"]
pub type VSH_R = crate::FieldReader<u16>;
#[doc = "Field `VSH` writer - VSH"]
pub type VSH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HSW` reader - HSW"]
pub type HSW_R = crate::FieldReader<u16>;
#[doc = "Field `HSW` writer - HSW"]
pub type HSW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - VSH"]
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HSW"]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - VSH"]
    #[inline(always)]
    #[must_use]
    pub fn vsh(&mut self) -> VSH_W<LTDC_SSCRrs> {
        VSH_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - HSW"]
    #[inline(always)]
    #[must_use]
    pub fn hsw(&mut self) -> HSW_W<LTDC_SSCRrs> {
        HSW_W::new(self, 16)
    }
}
#[doc = "This register defines the number of horizontal synchronization pixels minus 1 and the number of vertical synchronization lines minus 1. Refer to Figure274 and Section36.4: LTDC programmable parameters for an example of configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_sscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_sscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_SSCRrs;
impl crate::RegisterSpec for LTDC_SSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_sscr::R`](R) reader structure"]
impl crate::Readable for LTDC_SSCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_sscr::W`](W) writer structure"]
impl crate::Writable for LTDC_SSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_SSCR to value 0"]
impl crate::Resettable for LTDC_SSCRrs {
    const RESET_VALUE: u32 = 0;
}
