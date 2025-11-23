///Register `APB1RSTR` reader
pub type R = crate::R<APB1RSTRrs>;
///Register `APB1RSTR` writer
pub type W = crate::W<APB1RSTRrs>;
/**TIM5 reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5RST {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<TIM5RST> for bool {
    #[inline(always)]
    fn from(variant: TIM5RST) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM5RST` reader - TIM5 reset
pub type TIM5RST_R = crate::BitReader<TIM5RST>;
impl TIM5RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIM5RST> {
        match self.bits {
            true => Some(TIM5RST::Reset),
            _ => None,
        }
    }
    ///Reset the selected module
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM5RST::Reset
    }
}
///Field `TIM5RST` writer - TIM5 reset
pub type TIM5RST_W<'a, REG> = crate::BitWriter<'a, REG, TIM5RST>;
impl<'a, REG> TIM5RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5RST::Reset)
    }
}
///Field `TIM6RST` reader - TIM6 reset
pub use TIM5RST_R as TIM6RST_R;
///Field `LPTIM1RST` reader - LPTIM1 reset
pub use TIM5RST_R as LPTIM1RST_R;
///Field `WWDGRST` reader - Window watchdog reset
pub use TIM5RST_R as WWDGRST_R;
///Field `SPI2RST` reader - SPI 2 reset
pub use TIM5RST_R as SPI2RST_R;
///Field `USART2RST` reader - USART 2 reset
pub use TIM5RST_R as USART2RST_R;
///Field `I2C1RST` reader - I2C 1 reset
pub use TIM5RST_R as I2C1RST_R;
///Field `I2C2RST` reader - I2C 2 reset
pub use TIM5RST_R as I2C2RST_R;
///Field `FMPI2C1RST` reader - FMPI2C1 reset
pub use TIM5RST_R as FMPI2C1RST_R;
///Field `PWRRST` reader - Power interface reset
pub use TIM5RST_R as PWRRST_R;
///Field `DACRST` reader - DAC reset
pub use TIM5RST_R as DACRST_R;
///Field `TIM6RST` writer - TIM6 reset
pub use TIM5RST_W as TIM6RST_W;
///Field `LPTIM1RST` writer - LPTIM1 reset
pub use TIM5RST_W as LPTIM1RST_W;
///Field `WWDGRST` writer - Window watchdog reset
pub use TIM5RST_W as WWDGRST_W;
///Field `SPI2RST` writer - SPI 2 reset
pub use TIM5RST_W as SPI2RST_W;
///Field `USART2RST` writer - USART 2 reset
pub use TIM5RST_W as USART2RST_W;
///Field `I2C1RST` writer - I2C 1 reset
pub use TIM5RST_W as I2C1RST_W;
///Field `I2C2RST` writer - I2C 2 reset
pub use TIM5RST_W as I2C2RST_W;
///Field `FMPI2C1RST` writer - FMPI2C1 reset
pub use TIM5RST_W as FMPI2C1RST_W;
///Field `PWRRST` writer - Power interface reset
pub use TIM5RST_W as PWRRST_W;
///Field `DACRST` writer - DAC reset
pub use TIM5RST_W as DACRST_W;
impl R {
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI 2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 21 - I2C 1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C 2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - FMPI2C1 reset
    #[inline(always)]
    pub fn fmpi2c1rst(&self) -> FMPI2C1RST_R {
        FMPI2C1RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC reset
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR")
            .field("tim5rst", &self.tim5rst())
            .field("tim6rst", &self.tim6rst())
            .field("lptim1rst", &self.lptim1rst())
            .field("wwdgrst", &self.wwdgrst())
            .field("spi2rst", &self.spi2rst())
            .field("usart2rst", &self.usart2rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("fmpi2c1rst", &self.fmpi2c1rst())
            .field("pwrrst", &self.pwrrst())
            .field("dacrst", &self.dacrst())
            .finish()
    }
}
impl W {
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<'_, APB1RSTRrs> {
        TIM5RST_W::new(self, 3)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<'_, APB1RSTRrs> {
        TIM6RST_W::new(self, 4)
    }
    ///Bit 9 - LPTIM1 reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<'_, APB1RSTRrs> {
        LPTIM1RST_W::new(self, 9)
    }
    ///Bit 11 - Window watchdog reset
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<'_, APB1RSTRrs> {
        WWDGRST_W::new(self, 11)
    }
    ///Bit 14 - SPI 2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<'_, APB1RSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    ///Bit 17 - USART 2 reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<'_, APB1RSTRrs> {
        USART2RST_W::new(self, 17)
    }
    ///Bit 21 - I2C 1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<'_, APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    ///Bit 22 - I2C 2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<'_, APB1RSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    ///Bit 24 - FMPI2C1 reset
    #[inline(always)]
    pub fn fmpi2c1rst(&mut self) -> FMPI2C1RST_W<'_, APB1RSTRrs> {
        FMPI2C1RST_W::new(self, 24)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
    ///Bit 29 - DAC reset
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W<'_, APB1RSTRrs> {
        DACRST_W::new(self, 29)
    }
}
/**APB1 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#RCC:APB1RSTR)*/
pub struct APB1RSTRrs;
impl crate::RegisterSpec for APB1RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1rstr::R`](R) reader structure
impl crate::Readable for APB1RSTRrs {}
///`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure
impl crate::Writable for APB1RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1RSTR to value 0
impl crate::Resettable for APB1RSTRrs {}
