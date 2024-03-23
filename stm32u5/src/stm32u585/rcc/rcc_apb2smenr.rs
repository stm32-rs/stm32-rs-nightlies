#[doc = "Register `RCC_APB2SMENR` reader"]
pub type R = crate::R<RCC_APB2SMENRrs>;
#[doc = "Register `RCC_APB2SMENR` writer"]
pub type W = crate::W<RCC_APB2SMENRrs>;
#[doc = "Field `TIM1SMEN` reader - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM1SMEN_R = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type SPI1SMEN_R = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8SMEN` reader - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM8SMEN_R = crate::BitReader;
#[doc = "Field `TIM8SMEN` writer - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM8SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type USART1SMEN_R = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SMEN` reader - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM15SMEN_R = crate::BitReader;
#[doc = "Field `TIM15SMEN` writer - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM15SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM16SMEN_R = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM17SMEN_R = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1SMEN` reader - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type SAI1SMEN_R = crate::BitReader;
#[doc = "Field `SAI1SMEN` writer - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
pub type SAI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2SMEN` reader - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAI2SMEN_R = crate::BitReader;
#[doc = "Field `SAI2SMEN` writer - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type SAI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSMEN` reader - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type USBSMEN_R = crate::BitReader;
#[doc = "Field `USBSMEN` writer - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type USBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFXTIMSMEN` reader - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXTIMSMEN_R = crate::BitReader;
#[doc = "Field `GFXTIMSMEN` writer - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type GFXTIMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCSMEN` reader - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type LTDCSMEN_R = crate::BitReader;
#[doc = "Field `LTDCSMEN` writer - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type LTDCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSISMEN` reader - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DSISMEN_R = crate::BitReader;
#[doc = "Field `DSISMEN` writer - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
pub type DSISMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim8smen(&self) -> TIM8SMEN_R {
        TIM8SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn sai2smen(&self) -> SAI2SMEN_R {
        SAI2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn gfxtimsmen(&self) -> GFXTIMSMEN_R {
        GFXTIMSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn ltdcsmen(&self) -> LTDCSMEN_R {
        LTDCSMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    pub fn dsismen(&self) -> DSISMEN_R {
        DSISMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TIM1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W<RCC_APB2SMENRrs> {
        TIM1SMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W<RCC_APB2SMENRrs> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TIM8 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim8smen(&mut self) -> TIM8SMEN_W<RCC_APB2SMENRrs> {
        TIM8SMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
    #[inline(always)]
    #[must_use]
    pub fn usart1smen(&mut self) -> USART1SMEN_W<RCC_APB2SMENRrs> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 16 - TIM15 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W<RCC_APB2SMENRrs> {
        TIM15SMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<RCC_APB2SMENRrs> {
        TIM16SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W<RCC_APB2SMENRrs> {
        TIM17SMEN_W::new(self, 18)
    }
    #[doc = "Bit 21 - SAI1 clock enable during Sleep and Stop modes This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W<RCC_APB2SMENRrs> {
        SAI1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SAI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn sai2smen(&mut self) -> SAI2SMEN_W<RCC_APB2SMENRrs> {
        SAI2SMEN_W::new(self, 22)
    }
    #[doc = "Bit 24 - USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> USBSMEN_W<RCC_APB2SMENRrs> {
        USBSMEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn gfxtimsmen(&mut self) -> GFXTIMSMEN_W<RCC_APB2SMENRrs> {
        GFXTIMSMEN_W::new(self, 25)
    }
    #[doc = "Bit 26 - LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn ltdcsmen(&mut self) -> LTDCSMEN_W<RCC_APB2SMENRrs> {
        LTDCSMEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
    #[inline(always)]
    #[must_use]
    pub fn dsismen(&mut self) -> DSISMEN_W<RCC_APB2SMENRrs> {
        DSISMEN_W::new(self, 27)
    }
}
#[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_apb2smenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_apb2smenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_APB2SMENRrs;
impl crate::RegisterSpec for RCC_APB2SMENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_apb2smenr::R`](R) reader structure"]
impl crate::Readable for RCC_APB2SMENRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_apb2smenr::W`](W) writer structure"]
impl crate::Writable for RCC_APB2SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_APB2SMENR to value 0xffff_ffff"]
impl crate::Resettable for RCC_APB2SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
