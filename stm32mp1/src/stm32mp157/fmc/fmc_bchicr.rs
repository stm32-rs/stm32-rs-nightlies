#[doc = "Register `FMC_BCHICR` writer"]
pub type W = crate::W<FMC_BCHICRrs>;
#[doc = "Field `CDUEF` writer - CDUEF"]
pub type CDUEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDERF` writer - CDERF"]
pub type CDERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEFF` writer - CDEFF"]
pub type CDEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDSRF` writer - CDSRF"]
pub type CDSRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEPBRF` writer - CEPBRF"]
pub type CEPBRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CDUEF"]
    #[inline(always)]
    #[must_use]
    pub fn cduef(&mut self) -> CDUEF_W<FMC_BCHICRrs> {
        CDUEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CDERF"]
    #[inline(always)]
    #[must_use]
    pub fn cderf(&mut self) -> CDERF_W<FMC_BCHICRrs> {
        CDERF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CDEFF"]
    #[inline(always)]
    #[must_use]
    pub fn cdeff(&mut self) -> CDEFF_W<FMC_BCHICRrs> {
        CDEFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CDSRF"]
    #[inline(always)]
    #[must_use]
    pub fn cdsrf(&mut self) -> CDSRF_W<FMC_BCHICRrs> {
        CDSRF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CEPBRF"]
    #[inline(always)]
    #[must_use]
    pub fn cepbrf(&mut self) -> CEPBRF_W<FMC_BCHICRrs> {
        CEPBRF_W::new(self, 4)
    }
}
#[doc = "FMC BCH Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bchicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHICRrs;
impl crate::RegisterSpec for FMC_BCHICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fmc_bchicr::W`](W) writer structure"]
impl crate::Writable for FMC_BCHICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_BCHICR to value 0"]
impl crate::Resettable for FMC_BCHICRrs {
    const RESET_VALUE: u32 = 0;
}
