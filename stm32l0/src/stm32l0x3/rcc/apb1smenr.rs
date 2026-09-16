///Register `APB1SMENR` reader
pub type R = crate::R<APB1SMENRrs>;
///Register `APB1SMENR` writer
pub type W = crate::W<APB1SMENRrs>;
/**Timer2 clock enable during sleep mode bit

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2SMEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2SMEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2SMEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2SMEN` reader - Timer2 clock enable during sleep mode bit
pub type TIM2SMEN_R = crate::BitReader<TIM2SMEN>;
impl TIM2SMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2SMEN {
        match self.bits {
            false => TIM2SMEN::Disabled,
            true => TIM2SMEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2SMEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2SMEN::Enabled
    }
}
///Field `TIM2SMEN` writer - Timer2 clock enable during sleep mode bit
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2SMEN>;
impl<'a, REG> TIM2SMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2SMEN::Enabled)
    }
}
///Field `TIM3SMEN` reader - Timer3 clock enable during Sleep mode bit
pub use TIM2SMEN_R as TIM3SMEN_R;
///Field `TIM6SMEN` reader - Timer 6 clock enable during sleep mode bit
pub use TIM2SMEN_R as TIM6SMEN_R;
///Field `TIM7SMEN` reader - Timer 7 clock enable during Sleep mode bit
pub use TIM2SMEN_R as TIM7SMEN_R;
///Field `WWDGSMEN` reader - Window watchdog clock enable during sleep mode bit
pub use TIM2SMEN_R as WWDGSMEN_R;
///Field `SPI2SMEN` reader - SPI2 clock enable during sleep mode bit
pub use TIM2SMEN_R as SPI2SMEN_R;
///Field `USART2SMEN` reader - UART2 clock enable during sleep mode bit
pub use TIM2SMEN_R as USART2SMEN_R;
///Field `LPUART1SMEN` reader - LPUART1 clock enable during sleep mode bit
pub use TIM2SMEN_R as LPUART1SMEN_R;
///Field `USART4SMEN` reader - USART4 clock enable during Sleep mode bit
pub use TIM2SMEN_R as USART4SMEN_R;
///Field `USART5SMEN` reader - USART5 clock enable during Sleep mode bit
pub use TIM2SMEN_R as USART5SMEN_R;
///Field `I2C1SMEN` reader - I2C1 clock enable during sleep mode bit
pub use TIM2SMEN_R as I2C1SMEN_R;
///Field `I2C2SMEN` reader - I2C2 clock enable during sleep mode bit
pub use TIM2SMEN_R as I2C2SMEN_R;
///Field `USBSMEN` reader - USB clock enable during sleep mode bit
pub use TIM2SMEN_R as USBSMEN_R;
///Field `CRSSMEN` reader - Clock recovery system clock enable during sleep mode bit
pub use TIM2SMEN_R as CRSSMEN_R;
///Field `PWRSMEN` reader - Power interface clock enable during sleep mode bit
pub use TIM2SMEN_R as PWRSMEN_R;
///Field `DACSMEN` reader - DAC interface clock enable during sleep mode bit
pub use TIM2SMEN_R as DACSMEN_R;
///Field `I2C3SMEN` reader - 2C3 clock enable during Sleep mode bit
pub use TIM2SMEN_R as I2C3SMEN_R;
///Field `LPTIM1SMEN` reader - Low power timer clock enable during sleep mode bit
pub use TIM2SMEN_R as LPTIM1SMEN_R;
///Field `TIM3SMEN` writer - Timer3 clock enable during Sleep mode bit
pub use TIM2SMEN_W as TIM3SMEN_W;
///Field `TIM6SMEN` writer - Timer 6 clock enable during sleep mode bit
pub use TIM2SMEN_W as TIM6SMEN_W;
///Field `TIM7SMEN` writer - Timer 7 clock enable during Sleep mode bit
pub use TIM2SMEN_W as TIM7SMEN_W;
///Field `WWDGSMEN` writer - Window watchdog clock enable during sleep mode bit
pub use TIM2SMEN_W as WWDGSMEN_W;
///Field `SPI2SMEN` writer - SPI2 clock enable during sleep mode bit
pub use TIM2SMEN_W as SPI2SMEN_W;
///Field `USART2SMEN` writer - UART2 clock enable during sleep mode bit
pub use TIM2SMEN_W as USART2SMEN_W;
///Field `LPUART1SMEN` writer - LPUART1 clock enable during sleep mode bit
pub use TIM2SMEN_W as LPUART1SMEN_W;
///Field `USART4SMEN` writer - USART4 clock enable during Sleep mode bit
pub use TIM2SMEN_W as USART4SMEN_W;
///Field `USART5SMEN` writer - USART5 clock enable during Sleep mode bit
pub use TIM2SMEN_W as USART5SMEN_W;
///Field `I2C1SMEN` writer - I2C1 clock enable during sleep mode bit
pub use TIM2SMEN_W as I2C1SMEN_W;
///Field `I2C2SMEN` writer - I2C2 clock enable during sleep mode bit
pub use TIM2SMEN_W as I2C2SMEN_W;
///Field `USBSMEN` writer - USB clock enable during sleep mode bit
pub use TIM2SMEN_W as USBSMEN_W;
///Field `CRSSMEN` writer - Clock recovery system clock enable during sleep mode bit
pub use TIM2SMEN_W as CRSSMEN_W;
///Field `PWRSMEN` writer - Power interface clock enable during sleep mode bit
pub use TIM2SMEN_W as PWRSMEN_W;
///Field `DACSMEN` writer - DAC interface clock enable during sleep mode bit
pub use TIM2SMEN_W as DACSMEN_W;
///Field `I2C3SMEN` writer - 2C3 clock enable during Sleep mode bit
pub use TIM2SMEN_W as I2C3SMEN_W;
///Field `LPTIM1SMEN` writer - Low power timer clock enable during sleep mode bit
pub use TIM2SMEN_W as LPTIM1SMEN_W;
impl R {
    ///Bit 0 - Timer2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer3 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Timer 6 clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable during sleep mode bit
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - UART2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LPUART1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - USART5 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - USB clock enable during sleep mode bit
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - Clock recovery system clock enable during sleep mode bit
    #[inline(always)]
    pub fn crssmen(&self) -> CRSSMEN_R {
        CRSSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn dacsmen(&self) -> DACSMEN_R {
        DACSMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - 2C3 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1SMENR")
            .field("tim2smen", &self.tim2smen())
            .field("lptim1smen", &self.lptim1smen())
            .field("dacsmen", &self.dacsmen())
            .field("pwrsmen", &self.pwrsmen())
            .field("crssmen", &self.crssmen())
            .field("usbsmen", &self.usbsmen())
            .field("i2c2smen", &self.i2c2smen())
            .field("i2c1smen", &self.i2c1smen())
            .field("lpuart1smen", &self.lpuart1smen())
            .field("usart2smen", &self.usart2smen())
            .field("spi2smen", &self.spi2smen())
            .field("wwdgsmen", &self.wwdgsmen())
            .field("tim6smen", &self.tim6smen())
            .field("tim3smen", &self.tim3smen())
            .field("tim7smen", &self.tim7smen())
            .field("usart4smen", &self.usart4smen())
            .field("usart5smen", &self.usart5smen())
            .field("i2c3smen", &self.i2c3smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB1SMENRrs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - Timer3 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<'_, APB1SMENRrs> {
        TIM3SMEN_W::new(self, 1)
    }
    ///Bit 4 - Timer 6 clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<'_, APB1SMENRrs> {
        TIM6SMEN_W::new(self, 4)
    }
    ///Bit 5 - Timer 7 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<'_, APB1SMENRrs> {
        TIM7SMEN_W::new(self, 5)
    }
    ///Bit 11 - Window watchdog clock enable during sleep mode bit
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<'_, APB1SMENRrs> {
        WWDGSMEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<'_, APB1SMENRrs> {
        SPI2SMEN_W::new(self, 14)
    }
    ///Bit 17 - UART2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<'_, APB1SMENRrs> {
        USART2SMEN_W::new(self, 17)
    }
    ///Bit 18 - LPUART1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<'_, APB1SMENRrs> {
        LPUART1SMEN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<'_, APB1SMENRrs> {
        USART4SMEN_W::new(self, 19)
    }
    ///Bit 20 - USART5 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn usart5smen(&mut self) -> USART5SMEN_W<'_, APB1SMENRrs> {
        USART5SMEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<'_, APB1SMENRrs> {
        I2C1SMEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during sleep mode bit
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<'_, APB1SMENRrs> {
        I2C2SMEN_W::new(self, 22)
    }
    ///Bit 23 - USB clock enable during sleep mode bit
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W<'_, APB1SMENRrs> {
        USBSMEN_W::new(self, 23)
    }
    ///Bit 27 - Clock recovery system clock enable during sleep mode bit
    #[inline(always)]
    pub fn crssmen(&mut self) -> CRSSMEN_W<'_, APB1SMENRrs> {
        CRSSMEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<'_, APB1SMENRrs> {
        PWRSMEN_W::new(self, 28)
    }
    ///Bit 29 - DAC interface clock enable during sleep mode bit
    #[inline(always)]
    pub fn dacsmen(&mut self) -> DACSMEN_W<'_, APB1SMENRrs> {
        DACSMEN_W::new(self, 29)
    }
    ///Bit 30 - 2C3 clock enable during Sleep mode bit
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<'_, APB1SMENRrs> {
        I2C3SMEN_W::new(self, 30)
    }
    ///Bit 31 - Low power timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<'_, APB1SMENRrs> {
        LPTIM1SMEN_W::new(self, 31)
    }
}
/**APB1 peripheral clock enable in sleep mode register

You can [`read`](crate::Reg::read) this register and get [`apb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#RCC:APB1SMENR)*/
pub struct APB1SMENRrs;
impl crate::RegisterSpec for APB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1smenr::R`](R) reader structure
impl crate::Readable for APB1SMENRrs {}
///`write(|w| ..)` method takes [`apb1smenr::W`](W) writer structure
impl crate::Writable for APB1SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1SMENR to value 0xb8e6_4a11
impl crate::Resettable for APB1SMENRrs {
    const RESET_VALUE: u32 = 0xb8e6_4a11;
}
