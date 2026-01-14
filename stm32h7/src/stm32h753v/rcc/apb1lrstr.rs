///Register `APB1LRSTR` reader
pub type R = crate::R<APB1LRSTRrs>;
///Register `APB1LRSTR` writer
pub type W = crate::W<APB1LRSTRrs>;
/**TIM block reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<TIM2RST> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2RST` reader - TIM block reset
pub type TIM2RST_R = crate::BitReader<TIM2RST>;
impl TIM2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM2RST> {
        match self.bits {
            true => Some(TIM2RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST::Reset
    }
}
///Field `TIM2RST` writer - TIM block reset
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM2RST>;
impl<'a, REG> TIM2RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2RST::Reset)
    }
}
///Field `TIM3RST` reader - TIM block reset
pub use TIM2RST_R as TIM3RST_R;
///Field `TIM4RST` reader - TIM block reset
pub use TIM2RST_R as TIM4RST_R;
///Field `TIM5RST` reader - TIM block reset
pub use TIM2RST_R as TIM5RST_R;
///Field `TIM6RST` reader - TIM block reset
pub use TIM2RST_R as TIM6RST_R;
///Field `TIM7RST` reader - TIM block reset
pub use TIM2RST_R as TIM7RST_R;
///Field `TIM12RST` reader - TIM block reset
pub use TIM2RST_R as TIM12RST_R;
///Field `TIM13RST` reader - TIM block reset
pub use TIM2RST_R as TIM13RST_R;
///Field `TIM14RST` reader - TIM block reset
pub use TIM2RST_R as TIM14RST_R;
///Field `LPTIM1RST` reader - TIM block reset
pub use TIM2RST_R as LPTIM1RST_R;
///Field `SPI2RST` reader - SPI2 block reset
pub use TIM2RST_R as SPI2RST_R;
///Field `SPI3RST` reader - SPI3 block reset
pub use TIM2RST_R as SPI3RST_R;
///Field `SPDIFRXRST` reader - SPDIFRX block reset
pub use TIM2RST_R as SPDIFRXRST_R;
///Field `USART2RST` reader - USART2 block reset
pub use TIM2RST_R as USART2RST_R;
///Field `USART3RST` reader - USART3 block reset
pub use TIM2RST_R as USART3RST_R;
///Field `UART4RST` reader - UART4 block reset
pub use TIM2RST_R as UART4RST_R;
///Field `UART5RST` reader - UART5 block reset
pub use TIM2RST_R as UART5RST_R;
///Field `I2C1RST` reader - I2C1 block reset
pub use TIM2RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C2 block reset
pub use TIM2RST_R as I2C2RST_R;
///Field `I2C3RST` reader - I2C3 block reset
pub use TIM2RST_R as I2C3RST_R;
///Field `CECRST` reader - HDMI-CEC block reset
pub use TIM2RST_R as CECRST_R;
///Field `DAC12RST` reader - DAC1 and 2 Blocks Reset
pub use TIM2RST_R as DAC12RST_R;
///Field `UART7RST` reader - UART7 block reset
pub use TIM2RST_R as UART7RST_R;
///Field `UART8RST` reader - UART8 block reset
pub use TIM2RST_R as UART8RST_R;
///Field `TIM3RST` writer - TIM block reset
pub use TIM2RST_W as TIM3RST_W;
///Field `TIM4RST` writer - TIM block reset
pub use TIM2RST_W as TIM4RST_W;
///Field `TIM5RST` writer - TIM block reset
pub use TIM2RST_W as TIM5RST_W;
///Field `TIM6RST` writer - TIM block reset
pub use TIM2RST_W as TIM6RST_W;
///Field `TIM7RST` writer - TIM block reset
pub use TIM2RST_W as TIM7RST_W;
///Field `TIM12RST` writer - TIM block reset
pub use TIM2RST_W as TIM12RST_W;
///Field `TIM13RST` writer - TIM block reset
pub use TIM2RST_W as TIM13RST_W;
///Field `TIM14RST` writer - TIM block reset
pub use TIM2RST_W as TIM14RST_W;
///Field `LPTIM1RST` writer - TIM block reset
pub use TIM2RST_W as LPTIM1RST_W;
///Field `SPI2RST` writer - SPI2 block reset
pub use TIM2RST_W as SPI2RST_W;
///Field `SPI3RST` writer - SPI3 block reset
pub use TIM2RST_W as SPI3RST_W;
///Field `SPDIFRXRST` writer - SPDIFRX block reset
pub use TIM2RST_W as SPDIFRXRST_W;
///Field `USART2RST` writer - USART2 block reset
pub use TIM2RST_W as USART2RST_W;
///Field `USART3RST` writer - USART3 block reset
pub use TIM2RST_W as USART3RST_W;
///Field `UART4RST` writer - UART4 block reset
pub use TIM2RST_W as UART4RST_W;
///Field `UART5RST` writer - UART5 block reset
pub use TIM2RST_W as UART5RST_W;
///Field `I2C1RST` writer - I2C1 block reset
pub use TIM2RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C2 block reset
pub use TIM2RST_W as I2C2RST_W;
///Field `I2C3RST` writer - I2C3 block reset
pub use TIM2RST_W as I2C3RST_W;
///Field `CECRST` writer - HDMI-CEC block reset
pub use TIM2RST_W as CECRST_W;
///Field `DAC12RST` writer - DAC1 and 2 Blocks Reset
pub use TIM2RST_W as DAC12RST_W;
///Field `UART7RST` writer - UART7 block reset
pub use TIM2RST_W as UART7RST_W;
///Field `UART8RST` writer - UART8 block reset
pub use TIM2RST_W as UART8RST_W;
impl R {
    ///Bit 0 - TIM block reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM block reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM block reset
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM block reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM block reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM block reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM block reset
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM block reset
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM block reset
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIM block reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - SPI2 block reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 block reset
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SPDIFRX block reset
    #[inline(always)]
    pub fn spdifrxrst(&self) -> SPDIFRXRST_R {
        SPDIFRXRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 block reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 block reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 block reset
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 block reset
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 block reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 block reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 block reset
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 27 - HDMI-CEC block reset
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - DAC1 and 2 Blocks Reset
    #[inline(always)]
    pub fn dac12rst(&self) -> DAC12RST_R {
        DAC12RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - UART7 block reset
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UART8 block reset
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LRSTR")
            .field("tim2rst", &self.tim2rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("tim7rst", &self.tim7rst())
            .field("tim12rst", &self.tim12rst())
            .field("tim13rst", &self.tim13rst())
            .field("tim14rst", &self.tim14rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("spi2rst", &self.spi2rst())
            .field("spi3rst", &self.spi3rst())
            .field("spdifrxrst", &self.spdifrxrst())
            .field("usart2rst", &self.usart2rst())
            .field("usart3rst", &self.usart3rst())
            .field("uart4rst", &self.uart4rst())
            .field("uart5rst", &self.uart5rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("i2c3rst", &self.i2c3rst())
            .field("cecrst", &self.cecrst())
            .field("dac12rst", &self.dac12rst())
            .field("uart7rst", &self.uart7rst())
            .field("uart8rst", &self.uart8rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM block reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<'_, APB1LRSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    ///Bit 1 - TIM block reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<'_, APB1LRSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    ///Bit 2 - TIM block reset
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<'_, APB1LRSTRrs> {
        TIM4RST_W::new(self, 2)
    }
    ///Bit 3 - TIM block reset
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<'_, APB1LRSTRrs> {
        TIM5RST_W::new(self, 3)
    }
    ///Bit 4 - TIM block reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<'_, APB1LRSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 5 - TIM block reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<'_, APB1LRSTRrs> {
        TIM7RST_W::new(self, 5)
    }
    ///Bit 6 - TIM block reset
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W<'_, APB1LRSTRrs> {
        TIM12RST_W::new(self, 6)
    }
    ///Bit 7 - TIM block reset
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W<'_, APB1LRSTRrs> {
        TIM13RST_W::new(self, 7)
    }
    ///Bit 8 - TIM block reset
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<'_, APB1LRSTRrs> {
        TIM14RST_W::new(self, 8)
    }
    ///Bit 9 - TIM block reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB1LRSTRrs> {
        LPTIM1RST_W::new(self, 9)
    }
    ///Bit 14 - SPI2 block reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APB1LRSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 15 - SPI3 block reset
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<'_, APB1LRSTRrs> {
        SPI3RST_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX block reset
    #[inline(always)]
    pub fn spdifrxrst(&mut self) -> SPDIFRXRST_W<'_, APB1LRSTRrs> {
        SPDIFRXRST_W::new(self, 16)
    }
    ///Bit 17 - USART2 block reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, APB1LRSTRrs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 18 - USART3 block reset
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<'_, APB1LRSTRrs> {
        USART3RST_W::new(self, 18)
    }
    ///Bit 19 - UART4 block reset
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W<'_, APB1LRSTRrs> {
        UART4RST_W::new(self, 19)
    }
    ///Bit 20 - UART5 block reset
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W<'_, APB1LRSTRrs> {
        UART5RST_W::new(self, 20)
    }
    ///Bit 21 - I2C1 block reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APB1LRSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C2 block reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, APB1LRSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 23 - I2C3 block reset
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<'_, APB1LRSTRrs> {
        I2C3RST_W::new(self, 23)
    }
    ///Bit 27 - HDMI-CEC block reset
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W<'_, APB1LRSTRrs> {
        CECRST_W::new(self, 27)
    }
    ///Bit 29 - DAC1 and 2 Blocks Reset
    #[inline(always)]
    pub fn dac12rst(&mut self) -> DAC12RST_W<'_, APB1LRSTRrs> {
        DAC12RST_W::new(self, 29)
    }
    ///Bit 30 - UART7 block reset
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W<'_, APB1LRSTRrs> {
        UART7RST_W::new(self, 30)
    }
    ///Bit 31 - UART8 block reset
    #[inline(always)]
    pub fn uart8rst(&mut self) -> UART8RST_W<'_, APB1LRSTRrs> {
        UART8RST_W::new(self, 31)
    }
}
/**RCC APB1 Peripheral Reset Register

You can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#RCC:APB1LRSTR)*/
pub struct APB1LRSTRrs;
impl crate::RegisterSpec for APB1LRSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1lrstr::R`](R) reader structure
impl crate::Readable for APB1LRSTRrs {}
///`write(|w| ..)` method takes [`apb1lrstr::W`](W) writer structure
impl crate::Writable for APB1LRSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LRSTR to value 0
impl crate::Resettable for APB1LRSTRrs {}
