///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
/**HSI clock configuration and status bits security Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSISEC {
    ///0: Non secure
    NonSecure = 0,
    ///1: Secure
    Secure = 1,
}
impl From<HSISEC> for bool {
    #[inline(always)]
    fn from(variant: HSISEC) -> Self {
        variant as u8 != 0
    }
}
///Field `HSISEC` reader - HSI clock configuration and status bits security Set and reset by software.
pub type HSISEC_R = crate::BitReader<HSISEC>;
impl HSISEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSISEC {
        match self.bits {
            false => HSISEC::NonSecure,
            true => HSISEC::Secure,
        }
    }
    ///Non secure
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == HSISEC::NonSecure
    }
    ///Secure
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == HSISEC::Secure
    }
}
///Field `HSISEC` writer - HSI clock configuration and status bits security Set and reset by software.
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG, HSISEC>;
impl<'a, REG> HSISEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Non secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(HSISEC::NonSecure)
    }
    ///Secure
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(HSISEC::Secure)
    }
}
///Field `HSESEC` reader - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software.
pub use HSISEC_R as HSESEC_R;
///Field `CSISEC` reader - CSI clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as CSISEC_R;
///Field `LSISEC` reader - LSI clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as LSISEC_R;
///Field `LSESEC` reader - LSE clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as LSESEC_R;
///Field `SYSCLKSEC` reader - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software.
pub use HSISEC_R as SYSCLKSEC_R;
///Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security Set and reset by software.
pub use HSISEC_R as PRESCSEC_R;
///Field `PLL1SEC` reader - PLL1 clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as PLL1SEC_R;
///Field `PLL2SEC` reader - PLL2 clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as PLL2SEC_R;
///Field `PLL3SEC` reader - PLL3 clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as PLL3SEC_R;
///Field `HSI48SEC` reader - HSI48 clock configuration and status bits security Set and reset by software.
pub use HSISEC_R as HSI48SEC_R;
///Field `RMVFSEC` reader - Remove reset flag security Set and reset by software.
pub use HSISEC_R as RMVFSEC_R;
///Field `CKPERSELSEC` reader - per_ck selection security Set and reset by software.
pub use HSISEC_R as CKPERSELSEC_R;
///Field `HSESEC` writer - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software.
pub use HSISEC_W as HSESEC_W;
///Field `CSISEC` writer - CSI clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as CSISEC_W;
///Field `LSISEC` writer - LSI clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as LSISEC_W;
///Field `LSESEC` writer - LSE clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as LSESEC_W;
///Field `SYSCLKSEC` writer - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software.
pub use HSISEC_W as SYSCLKSEC_W;
///Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security Set and reset by software.
pub use HSISEC_W as PRESCSEC_W;
///Field `PLL1SEC` writer - PLL1 clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as PLL1SEC_W;
///Field `PLL2SEC` writer - PLL2 clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as PLL2SEC_W;
///Field `PLL3SEC` writer - PLL3 clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as PLL3SEC_W;
///Field `HSI48SEC` writer - HSI48 clock configuration and status bits security Set and reset by software.
pub use HSISEC_W as HSI48SEC_W;
///Field `RMVFSEC` writer - Remove reset flag security Set and reset by software.
pub use HSISEC_W as RMVFSEC_W;
///Field `CKPERSELSEC` writer - per_ck selection security Set and reset by software.
pub use HSISEC_W as CKPERSELSEC_W;
impl R {
    ///Bit 0 - HSI clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software.
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CSI clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn csisec(&self) -> CSISEC_R {
        CSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSI clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSE clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software.
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHBx/APBx prescaler configuration bits security Set and reset by software.
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLL1 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLL2 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLL3 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - HSI48 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Remove reset flag security Set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - per_ck selection security Set and reset by software.
    #[inline(always)]
    pub fn ckperselsec(&self) -> CKPERSELSEC_R {
        CKPERSELSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("hsisec", &self.hsisec())
            .field("hsesec", &self.hsesec())
            .field("csisec", &self.csisec())
            .field("lsisec", &self.lsisec())
            .field("lsesec", &self.lsesec())
            .field("sysclksec", &self.sysclksec())
            .field("prescsec", &self.prescsec())
            .field("pll1sec", &self.pll1sec())
            .field("pll2sec", &self.pll2sec())
            .field("pll3sec", &self.pll3sec())
            .field("hsi48sec", &self.hsi48sec())
            .field("rmvfsec", &self.rmvfsec())
            .field("ckperselsec", &self.ckperselsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W<'_, SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    ///Bit 1 - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software.
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W<'_, SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    ///Bit 2 - CSI clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn csisec(&mut self) -> CSISEC_W<'_, SECCFGRrs> {
        CSISEC_W::new(self, 2)
    }
    ///Bit 3 - LSI clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn lsisec(&mut self) -> LSISEC_W<'_, SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    ///Bit 4 - LSE clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn lsesec(&mut self) -> LSESEC_W<'_, SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    ///Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software.
    #[inline(always)]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<'_, SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    ///Bit 6 - AHBx/APBx prescaler configuration bits security Set and reset by software.
    #[inline(always)]
    pub fn prescsec(&mut self) -> PRESCSEC_W<'_, SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    ///Bit 7 - PLL1 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<'_, SECCFGRrs> {
        PLL1SEC_W::new(self, 7)
    }
    ///Bit 8 - PLL2 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<'_, SECCFGRrs> {
        PLL2SEC_W::new(self, 8)
    }
    ///Bit 9 - PLL3 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<'_, SECCFGRrs> {
        PLL3SEC_W::new(self, 9)
    }
    ///Bit 11 - HSI48 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<'_, SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    ///Bit 12 - Remove reset flag security Set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<'_, SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
    ///Bit 13 - per_ck selection security Set and reset by software.
    #[inline(always)]
    pub fn ckperselsec(&mut self) -> CKPERSELSEC_W<'_, SECCFGRrs> {
        CKPERSELSEC_W::new(self, 13)
    }
}
/**RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
