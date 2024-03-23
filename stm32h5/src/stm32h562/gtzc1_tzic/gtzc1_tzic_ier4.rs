#[doc = "Register `GTZC1_TZIC_IER4` reader"]
pub type R = crate::R<GTZC1_TZIC_IER4rs>;
#[doc = "Register `GTZC1_TZIC_IER4` writer"]
pub type W = crate::W<GTZC1_TZIC_IER4rs>;
#[doc = "Field `GPDMA1IE` reader - illegal access interrupt enable for GPDMA1"]
pub type GPDMA1IE_R = crate::BitReader;
#[doc = "Field `GPDMA1IE` writer - illegal access interrupt enable for GPDMA1"]
pub type GPDMA1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPDMA2IE` reader - illegal access interrupt enable for GPDMA2"]
pub type GPDMA2IE_R = crate::BitReader;
#[doc = "Field `GPDMA2IE` writer - illegal access interrupt enable for GPDMA2"]
pub type GPDMA2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_REGIE` reader - illegal access interrupt enable for FLASH registers"]
pub type FLASH_REGIE_R = crate::BitReader;
#[doc = "Field `FLASH_REGIE` writer - illegal access interrupt enable for FLASH registers"]
pub type FLASH_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHIE` reader - illegal access interrupt enable for FLASH memory"]
pub type FLASHIE_R = crate::BitReader;
#[doc = "Field `FLASHIE` writer - illegal access interrupt enable for FLASH memory"]
pub type FLASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBSIE` reader - illegal access interrupt enable for SBS"]
pub type SBSIE_R = crate::BitReader;
#[doc = "Field `SBSIE` writer - illegal access interrupt enable for SBS"]
pub type SBSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCIE` reader - illegal access interrupt enable for RTC"]
pub type RTCIE_R = crate::BitReader;
#[doc = "Field `RTCIE` writer - illegal access interrupt enable for RTC"]
pub type RTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - illegal access interrupt enable for TAMP"]
pub type TAMPIE_R = crate::BitReader;
#[doc = "Field `TAMPIE` writer - illegal access interrupt enable for TAMP"]
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRIE` reader - illegal access interrupt enable for PWR"]
pub type PWRIE_R = crate::BitReader;
#[doc = "Field `PWRIE` writer - illegal access interrupt enable for PWR"]
pub type PWRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCCIE` reader - illegal access interrupt enable for RCC"]
pub type RCCIE_R = crate::BitReader;
#[doc = "Field `RCCIE` writer - illegal access interrupt enable for RCC"]
pub type RCCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTIIE` reader - illegal access interrupt enable for EXTI"]
pub type EXTIIE_R = crate::BitReader;
#[doc = "Field `EXTIIE` writer - illegal access interrupt enable for EXTI"]
pub type EXTIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSC1IE` reader - illegal access interrupt enable for GTZC1 TZSC registers"]
pub type TZSC1IE_R = crate::BitReader;
#[doc = "Field `TZSC1IE` writer - illegal access interrupt enable for GTZC1 TZSC registers"]
pub type TZSC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZIC1IE` reader - illegal access interrupt enable for GTZC1 TZIC registers"]
pub type TZIC1IE_R = crate::BitReader;
#[doc = "Field `TZIC1IE` writer - illegal access interrupt enable for GTZC1 TZIC registers"]
pub type TZIC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1_MEMIE` reader - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank"]
pub type OCTOSPI1_MEMIE_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_MEMIE` writer - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank"]
pub type OCTOSPI1_MEMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMC_MEMIE` reader - illegal access interrupt enable for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
pub type FMC_MEMIE_R = crate::BitReader;
#[doc = "Field `FMC_MEMIE` writer - illegal access interrupt enable for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
pub type FMC_MEMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMIE` reader - illegal access interrupt enable for MPCWM4 (BKPSRAM) memory bank"]
pub type BKPSRAMIE_R = crate::BitReader;
#[doc = "Field `BKPSRAMIE` writer - illegal access interrupt enable for MPCWM4 (BKPSRAM) memory bank"]
pub type BKPSRAMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1IE` reader - illegal access interrupt enable for SRAM1"]
pub type SRAM1IE_R = crate::BitReader;
#[doc = "Field `SRAM1IE` writer - illegal access interrupt enable for SRAM1"]
pub type SRAM1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1_REGIE` reader - illegal access interrupt enable for MPCBB1 registers"]
pub type MPCBB1_REGIE_R = crate::BitReader;
#[doc = "Field `MPCBB1_REGIE` writer - illegal access interrupt enable for MPCBB1 registers"]
pub type MPCBB1_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2IE` reader - illegal access interrupt enable for SRAM2"]
pub type SRAM2IE_R = crate::BitReader;
#[doc = "Field `SRAM2IE` writer - illegal access interrupt enable for SRAM2"]
pub type SRAM2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2_REGIE` reader - illegal access interrupt enable for MPCBB2 registers"]
pub type MPCBB2_REGIE_R = crate::BitReader;
#[doc = "Field `MPCBB2_REGIE` writer - illegal access interrupt enable for MPCBB2 registers"]
pub type MPCBB2_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3IE` reader - illegal access interrupt enable for SRAM3"]
pub type SRAM3IE_R = crate::BitReader;
#[doc = "Field `SRAM3IE` writer - illegal access interrupt enable for SRAM3"]
pub type SRAM3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB3_REGIE` reader - illegal access interrupt enable for MPCBB3 registers"]
pub type MPCBB3_REGIE_R = crate::BitReader;
#[doc = "Field `MPCBB3_REGIE` writer - illegal access interrupt enable for MPCBB3 registers"]
pub type MPCBB3_REGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - illegal access interrupt enable for GPDMA1"]
    #[inline(always)]
    pub fn gpdma1ie(&self) -> GPDMA1IE_R {
        GPDMA1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for GPDMA2"]
    #[inline(always)]
    pub fn gpdma2ie(&self) -> GPDMA2IE_R {
        GPDMA2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for FLASH registers"]
    #[inline(always)]
    pub fn flash_regie(&self) -> FLASH_REGIE_R {
        FLASH_REGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for FLASH memory"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for SBS"]
    #[inline(always)]
    pub fn sbsie(&self) -> SBSIE_R {
        SBSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for RTC"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for TAMP"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for PWR"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for RCC"]
    #[inline(always)]
    pub fn rccie(&self) -> RCCIE_R {
        RCCIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for EXTI"]
    #[inline(always)]
    pub fn extiie(&self) -> EXTIIE_R {
        EXTIIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for GTZC1 TZSC registers"]
    #[inline(always)]
    pub fn tzsc1ie(&self) -> TZSC1IE_R {
        TZSC1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for GTZC1 TZIC registers"]
    #[inline(always)]
    pub fn tzic1ie(&self) -> TZIC1IE_R {
        TZIC1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank"]
    #[inline(always)]
    pub fn octospi1_memie(&self) -> OCTOSPI1_MEMIE_R {
        OCTOSPI1_MEMIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
    #[inline(always)]
    pub fn fmc_memie(&self) -> FMC_MEMIE_R {
        FMC_MEMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - illegal access interrupt enable for MPCWM4 (BKPSRAM) memory bank"]
    #[inline(always)]
    pub fn bkpsramie(&self) -> BKPSRAMIE_R {
        BKPSRAMIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for SRAM1"]
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - illegal access interrupt enable for MPCBB1 registers"]
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for SRAM2"]
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - illegal access interrupt enable for MPCBB2 registers"]
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - illegal access interrupt enable for SRAM3"]
    #[inline(always)]
    pub fn sram3ie(&self) -> SRAM3IE_R {
        SRAM3IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - illegal access interrupt enable for MPCBB3 registers"]
    #[inline(always)]
    pub fn mpcbb3_regie(&self) -> MPCBB3_REGIE_R {
        MPCBB3_REGIE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - illegal access interrupt enable for GPDMA1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdma1ie(&mut self) -> GPDMA1IE_W<GTZC1_TZIC_IER4rs> {
        GPDMA1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - illegal access interrupt enable for GPDMA2"]
    #[inline(always)]
    #[must_use]
    pub fn gpdma2ie(&mut self) -> GPDMA2IE_W<GTZC1_TZIC_IER4rs> {
        GPDMA2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - illegal access interrupt enable for FLASH registers"]
    #[inline(always)]
    #[must_use]
    pub fn flash_regie(&mut self) -> FLASH_REGIE_W<GTZC1_TZIC_IER4rs> {
        FLASH_REGIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - illegal access interrupt enable for FLASH memory"]
    #[inline(always)]
    #[must_use]
    pub fn flashie(&mut self) -> FLASHIE_W<GTZC1_TZIC_IER4rs> {
        FLASHIE_W::new(self, 3)
    }
    #[doc = "Bit 6 - illegal access interrupt enable for SBS"]
    #[inline(always)]
    #[must_use]
    pub fn sbsie(&mut self) -> SBSIE_W<GTZC1_TZIC_IER4rs> {
        SBSIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - illegal access interrupt enable for RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtcie(&mut self) -> RTCIE_W<GTZC1_TZIC_IER4rs> {
        RTCIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - illegal access interrupt enable for TAMP"]
    #[inline(always)]
    #[must_use]
    pub fn tampie(&mut self) -> TAMPIE_W<GTZC1_TZIC_IER4rs> {
        TAMPIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - illegal access interrupt enable for PWR"]
    #[inline(always)]
    #[must_use]
    pub fn pwrie(&mut self) -> PWRIE_W<GTZC1_TZIC_IER4rs> {
        PWRIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - illegal access interrupt enable for RCC"]
    #[inline(always)]
    #[must_use]
    pub fn rccie(&mut self) -> RCCIE_W<GTZC1_TZIC_IER4rs> {
        RCCIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - illegal access interrupt enable for EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn extiie(&mut self) -> EXTIIE_W<GTZC1_TZIC_IER4rs> {
        EXTIIE_W::new(self, 11)
    }
    #[doc = "Bit 16 - illegal access interrupt enable for GTZC1 TZSC registers"]
    #[inline(always)]
    #[must_use]
    pub fn tzsc1ie(&mut self) -> TZSC1IE_W<GTZC1_TZIC_IER4rs> {
        TZSC1IE_W::new(self, 16)
    }
    #[doc = "Bit 17 - illegal access interrupt enable for GTZC1 TZIC registers"]
    #[inline(always)]
    #[must_use]
    pub fn tzic1ie(&mut self) -> TZIC1IE_W<GTZC1_TZIC_IER4rs> {
        TZIC1IE_W::new(self, 17)
    }
    #[doc = "Bit 18 - illegal access interrupt enable for MPCWM1 (OCTOSPI1) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_memie(&mut self) -> OCTOSPI1_MEMIE_W<GTZC1_TZIC_IER4rs> {
        OCTOSPI1_MEMIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - illegal access interrupt enable for MPCWM2 (FMC_NOR bank), MPCWM3 (FMC_NAND bank and FMC_SDRAM bank 1), and MPCWM4 (FMC_SDRAM�bank 2)"]
    #[inline(always)]
    #[must_use]
    pub fn fmc_memie(&mut self) -> FMC_MEMIE_W<GTZC1_TZIC_IER4rs> {
        FMC_MEMIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - illegal access interrupt enable for MPCWM4 (BKPSRAM) memory bank"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramie(&mut self) -> BKPSRAMIE_W<GTZC1_TZIC_IER4rs> {
        BKPSRAMIE_W::new(self, 20)
    }
    #[doc = "Bit 24 - illegal access interrupt enable for SRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<GTZC1_TZIC_IER4rs> {
        SRAM1IE_W::new(self, 24)
    }
    #[doc = "Bit 25 - illegal access interrupt enable for MPCBB1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W<GTZC1_TZIC_IER4rs> {
        MPCBB1_REGIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - illegal access interrupt enable for SRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<GTZC1_TZIC_IER4rs> {
        SRAM2IE_W::new(self, 26)
    }
    #[doc = "Bit 27 - illegal access interrupt enable for MPCBB2 registers"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W<GTZC1_TZIC_IER4rs> {
        MPCBB2_REGIE_W::new(self, 27)
    }
    #[doc = "Bit 28 - illegal access interrupt enable for SRAM3"]
    #[inline(always)]
    #[must_use]
    pub fn sram3ie(&mut self) -> SRAM3IE_W<GTZC1_TZIC_IER4rs> {
        SRAM3IE_W::new(self, 28)
    }
    #[doc = "Bit 29 - illegal access interrupt enable for MPCBB3 registers"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb3_regie(&mut self) -> MPCBB3_REGIE_W<GTZC1_TZIC_IER4rs> {
        MPCBB3_REGIE_W::new(self, 29)
    }
}
#[doc = "GTZC1 TZIC interrupt enable register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzic_ier4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzic_ier4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZIC_IER4rs;
impl crate::RegisterSpec for GTZC1_TZIC_IER4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzic_ier4::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZIC_IER4rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzic_ier4::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZIC_IER4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZIC_IER4 to value 0"]
impl crate::Resettable for GTZC1_TZIC_IER4rs {
    const RESET_VALUE: u32 = 0;
}
