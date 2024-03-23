#[doc = "Register `TZSC_PRIVCFGR2` reader"]
pub type R = crate::R<TZSC_PRIVCFGR2rs>;
#[doc = "Register `TZSC_PRIVCFGR2` writer"]
pub type W = crate::W<TZSC_PRIVCFGR2rs>;
#[doc = "Field `TIM1PRIV` reader - privileged access mode for TIM1"]
pub type TIM1PRIV_R = crate::BitReader;
#[doc = "Field `TIM1PRIV` writer - privileged access mode for TIM1"]
pub type TIM1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1PRIV` reader - privileged access mode for SPI1PRIV"]
pub type SPI1PRIV_R = crate::BitReader;
#[doc = "Field `SPI1PRIV` writer - privileged access mode for SPI1PRIV"]
pub type SPI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8PRIV` reader - privileged access mode for TIM8"]
pub type TIM8PRIV_R = crate::BitReader;
#[doc = "Field `TIM8PRIV` writer - privileged access mode for TIM8"]
pub type TIM8PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1PRIV` reader - privileged access mode for USART1"]
pub type USART1PRIV_R = crate::BitReader;
#[doc = "Field `USART1PRIV` writer - privileged access mode for USART1"]
pub type USART1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15PRIV` reader - privileged access mode for TIM15"]
pub type TIM15PRIV_R = crate::BitReader;
#[doc = "Field `TIM15PRIV` writer - privileged access mode for TIM15"]
pub type TIM15PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16PRIV` reader - privileged access mode for TIM16"]
pub type TIM16PRIV_R = crate::BitReader;
#[doc = "Field `TIM16PRIV` writer - privileged access mode for TIM16"]
pub type TIM16PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17PRIV` reader - privileged access mode for TIM17"]
pub type TIM17PRIV_R = crate::BitReader;
#[doc = "Field `TIM17PRIV` writer - privileged access mode for TIM17"]
pub type TIM17PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1PRIV` reader - privileged access mode for SAI1"]
pub type SAI1PRIV_R = crate::BitReader;
#[doc = "Field `SAI1PRIV` writer - privileged access mode for SAI1"]
pub type SAI1PRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - privileged access mode for TIM1"]
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - privileged access mode for SPI1PRIV"]
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - privileged access mode for TIM8"]
    #[inline(always)]
    pub fn tim8priv(&self) -> TIM8PRIV_R {
        TIM8PRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - privileged access mode for USART1"]
    #[inline(always)]
    pub fn usart1priv(&self) -> USART1PRIV_R {
        USART1PRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - privileged access mode for TIM15"]
    #[inline(always)]
    pub fn tim15priv(&self) -> TIM15PRIV_R {
        TIM15PRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - privileged access mode for TIM16"]
    #[inline(always)]
    pub fn tim16priv(&self) -> TIM16PRIV_R {
        TIM16PRIV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - privileged access mode for TIM17"]
    #[inline(always)]
    pub fn tim17priv(&self) -> TIM17PRIV_R {
        TIM17PRIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - privileged access mode for SAI1"]
    #[inline(always)]
    pub fn sai1priv(&self) -> SAI1PRIV_R {
        SAI1PRIV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - privileged access mode for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W<TZSC_PRIVCFGR2rs> {
        TIM1PRIV_W::new(self, 0)
    }
    #[doc = "Bit 1 - privileged access mode for SPI1PRIV"]
    #[inline(always)]
    #[must_use]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W<TZSC_PRIVCFGR2rs> {
        SPI1PRIV_W::new(self, 1)
    }
    #[doc = "Bit 2 - privileged access mode for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8priv(&mut self) -> TIM8PRIV_W<TZSC_PRIVCFGR2rs> {
        TIM8PRIV_W::new(self, 2)
    }
    #[doc = "Bit 3 - privileged access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1priv(&mut self) -> USART1PRIV_W<TZSC_PRIVCFGR2rs> {
        USART1PRIV_W::new(self, 3)
    }
    #[doc = "Bit 4 - privileged access mode for TIM15"]
    #[inline(always)]
    #[must_use]
    pub fn tim15priv(&mut self) -> TIM15PRIV_W<TZSC_PRIVCFGR2rs> {
        TIM15PRIV_W::new(self, 4)
    }
    #[doc = "Bit 5 - privileged access mode for TIM16"]
    #[inline(always)]
    #[must_use]
    pub fn tim16priv(&mut self) -> TIM16PRIV_W<TZSC_PRIVCFGR2rs> {
        TIM16PRIV_W::new(self, 5)
    }
    #[doc = "Bit 6 - privileged access mode for TIM17"]
    #[inline(always)]
    #[must_use]
    pub fn tim17priv(&mut self) -> TIM17PRIV_W<TZSC_PRIVCFGR2rs> {
        TIM17PRIV_W::new(self, 6)
    }
    #[doc = "Bit 7 - privileged access mode for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1priv(&mut self) -> SAI1PRIV_W<TZSC_PRIVCFGR2rs> {
        SAI1PRIV_W::new(self, 7)
    }
}
#[doc = "TZSC privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_privcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_privcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_PRIVCFGR2rs;
impl crate::RegisterSpec for TZSC_PRIVCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_privcfgr2::R`](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_privcfgr2::W`](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR2 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
