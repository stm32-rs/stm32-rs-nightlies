#[doc = "Register `PRIVCFGR2` reader"]
pub type R = crate::R<PRIVCFGR2rs>;
#[doc = "Register `PRIVCFGR2` writer"]
pub type W = crate::W<PRIVCFGR2rs>;
#[doc = "Field `TIM8PRIV` reader - TIM8PRIV"]
pub type TIM8PRIV_R = crate::BitReader;
#[doc = "Field `TIM8PRIV` writer - TIM8PRIV"]
pub type TIM8PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1PRIV` reader - USART1PRIV"]
pub type USART1PRIV_R = crate::BitReader;
#[doc = "Field `USART1PRIV` writer - USART1PRIV"]
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15PRIV` reader - TIM15PRIV"]
pub type TIM15PRIV_R = crate::BitReader;
#[doc = "Field `TIM15PRIV` writer - TIM15PRIV"]
pub type TIM15PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16PRIV` reader - TIM16PRIV"]
pub type TIM16PRIV_R = crate::BitReader;
#[doc = "Field `TIM16PRIV` writer - TIM16PRIV"]
pub type TIM16PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17PRIV` reader - TIM17PRIV"]
pub type TIM17PRIV_R = crate::BitReader;
#[doc = "Field `TIM17PRIV` writer - TIM17PRIV"]
pub type TIM17PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1PRIV` reader - SAI1PRIV"]
pub type SAI1PRIV_R = crate::BitReader;
#[doc = "Field `SAI1PRIV` writer - SAI1PRIV"]
pub type SAI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2PRIV` reader - SAI2PRIV"]
pub type SAI2PRIV_R = crate::BitReader;
#[doc = "Field `SAI2PRIV` writer - SAI2PRIV"]
pub type SAI2PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDM1PRIV` reader - DFSDM1PRIV"]
pub type DFSDM1PRIV_R = crate::BitReader;
#[doc = "Field `DFSDM1PRIV` writer - DFSDM1PRIV"]
pub type DFSDM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCPRIV` reader - CRCPRIV"]
pub type CRCPRIV_R = crate::BitReader;
#[doc = "Field `CRCPRIV` writer - CRCPRIV"]
pub type CRCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCPRIV` reader - TSCPRIV"]
pub type TSCPRIV_R = crate::BitReader;
#[doc = "Field `TSCPRIV` writer - TSCPRIV"]
pub type TSCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHEPRIV` reader - ICACHEPRIV"]
pub type ICACHEPRIV_R = crate::BitReader;
#[doc = "Field `ICACHEPRIV` writer - ICACHEPRIV"]
pub type ICACHEPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPRIV` reader - ADCPRIV"]
pub type ADCPRIV_R = crate::BitReader;
#[doc = "Field `ADCPRIV` writer - ADCPRIV"]
pub type ADCPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESPRIV` reader - AESPRIV"]
pub type AESPRIV_R = crate::BitReader;
#[doc = "Field `AESPRIV` writer - AESPRIV"]
pub type AESPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHPRIV` reader - HASHPRIV"]
pub type HASHPRIV_R = crate::BitReader;
#[doc = "Field `HASHPRIV` writer - HASHPRIV"]
pub type HASHPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGPRIV` reader - RNGPRIV"]
pub type RNGPRIV_R = crate::BitReader;
#[doc = "Field `RNGPRIV` writer - RNGPRIV"]
pub type RNGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKAPRIV` reader - PKAPRIV"]
pub type PKAPRIV_R = crate::BitReader;
#[doc = "Field `PKAPRIV` writer - PKAPRIV"]
pub type PKAPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1PRIV` reader - SDMMC1PRIV"]
pub type SDMMC1PRIV_R = crate::BitReader;
#[doc = "Field `SDMMC1PRIV` writer - SDMMC1PRIV"]
pub type SDMMC1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMC_REGPRIV` reader - FSMC_REGPRIV"]
pub type FSMC_REGPRIV_R = crate::BitReader;
#[doc = "Field `FSMC_REGPRIV` writer - FSMC_REGPRIV"]
pub type FSMC_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1_REGPRIV` reader - OCTOSPI1_REGRIV"]
pub type OCTOSPI1_REGPRIV_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_REGPRIV` writer - OCTOSPI1_REGRIV"]
pub type OCTOSPI1_REGPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM8PRIV"]
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART1PRIV"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15PRIV"]
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16PRIV"]
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17PRIV"]
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAI1PRIV"]
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAI2PRIV"]
    #[inline(always)]
    pub fn sai2priv(&self) -> SAI2PRIV_R {
        SAI2PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFSDM1PRIV"]
    #[inline(always)]
    pub fn dfsdm1priv(&self) -> DFSDM1PRIV_R {
        DFSDM1PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CRCPRIV"]
    #[inline(always)]
    pub fn crcpriv(&self) -> CRCPRIV_R {
        CRCPRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCPRIV"]
    #[inline(always)]
    pub fn tscpriv(&self) -> TSCPRIV_R {
        TSCPRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHEPRIV"]
    #[inline(always)]
    pub fn icachepriv(&self) -> ICACHEPRIV_R {
        ICACHEPRIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADCPRIV"]
    #[inline(always)]
    pub fn adcpriv(&self) -> ADCPRIV_R {
        ADCPRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AESPRIV"]
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HASHPRIV"]
    #[inline(always)]
    pub fn hashpriv(&self) -> HASHPRIV_R {
        HASHPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RNGPRIV"]
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKAPRIV"]
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1PRIV"]
    #[inline(always)]
    pub fn sdmmc1priv(&self) -> SDMMC1PRIV_R {
        SDMMC1PRIV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FSMC_REGPRIV"]
    #[inline(always)]
    pub fn fsmc_regpriv(&self) -> FSMC_REGPRIV_R {
        FSMC_REGPRIV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGRIV"]
    #[inline(always)]
    pub fn octospi1_regpriv(&self) -> OCTOSPI1_REGPRIV_R {
        OCTOSPI1_REGPRIV_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W<PRIVCFGR2rs> {
        TIM8PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - USART1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<PRIVCFGR2rs> {
        USART1PRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM15PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W<PRIVCFGR2rs> {
        TIM15PRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM16PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<PRIVCFGR2rs> {
        TIM16PRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM17PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<PRIVCFGR2rs> {
        TIM17PRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - SAI1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W<PRIVCFGR2rs> {
        SAI1PRIV_W::new(self, 5)
    }
    #[doc = "Bit 6 - SAI2PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn sai2priv(&mut self) -> SAI2PRIV_W<PRIVCFGR2rs> {
        SAI2PRIV_W::new(self, 6)
    }
    #[doc = "Bit 7 - DFSDM1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1priv(&mut self) -> DFSDM1PRIV_W<PRIVCFGR2rs> {
        DFSDM1PRIV_W::new(self, 7)
    }
    #[doc = "Bit 8 - CRCPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn crcpriv(&mut self) -> CRCPRIV_W<PRIVCFGR2rs> {
        CRCPRIV_W::new(self, 8)
    }
    #[doc = "Bit 9 - TSCPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn tscpriv(&mut self) -> TSCPRIV_W<PRIVCFGR2rs> {
        TSCPRIV_W::new(self, 9)
    }
    #[doc = "Bit 10 - ICACHEPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn icachepriv(&mut self) -> ICACHEPRIV_W<PRIVCFGR2rs> {
        ICACHEPRIV_W::new(self, 10)
    }
    #[doc = "Bit 11 - ADCPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn adcpriv(&mut self) -> ADCPRIV_W<PRIVCFGR2rs> {
        ADCPRIV_W::new(self, 11)
    }
    #[doc = "Bit 12 - AESPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn aespriv(&mut self) -> AESPRIV_W<PRIVCFGR2rs> {
        AESPRIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - HASHPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn hashpriv(&mut self) -> HASHPRIV_W<PRIVCFGR2rs> {
        HASHPRIV_W::new(self, 13)
    }
    #[doc = "Bit 14 - RNGPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<PRIVCFGR2rs> {
        RNGPRIV_W::new(self, 14)
    }
    #[doc = "Bit 15 - PKAPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<PRIVCFGR2rs> {
        PKAPRIV_W::new(self, 15)
    }
    #[doc = "Bit 16 - SDMMC1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1priv(&mut self) -> SDMMC1PRIV_W<PRIVCFGR2rs> {
        SDMMC1PRIV_W::new(self, 16)
    }
    #[doc = "Bit 17 - FSMC_REGPRIV"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_regpriv(&mut self) -> FSMC_REGPRIV_W<PRIVCFGR2rs> {
        FSMC_REGPRIV_W::new(self, 17)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGRIV"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regpriv(&mut self) -> OCTOSPI1_REGPRIV_W<PRIVCFGR2rs> {
        OCTOSPI1_REGPRIV_W::new(self, 18)
    }
}
#[doc = "TZSC privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr2::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
