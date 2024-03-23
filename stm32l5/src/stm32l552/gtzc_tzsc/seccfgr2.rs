#[doc = "Register `SECCFGR2` reader"]
pub type R = crate::R<SECCFGR2rs>;
#[doc = "Register `SECCFGR2` writer"]
pub type W = crate::W<SECCFGR2rs>;
#[doc = "Field `TIM8SEC` reader - TIM8SEC"]
pub type TIM8SEC_R = crate::BitReader;
#[doc = "Field `TIM8SEC` writer - TIM8SEC"]
pub type TIM8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SEC` reader - USART1SEC"]
pub type USART1SEC_R = crate::BitReader;
#[doc = "Field `USART1SEC` writer - USART1SEC"]
pub type USART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SEC` reader - TIM15SEC"]
pub type TIM15SEC_R = crate::BitReader;
#[doc = "Field `TIM15SEC` writer - TIM15SEC"]
pub type TIM15SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SEC` reader - TIM16SEC"]
pub type TIM16SEC_R = crate::BitReader;
#[doc = "Field `TIM16SEC` writer - TIM16SEC"]
pub type TIM16SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SEC` reader - TIM17SEC"]
pub type TIM17SEC_R = crate::BitReader;
#[doc = "Field `TIM17SEC` writer - TIM17SEC"]
pub type TIM17SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1SEC` reader - SAI1SEC"]
pub type SAI1SEC_R = crate::BitReader;
#[doc = "Field `SAI1SEC` writer - SAI1SEC"]
pub type SAI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2SEC` reader - SAI2SEC"]
pub type SAI2SEC_R = crate::BitReader;
#[doc = "Field `SAI2SEC` writer - SAI2SEC"]
pub type SAI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDM1SEC` reader - DFSDM1SEC"]
pub type DFSDM1SEC_R = crate::BitReader;
#[doc = "Field `DFSDM1SEC` writer - DFSDM1SEC"]
pub type DFSDM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSEC` reader - CRCSEC"]
pub type CRCSEC_R = crate::BitReader;
#[doc = "Field `CRCSEC` writer - CRCSEC"]
pub type CRCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCSEC` reader - TSCSEC"]
pub type TSCSEC_R = crate::BitReader;
#[doc = "Field `TSCSEC` writer - TSCSEC"]
pub type TSCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHESEC` reader - ICACHESEC"]
pub type ICACHESEC_R = crate::BitReader;
#[doc = "Field `ICACHESEC` writer - ICACHESEC"]
pub type ICACHESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSEC` reader - ADCSEC"]
pub type ADCSEC_R = crate::BitReader;
#[doc = "Field `ADCSEC` writer - ADCSEC"]
pub type ADCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESSEC` reader - AESSEC"]
pub type AESSEC_R = crate::BitReader;
#[doc = "Field `AESSEC` writer - AESSEC"]
pub type AESSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHSEC` reader - HASHSEC"]
pub type HASHSEC_R = crate::BitReader;
#[doc = "Field `HASHSEC` writer - HASHSEC"]
pub type HASHSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGSEC` reader - RNGSEC"]
pub type RNGSEC_R = crate::BitReader;
#[doc = "Field `RNGSEC` writer - RNGSEC"]
pub type RNGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKASEC` reader - PKASEC"]
pub type PKASEC_R = crate::BitReader;
#[doc = "Field `PKASEC` writer - PKASEC"]
pub type PKASEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1SEC` reader - SDMMC1SEC"]
pub type SDMMC1SEC_R = crate::BitReader;
#[doc = "Field `SDMMC1SEC` writer - SDMMC1SEC"]
pub type SDMMC1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSMC_REGSEC` reader - FSMC_REGSEC"]
pub type FSMC_REGSEC_R = crate::BitReader;
#[doc = "Field `FSMC_REGSEC` writer - FSMC_REGSEC"]
pub type FSMC_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCTOSPI1_REGSEC` reader - OCTOSPI1_REGSEC"]
pub type OCTOSPI1_REGSEC_R = crate::BitReader;
#[doc = "Field `OCTOSPI1_REGSEC` writer - OCTOSPI1_REGSEC"]
pub type OCTOSPI1_REGSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM8SEC"]
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART1SEC"]
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM15SEC"]
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM16SEC"]
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM17SEC"]
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAI1SEC"]
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SAI2SEC"]
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DFSDM1SEC"]
    #[inline(always)]
    pub fn dfsdm1sec(&self) -> DFSDM1SEC_R {
        DFSDM1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CRCSEC"]
    #[inline(always)]
    pub fn crcsec(&self) -> CRCSEC_R {
        CRCSEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCSEC"]
    #[inline(always)]
    pub fn tscsec(&self) -> TSCSEC_R {
        TSCSEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHESEC"]
    #[inline(always)]
    pub fn icachesec(&self) -> ICACHESEC_R {
        ICACHESEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADCSEC"]
    #[inline(always)]
    pub fn adcsec(&self) -> ADCSEC_R {
        ADCSEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AESSEC"]
    #[inline(always)]
    pub fn aessec(&self) -> AESSEC_R {
        AESSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HASHSEC"]
    #[inline(always)]
    pub fn hashsec(&self) -> HASHSEC_R {
        HASHSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RNGSEC"]
    #[inline(always)]
    pub fn rngsec(&self) -> RNGSEC_R {
        RNGSEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PKASEC"]
    #[inline(always)]
    pub fn pkasec(&self) -> PKASEC_R {
        PKASEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1SEC"]
    #[inline(always)]
    pub fn sdmmc1sec(&self) -> SDMMC1SEC_R {
        SDMMC1SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FSMC_REGSEC"]
    #[inline(always)]
    pub fn fsmc_regsec(&self) -> FSMC_REGSEC_R {
        FSMC_REGSEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGSEC"]
    #[inline(always)]
    pub fn octospi1_regsec(&self) -> OCTOSPI1_REGSEC_R {
        OCTOSPI1_REGSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM8SEC"]
    #[inline(always)]
    #[must_use]
    pub fn tim8sec(&mut self) -> TIM8SEC_W<SECCFGR2rs> {
        TIM8SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - USART1SEC"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sec(&mut self) -> USART1SEC_W<SECCFGR2rs> {
        USART1SEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM15SEC"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sec(&mut self) -> TIM15SEC_W<SECCFGR2rs> {
        TIM15SEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM16SEC"]
    #[inline(always)]
    #[must_use]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<SECCFGR2rs> {
        TIM16SEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM17SEC"]
    #[inline(always)]
    #[must_use]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<SECCFGR2rs> {
        TIM17SEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - SAI1SEC"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sec(&mut self) -> SAI1SEC_W<SECCFGR2rs> {
        SAI1SEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - SAI2SEC"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sec(&mut self) -> SAI2SEC_W<SECCFGR2rs> {
        SAI2SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - DFSDM1SEC"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1sec(&mut self) -> DFSDM1SEC_W<SECCFGR2rs> {
        DFSDM1SEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - CRCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn crcsec(&mut self) -> CRCSEC_W<SECCFGR2rs> {
        CRCSEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - TSCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn tscsec(&mut self) -> TSCSEC_W<SECCFGR2rs> {
        TSCSEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - ICACHESEC"]
    #[inline(always)]
    #[must_use]
    pub fn icachesec(&mut self) -> ICACHESEC_W<SECCFGR2rs> {
        ICACHESEC_W::new(self, 10)
    }
    #[doc = "Bit 11 - ADCSEC"]
    #[inline(always)]
    #[must_use]
    pub fn adcsec(&mut self) -> ADCSEC_W<SECCFGR2rs> {
        ADCSEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - AESSEC"]
    #[inline(always)]
    #[must_use]
    pub fn aessec(&mut self) -> AESSEC_W<SECCFGR2rs> {
        AESSEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - HASHSEC"]
    #[inline(always)]
    #[must_use]
    pub fn hashsec(&mut self) -> HASHSEC_W<SECCFGR2rs> {
        HASHSEC_W::new(self, 13)
    }
    #[doc = "Bit 14 - RNGSEC"]
    #[inline(always)]
    #[must_use]
    pub fn rngsec(&mut self) -> RNGSEC_W<SECCFGR2rs> {
        RNGSEC_W::new(self, 14)
    }
    #[doc = "Bit 15 - PKASEC"]
    #[inline(always)]
    #[must_use]
    pub fn pkasec(&mut self) -> PKASEC_W<SECCFGR2rs> {
        PKASEC_W::new(self, 15)
    }
    #[doc = "Bit 16 - SDMMC1SEC"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sec(&mut self) -> SDMMC1SEC_W<SECCFGR2rs> {
        SDMMC1SEC_W::new(self, 16)
    }
    #[doc = "Bit 17 - FSMC_REGSEC"]
    #[inline(always)]
    #[must_use]
    pub fn fsmc_regsec(&mut self) -> FSMC_REGSEC_W<SECCFGR2rs> {
        FSMC_REGSEC_W::new(self, 17)
    }
    #[doc = "Bit 18 - OCTOSPI1_REGSEC"]
    #[inline(always)]
    #[must_use]
    pub fn octospi1_regsec(&mut self) -> OCTOSPI1_REGSEC_W<SECCFGR2rs> {
        OCTOSPI1_REGSEC_W::new(self, 18)
    }
}
#[doc = "TZSC secure configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGR2rs;
impl crate::RegisterSpec for SECCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr2::R`](R) reader structure"]
impl crate::Readable for SECCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr2::W`](W) writer structure"]
impl crate::Writable for SECCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR2 to value 0"]
impl crate::Resettable for SECCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
