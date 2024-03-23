#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `HSISEC` reader - HSI clock configuration and status bits security Set and reset by software."]
pub type HSISEC_R = crate::BitReader;
#[doc = "Field `HSISEC` writer - HSI clock configuration and status bits security Set and reset by software."]
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSESEC` reader - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
pub type HSESEC_R = crate::BitReader;
#[doc = "Field `HSESEC` writer - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSISEC` reader - CSI clock configuration and status bits security Set and reset by software."]
pub type CSISEC_R = crate::BitReader;
#[doc = "Field `CSISEC` writer - CSI clock configuration and status bits security Set and reset by software."]
pub type CSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSISEC` reader - LSI clock configuration and status bits security Set and reset by software."]
pub type LSISEC_R = crate::BitReader;
#[doc = "Field `LSISEC` writer - LSI clock configuration and status bits security Set and reset by software."]
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSESEC` reader - LSE clock configuration and status bits security Set and reset by software."]
pub type LSESEC_R = crate::BitReader;
#[doc = "Field `LSESEC` writer - LSE clock configuration and status bits security Set and reset by software."]
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCLKSEC` reader - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
pub type SYSCLKSEC_R = crate::BitReader;
#[doc = "Field `SYSCLKSEC` writer - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
pub type SYSCLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security Set and reset by software."]
pub type PRESCSEC_R = crate::BitReader;
#[doc = "Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security Set and reset by software."]
pub type PRESCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1SEC` reader - PLL1 clock configuration and status bits security Set and reset by software."]
pub type PLL1SEC_R = crate::BitReader;
#[doc = "Field `PLL1SEC` writer - PLL1 clock configuration and status bits security Set and reset by software."]
pub type PLL1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2SEC` reader - PLL2 clock configuration and status bits security Set and reset by software."]
pub type PLL2SEC_R = crate::BitReader;
#[doc = "Field `PLL2SEC` writer - PLL2 clock configuration and status bits security Set and reset by software."]
pub type PLL2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3SEC` reader - PLL3 clock configuration and status bits security Set and reset by software."]
pub type PLL3SEC_R = crate::BitReader;
#[doc = "Field `PLL3SEC` writer - PLL3 clock configuration and status bits security Set and reset by software."]
pub type PLL3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSI48SEC` reader - HSI48 clock configuration and status bits security Set and reset by software."]
pub type HSI48SEC_R = crate::BitReader;
#[doc = "Field `HSI48SEC` writer - HSI48 clock configuration and status bits security Set and reset by software."]
pub type HSI48SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMVFSEC` reader - Remove reset flag security Set and reset by software."]
pub type RMVFSEC_R = crate::BitReader;
#[doc = "Field `RMVFSEC` writer - Remove reset flag security Set and reset by software."]
pub type RMVFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPERSELSEC` reader - per_ck selection security Set and reset by software."]
pub type CKPERSELSEC_R = crate::BitReader;
#[doc = "Field `CKPERSELSEC` writer - per_ck selection security Set and reset by software."]
pub type CKPERSELSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn csisec(&self) -> CSISEC_R {
        CSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSE clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHBx/APBx prescaler configuration bits security Set and reset by software."]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL1 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL2 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remove reset flag security Set and reset by software."]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - per_ck selection security Set and reset by software."]
    #[inline(always)]
    pub fn ckperselsec(&self) -> CKPERSELSEC_R {
        CKPERSELSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsisec(&mut self) -> HSISEC_W<SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsesec(&mut self) -> HSESEC_W<SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - CSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn csisec(&mut self) -> CSISEC_W<SECCFGRrs> {
        CSISEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - LSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsisec(&mut self) -> LSISEC_W<SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - LSE clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsesec(&mut self) -> LSESEC_W<SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - AHBx/APBx prescaler configuration bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn prescsec(&mut self) -> PRESCSEC_W<SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - PLL1 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<SECCFGRrs> {
        PLL1SEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - PLL2 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<SECCFGRrs> {
        PLL2SEC_W::new(self, 8)
    }
    #[doc = "Bit 9 - PLL3 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<SECCFGRrs> {
        PLL3SEC_W::new(self, 9)
    }
    #[doc = "Bit 11 - HSI48 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    #[doc = "Bit 12 - Remove reset flag security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - per_ck selection security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ckperselsec(&mut self) -> CKPERSELSEC_W<SECCFGRrs> {
        CKPERSELSEC_W::new(self, 13)
    }
}
#[doc = "RCC secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
