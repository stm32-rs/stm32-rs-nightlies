///Register `APB1LLPENR` reader
pub type R = crate::R<APB1LLPENRrs>;
///Register `APB1LLPENR` writer
pub type W = crate::W<APB1LLPENRrs>;
/**TIM2 peripheral clock enable during CSleep mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<TIM2LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2LPEN` reader - TIM2 peripheral clock enable during CSleep mode
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
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2LPEN::Enabled
    }
}
///Field `TIM2LPEN` writer - TIM2 peripheral clock enable during CSleep mode
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2LPEN>;
impl<'a, REG> TIM2LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2LPEN::Enabled)
    }
}
///Field `TIM3LPEN` reader - TIM3 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM3LPEN_R;
///Field `TIM4LPEN` reader - TIM4 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM4LPEN_R;
///Field `TIM5LPEN` reader - TIM5 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM5LPEN_R;
///Field `TIM6LPEN` reader - TIM6 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM6LPEN_R;
///Field `TIM7LPEN` reader - TIM7 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM7LPEN_R;
///Field `TIM12LPEN` reader - TIM12 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM12LPEN_R;
///Field `TIM13LPEN` reader - TIM13 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM13LPEN_R;
///Field `TIM14LPEN` reader - TIM14 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as TIM14LPEN_R;
///Field `LPTIM1LPEN` reader - LPTIM1 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as LPTIM1LPEN_R;
///Field `SPI2LPEN` reader - SPI2 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as SPI2LPEN_R;
///Field `SPI3LPEN` reader - SPI3 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as SPI3LPEN_R;
///Field `SPDIFRXLPEN` reader - SPDIFRX Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as SPDIFRXLPEN_R;
///Field `USART2LPEN` reader - USART2 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as USART2LPEN_R;
///Field `USART3LPEN` reader - USART3 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as USART3LPEN_R;
///Field `UART4LPEN` reader - UART4 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as UART4LPEN_R;
///Field `UART5LPEN` reader - UART5 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as UART5LPEN_R;
///Field `I2C1LPEN` reader - I2C1 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as I2C1LPEN_R;
///Field `I2C2LPEN` reader - I2C2 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as I2C2LPEN_R;
///Field `I2C3LPEN` reader - I2C3 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as I2C3LPEN_R;
///Field `CECLPEN` reader - HDMI-CEC Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as CECLPEN_R;
///Field `DAC12LPEN` reader - DAC1/2 peripheral clock enable during CSleep mode
pub use TIM2LPEN_R as DAC12LPEN_R;
///Field `UART7LPEN` reader - UART7 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as UART7LPEN_R;
///Field `UART8LPEN` reader - UART8 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_R as UART8LPEN_R;
///Field `TIM3LPEN` writer - TIM3 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM3LPEN_W;
///Field `TIM4LPEN` writer - TIM4 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM4LPEN_W;
///Field `TIM5LPEN` writer - TIM5 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM5LPEN_W;
///Field `TIM6LPEN` writer - TIM6 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM6LPEN_W;
///Field `TIM7LPEN` writer - TIM7 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM7LPEN_W;
///Field `TIM12LPEN` writer - TIM12 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM12LPEN_W;
///Field `TIM13LPEN` writer - TIM13 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM13LPEN_W;
///Field `TIM14LPEN` writer - TIM14 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as TIM14LPEN_W;
///Field `LPTIM1LPEN` writer - LPTIM1 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as LPTIM1LPEN_W;
///Field `SPI2LPEN` writer - SPI2 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as SPI2LPEN_W;
///Field `SPI3LPEN` writer - SPI3 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as SPI3LPEN_W;
///Field `SPDIFRXLPEN` writer - SPDIFRX Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as SPDIFRXLPEN_W;
///Field `USART2LPEN` writer - USART2 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as USART2LPEN_W;
///Field `USART3LPEN` writer - USART3 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as USART3LPEN_W;
///Field `UART4LPEN` writer - UART4 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as UART4LPEN_W;
///Field `UART5LPEN` writer - UART5 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as UART5LPEN_W;
///Field `I2C1LPEN` writer - I2C1 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as I2C1LPEN_W;
///Field `I2C2LPEN` writer - I2C2 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as I2C2LPEN_W;
///Field `I2C3LPEN` writer - I2C3 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as I2C3LPEN_W;
///Field `CECLPEN` writer - HDMI-CEC Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as CECLPEN_W;
///Field `DAC12LPEN` writer - DAC1/2 peripheral clock enable during CSleep mode
pub use TIM2LPEN_W as DAC12LPEN_W;
///Field `UART7LPEN` writer - UART7 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as UART7LPEN_W;
///Field `UART8LPEN` writer - UART8 Peripheral Clocks Enable During CSleep Mode
pub use TIM2LPEN_W as UART8LPEN_W;
impl R {
    ///Bit 0 - TIM2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - SPI2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPDIFRX Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spdifrxlpen(&self) -> SPDIFRXLPEN_R {
        SPDIFRXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - HDMI-CEC Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn ceclpen(&self) -> CECLPEN_R {
        CECLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DAC1/2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn dac12lpen(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - UART7 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UART8 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LLPENR")
            .field("tim2lpen", &self.tim2lpen())
            .field("tim3lpen", &self.tim3lpen())
            .field("tim4lpen", &self.tim4lpen())
            .field("tim5lpen", &self.tim5lpen())
            .field("tim6lpen", &self.tim6lpen())
            .field("tim7lpen", &self.tim7lpen())
            .field("tim12lpen", &self.tim12lpen())
            .field("tim13lpen", &self.tim13lpen())
            .field("tim14lpen", &self.tim14lpen())
            .field("lptim1lpen", &self.lptim1lpen())
            .field("spi2lpen", &self.spi2lpen())
            .field("spi3lpen", &self.spi3lpen())
            .field("spdifrxlpen", &self.spdifrxlpen())
            .field("usart2lpen", &self.usart2lpen())
            .field("usart3lpen", &self.usart3lpen())
            .field("uart4lpen", &self.uart4lpen())
            .field("uart5lpen", &self.uart5lpen())
            .field("i2c1lpen", &self.i2c1lpen())
            .field("i2c2lpen", &self.i2c2lpen())
            .field("i2c3lpen", &self.i2c3lpen())
            .field("ceclpen", &self.ceclpen())
            .field("dac12lpen", &self.dac12lpen())
            .field("uart7lpen", &self.uart7lpen())
            .field("uart8lpen", &self.uart8lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<APB1LLPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<APB1LLPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<APB1LLPENRrs> {
        TIM4LPEN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<APB1LLPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<APB1LLPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<APB1LLPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    ///Bit 6 - TIM12 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<APB1LLPENRrs> {
        TIM12LPEN_W::new(self, 6)
    }
    ///Bit 7 - TIM13 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<APB1LLPENRrs> {
        TIM13LPEN_W::new(self, 7)
    }
    ///Bit 8 - TIM14 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<APB1LLPENRrs> {
        TIM14LPEN_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<APB1LLPENRrs> {
        LPTIM1LPEN_W::new(self, 9)
    }
    ///Bit 14 - SPI2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LLPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<APB1LLPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spdifrxlpen(&mut self) -> SPDIFRXLPEN_W<APB1LLPENRrs> {
        SPDIFRXLPEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LLPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<APB1LLPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    ///Bit 19 - UART4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<APB1LLPENRrs> {
        UART4LPEN_W::new(self, 19)
    }
    ///Bit 20 - UART5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<APB1LLPENRrs> {
        UART5LPEN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<APB1LLPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<APB1LLPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<APB1LLPENRrs> {
        I2C3LPEN_W::new(self, 23)
    }
    ///Bit 27 - HDMI-CEC Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn ceclpen(&mut self) -> CECLPEN_W<APB1LLPENRrs> {
        CECLPEN_W::new(self, 27)
    }
    ///Bit 29 - DAC1/2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn dac12lpen(&mut self) -> DAC12LPEN_W<APB1LLPENRrs> {
        DAC12LPEN_W::new(self, 29)
    }
    ///Bit 30 - UART7 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<APB1LLPENRrs> {
        UART7LPEN_W::new(self, 30)
    }
    ///Bit 31 - UART8 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<APB1LLPENRrs> {
        UART8LPEN_W::new(self, 31)
    }
}
/**RCC APB1 Low Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#RCC:APB1LLPENR)*/
pub struct APB1LLPENRrs;
impl crate::RegisterSpec for APB1LLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1llpenr::R`](R) reader structure
impl crate::Readable for APB1LLPENRrs {}
///`write(|w| ..)` method takes [`apb1llpenr::W`](W) writer structure
impl crate::Writable for APB1LLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LLPENR to value 0
impl crate::Resettable for APB1LLPENRrs {}
