#[doc = "Register `RCC_SECCFGR` reader"]
pub type R = crate::R<RCC_SECCFGRrs>;
#[doc = "Register `RCC_SECCFGR` writer"]
pub type W = crate::W<RCC_SECCFGRrs>;
#[doc = "Field `HSISEC` reader - HSI clock configuration and status bit security This bit is set and reset by software."]
pub type HSISEC_R = crate::BitReader;
#[doc = "Field `HSISEC` writer - HSI clock configuration and status bit security This bit is set and reset by software."]
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSESEC` reader - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software."]
pub type HSESEC_R = crate::BitReader;
#[doc = "Field `HSESEC` writer - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software."]
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSISEC` reader - MSI clock configuration and status bit security This bit is set and reset by software."]
pub type MSISEC_R = crate::BitReader;
#[doc = "Field `MSISEC` writer - MSI clock configuration and status bit security This bit is set and reset by software."]
pub type MSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSISEC` reader - LSI clock configuration and status bit security This bit is set and reset by software."]
pub type LSISEC_R = crate::BitReader;
#[doc = "Field `LSISEC` writer - LSI clock configuration and status bit security This bit is set and reset by software."]
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSESEC` reader - LSE clock configuration and status bit security This bit is set and reset by software."]
pub type LSESEC_R = crate::BitReader;
#[doc = "Field `LSESEC` writer - LSE clock configuration and status bit security This bit is set and reset by software."]
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCLKSEC` reader - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software."]
pub type SYSCLKSEC_R = crate::BitReader;
#[doc = "Field `SYSCLKSEC` writer - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software."]
pub type SYSCLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security This bit is set and reset by software."]
pub type PRESCSEC_R = crate::BitReader;
#[doc = "Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security This bit is set and reset by software."]
pub type PRESCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1SEC` reader - PLL1 clock configuration and status bit security This bit is set and reset by software."]
pub type PLL1SEC_R = crate::BitReader;
#[doc = "Field `PLL1SEC` writer - PLL1 clock configuration and status bit security This bit is set and reset by software."]
pub type PLL1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2SEC` reader - PLL2 clock configuration and status bit security Set and reset by software."]
pub type PLL2SEC_R = crate::BitReader;
#[doc = "Field `PLL2SEC` writer - PLL2 clock configuration and status bit security Set and reset by software."]
pub type PLL2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3SEC` reader - PLL3 clock configuration and status bit security This bit is set and reset by software."]
pub type PLL3SEC_R = crate::BitReader;
#[doc = "Field `PLL3SEC` writer - PLL3 clock configuration and status bit security This bit is set and reset by software."]
pub type PLL3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICLKSEC` reader - Intermediate clock source selection security This bit is set and reset by software."]
pub type ICLKSEC_R = crate::BitReader;
#[doc = "Field `ICLKSEC` writer - Intermediate clock source selection security This bit is set and reset by software."]
pub type ICLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48SEC` reader - HSI48 clock configuration and status bit security This bit is set and reset by software."]
pub type HSI48SEC_R = crate::BitReader;
#[doc = "Field `HSI48SEC` writer - HSI48 clock configuration and status bit security This bit is set and reset by software."]
pub type HSI48SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMVFSEC` reader - Remove reset flag security This bit is set and reset by software."]
pub type RMVFSEC_R = crate::BitReader;
#[doc = "Field `RMVFSEC` writer - Remove reset flag security This bit is set and reset by software."]
pub type RMVFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSI clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software."]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSI clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSE clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software."]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHBx/APBx prescaler configuration bits security This bit is set and reset by software."]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL1 clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL2 clock configuration and status bit security Set and reset by software."]
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Intermediate clock source selection security This bit is set and reset by software."]
    #[inline(always)]
    pub fn iclksec(&self) -> ICLKSEC_R {
        ICLKSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48 clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remove reset flag security This bit is set and reset by software."]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsisec(&mut self) -> HSISEC_W<RCC_SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsesec(&mut self) -> HSESEC_W<RCC_SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - MSI clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn msisec(&mut self) -> MSISEC_W<RCC_SECCFGRrs> {
        MSISEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - LSI clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsisec(&mut self) -> LSISEC_W<RCC_SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - LSE clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsesec(&mut self) -> LSESEC_W<RCC_SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<RCC_SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - AHBx/APBx prescaler configuration bits security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn prescsec(&mut self) -> PRESCSEC_W<RCC_SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL1 clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<RCC_SECCFGRrs> {
        PLL1SEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - PLL2 clock configuration and status bit security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<RCC_SECCFGRrs> {
        PLL2SEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLL3 clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<RCC_SECCFGRrs> {
        PLL3SEC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Intermediate clock source selection security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn iclksec(&mut self) -> ICLKSEC_W<RCC_SECCFGRrs> {
        ICLKSEC_W::new(self, 10)
    }
    #[doc = "Bit 11 - HSI48 clock configuration and status bit security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<RCC_SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - Remove reset flag security This bit is set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<RCC_SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
}
#[doc = "RCC secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_SECCFGRrs;
impl crate::RegisterSpec for RCC_SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_seccfgr::R`](R) reader structure"]
impl crate::Readable for RCC_SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_seccfgr::W`](W) writer structure"]
impl crate::Writable for RCC_SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_SECCFGR to value 0"]
impl crate::Resettable for RCC_SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
