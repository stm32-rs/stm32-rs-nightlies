///Register `APB1LENR` reader
pub type R = crate::R<APB1LENRrs>;
///Register `APB1LENR` writer
pub type W = crate::W<APB1LENRrs>;
/**TIM2 clock enable Set and reset by software.

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
///Field `TIM2EN` reader - TIM2 clock enable Set and reset by software.
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
///Field `TIM2EN` writer - TIM2 clock enable Set and reset by software.
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
///Field `TIM3EN` reader - TIM3 clock enable Set and reset by software.
pub use TIM2EN_R as TIM3EN_R;
///Field `TIM6EN` reader - TIM6 clock enable Set and reset by software.
pub use TIM2EN_R as TIM6EN_R;
///Field `TIM7EN` reader - TIM7 clock enable Set and reset by software.
pub use TIM2EN_R as TIM7EN_R;
///Field `WWDGEN` reader - WWDG clock enable Set and reset by software.
pub use TIM2EN_R as WWDGEN_R;
///Field `OPAMPEN` reader - OPAMP clock enable Set and reset by software.
pub use TIM2EN_R as OPAMPEN_R;
///Field `SPI2EN` reader - SPI2 clock enable Set and reset by software.
pub use TIM2EN_R as SPI2EN_R;
///Field `SPI3EN` reader - SPI3 clock enable Set and reset by software.
pub use TIM2EN_R as SPI3EN_R;
///Field `COMPEN` reader - COMP clock enable Set and reset by software.
pub use TIM2EN_R as COMPEN_R;
///Field `USART2EN` reader - USART2 clock enable Set and reset by software.
pub use TIM2EN_R as USART2EN_R;
///Field `USART3EN` reader - USART3 clock enable Set and reset by software.
pub use TIM2EN_R as USART3EN_R;
///Field `I2C1EN` reader - I2C1 clock enable Set and reset by software.
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 clock enable Set and reset by software.
pub use TIM2EN_R as I2C2EN_R;
///Field `I3C1EN` reader - I3C1 clock enable Set and reset by software.
pub use TIM2EN_R as I3C1EN_R;
///Field `CRSEN` reader - CRS clock enable Set and reset by software.
pub use TIM2EN_R as CRSEN_R;
///Field `TIM3EN` writer - TIM3 clock enable Set and reset by software.
pub use TIM2EN_W as TIM3EN_W;
///Field `TIM6EN` writer - TIM6 clock enable Set and reset by software.
pub use TIM2EN_W as TIM6EN_W;
///Field `TIM7EN` writer - TIM7 clock enable Set and reset by software.
pub use TIM2EN_W as TIM7EN_W;
///Field `WWDGEN` writer - WWDG clock enable Set and reset by software.
pub use TIM2EN_W as WWDGEN_W;
///Field `OPAMPEN` writer - OPAMP clock enable Set and reset by software.
pub use TIM2EN_W as OPAMPEN_W;
///Field `SPI2EN` writer - SPI2 clock enable Set and reset by software.
pub use TIM2EN_W as SPI2EN_W;
///Field `SPI3EN` writer - SPI3 clock enable Set and reset by software.
pub use TIM2EN_W as SPI3EN_W;
///Field `COMPEN` writer - COMP clock enable Set and reset by software.
pub use TIM2EN_W as COMPEN_W;
///Field `USART2EN` writer - USART2 clock enable Set and reset by software.
pub use TIM2EN_W as USART2EN_W;
///Field `USART3EN` writer - USART3 clock enable Set and reset by software.
pub use TIM2EN_W as USART3EN_W;
///Field `I2C1EN` writer - I2C1 clock enable Set and reset by software.
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 clock enable Set and reset by software.
pub use TIM2EN_W as I2C2EN_W;
///Field `I3C1EN` writer - I3C1 clock enable Set and reset by software.
pub use TIM2EN_W as I3C1EN_W;
///Field `CRSEN` writer - CRS clock enable Set and reset by software.
pub use TIM2EN_W as CRSEN_W;
impl R {
    ///Bit 0 - TIM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable Set and reset by software.
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - OPAMP clock enable Set and reset by software.
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - COMP clock enable Set and reset by software.
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I3C1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i3c1en(&self) -> I3C1EN_R {
        I3C1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CRS clock enable Set and reset by software.
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LENR")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("wwdgen", &self.wwdgen())
            .field("opampen", &self.opampen())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("compen", &self.compen())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("i3c1en", &self.i3c1en())
            .field("crsen", &self.crsen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1LENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1LENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 4 - TIM6 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1LENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APB1LENRrs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 11 - WWDG clock enable Set and reset by software.
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1LENRrs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 13 - OPAMP clock enable Set and reset by software.
    #[inline(always)]
    pub fn opampen(&mut self) -> OPAMPEN_W<'_, APB1LENRrs> {
        OPAMPEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1LENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1LENRrs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 16 - COMP clock enable Set and reset by software.
    #[inline(always)]
    pub fn compen(&mut self) -> COMPEN_W<'_, APB1LENRrs> {
        COMPEN_W::new(self, 16)
    }
    ///Bit 17 - USART2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1LENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable Set and reset by software.
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APB1LENRrs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 21 - I2C1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1LENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1LENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - I3C1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn i3c1en(&mut self) -> I3C1EN_W<'_, APB1LENRrs> {
        I3C1EN_W::new(self, 23)
    }
    ///Bit 24 - CRS clock enable Set and reset by software.
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W<'_, APB1LENRrs> {
        CRSEN_W::new(self, 24)
    }
}
/**RCC APB1 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:APB1LENR)*/
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
