#[doc = "Register `SDMMC_IDMACTRLR` reader"]
pub type R = crate::R<SDMMC_IDMACTRLRrs>;
#[doc = "Register `SDMMC_IDMACTRLR` writer"]
pub type W = crate::W<SDMMC_IDMACTRLRrs>;
#[doc = "Field `IDMAEN` reader - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMAEN_R = crate::BitReader;
#[doc = "Field `IDMAEN` writer - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABMODE` reader - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMABMODE_R = crate::BitReader;
#[doc = "Field `IDMABMODE` writer - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type IDMABMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDMA enable This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmaen(&mut self) -> IDMAEN_W<SDMMC_IDMACTRLRrs> {
        IDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn idmabmode(&mut self) -> IDMABMODE_W<SDMMC_IDMACTRLRrs> {
        IDMABMODE_W::new(self, 1)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_idmactrlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_idmactrlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IDMACTRLRrs;
impl crate::RegisterSpec for SDMMC_IDMACTRLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_idmactrlr::R`](R) reader structure"]
impl crate::Readable for SDMMC_IDMACTRLRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_idmactrlr::W`](W) writer structure"]
impl crate::Writable for SDMMC_IDMACTRLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_IDMACTRLR to value 0"]
impl crate::Resettable for SDMMC_IDMACTRLRrs {
    const RESET_VALUE: u32 = 0;
}
