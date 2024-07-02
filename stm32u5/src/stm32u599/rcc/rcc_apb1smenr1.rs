///Register `RCC_APB1SMENR1` reader
pub type R = crate::R<RCC_APB1SMENR1rs>;
///Register `RCC_APB1SMENR1` writer
pub type W = crate::W<RCC_APB1SMENR1rs>;
///Field `TIM2SMEN` reader - TIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM2SMEN_R = crate::BitReader;
///Field `TIM2SMEN` writer - TIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3SMEN` reader - TIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM3SMEN_R = crate::BitReader;
///Field `TIM3SMEN` writer - TIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4SMEN` reader - TIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM4SMEN_R = crate::BitReader;
///Field `TIM4SMEN` writer - TIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5SMEN` reader - TIM5 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM5SMEN_R = crate::BitReader;
///Field `TIM5SMEN` writer - TIM5 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6SMEN` reader - TIM6 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM6SMEN_R = crate::BitReader;
///Field `TIM6SMEN` writer - TIM6 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7SMEN` reader - TIM7 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM7SMEN_R = crate::BitReader;
///Field `TIM7SMEN` writer - TIM7 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGSMEN` reader - Window watchdog clock enable during Sleep and Stop modes This bit is set and cleared by software. It is forced to one by hardware when the hardware WWDG option is activated.
pub type WWDGSMEN_R = crate::BitReader;
///Field `WWDGSMEN` writer - Window watchdog clock enable during Sleep and Stop modes This bit is set and cleared by software. It is forced to one by hardware when the hardware WWDG option is activated.
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI2SMEN_R = crate::BitReader;
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2SMEN` reader - USART2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART2SMEN_R = crate::BitReader;
///Field `USART2SMEN` writer - USART2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3SMEN` reader - USART3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART3SMEN_R = crate::BitReader;
///Field `USART3SMEN` writer - USART3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type USART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4SMEN` reader - UART4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART4SMEN_R = crate::BitReader;
///Field `UART4SMEN` writer - UART4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5SMEN` reader - UART5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART5SMEN_R = crate::BitReader;
///Field `UART5SMEN` writer - UART5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type UART5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C1SMEN_R = crate::BitReader;
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C2SMEN_R = crate::BitReader;
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRSSMEN` reader - CRS clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type CRSSMEN_R = crate::BitReader;
///Field `CRSSMEN` writer - CRS clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type CRSSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6SMEN` reader - USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART6SMEN_R = crate::BitReader;
///Field `USART6SMEN` writer - USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type USART6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim5smen(&self) -> TIM5SMEN_R {
        TIM5SMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep and Stop modes This bit is set and cleared by software. It is forced to one by hardware when the hardware WWDG option is activated.
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn uart4smen(&self) -> UART4SMEN_R {
        UART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn uart5smen(&self) -> UART5SMEN_R {
        UART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn usart6smen(&self) -> USART6SMEN_R {
        USART6SMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1SMENR1")
            .field("tim2smen", &self.tim2smen())
            .field("tim3smen", &self.tim3smen())
            .field("tim4smen", &self.tim4smen())
            .field("tim5smen", &self.tim5smen())
            .field("tim6smen", &self.tim6smen())
            .field("tim7smen", &self.tim7smen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("spi2smen", &self.spi2smen())
            .field("usart2smen", &self.usart2smen())
            .field("usart3smen", &self.usart3smen())
            .field("uart4smen", &self.uart4smen())
            .field("uart5smen", &self.uart5smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("i2c2smen", &self.i2c2smen())
            .field("crssmen", &self.crssmen())
            .field("usart6smen", &self.usart6smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<RCC_APB1SMENR1rs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<RCC_APB1SMENR1rs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<RCC_APB1SMENR1rs> {
        TIM4SMEN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim5smen(&mut self) -> TIM5SMEN_W<RCC_APB1SMENR1rs> {
        TIM5SMEN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<RCC_APB1SMENR1rs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<RCC_APB1SMENR1rs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep and Stop modes This bit is set and cleared by software. It is forced to one by hardware when the hardware WWDG option is activated.
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<RCC_APB1SMENR1rs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<RCC_APB1SMENR1rs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<RCC_APB1SMENR1rs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<RCC_APB1SMENR1rs> {
        USART3SMEN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn uart4smen(&mut self) -> UART4SMEN_W<RCC_APB1SMENR1rs> {
        UART4SMEN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn uart5smen(&mut self) -> UART5SMEN_W<RCC_APB1SMENR1rs> {
        UART5SMEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<RCC_APB1SMENR1rs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<RCC_APB1SMENR1rs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 24 - CRS clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn crssmen(&mut self) -> CRSSMEN_W<RCC_APB1SMENR1rs> {
        CRSSMEN_W::new(self, 24)
    }
    ///Bit 25 - USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn usart6smen(&mut self) -> USART6SMEN_W<RCC_APB1SMENR1rs> {
        USART6SMEN_W::new(self, 25)
    }
}
/**RCC APB1 peripheral clock enable in Sleep and Stop modes register 1

You can [`read`](crate::Reg::read) this register and get [`rcc_apb1smenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_apb1smenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#RCC:RCC_APB1SMENR1)*/
pub struct RCC_APB1SMENR1rs;
impl crate::RegisterSpec for RCC_APB1SMENR1rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_apb1smenr1::R`](R) reader structure
impl crate::Readable for RCC_APB1SMENR1rs {}
///`write(|w| ..)` method takes [`rcc_apb1smenr1::W`](W) writer structure
impl crate::Writable for RCC_APB1SMENR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_APB1SMENR1 to value 0xffff_ffff
impl crate::Resettable for RCC_APB1SMENR1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
