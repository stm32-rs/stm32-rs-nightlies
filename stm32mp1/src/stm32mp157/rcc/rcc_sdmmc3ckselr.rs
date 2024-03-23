#[doc = "Register `RCC_SDMMC3CKSELR` reader"]
pub type R = crate::R<RCC_SDMMC3CKSELRrs>;
#[doc = "Register `RCC_SDMMC3CKSELR` writer"]
pub type W = crate::W<RCC_SDMMC3CKSELRrs>;
#[doc = "Field `SDMMC3SRC` reader - SDMMC3SRC"]
pub type SDMMC3SRC_R = crate::FieldReader;
#[doc = "Field `SDMMC3SRC` writer - SDMMC3SRC"]
pub type SDMMC3SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SDMMC3SRC"]
    #[inline(always)]
    pub fn sdmmc3src(&self) -> SDMMC3SRC_R {
        SDMMC3SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDMMC3SRC"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc3src(&mut self) -> SDMMC3SRC_W<RCC_SDMMC3CKSELRrs> {
        SDMMC3SRC_W::new(self, 0)
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_sdmmc3ckselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_sdmmc3ckselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_SDMMC3CKSELRrs;
impl crate::RegisterSpec for RCC_SDMMC3CKSELRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_sdmmc3ckselr::R`](R) reader structure"]
impl crate::Readable for RCC_SDMMC3CKSELRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_sdmmc3ckselr::W`](W) writer structure"]
impl crate::Writable for RCC_SDMMC3CKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_SDMMC3CKSELR to value 0"]
impl crate::Resettable for RCC_SDMMC3CKSELRrs {
    const RESET_VALUE: u32 = 0;
}
