#[doc = "Register `TZSC_SECCFGR2` reader"]
pub type R = crate::R<TZSC_SECCFGR2rs>;
#[doc = "Register `TZSC_SECCFGR2` writer"]
pub type W = crate::W<TZSC_SECCFGR2rs>;
#[doc = "Field `TIM1SEC` reader - secure access mode for TIM1"]
pub type TIM1SEC_R = crate::BitReader;
#[doc = "Field `TIM1SEC` writer - secure access mode for TIM1"]
pub type TIM1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SEC` reader - secure access mode for SPI1"]
pub type SPI1SEC_R = crate::BitReader;
#[doc = "Field `SPI1SEC` writer - secure access mode for SPI1"]
pub type SPI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8SEC` reader - secure access mode for TIM8"]
pub type TIM8SEC_R = crate::BitReader;
#[doc = "Field `TIM8SEC` writer - secure access mode for TIM8"]
pub type TIM8SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SEC` reader - secure access mode for USART1"]
pub type USART1SEC_R = crate::BitReader;
#[doc = "Field `USART1SEC` writer - secure access mode for USART1"]
pub type USART1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SEC` reader - secure access mode for TIM5"]
pub type TIM15SEC_R = crate::BitReader;
#[doc = "Field `TIM15SEC` writer - secure access mode for TIM5"]
pub type TIM15SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SEC` reader - secure access mode for TIM6"]
pub type TIM16SEC_R = crate::BitReader;
#[doc = "Field `TIM16SEC` writer - secure access mode for TIM6"]
pub type TIM16SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SEC` reader - secure access mode for TIM7"]
pub type TIM17SEC_R = crate::BitReader;
#[doc = "Field `TIM17SEC` writer - secure access mode for TIM7"]
pub type TIM17SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1SEC` reader - secure access mode for SAI1"]
pub type SAI1SEC_R = crate::BitReader;
#[doc = "Field `SAI1SEC` writer - secure access mode for SAI1"]
pub type SAI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2SEC` reader - secure access mode for SAI2"]
pub type SAI2SEC_R = crate::BitReader;
#[doc = "Field `SAI2SEC` writer - secure access mode for SAI2"]
pub type SAI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - secure access mode for TIM1"]
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - secure access mode for SPI1"]
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - secure access mode for TIM8"]
    #[inline(always)]
    pub fn tim8sec(&self) -> TIM8SEC_R {
        TIM8SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - secure access mode for USART1"]
    #[inline(always)]
    pub fn usart1sec(&self) -> USART1SEC_R {
        USART1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - secure access mode for TIM5"]
    #[inline(always)]
    pub fn tim15sec(&self) -> TIM15SEC_R {
        TIM15SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - secure access mode for TIM6"]
    #[inline(always)]
    pub fn tim16sec(&self) -> TIM16SEC_R {
        TIM16SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - secure access mode for TIM7"]
    #[inline(always)]
    pub fn tim17sec(&self) -> TIM17SEC_R {
        TIM17SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - secure access mode for SAI1"]
    #[inline(always)]
    pub fn sai1sec(&self) -> SAI1SEC_R {
        SAI1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - secure access mode for SAI2"]
    #[inline(always)]
    pub fn sai2sec(&self) -> SAI2SEC_R {
        SAI2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - secure access mode for TIM1"]
    #[inline(always)]
    #[must_use]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<TZSC_SECCFGR2rs> {
        TIM1SEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - secure access mode for SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<TZSC_SECCFGR2rs> {
        SPI1SEC_W::new(self, 1)
    }
    #[doc = "Bit 2 - secure access mode for TIM8"]
    #[inline(always)]
    #[must_use]
    pub fn tim8sec(&mut self) -> TIM8SEC_W<TZSC_SECCFGR2rs> {
        TIM8SEC_W::new(self, 2)
    }
    #[doc = "Bit 3 - secure access mode for USART1"]
    #[inline(always)]
    #[must_use]
    pub fn usart1sec(&mut self) -> USART1SEC_W<TZSC_SECCFGR2rs> {
        USART1SEC_W::new(self, 3)
    }
    #[doc = "Bit 4 - secure access mode for TIM5"]
    #[inline(always)]
    #[must_use]
    pub fn tim15sec(&mut self) -> TIM15SEC_W<TZSC_SECCFGR2rs> {
        TIM15SEC_W::new(self, 4)
    }
    #[doc = "Bit 5 - secure access mode for TIM6"]
    #[inline(always)]
    #[must_use]
    pub fn tim16sec(&mut self) -> TIM16SEC_W<TZSC_SECCFGR2rs> {
        TIM16SEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - secure access mode for TIM7"]
    #[inline(always)]
    #[must_use]
    pub fn tim17sec(&mut self) -> TIM17SEC_W<TZSC_SECCFGR2rs> {
        TIM17SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - secure access mode for SAI1"]
    #[inline(always)]
    #[must_use]
    pub fn sai1sec(&mut self) -> SAI1SEC_W<TZSC_SECCFGR2rs> {
        SAI1SEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - secure access mode for SAI2"]
    #[inline(always)]
    #[must_use]
    pub fn sai2sec(&mut self) -> SAI2SEC_W<TZSC_SECCFGR2rs> {
        SAI2SEC_W::new(self, 8)
    }
}
#[doc = "TZSC secure configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzsc_seccfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzsc_seccfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZSC_SECCFGR2rs;
impl crate::RegisterSpec for TZSC_SECCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsc_seccfgr2::R`](R) reader structure"]
impl crate::Readable for TZSC_SECCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`tzsc_seccfgr2::W`](W) writer structure"]
impl crate::Writable for TZSC_SECCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSC_SECCFGR2 to value 0"]
impl crate::Resettable for TZSC_SECCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
