///Register `TZSC_PRIVCFGR2` reader
pub type R = crate::R<TZSC_PRIVCFGR2rs>;
///Register `TZSC_PRIVCFGR2` writer
pub type W = crate::W<TZSC_PRIVCFGR2rs>;
///Field `FDCAN1PRIV` reader - privileged access mode for FDCAN1
pub type FDCAN1PRIV_R = crate::BitReader;
///Field `FDCAN1PRIV` writer - privileged access mode for FDCAN1
pub type FDCAN1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPAMPPRIV` reader - privileged access mode for OPAMP
pub type OPAMPPRIV_R = crate::BitReader;
///Field `OPAMPPRIV` writer - privileged access mode for OPAMP
pub type OPAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPPRIV` reader - privileged access mode for COMP
pub type COMPPRIV_R = crate::BitReader;
///Field `COMPPRIV` writer - privileged access mode for COMP
pub type COMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1PRIV` reader - privileged access mode for TIM1
pub type TIM1PRIV_R = crate::BitReader;
///Field `TIM1PRIV` writer - privileged access mode for TIM1
pub type TIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1PRIV` reader - privileged access mode for SPI1
pub type SPI1PRIV_R = crate::BitReader;
///Field `SPI1PRIV` writer - privileged access mode for SPI1
pub type SPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1PRIV` reader - privileged access mode for USART1
pub type USART1PRIV_R = crate::BitReader;
///Field `USART1PRIV` writer - privileged access mode for USART1
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBFSPRIV` reader - privileged access mode for USBSF
pub type USBFSPRIV_R = crate::BitReader;
///Field `USBFSPRIV` writer - privileged access mode for USBSF
pub type USBFSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPUART1PRIV` reader - privileged access mode for LPUART
pub type LPUART1PRIV_R = crate::BitReader;
///Field `LPUART1PRIV` writer - privileged access mode for LPUART
pub type LPUART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1
pub type LPTIM1PRIV_R = crate::BitReader;
///Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1
pub type LPTIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privileged access mode for FDCAN1
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - privileged access mode for OPAMP
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - privileged access mode for COMP
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - privileged access mode for SPI1
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 19 - privileged access mode for USBSF
    #[inline(always)]
    pub fn usbfspriv(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 25 - privileged access mode for LPUART
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZSC_PRIVCFGR2")
            .field("fdcan1priv", &self.fdcan1priv())
            .field("opamppriv", &self.opamppriv())
            .field("comppriv", &self.comppriv())
            .field("tim1priv", &self.tim1priv())
            .field("spi1priv", &self.spi1priv())
            .field("usart1priv", &self.usart1priv())
            .field("usbfspriv", &self.usbfspriv())
            .field("lpuart1priv", &self.lpuart1priv())
            .field("lptim1priv", &self.lptim1priv())
            .finish()
    }
}
impl W {
    ///Bit 0 - privileged access mode for FDCAN1
    #[inline(always)]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<'_, TZSC_PRIVCFGR2rs> {
        FDCAN1PRIV_W::new(self, 0)
    }
    ///Bit 3 - privileged access mode for OPAMP
    #[inline(always)]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<'_, TZSC_PRIVCFGR2rs> {
        OPAMPPRIV_W::new(self, 3)
    }
    ///Bit 4 - privileged access mode for COMP
    #[inline(always)]
    pub fn comppriv(&mut self) -> COMPPRIV_W<'_, TZSC_PRIVCFGR2rs> {
        COMPPRIV_W::new(self, 4)
    }
    ///Bit 8 - privileged access mode for TIM1
    #[inline(always)]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<'_, TZSC_PRIVCFGR2rs> {
        TIM1PRIV_W::new(self, 8)
    }
    ///Bit 9 - privileged access mode for SPI1
    #[inline(always)]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<'_, TZSC_PRIVCFGR2rs> {
        SPI1PRIV_W::new(self, 9)
    }
    ///Bit 11 - privileged access mode for USART1
    #[inline(always)]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<'_, TZSC_PRIVCFGR2rs> {
        USART1PRIV_W::new(self, 11)
    }
    ///Bit 19 - privileged access mode for USBSF
    #[inline(always)]
    pub fn usbfspriv(&mut self) -> USBFSPRIV_W<'_, TZSC_PRIVCFGR2rs> {
        USBFSPRIV_W::new(self, 19)
    }
    ///Bit 25 - privileged access mode for LPUART
    #[inline(always)]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<'_, TZSC_PRIVCFGR2rs> {
        LPUART1PRIV_W::new(self, 25)
    }
    ///Bit 28 - privileged access mode for LPTIM1
    #[inline(always)]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<'_, TZSC_PRIVCFGR2rs> {
        LPTIM1PRIV_W::new(self, 28)
    }
}
/**GTZC1 TZSC privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`tzsc_privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#GTZC1:TZSC_PRIVCFGR2)*/
pub struct TZSC_PRIVCFGR2rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`tzsc_privcfgr2::R`](R) reader structure
impl crate::Readable for TZSC_PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`tzsc_privcfgr2::W`](W) writer structure
impl crate::Writable for TZSC_PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZSC_PRIVCFGR2 to value 0
impl crate::Resettable for TZSC_PRIVCFGR2rs {}
