///Register `RCC_APB1ENR1` reader
pub type R = crate::R<RCC_APB1ENR1rs>;
///Register `RCC_APB1ENR1` writer
pub type W = crate::W<RCC_APB1ENR1rs>;
///Field `TIM2EN` reader - TIM2 clock enable This bit is set and cleared by software.
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2 clock enable This bit is set and cleared by software.
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - TIM3 clock enable This bit is set and cleared by software.
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 clock enable This bit is set and cleared by software.
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4EN` reader - TIM4 clock enable This bit is set and cleared by software.
pub type TIM4EN_R = crate::BitReader;
///Field `TIM4EN` writer - TIM4 clock enable This bit is set and cleared by software.
pub type TIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5EN` reader - TIM5 clock enable This bit is set and cleared by software.
pub type TIM5EN_R = crate::BitReader;
///Field `TIM5EN` writer - TIM5 clock enable This bit is set and cleared by software.
pub type TIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6EN` reader - TIM6 clock enable This bit is set and cleared by software.
pub type TIM6EN_R = crate::BitReader;
///Field `TIM6EN` writer - TIM6 clock enable This bit is set and cleared by software.
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7EN` reader - TIM7 clock enable This bit is set and cleared by software.
pub type TIM7EN_R = crate::BitReader;
///Field `TIM7EN` writer - TIM7 clock enable This bit is set and cleared by software.
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - WWDG clock enable This bit is set by software to enable the window watchdog clock. It is reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - WWDG clock enable This bit is set by software to enable the window watchdog clock. It is reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - SPI2 clock enable This bit is set and cleared by software.
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - SPI2 clock enable This bit is set and cleared by software.
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3EN` reader - USART3 clock enable This bit is set and cleared by software.
pub type USART3EN_R = crate::BitReader;
///Field `USART3EN` writer - USART3 clock enable This bit is set and cleared by software.
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4EN` reader - UART4 clock enable This bit is set and cleared by software.
pub type UART4EN_R = crate::BitReader;
///Field `UART4EN` writer - UART4 clock enable This bit is set and cleared by software.
pub type UART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5EN` reader - UART5 clock enable This bit is set and cleared by software.
pub type UART5EN_R = crate::BitReader;
///Field `UART5EN` writer - UART5 clock enable This bit is set and cleared by software.
pub type UART5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 clock enable This bit is set and cleared by software.
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 clock enable This bit is set and cleared by software.
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2EN` reader - I2C2 clock enable This bit is set and cleared by software.
pub type I2C2EN_R = crate::BitReader;
///Field `I2C2EN` writer - I2C2 clock enable This bit is set and cleared by software.
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSEN` reader - CRS clock enable This bit is set and cleared by software.
pub type CRSEN_R = crate::BitReader;
///Field `CRSEN` writer - CRS clock enable This bit is set and cleared by software.
pub type CRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6EN` reader - USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART6EN_R = crate::BitReader;
///Field `USART6EN` writer - USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable This bit is set by software to enable the window watchdog clock. It is reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1ENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim5en", &self.tim5en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("uart4en", &self.uart4en())
            .field("uart5en", &self.uart5en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("crsen", &self.crsen())
            .field("usart6en", &self.usart6en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<RCC_APB1ENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<RCC_APB1ENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<RCC_APB1ENR1rs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<RCC_APB1ENR1rs> {
        TIM5EN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<RCC_APB1ENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<RCC_APB1ENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 11 - WWDG clock enable This bit is set by software to enable the window watchdog clock. It is reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
    #[inline(always)]
    #[must_use]
    pub fn wwdgen(&mut self) -> WWDGEN_W<RCC_APB1ENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<RCC_APB1ENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<RCC_APB1ENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<RCC_APB1ENR1rs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<RCC_APB1ENR1rs> {
        UART4EN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<RCC_APB1ENR1rs> {
        UART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<RCC_APB1ENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<RCC_APB1ENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 24 - CRS clock enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crsen(&mut self) -> CRSEN_W<RCC_APB1ENR1rs> {
        CRSEN_W::new(self, 24)
    }
    ///Bit 25 - USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<RCC_APB1ENR1rs> {
        USART6EN_W::new(self, 25)
    }
}
/**RCC APB1 peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1enr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1enr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#RCC:RCC_APB1ENR1)*/
pub struct RCC_APB1ENR1rs;
impl crate::RegisterSpec for RCC_APB1ENR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb1enr1::R`](R) reader structure
impl crate::Readable for RCC_APB1ENR1rs {}
///`write(|w| ..)` method takes [`rcc_apb1enr1::W`](W) writer structure
impl crate::Writable for RCC_APB1ENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB1ENR1 to value 0
impl crate::Resettable for RCC_APB1ENR1rs {
    const RESET_VALUE: u32 = 0;
}
