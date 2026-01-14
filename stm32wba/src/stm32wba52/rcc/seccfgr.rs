///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `HSISEC` reader - HSI16 clock configuration and status bits security Set and reset by software.
pub type HSISEC_R = crate::BitReader;
///Field `HSISEC` writer - HSI16 clock configuration and status bits security Set and reset by software.
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEC` reader - HSE32 clock configuration bits, status bits and HSECSS security Set and reset by software.
pub type HSESEC_R = crate::BitReader;
///Field `HSESEC` writer - HSE32 clock configuration bits, status bits and HSECSS security Set and reset by software.
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSISEC` reader - LSI clock configuration and status bits security Set and reset by software.
pub type LSISEC_R = crate::BitReader;
///Field `LSISEC` writer - LSI clock configuration and status bits security Set and reset by software.
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESEC` reader - LSE clock configuration and status bits security Set and reset by software.
pub type LSESEC_R = crate::BitReader;
///Field `LSESEC` writer - LSE clock configuration and status bits security Set and reset by software.
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCLKSEC` reader - SYSCLK selection, clock output on MCO configuration security Set and reset by software.
pub type SYSCLKSEC_R = crate::BitReader;
///Field `SYSCLKSEC` writer - SYSCLK selection, clock output on MCO configuration security Set and reset by software.
pub type SYSCLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security Set and reset by software.
pub type PRESCSEC_R = crate::BitReader;
///Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security Set and reset by software.
pub type PRESCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1SEC` reader - PLL1 clock configuration and status bits security Set and reset by software.
pub type PLL1SEC_R = crate::BitReader;
///Field `PLL1SEC` writer - PLL1 clock configuration and status bits security Set and reset by software.
pub type PLL1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMVFSEC` reader - Remove reset flag security Set and reset by software.
pub type RMVFSEC_R = crate::BitReader;
///Field `RMVFSEC` writer - Remove reset flag security Set and reset by software.
pub type RMVFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSI16 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSE32 clock configuration bits, status bits and HSECSS security Set and reset by software.
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
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
    ///Bit 5 - SYSCLK selection, clock output on MCO configuration security Set and reset by software.
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
    ///Bit 12 - Remove reset flag security Set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("hsisec", &self.hsisec())
            .field("hsesec", &self.hsesec())
            .field("lsisec", &self.lsisec())
            .field("lsesec", &self.lsesec())
            .field("sysclksec", &self.sysclksec())
            .field("prescsec", &self.prescsec())
            .field("pll1sec", &self.pll1sec())
            .field("rmvfsec", &self.rmvfsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSI16 clock configuration and status bits security Set and reset by software.
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W<'_, SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    ///Bit 1 - HSE32 clock configuration bits, status bits and HSECSS security Set and reset by software.
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W<'_, SECCFGRrs> {
        HSESEC_W::new(self, 1)
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
    ///Bit 5 - SYSCLK selection, clock output on MCO configuration security Set and reset by software.
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
    ///Bit 12 - Remove reset flag security Set and reset by software.
    #[inline(always)]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<'_, SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
}
/**RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RCC:SECCFGR)*/
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
