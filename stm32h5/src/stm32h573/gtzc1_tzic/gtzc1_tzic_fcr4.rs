#[doc = "Register `GTZC1_TZIC_FCR4` writer"]
pub type W = crate::W<GTZC1_TZIC_FCR4rs>;
#[doc = "Field `CGPDMA1F` writer - clear the illegal access flag for GPDMA1"]
pub type CGPDMA1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGPDMA2F` writer - clear the illegal access flag for GPDMA2"]
pub type CGPDMA2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFLASH_REGF` writer - clear the illegal access flag for FLASH registers"]
pub type CFLASH_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFLASHF` writer - clear the illegal access flag for FLASH memory"]
pub type CFLASHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COTFDEC1F` writer - clear the illegal access flag for OTFDEC1"]
pub type COTFDEC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSBSF` writer - clear the illegal access flag for SBS"]
pub type CSBSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRTCF` writer - clear the illegal access flag for RTC"]
pub type CRTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMPF` writer - clear the illegal access flag for TAMP"]
pub type CTAMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPWRF` writer - clear the illegal access flag for PWR"]
pub type CPWRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCCF` writer - clear the illegal access flag for RCC"]
pub type CRCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEXTIF` writer - clear the illegal access flag for EXTI"]
pub type CEXTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTZSC1F` writer - clear the illegal access flag for GTZC1 TZSC registers"]
pub type CTZSC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTZIC1F` writer - clear the illegal access flag for GTZC1 TZIC registers"]
pub type CTZIC1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCTOSPI1_MEMF` writer - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank"]
pub type COCTOSPI1_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFMC_MEMF` writer - clear the illegal access flag for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
pub type CFMC_MEMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBKPSRAMF` writer - clear the illegal access flag for MPCWM4 (BKPSRAM) memory bank"]
pub type CBKPSRAMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM1F` writer - clear the illegal access flag for SRAM1"]
pub type CSRAM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCBB1_REGF` writer - clear the illegal access flag for MPCBB1 registers"]
pub type CMPCBB1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM2F` writer - clear the illegal access flag for SRAM2"]
pub type CSRAM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCBB2_REGF` writer - clear the illegal access flag for MPCBB2 registers"]
pub type CMPCBB2_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM3F` writer - clear the illegal access flag for SRAM3"]
pub type CSRAM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCBB3_REGF` writer - clear the illegal access flag for MPCBB3 registers"]
pub type CMPCBB3_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for GPDMA1"]
    #[inline(always)]
    #[must_use]
    pub fn cgpdma1f(&mut self) -> CGPDMA1F_W<GTZC1_TZIC_FCR4rs> {
        CGPDMA1F_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for GPDMA2"]
    #[inline(always)]
    #[must_use]
    pub fn cgpdma2f(&mut self) -> CGPDMA2F_W<GTZC1_TZIC_FCR4rs> {
        CGPDMA2F_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for FLASH registers"]
    #[inline(always)]
    #[must_use]
    pub fn cflash_regf(&mut self) -> CFLASH_REGF_W<GTZC1_TZIC_FCR4rs> {
        CFLASH_REGF_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for FLASH memory"]
    #[inline(always)]
    #[must_use]
    pub fn cflashf(&mut self) -> CFLASHF_W<GTZC1_TZIC_FCR4rs> {
        CFLASHF_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for OTFDEC1"]
    #[inline(always)]
    #[must_use]
    pub fn cotfdec1f(&mut self) -> COTFDEC1F_W<GTZC1_TZIC_FCR4rs> {
        COTFDEC1F_W::new(self, 4)
    }
    #[doc = "Bit 6 - clear the illegal access flag for SBS"]
    #[inline(always)]
    #[must_use]
    pub fn csbsf(&mut self) -> CSBSF_W<GTZC1_TZIC_FCR4rs> {
        CSBSF_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear the illegal access flag for RTC"]
    #[inline(always)]
    #[must_use]
    pub fn crtcf(&mut self) -> CRTCF_W<GTZC1_TZIC_FCR4rs> {
        CRTCF_W::new(self, 7)
    }
    #[doc = "Bit 8 - clear the illegal access flag for TAMP"]
    #[inline(always)]
    #[must_use]
    pub fn ctampf(&mut self) -> CTAMPF_W<GTZC1_TZIC_FCR4rs> {
        CTAMPF_W::new(self, 8)
    }
    #[doc = "Bit 9 - clear the illegal access flag for PWR"]
    #[inline(always)]
    #[must_use]
    pub fn cpwrf(&mut self) -> CPWRF_W<GTZC1_TZIC_FCR4rs> {
        CPWRF_W::new(self, 9)
    }
    #[doc = "Bit 10 - clear the illegal access flag for RCC"]
    #[inline(always)]
    #[must_use]
    pub fn crccf(&mut self) -> CRCCF_W<GTZC1_TZIC_FCR4rs> {
        CRCCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - clear the illegal access flag for EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn cextif(&mut self) -> CEXTIF_W<GTZC1_TZIC_FCR4rs> {
        CEXTIF_W::new(self, 11)
    }
    #[doc = "Bit 16 - clear the illegal access flag for GTZC1 TZSC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzsc1f(&mut self) -> CTZSC1F_W<GTZC1_TZIC_FCR4rs> {
        CTZSC1F_W::new(self, 16)
    }
    #[doc = "Bit 17 - clear the illegal access flag for GTZC1 TZIC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzic1f(&mut self) -> CTZIC1F_W<GTZC1_TZIC_FCR4rs> {
        CTZIC1F_W::new(self, 17)
    }
    #[doc = "Bit 18 - clear the illegal access flag for MPCWM1 (OCTOSPI1) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn coctospi1_memf(&mut self) -> COCTOSPI1_MEMF_W<GTZC1_TZIC_FCR4rs> {
        COCTOSPI1_MEMF_W::new(self, 18)
    }
    #[doc = "Bit 19 - clear the illegal access flag for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
    #[inline(always)]
    #[must_use]
    pub fn cfmc_memf(&mut self) -> CFMC_MEMF_W<GTZC1_TZIC_FCR4rs> {
        CFMC_MEMF_W::new(self, 19)
    }
    #[doc = "Bit 20 - clear the illegal access flag for MPCWM4 (BKPSRAM) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn cbkpsramf(&mut self) -> CBKPSRAMF_W<GTZC1_TZIC_FCR4rs> {
        CBKPSRAMF_W::new(self, 20)
    }
    #[doc = "Bit 24 - clear the illegal access flag for SRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn csram1f(&mut self) -> CSRAM1F_W<GTZC1_TZIC_FCR4rs> {
        CSRAM1F_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear the illegal access flag for MPCBB1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb1_regf(&mut self) -> CMPCBB1_REGF_W<GTZC1_TZIC_FCR4rs> {
        CMPCBB1_REGF_W::new(self, 25)
    }
    #[doc = "Bit 26 - clear the illegal access flag for SRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn csram2f(&mut self) -> CSRAM2F_W<GTZC1_TZIC_FCR4rs> {
        CSRAM2F_W::new(self, 26)
    }
    #[doc = "Bit 27 - clear the illegal access flag for MPCBB2 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb2_regf(&mut self) -> CMPCBB2_REGF_W<GTZC1_TZIC_FCR4rs> {
        CMPCBB2_REGF_W::new(self, 27)
    }
    #[doc = "Bit 28 - clear the illegal access flag for SRAM3"]
    #[inline(always)]
    #[must_use]
    pub fn csram3f(&mut self) -> CSRAM3F_W<GTZC1_TZIC_FCR4rs> {
        CSRAM3F_W::new(self, 28)
    }
    #[doc = "Bit 29 - clear the illegal access flag for MPCBB3 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb3_regf(&mut self) -> CMPCBB3_REGF_W<GTZC1_TZIC_FCR4rs> {
        CMPCBB3_REGF_W::new(self, 29)
    }
}
#[doc = "GTZC1 TZIC flag clear register 4\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_fcr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_FCR4rs;
impl crate::RegisterSpec for GTZC1_TZIC_FCR4rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_fcr4::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_FCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_FCR4 to value 0"]
impl crate::Resettable for GTZC1_TZIC_FCR4rs {
    const RESET_VALUE: u32 = 0;
}
