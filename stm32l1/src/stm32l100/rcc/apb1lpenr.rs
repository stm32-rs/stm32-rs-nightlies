///Register `APB1LPENR` reader
pub type R = crate::R<APB1LPENRrs>;
///Register `APB1LPENR` writer
pub type W = crate::W<APB1LPENRrs>;
/**Timer 2 clock enable during Sleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2LPEN` reader - Timer 2 clock enable during Sleep mode
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN>;
impl TIM2LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2LPEN {
        match self.bits {
            false => TIM2LPEN::Disabled,
            true => TIM2LPEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2LPEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2LPEN::Enabled
    }
}
///Field `TIM2LPEN` writer - Timer 2 clock enable during Sleep mode
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2LPEN>;
impl<'a, REG> TIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Enabled)
    }
}
///Field `TIM3LPEN` reader - Timer 3 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM3LPEN_R;
///Field `TIM4LPEN` reader - Timer 4 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM4LPEN_R;
///Field `TIM5LPEN` reader - Timer 5 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM5LPEN_R;
///Field `TIM6LPEN` reader - Timer 6 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM6LPEN_R;
///Field `TIM7LPEN` reader - Timer 7 clock enable during Sleep mode
pub use TIM2LPEN_R as TIM7LPEN_R;
///Field `LCDLPEN` reader - LCD clock enable during Sleep mode
pub use TIM2LPEN_R as LCDLPEN_R;
///Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode
pub use TIM2LPEN_R as WWDGLPEN_R;
///Field `SPI2LPEN` reader - SPI 2 clock enable during Sleep mode
pub use TIM2LPEN_R as SPI2LPEN_R;
///Field `SPI3LPEN` reader - SPI 3 clock enable during Sleep mode
pub use TIM2LPEN_R as SPI3LPEN_R;
///Field `USART2LPEN` reader - USART 2 clock enable during Sleep mode
pub use TIM2LPEN_R as USART2LPEN_R;
///Field `USART3LPEN` reader - USART 3 clock enable during Sleep mode
pub use TIM2LPEN_R as USART3LPEN_R;
///Field `UART4LPEN` reader - USART 4 clock enable during Sleep mode
pub use TIM2LPEN_R as UART4LPEN_R;
///Field `UART5LPEN` reader - USART 5 clock enable during Sleep mode
pub use TIM2LPEN_R as UART5LPEN_R;
///Field `I2C1LPEN` reader - I2C 1 clock enable during Sleep mode
pub use TIM2LPEN_R as I2C1LPEN_R;
///Field `I2C2LPEN` reader - I2C 2 clock enable during Sleep mode
pub use TIM2LPEN_R as I2C2LPEN_R;
///Field `USBLPEN` reader - USB clock enable during Sleep mode
pub use TIM2LPEN_R as USBLPEN_R;
///Field `PWRLPEN` reader - Power interface clock enable during Sleep mode
pub use TIM2LPEN_R as PWRLPEN_R;
///Field `DACLPEN` reader - DAC interface clock enable during Sleep mode
pub use TIM2LPEN_R as DACLPEN_R;
///Field `COMPLPEN` reader - COMP interface clock enable during Sleep mode
pub use TIM2LPEN_R as COMPLPEN_R;
///Field `TIM3LPEN` writer - Timer 3 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM3LPEN_W;
///Field `TIM4LPEN` writer - Timer 4 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM4LPEN_W;
///Field `TIM5LPEN` writer - Timer 5 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM5LPEN_W;
///Field `TIM6LPEN` writer - Timer 6 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM6LPEN_W;
///Field `TIM7LPEN` writer - Timer 7 clock enable during Sleep mode
pub use TIM2LPEN_W as TIM7LPEN_W;
///Field `LCDLPEN` writer - LCD clock enable during Sleep mode
pub use TIM2LPEN_W as LCDLPEN_W;
///Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode
pub use TIM2LPEN_W as WWDGLPEN_W;
///Field `SPI2LPEN` writer - SPI 2 clock enable during Sleep mode
pub use TIM2LPEN_W as SPI2LPEN_W;
///Field `SPI3LPEN` writer - SPI 3 clock enable during Sleep mode
pub use TIM2LPEN_W as SPI3LPEN_W;
///Field `USART2LPEN` writer - USART 2 clock enable during Sleep mode
pub use TIM2LPEN_W as USART2LPEN_W;
///Field `USART3LPEN` writer - USART 3 clock enable during Sleep mode
pub use TIM2LPEN_W as USART3LPEN_W;
///Field `UART4LPEN` writer - USART 4 clock enable during Sleep mode
pub use TIM2LPEN_W as UART4LPEN_W;
///Field `UART5LPEN` writer - USART 5 clock enable during Sleep mode
pub use TIM2LPEN_W as UART5LPEN_W;
///Field `I2C1LPEN` writer - I2C 1 clock enable during Sleep mode
pub use TIM2LPEN_W as I2C1LPEN_W;
///Field `I2C2LPEN` writer - I2C 2 clock enable during Sleep mode
pub use TIM2LPEN_W as I2C2LPEN_W;
///Field `USBLPEN` writer - USB clock enable during Sleep mode
pub use TIM2LPEN_W as USBLPEN_W;
///Field `PWRLPEN` writer - Power interface clock enable during Sleep mode
pub use TIM2LPEN_W as PWRLPEN_W;
///Field `DACLPEN` writer - DAC interface clock enable during Sleep mode
pub use TIM2LPEN_W as DACLPEN_W;
///Field `COMPLPEN` writer - COMP interface clock enable during Sleep mode
pub use TIM2LPEN_W as COMPLPEN_W;
impl R {
    ///Bit 0 - Timer 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer 6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LCD clock enable during Sleep mode
    #[inline(always)]
    pub fn lcdlpen(&self) -> LCDLPEN_R {
        LCDLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - USART 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - USB clock enable during Sleep mode
    #[inline(always)]
    pub fn usblpen(&self) -> USBLPEN_R {
        USBLPEN_R::new(((self.bits >> 23) & 1) != 0)
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
    ///Bit 31 - COMP interface clock enable during Sleep mode
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LPENR")
            .field("tim2lpen", &self.tim2lpen())
            .field("complpen", &self.complpen())
            .field("daclpen", &self.daclpen())
            .field("pwrlpen", &self.pwrlpen())
            .field("usblpen", &self.usblpen())
            .field("i2c2lpen", &self.i2c2lpen())
            .field("i2c1lpen", &self.i2c1lpen())
            .field("usart3lpen", &self.usart3lpen())
            .field("usart2lpen", &self.usart2lpen())
            .field("spi2lpen", &self.spi2lpen())
            .field("wwdglpen", &self.wwdglpen())
            .field("lcdlpen", &self.lcdlpen())
            .field("tim7lpen", &self.tim7lpen())
            .field("tim6lpen", &self.tim6lpen())
            .field("tim4lpen", &self.tim4lpen())
            .field("tim3lpen", &self.tim3lpen())
            .field("uart5lpen", &self.uart5lpen())
            .field("uart4lpen", &self.uart4lpen())
            .field("spi3lpen", &self.spi3lpen())
            .field("tim5lpen", &self.tim5lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<'_, APB1LPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    ///Bit 1 - Timer 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<'_, APB1LPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    ///Bit 2 - Timer 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<'_, APB1LPENRrs> {
        TIM4LPEN_W::new(self, 2)
    }
    ///Bit 3 - Timer 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<'_, APB1LPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    ///Bit 4 - Timer 6 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<'_, APB1LPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    ///Bit 5 - Timer 7 clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<'_, APB1LPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    ///Bit 9 - LCD clock enable during Sleep mode
    #[inline(always)]
    pub fn lcdlpen(&mut self) -> LCDLPEN_W<'_, APB1LPENRrs> {
        LCDLPEN_W::new(self, 9)
    }
    ///Bit 11 - Window watchdog clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<'_, APB1LPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    ///Bit 14 - SPI 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<'_, APB1LPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    ///Bit 15 - SPI 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<'_, APB1LPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    ///Bit 17 - USART 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<'_, APB1LPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    ///Bit 18 - USART 3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<'_, APB1LPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    ///Bit 19 - USART 4 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<'_, APB1LPENRrs> {
        UART4LPEN_W::new(self, 19)
    }
    ///Bit 20 - USART 5 clock enable during Sleep mode
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<'_, APB1LPENRrs> {
        UART5LPEN_W::new(self, 20)
    }
    ///Bit 21 - I2C 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<'_, APB1LPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    ///Bit 22 - I2C 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<'_, APB1LPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    ///Bit 23 - USB clock enable during Sleep mode
    #[inline(always)]
    pub fn usblpen(&mut self) -> USBLPEN_W<'_, APB1LPENRrs> {
        USBLPEN_W::new(self, 23)
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
    ///Bit 31 - COMP interface clock enable during Sleep mode
    #[inline(always)]
    pub fn complpen(&mut self) -> COMPLPEN_W<'_, APB1LPENRrs> {
        COMPLPEN_W::new(self, 31)
    }
}
/**APB1 peripheral clock enable in low power mode register

You can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:APB1LPENR)*/
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
///`reset()` method sets APB1LPENR to value 0
impl crate::Resettable for APB1LPENRrs {}
