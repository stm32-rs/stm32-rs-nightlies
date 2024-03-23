#[doc = "Register `SDMMC_IDMABSIZER` reader"]
pub type R = crate::R<SDMMC_IDMABSIZERrs>;
#[doc = "Register `SDMMC_IDMABSIZER` writer"]
pub type W = crate::W<SDMMC_IDMABSIZERrs>;
#[doc = "Field `IDMABNDT` reader - IDMABNDT"]
pub type IDMABNDT_R = crate::FieldReader<u16>;
#[doc = "Field `IDMABNDT` writer - IDMABNDT"]
pub type IDMABNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 5:16 - IDMABNDT"]
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:16 - IDMABNDT"]
    #[inline(always)]
    #[must_use]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<SDMMC_IDMABSIZERrs> {
        IDMABNDT_W::new(self, 5)
    }
}
#[doc = "The SDMMC_IDMABSIZER register contains the buffer size when in linked list configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmabsizer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmabsizer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IDMABSIZERrs;
impl crate::RegisterSpec for SDMMC_IDMABSIZERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idmabsizer::R`](R) reader structure"]
impl crate::Readable for SDMMC_IDMABSIZERrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idmabsizer::W`](W) writer structure"]
impl crate::Writable for SDMMC_IDMABSIZERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMABSIZER to value 0"]
impl crate::Resettable for SDMMC_IDMABSIZERrs {
    const RESET_VALUE: u32 = 0;
}
