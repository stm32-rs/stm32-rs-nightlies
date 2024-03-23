#[doc = "Register `GTZC1_TZSC_PRIVCFGR2` reader"]
pub type R = crate::R<GTZC1_TZSC_PRIVCFGR2rs>;
#[doc = "Register `GTZC1_TZSC_PRIVCFGR2` writer"]
pub type W = crate::W<GTZC1_TZSC_PRIVCFGR2rs>;
#[doc = "Field `FDCAN1PRIV` reader - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_R = crate::BitReader;
#[doc = "Field `FDCAN1PRIV` writer - privileged access mode for FDCAN1"]
pub type FDCAN1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPPRIV` reader - privileged access mode for OPAMP"]
pub type OPAMPPRIV_R = crate::BitReader;
#[doc = "Field `OPAMPPRIV` writer - privileged access mode for OPAMP"]
pub type OPAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPPRIV` reader - privileged access mode for COMP"]
pub type COMPPRIV_R = crate::BitReader;
#[doc = "Field `COMPPRIV` writer - privileged access mode for COMP"]
pub type COMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1PRIV` reader - privileged access mode for TIM1"]
pub type TIM1PRIV_R = crate::BitReader;
#[doc = "Field `TIM1PRIV` writer - privileged access mode for TIM1"]
pub type TIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1PRIV` reader - privileged access mode for SPI1"]
pub type SPI1PRIV_R = crate::BitReader;
#[doc = "Field `SPI1PRIV` writer - privileged access mode for SPI1"]
pub type SPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1PRIV` reader - privileged access mode for USART1"]
pub type USART1PRIV_R = crate::BitReader;
#[doc = "Field `USART1PRIV` writer - privileged access mode for USART1"]
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSPRIV` reader - privileged access mode for USBSF"]
pub type USBFSPRIV_R = crate::BitReader;
#[doc = "Field `USBFSPRIV` writer - privileged access mode for USBSF"]
pub type USBFSPRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1PRIV` reader - privileged access mode for LPUART"]
pub type LPUART1PRIV_R = crate::BitReader;
#[doc = "Field `LPUART1PRIV` writer - privileged access mode for LPUART"]
pub type LPUART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1PRIV` reader - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_R = crate::BitReader;
#[doc = "Field `LPTIM1PRIV` writer - privileged access mode for LPTIM1"]
pub type LPTIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for OPAMP"]
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for COMP"]
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - privileged access mode for TIM1"]
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - privileged access mode for SPI1"]
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - privileged access mode for USBSF"]
    #[inline(always)]
    pub fn usbfspriv(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for FDCAN1"]
    #[inline(always)]
    #[must_use]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        FDCAN1PRIV_W::new(self, 0)
    }
    #[doc = "Bit 3 - privileged access mode for OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        OPAMPPRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged access mode for COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comppriv(&mut self) -> COMPPRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        COMPPRIV_W::new(self, 4)
    }
    #[doc = "Bit 8 - privileged access mode for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        TIM1PRIV_W::new(self, 8)
    }
    #[doc = "Bit 9 - privileged access mode for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        SPI1PRIV_W::new(self, 9)
    }
    #[doc = "Bit 11 - privileged access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        USART1PRIV_W::new(self, 11)
    }
    #[doc = "Bit 19 - privileged access mode for USBSF"]
    #[inline(always)]
    #[must_use]
    pub fn usbfspriv(&mut self) -> USBFSPRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        USBFSPRIV_W::new(self, 19)
    }
    #[doc = "Bit 25 - privileged access mode for LPUART"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        LPUART1PRIV_W::new(self, 25)
    }
    #[doc = "Bit 28 - privileged access mode for LPTIM1"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W<GTZC1_TZSC_PRIVCFGR2rs> {
        LPTIM1PRIV_W::new(self, 28)
    }
}
#[doc = "GTZC1 TZSC privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_privcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_privcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_TZSC_PRIVCFGR2rs;
impl crate::RegisterSpec for GTZC1_TZSC_PRIVCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_tzsc_privcfgr2::R`](R) reader structure"]
impl crate::Readable for GTZC1_TZSC_PRIVCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_tzsc_privcfgr2::W`](W) writer structure"]
impl crate::Writable for GTZC1_TZSC_PRIVCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_TZSC_PRIVCFGR2 to value 0"]
impl crate::Resettable for GTZC1_TZSC_PRIVCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
