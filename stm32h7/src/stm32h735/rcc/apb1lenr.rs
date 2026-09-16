///Register `APB1LENR` reader
pub type R = crate::R<APB1LENRrs>;
///Register `APB1LENR` writer
pub type W = crate::W<APB1LENRrs>;
/**TIM peripheral clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - TIM peripheral clock enable
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
///Field `TIM2EN` writer - TIM peripheral clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
///Field `TIM3EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM3EN_R;
///Field `TIM4EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM4EN_R;
///Field `TIM5EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM5EN_R;
///Field `TIM6EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM6EN_R;
///Field `TIM7EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM7EN_R;
///Field `TIM12EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM12EN_R;
///Field `TIM13EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM13EN_R;
///Field `TIM14EN` reader - TIM peripheral clock enable
pub use TIM2EN_R as TIM14EN_R;
///Field `LPTIM1EN` reader - LPTIM1 Peripheral Clocks Enable
pub use TIM2EN_R as LPTIM1EN_R;
///Field `SPI2EN` reader - SPI2 Peripheral Clocks Enable
pub use TIM2EN_R as SPI2EN_R;
///Field `SPI3EN` reader - SPI3 Peripheral Clocks Enable
pub use TIM2EN_R as SPI3EN_R;
///Field `SPDIFRXEN` reader - SPDIFRX Peripheral Clocks Enable
pub use TIM2EN_R as SPDIFRXEN_R;
///Field `USART2EN` reader - USART2 Peripheral Clocks Enable
pub use TIM2EN_R as USART2EN_R;
///Field `USART3EN` reader - USART3 Peripheral Clocks Enable
pub use TIM2EN_R as USART3EN_R;
///Field `UART4EN` reader - UART4 Peripheral Clocks Enable
pub use TIM2EN_R as UART4EN_R;
///Field `UART5EN` reader - UART5 Peripheral Clocks Enable
pub use TIM2EN_R as UART5EN_R;
///Field `I2C1EN` reader - I2C1 Peripheral Clocks Enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 Peripheral Clocks Enable
pub use TIM2EN_R as I2C2EN_R;
///Field `I2C3EN` reader - I2C3 Peripheral Clocks Enable
pub use TIM2EN_R as I2C3EN_R;
///Field `I2C5EN` reader - I2C5 Peripheral Clocks Enable
pub use TIM2EN_R as I2C5EN_R;
///Field `CECEN` reader - HDMI-CEC peripheral clock enable
pub use TIM2EN_R as CECEN_R;
///Field `DAC12EN` reader - DAC1 and 2 peripheral clock enable
pub use TIM2EN_R as DAC12EN_R;
///Field `UART7EN` reader - UART7 Peripheral Clocks Enable
pub use TIM2EN_R as UART7EN_R;
///Field `UART8EN` reader - UART8 Peripheral Clocks Enable
pub use TIM2EN_R as UART8EN_R;
///Field `TIM3EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM3EN_W;
///Field `TIM4EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM4EN_W;
///Field `TIM5EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM5EN_W;
///Field `TIM6EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM6EN_W;
///Field `TIM7EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM7EN_W;
///Field `TIM12EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM12EN_W;
///Field `TIM13EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM13EN_W;
///Field `TIM14EN` writer - TIM peripheral clock enable
pub use TIM2EN_W as TIM14EN_W;
///Field `LPTIM1EN` writer - LPTIM1 Peripheral Clocks Enable
pub use TIM2EN_W as LPTIM1EN_W;
///Field `SPI2EN` writer - SPI2 Peripheral Clocks Enable
pub use TIM2EN_W as SPI2EN_W;
///Field `SPI3EN` writer - SPI3 Peripheral Clocks Enable
pub use TIM2EN_W as SPI3EN_W;
///Field `SPDIFRXEN` writer - SPDIFRX Peripheral Clocks Enable
pub use TIM2EN_W as SPDIFRXEN_W;
///Field `USART2EN` writer - USART2 Peripheral Clocks Enable
pub use TIM2EN_W as USART2EN_W;
///Field `USART3EN` writer - USART3 Peripheral Clocks Enable
pub use TIM2EN_W as USART3EN_W;
///Field `UART4EN` writer - UART4 Peripheral Clocks Enable
pub use TIM2EN_W as UART4EN_W;
///Field `UART5EN` writer - UART5 Peripheral Clocks Enable
pub use TIM2EN_W as UART5EN_W;
///Field `I2C1EN` writer - I2C1 Peripheral Clocks Enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 Peripheral Clocks Enable
pub use TIM2EN_W as I2C2EN_W;
///Field `I2C3EN` writer - I2C3 Peripheral Clocks Enable
pub use TIM2EN_W as I2C3EN_W;
///Field `I2C5EN` writer - I2C5 Peripheral Clocks Enable
pub use TIM2EN_W as I2C5EN_W;
///Field `CECEN` writer - HDMI-CEC peripheral clock enable
pub use TIM2EN_W as CECEN_W;
///Field `DAC12EN` writer - DAC1 and 2 peripheral clock enable
pub use TIM2EN_W as DAC12EN_W;
///Field `UART7EN` writer - UART7 Peripheral Clocks Enable
pub use TIM2EN_W as UART7EN_W;
///Field `UART8EN` writer - UART8 Peripheral Clocks Enable
pub use TIM2EN_W as UART8EN_W;
impl R {
    ///Bit 0 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - SPI2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPDIFRX Peripheral Clocks Enable
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - I2C5 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c5en(&self) -> I2C5EN_R {
        I2C5EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - HDMI-CEC peripheral clock enable
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DAC1 and 2 peripheral clock enable
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - UART7 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UART8 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart8en(&self) -> UART8EN_R {
        UART8EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LENR")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim5en", &self.tim5en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("tim12en", &self.tim12en())
            .field("tim13en", &self.tim13en())
            .field("tim14en", &self.tim14en())
            .field("lptim1en", &self.lptim1en())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("spdifrxen", &self.spdifrxen())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("uart4en", &self.uart4en())
            .field("uart5en", &self.uart5en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c3en", &self.i2c3en())
            .field("i2c5en", &self.i2c5en())
            .field("cecen", &self.cecen())
            .field("dac12en", &self.dac12en())
            .field("uart7en", &self.uart7en())
            .field("uart8en", &self.uart8en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1LENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1LENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APB1LENRrs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 3 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<'_, APB1LENRrs> {
        TIM5EN_W::new(self, 3)
    }
    ///Bit 4 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1LENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APB1LENRrs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 6 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim12en(&mut self) -> TIM12EN_W<'_, APB1LENRrs> {
        TIM12EN_W::new(self, 6)
    }
    ///Bit 7 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim13en(&mut self) -> TIM13EN_W<'_, APB1LENRrs> {
        TIM13EN_W::new(self, 7)
    }
    ///Bit 8 - TIM peripheral clock enable
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<'_, APB1LENRrs> {
        TIM14EN_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB1LENRrs> {
        LPTIM1EN_W::new(self, 9)
    }
    ///Bit 14 - SPI2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1LENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1LENRrs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX Peripheral Clocks Enable
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<'_, APB1LENRrs> {
        SPDIFRXEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1LENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APB1LENRrs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - UART4 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W<'_, APB1LENRrs> {
        UART4EN_W::new(self, 19)
    }
    ///Bit 20 - UART5 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart5en(&mut self) -> UART5EN_W<'_, APB1LENRrs> {
        UART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1LENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1LENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1LENRrs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 25 - I2C5 Peripheral Clocks Enable
    #[inline(always)]
    pub fn i2c5en(&mut self) -> I2C5EN_W<'_, APB1LENRrs> {
        I2C5EN_W::new(self, 25)
    }
    ///Bit 27 - HDMI-CEC peripheral clock enable
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W<'_, APB1LENRrs> {
        CECEN_W::new(self, 27)
    }
    ///Bit 29 - DAC1 and 2 peripheral clock enable
    #[inline(always)]
    pub fn dac12en(&mut self) -> DAC12EN_W<'_, APB1LENRrs> {
        DAC12EN_W::new(self, 29)
    }
    ///Bit 30 - UART7 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart7en(&mut self) -> UART7EN_W<'_, APB1LENRrs> {
        UART7EN_W::new(self, 30)
    }
    ///Bit 31 - UART8 Peripheral Clocks Enable
    #[inline(always)]
    pub fn uart8en(&mut self) -> UART8EN_W<'_, APB1LENRrs> {
        UART8EN_W::new(self, 31)
    }
}
/**RCC APB1 Clock Register

You can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#RCC:APB1LENR)*/
pub struct APB1LENRrs;
impl crate::RegisterSpec for APB1LENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1lenr::R`](R) reader structure
impl crate::Readable for APB1LENRrs {}
///`write(|w| ..)` method takes [`apb1lenr::W`](W) writer structure
impl crate::Writable for APB1LENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LENR to value 0
impl crate::Resettable for APB1LENRrs {}
