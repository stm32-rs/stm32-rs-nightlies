///Register `RCC_SECCFGR` reader
pub type R = crate::R<RCC_SECCFGRrs>;
///Register `RCC_SECCFGR` writer
pub type W = crate::W<RCC_SECCFGRrs>;
///Field `HSISEC` reader - HSI clock configuration and status bit security This bit is set and reset by software.
pub type HSISEC_R = crate::BitReader;
///Field `HSISEC` writer - HSI clock configuration and status bit security This bit is set and reset by software.
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEC` reader - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
pub type HSESEC_R = crate::BitReader;
///Field `HSESEC` writer - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISEC` reader - MSI clock configuration and status bit security This bit is set and reset by software.
pub type MSISEC_R = crate::BitReader;
///Field `MSISEC` writer - MSI clock configuration and status bit security This bit is set and reset by software.
pub type MSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSISEC` reader - LSI clock configuration and status bit security This bit is set and reset by software.
pub type LSISEC_R = crate::BitReader;
///Field `LSISEC` writer - LSI clock configuration and status bit security This bit is set and reset by software.
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESEC` reader - LSE clock configuration and status bit security This bit is set and reset by software.
pub type LSESEC_R = crate::BitReader;
///Field `LSESEC` writer - LSE clock configuration and status bit security This bit is set and reset by software.
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCLKSEC` reader - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
pub type SYSCLKSEC_R = crate::BitReader;
///Field `SYSCLKSEC` writer - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
pub type SYSCLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
pub type PRESCSEC_R = crate::BitReader;
///Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
pub type PRESCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1SEC` reader - PLL1 clock configuration and status bit security This bit is set and reset by software.
pub type PLL1SEC_R = crate::BitReader;
///Field `PLL1SEC` writer - PLL1 clock configuration and status bit security This bit is set and reset by software.
pub type PLL1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2SEC` reader - PLL2 clock configuration and status bit security Set and reset by software.
pub type PLL2SEC_R = crate::BitReader;
///Field `PLL2SEC` writer - PLL2 clock configuration and status bit security Set and reset by software.
pub type PLL2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3SEC` reader - PLL3 clock configuration and status bit security This bit is set and reset by software.
pub type PLL3SEC_R = crate::BitReader;
///Field `PLL3SEC` writer - PLL3 clock configuration and status bit security This bit is set and reset by software.
pub type PLL3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICLKSEC` reader - Intermediate clock source selection security This bit is set and reset by software.
pub type ICLKSEC_R = crate::BitReader;
///Field `ICLKSEC` writer - Intermediate clock source selection security This bit is set and reset by software.
pub type ICLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48SEC` reader - HSI48 clock configuration and status bit security This bit is set and reset by software.
pub type HSI48SEC_R = crate::BitReader;
///Field `HSI48SEC` writer - HSI48 clock configuration and status bit security This bit is set and reset by software.
pub type HSI48SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMVFSEC` reader - Remove reset flag security This bit is set and reset by software.
pub type RMVFSEC_R = crate::BitReader;
///Field `RMVFSEC` writer - Remove reset flag security This bit is set and reset by software.
pub type RMVFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSE clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL1 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL2 clock configuration and status bit security Set and reset by software.
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL3 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Intermediate clock source selection security This bit is set and reset by software.
    #[inline(always)]
    pub fn iclksec(&self) -> ICLKSEC_R {
        ICLKSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI48 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Remove reset flag security This bit is set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SECCFGR")
            .field("hsisec", &self.hsisec())
            .field("hsesec", &self.hsesec())
            .field("msisec", &self.msisec())
            .field("lsisec", &self.lsisec())
            .field("lsesec", &self.lsesec())
            .field("sysclksec", &self.sysclksec())
            .field("prescsec", &self.prescsec())
            .field("pll1sec", &self.pll1sec())
            .field("pll2sec", &self.pll2sec())
            .field("pll3sec", &self.pll3sec())
            .field("iclksec", &self.iclksec())
            .field("hsi48sec", &self.hsi48sec())
            .field("rmvfsec", &self.rmvfsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn hsisec(&mut self) -> HSISEC_W<RCC_SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    ///Bit 1 - HSE clock configuration bits, status bit and HSE_CSS security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn hsesec(&mut self) -> HSESEC_W<RCC_SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    ///Bit 2 - MSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn msisec(&mut self) -> MSISEC_W<RCC_SECCFGRrs> {
        MSISEC_W::new(self, 2)
    }
    ///Bit 3 - LSI clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn lsisec(&mut self) -> LSISEC_W<RCC_SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    ///Bit 4 - LSE clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn lsesec(&mut self) -> LSESEC_W<RCC_SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    ///Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<RCC_SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    ///Bit 6 - AHBx/APBx prescaler configuration bits security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn prescsec(&mut self) -> PRESCSEC_W<RCC_SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    ///Bit 7 - PLL1 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<RCC_SECCFGRrs> {
        PLL1SEC_W::new(self, 7)
    }
    ///Bit 8 - PLL2 clock configuration and status bit security Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<RCC_SECCFGRrs> {
        PLL2SEC_W::new(self, 8)
    }
    ///Bit 9 - PLL3 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<RCC_SECCFGRrs> {
        PLL3SEC_W::new(self, 9)
    }
    ///Bit 10 - Intermediate clock source selection security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn iclksec(&mut self) -> ICLKSEC_W<RCC_SECCFGRrs> {
        ICLKSEC_W::new(self, 10)
    }
    ///Bit 11 - HSI48 clock configuration and status bit security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<RCC_SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    ///Bit 12 - Remove reset flag security This bit is set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<RCC_SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
}
/**RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#RCC:RCC_SECCFGR)*/
pub struct RCC_SECCFGRrs;
impl crate::RegisterSpec for RCC_SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_seccfgr::R`](R) reader structure
impl crate::Readable for RCC_SECCFGRrs {}
///`write(|w| ..)` method takes [`rcc_seccfgr::W`](W) writer structure
impl crate::Writable for RCC_SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_SECCFGR to value 0
impl crate::Resettable for RCC_SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
