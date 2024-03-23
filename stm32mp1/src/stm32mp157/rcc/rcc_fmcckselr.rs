#[doc = "Register `RCC_FMCCKSELR` reader"]
pub type R = crate::R<RCC_FMCCKSELRrs>;
#[doc = "Register `RCC_FMCCKSELR` writer"]
pub type W = crate::W<RCC_FMCCKSELRrs>;
#[doc = "Field `FMCSRC` reader - FMCSRC"]
pub type FMCSRC_R = crate::FieldReader;
#[doc = "Field `FMCSRC` writer - FMCSRC"]
pub type FMCSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    pub fn fmcsrc(&self) -> FMCSRC_R {
        FMCSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsrc(&mut self) -> FMCSRC_W<RCC_FMCCKSELRrs> {
        FMCSRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_fmcckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_fmcckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_FMCCKSELRrs;
impl crate::RegisterSpec for RCC_FMCCKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_fmcckselr::R`](R) reader structure"]
impl crate::Readable for RCC_FMCCKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_fmcckselr::W`](W) writer structure"]
impl crate::Writable for RCC_FMCCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_FMCCKSELR to value 0"]
impl crate::Resettable for RCC_FMCCKSELRrs {
    const RESET_VALUE: u32 = 0;
}
