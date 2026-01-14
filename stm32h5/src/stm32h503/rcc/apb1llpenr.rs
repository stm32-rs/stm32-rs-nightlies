///Register `APB1LLPENR` reader
pub type R = crate::R<APB1LLPENRrs>;
///Register `APB1LLPENR` writer
pub type W = crate::W<APB1LLPENRrs>;
/**TIM2 clock enable during sleep mode Set and reset by software.

Value on reset: 1*/
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
///Field `TIM2LPEN` reader - TIM2 clock enable during sleep mode Set and reset by software.
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
///Field `TIM2LPEN` writer - TIM2 clock enable during sleep mode Set and reset by software.
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
///Field `TIM3LPEN` reader - TIM3 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as TIM3LPEN_R;
///Field `TIM6LPEN` reader - TIM6 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as TIM6LPEN_R;
///Field `TIM7LPEN` reader - TIM7 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as TIM7LPEN_R;
///Field `WWDGLPEN` reader - WWDG clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as WWDGLPEN_R;
///Field `OPAMPLPEN` reader - OPAMP clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as OPAMPLPEN_R;
///Field `SPI2LPEN` reader - SPI2 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as SPI2LPEN_R;
///Field `SPI3LPEN` reader - SPI3 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as SPI3LPEN_R;
///Field `COMPLPEN` reader - COMP clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as COMPLPEN_R;
///Field `USART2LPEN` reader - USART2 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as USART2LPEN_R;
///Field `USART3LPEN` reader - USART3 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as USART3LPEN_R;
///Field `I2C1LPEN` reader - I2C1 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as I2C1LPEN_R;
///Field `I2C2LPEN` reader - I2C2 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as I2C2LPEN_R;
///Field `I3C1LPEN` reader - I3C1 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as I3C1LPEN_R;
///Field `CRSLPEN` reader - CRS clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_R as CRSLPEN_R;
///Field `TIM3LPEN` writer - TIM3 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as TIM3LPEN_W;
///Field `TIM6LPEN` writer - TIM6 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as TIM6LPEN_W;
///Field `TIM7LPEN` writer - TIM7 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as TIM7LPEN_W;
///Field `WWDGLPEN` writer - WWDG clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as WWDGLPEN_W;
///Field `OPAMPLPEN` writer - OPAMP clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as OPAMPLPEN_W;
///Field `SPI2LPEN` writer - SPI2 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as SPI2LPEN_W;
///Field `SPI3LPEN` writer - SPI3 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as SPI3LPEN_W;
///Field `COMPLPEN` writer - COMP clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as COMPLPEN_W;
///Field `USART2LPEN` writer - USART2 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as USART2LPEN_W;
///Field `USART3LPEN` writer - USART3 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as USART3LPEN_W;
///Field `I2C1LPEN` writer - I2C1 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as I2C1LPEN_W;
///Field `I2C2LPEN` writer - I2C2 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as I2C2LPEN_W;
///Field `I3C1LPEN` writer - I3C1 clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as I3C1LPEN_W;
///Field `CRSLPEN` writer - CRS clock enable during sleep mode Set and reset by software.
pub use TIM2LPEN_W as CRSLPEN_W;
impl R {
    ///Bit 0 - TIM2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - OPAMP clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn opamplpen(&self) -> OPAMPLPEN_R {
        OPAMPLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - COMP clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I3C1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn i3c1lpen(&self) -> I3C1LPEN_R {
        I3C1LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn crslpen(&self) -> CRSLPEN_R {
        CRSLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LLPENR")
            .field("tim2lpen", &self.tim2lpen())
            .field("tim3lpen", &self.tim3lpen())
            .field("tim6lpen", &self.tim6lpen())
            .field("tim7lpen", &self.tim7lpen())
            .field("wwdglpen", &self.wwdglpen())
            .field("opamplpen", &self.opamplpen())
            .field("spi2lpen", &self.spi2lpen())
            .field("spi3lpen", &self.spi3lpen())
            .field("complpen", &self.complpen())
            .field("usart2lpen", &self.usart2lpen())
            .field("usart3lpen", &self.usart3lpen())
            .field("i2c1lpen", &self.i2c1lpen())
            .field("i2c2lpen", &self.i2c2lpen())
            .field("i3c1lpen", &self.i3c1lpen())
            .field("crslpen", &self.crslpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<'_, APB1LLPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<'_, APB1LLPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    ///Bit 4 - TIM6 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<'_, APB1LLPENRrs> {
        TIM6LPEN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<'_, APB1LLPENRrs> {
        TIM7LPEN_W::new(self, 5)
    }
    ///Bit 11 - WWDG clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<'_, APB1LLPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    ///Bit 13 - OPAMP clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn opamplpen(&mut self) -> OPAMPLPEN_W<'_, APB1LLPENRrs> {
        OPAMPLPEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<'_, APB1LLPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<'_, APB1LLPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    ///Bit 16 - COMP clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn complpen(&mut self) -> COMPLPEN_W<'_, APB1LLPENRrs> {
        COMPLPEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<'_, APB1LLPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<'_, APB1LLPENRrs> {
        USART3LPEN_W::new(self, 18)
    }
    ///Bit 21 - I2C1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<'_, APB1LLPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<'_, APB1LLPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    ///Bit 23 - I3C1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn i3c1lpen(&mut self) -> I3C1LPEN_W<'_, APB1LLPENRrs> {
        I3C1LPEN_W::new(self, 23)
    }
    ///Bit 24 - CRS clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn crslpen(&mut self) -> CRSLPEN_W<'_, APB1LLPENRrs> {
        CRSLPEN_W::new(self, 24)
    }
}
/**RCC APB1 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:APB1LLPENR)*/
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
///`reset()` method sets APB1LLPENR to value 0xffff_ffff
impl crate::Resettable for APB1LLPENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
