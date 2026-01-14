///Register `APB1LPENR` reader
pub type R = crate::R<APB1LPENRrs>;
///Register `APB1LPENR` writer
pub type W = crate::W<APB1LPENRrs>;
/**TIM2 clock enable during Sleep mode

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN {
    ///0: Selected module is disabled during Sleep mode
    DisabledInSleep = 0,
    ///1: Selected module is enabled during Sleep mode
    EnabledInSleep = 1,
}
impl From<TIM2LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2LPEN` reader - TIM2 clock enable during Sleep mode
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN>;
impl TIM2LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2LPEN {
        match self.bits {
            false => TIM2LPEN::DisabledInSleep,
            true => TIM2LPEN::EnabledInSleep,
        }
    }
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM2LPEN::DisabledInSleep
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM2LPEN::EnabledInSleep
    }
}
///Field `TIM2LPEN` writer - TIM2 clock enable during Sleep mode
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2LPEN>;
impl<'a, REG> TIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected module is disabled during Sleep mode
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::DisabledInSleep)
    }
    ///Selected module is enabled during Sleep mode
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::EnabledInSleep)
    }
}
///Field `TIM3LPEN` reader - TIM3 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM3LPEN_R;
///Field `TIM4LPEN` reader - TIM4 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM4LPEN_R;
///Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM5LPEN_R;
///Field `TIM6LPEN` reader - TIM6 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM6LPEN_R;
///Field `TIM7LPEN` reader - TIM7 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM7LPEN_R;
///Field `TIM12LPEN` reader - TIM12 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM12LPEN_R;
///Field `TIM13LPEN` reader - TIM13 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM13LPEN_R;
///Field `TIM14LPEN` reader - TIM14 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM14LPEN_R;
///Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode
pub use TIM2LPEN_R as WWDGLPEN_R;
///Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode
pub use TIM2LPEN_R as SPI2LPEN_R;
///Field `SPI3LPEN` reader - SPI3 clock enable during Sleep mode
pub use TIM2LPEN_R as SPI3LPEN_R;
///Field `USART2LPEN` reader - USART2 clock enable during Sleep mode
pub use TIM2LPEN_R as USART2LPEN_R;
///Field `USART3LPEN` reader - USART3 clock enable during Sleep mode
pub use TIM2LPEN_R as USART3LPEN_R;
///Field `UART4LPEN` reader - UART4 clock enable during Sleep mode
pub use TIM2LPEN_R as UART4LPEN_R;
///Field `UART5LPEN` reader - UART5 clock enable during Sleep mode
pub use TIM2LPEN_R as UART5LPEN_R;
///Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode
pub use TIM2LPEN_R as I2C1LPEN_R;
///Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode
pub use TIM2LPEN_R as I2C2LPEN_R;
///Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode
pub use TIM2LPEN_R as I2C3LPEN_R;
///Field `CAN1LPEN` reader - CAN 1 clock enable during Sleep mode
pub use TIM2LPEN_R as CAN1LPEN_R;
///Field `CAN2LPEN` reader - CAN 2 clock enable during Sleep mode
pub use TIM2LPEN_R as CAN2LPEN_R;
///Field `PWRLPEN` reader - Power interface clock enable during Sleep mode
pub use TIM2LPEN_R as PWRLPEN_R;
///Field `DACLPEN` reader - DAC interface clock enable during Sleep mode
pub use TIM2LPEN_R as DACLPEN_R;
///Field `TIM3LPEN` writer - TIM3 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM3LPEN_W;
///Field `TIM4LPEN` writer - TIM4 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM4LPEN_W;
///Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM5LPEN_W;
///Field `TIM6LPEN` writer - TIM6 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM6LPEN_W;
///Field `TIM7LPEN` writer - TIM7 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM7LPEN_W;
///Field `TIM12LPEN` writer - TIM12 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM12LPEN_W;
///Field `TIM13LPEN` writer - TIM13 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM13LPEN_W;
///Field `TIM14LPEN` writer - TIM14 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM14LPEN_W;
///Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode
pub use TIM2LPEN_W as WWDGLPEN_W;
///Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode
pub use TIM2LPEN_W as SPI2LPEN_W;
///Field `SPI3LPEN` writer - SPI3 clock enable during Sleep mode
pub use TIM2LPEN_W as SPI3LPEN_W;
///Field `USART2LPEN` writer - USART2 clock enable during Sleep mode
pub use TIM2LPEN_W as USART2LPEN_W;
///Field `USART3LPEN` writer - USART3 clock enable during Sleep mode
pub use TIM2LPEN_W as USART3LPEN_W;
///Field `UART4LPEN` writer - UART4 clock enable during Sleep mode
pub use TIM2LPEN_W as UART4LPEN_W;
///Field `UART5LPEN` writer - UART5 clock enable during Sleep mode
pub use TIM2LPEN_W as UART5LPEN_W;
///Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode
pub use TIM2LPEN_W as I2C1LPEN_W;
///Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode
pub use TIM2LPEN_W as I2C2LPEN_W;
///Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode
pub use TIM2LPEN_W as I2C3LPEN_W;
///Field `CAN1LPEN` writer - CAN 1 clock enable during Sleep mode
pub use TIM2LPEN_W as CAN1LPEN_W;
///Field `CAN2LPEN` writer - CAN 2 clock enable during Sleep mode
pub use TIM2LPEN_W as CAN2LPEN_W;
///Field `PWRLPEN` writer - Power interface clock enable during Sleep mode
pub use TIM2LPEN_W as PWRLPEN_W;
///Field `DACLPEN` writer - DAC interface clock enable during Sleep mode
pub use TIM2LPEN_W as DACLPEN_W;
impl R {
    ///Bit 0 - TIM2 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CAN 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn can1lpen(&self) -> CAN1LPEN_R {
        CAN1LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CAN 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn can2lpen(&self) -> CAN2LPEN_R {
        CAN2LPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable during Sleep mode
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LPENR")
            .field("tim2lpen", &self.tim2lpen())
            .field("daclpen", &self.daclpen())
            .field("pwrlpen", &self.pwrlpen())
            .field("can2lpen", &self.can2lpen())
            .field("can1lpen", &self.can1lpen())
            .field("i2c3lpen", &self.i2c3lpen())
            .field("i2c2lpen", &self.i2c2lpen())
            .field("i2c1lpen", &self.i2c1lpen())
            .field("uart5lpen", &self.uart5lpen())
            .field("uart4lpen", &self.uart4lpen())
            .field("usart3lpen", &self.usart3lpen())
            .field("usart2lpen", &self.usart2lpen())
            .field("spi3lpen", &self.spi3lpen())
            .field("spi2lpen", &self.spi2lpen())
            .field("wwdglpen", &self.wwdglpen())
            .field("tim14lpen", &self.tim14lpen())
            .field("tim13lpen", &self.tim13lpen())
            .field("tim12lpen", &self.tim12lpen())
            .field("tim7lpen", &self.tim7lpen())
            .field("tim6lpen", &self.tim6lpen())
            .field("tim5lpen", &self.tim5lpen())
            .field("tim4lpen", &self.tim4lpen())
            .field("tim3lpen", &self.tim3lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<'_, APB1LPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<'_, APB1LPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<'_, APB1LPENRrs> {
        TIM4LPEN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<'_, APB1LPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<'_, APB1LPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<'_, APB1LPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    ///Bit 6 - TIM12 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<'_, APB1LPENRrs> {
        TIM12LPEN_W::new(self, 6)
    }
    ///Bit 7 - TIM13 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<'_, APB1LPENRrs> {
        TIM13LPEN_W::new(self, 7)
    }
    ///Bit 8 - TIM14 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<'_, APB1LPENRrs> {
        TIM14LPEN_W::new(self, 8)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<'_, APB1LPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<'_, APB1LPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<'_, APB1LPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<'_, APB1LPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<'_, APB1LPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<'_, APB1LPENRrs> {
        UART4LPEN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<'_, APB1LPENRrs> {
        UART5LPEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<'_, APB1LPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<'_, APB1LPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<'_, APB1LPENRrs> {
        I2C3LPEN_W::new(self, 23)
    }
    ///Bit 25 - CAN 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn can1lpen(&mut self) -> CAN1LPEN_W<'_, APB1LPENRrs> {
        CAN1LPEN_W::new(self, 25)
    }
    ///Bit 26 - CAN 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn can2lpen(&mut self) -> CAN2LPEN_W<'_, APB1LPENRrs> {
        CAN2LPEN_W::new(self, 26)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<'_, APB1LPENRrs> {
        PWRLPEN_W::new(self, 28)
    }
    ///Bit 29 - DAC interface clock enable during Sleep mode
    #[inline(always)]
    pub fn daclpen(&mut self) -> DACLPEN_W<'_, APB1LPENRrs> {
        DACLPEN_W::new(self, 29)
    }
}
/**APB1 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#RCC:APB1LPENR)*/
pub struct APB1LPENRrs;
impl crate::RegisterSpec for APB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1lpenr::R`](R) reader structure
impl crate::Readable for APB1LPENRrs {}
///`write(|w| ..)` method takes [`apb1lpenr::W`](W) writer structure
impl crate::Writable for APB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LPENR to value 0x36fe_c9ff
impl crate::Resettable for APB1LPENRrs {
    const RESET_VALUE: u32 = 0x36fe_c9ff;
}
