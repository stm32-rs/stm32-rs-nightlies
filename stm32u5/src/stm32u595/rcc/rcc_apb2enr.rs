#[doc = "Register `RCC_APB2ENR` reader"]
pub type R = crate::R<RCC_APB2ENRrs>;
#[doc = "Register `RCC_APB2ENR` writer"]
pub type W = crate::W<RCC_APB2ENRrs>;
#[doc = "Field `TIM1EN` reader - TIM1 clock enable This bit is set and cleared by software."]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 clock enable This bit is set and cleared by software."]
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable This bit is set and cleared by software."]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable This bit is set and cleared by software."]
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - TIM8 clock enable This bit is set and cleared by software."]
pub type TIM8EN_R = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8 clock enable This bit is set and cleared by software."]
pub type TIM8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1clock enable This bit is set and cleared by software."]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1clock enable This bit is set and cleared by software."]
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15EN` reader - TIM15 clock enable This bit is set and cleared by software."]
pub type TIM15EN_R = crate::BitReader;
#[doc = "Field `TIM15EN` writer - TIM15 clock enable This bit is set and cleared by software."]
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16 clock enable This bit is set and cleared by software."]
pub type TIM16EN_R = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 clock enable This bit is set and cleared by software."]
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - TIM17 clock enable This bit is set and cleared by software."]
pub type TIM17EN_R = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM17 clock enable This bit is set and cleared by software."]
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1EN` reader - SAI1 clock enable This bit is set and cleared by software."]
pub type SAI1EN_R = crate::BitReader;
#[doc = "Field `SAI1EN` writer - SAI1 clock enable This bit is set and cleared by software."]
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2EN` reader - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAI2EN_R = crate::BitReader;
#[doc = "Field `SAI2EN` writer - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEN` reader - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXTIMEN` reader - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXTIMEN_R = crate::BitReader;
#[doc = "Field `GFXTIMEN` writer - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXTIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCEN` reader - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type LTDCEN_R = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIEN` reader - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DSIEN_R = crate::BitReader;
#[doc = "Field `DSIEN` writer - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gfxtimen(&self) -> GFXTIMEN_R {
        GFXTIMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<RCC_APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<RCC_APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<RCC_APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<RCC_APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<RCC_APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<RCC_APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<RCC_APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    #[doc = "Bit 21 - SAI1 clock enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<RCC_APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SAI2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<RCC_APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
    #[doc = "Bit 24 - USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<RCC_APB2ENRrs> {
        USBEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gfxtimen(&mut self) -> GFXTIMEN_W<RCC_APB2ENRrs> {
        GFXTIMEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<RCC_APB2ENRrs> {
        LTDCEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<RCC_APB2ENRrs> {
        DSIEN_W::new(self, 27)
    }
}
#[doc = "RCC APB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB2ENRrs;
impl crate::RegisterSpec for RCC_APB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2enr::R`](R) reader structure"]
impl crate::Readable for RCC_APB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2enr::W`](W) writer structure"]
impl crate::Writable for RCC_APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB2ENR to value 0"]
impl crate::Resettable for RCC_APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
