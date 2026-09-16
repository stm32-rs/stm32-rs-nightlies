///Register `APB2LPENR` reader
pub type R = crate::R<APB2LPENRrs>;
///Register `APB2LPENR` writer
pub type W = crate::W<APB2LPENRrs>;
/**TIM1 clock enable during sleep mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1LPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<TIM1LPEN> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1LPEN` reader - TIM1 clock enable during sleep mode Set and reset by software.
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN>;
impl TIM1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1LPEN {
        match self.bits {
            false => TIM1LPEN::Disabled,
            true => TIM1LPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1LPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1LPEN::Enabled
    }
}
///Field `TIM1LPEN` writer - TIM1 clock enable during sleep mode Set and reset by software.
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1LPEN>;
impl<'a, REG> TIM1LPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1LPEN::Enabled)
    }
}
///Field `SPI1LPEN` reader - SPI1 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as SPI1LPEN_R;
///Field `TIM8LPEN` reader - TIM8 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as TIM8LPEN_R;
///Field `USART1LPEN` reader - USART1 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as USART1LPEN_R;
///Field `TIM15LPEN` reader - TIM15 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as TIM15LPEN_R;
///Field `TIM16LPEN` reader - TIM16 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as TIM16LPEN_R;
///Field `TIM17LPEN` reader - TIM17 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as TIM17LPEN_R;
///Field `SPI4LPEN` reader - SPI4 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as SPI4LPEN_R;
///Field `SPI6LPEN` reader - SPI6 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as SPI6LPEN_R;
///Field `SAI1LPEN` reader - SAI1 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as SAI1LPEN_R;
///Field `SAI2LPEN` reader - SAI2 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_R as SAI2LPEN_R;
///Field `USBLPEN` reader - USB clock enable during sleep mode
pub use TIM1LPEN_R as USBLPEN_R;
///Field `SPI1LPEN` writer - SPI1 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as SPI1LPEN_W;
///Field `TIM8LPEN` writer - TIM8 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as TIM8LPEN_W;
///Field `USART1LPEN` writer - USART1 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as USART1LPEN_W;
///Field `TIM15LPEN` writer - TIM15 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as TIM15LPEN_W;
///Field `TIM16LPEN` writer - TIM16 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as TIM16LPEN_W;
///Field `TIM17LPEN` writer - TIM17 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as TIM17LPEN_W;
///Field `SPI4LPEN` writer - SPI4 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as SPI4LPEN_W;
///Field `SPI6LPEN` writer - SPI6 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as SPI6LPEN_W;
///Field `SAI1LPEN` writer - SAI1 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as SAI1LPEN_W;
///Field `SAI2LPEN` writer - SAI2 clock enable during sleep mode Set and reset by software.
pub use TIM1LPEN_W as SAI2LPEN_W;
///Field `USBLPEN` writer - USB clock enable during sleep mode
pub use TIM1LPEN_W as USBLPEN_W;
impl R {
    ///Bit 11 - TIM1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPI4 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPI6 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SAI1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - USB clock enable during sleep mode
    #[inline(always)]
    pub fn usblpen(&self) -> USBLPEN_R {
        USBLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPENR")
            .field("tim1lpen", &self.tim1lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("tim8lpen", &self.tim8lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("tim15lpen", &self.tim15lpen())
            .field("tim16lpen", &self.tim16lpen())
            .field("tim17lpen", &self.tim17lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("spi6lpen", &self.spi6lpen())
            .field("sai1lpen", &self.sai1lpen())
            .field("sai2lpen", &self.sai2lpen())
            .field("usblpen", &self.usblpen())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<'_, APB2LPENRrs> {
        TIM1LPEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<'_, APB2LPENRrs> {
        TIM8LPEN_W::new(self, 13)
    }
    ///Bit 14 - USART1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<'_, APB2LPENRrs> {
        USART1LPEN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<'_, APB2LPENRrs> {
        TIM15LPEN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<'_, APB2LPENRrs> {
        TIM16LPEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<'_, APB2LPENRrs> {
        TIM17LPEN_W::new(self, 18)
    }
    ///Bit 19 - SPI4 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<'_, APB2LPENRrs> {
        SPI4LPEN_W::new(self, 19)
    }
    ///Bit 20 - SPI6 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<'_, APB2LPENRrs> {
        SPI6LPEN_W::new(self, 20)
    }
    ///Bit 21 - SAI1 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<'_, APB2LPENRrs> {
        SAI1LPEN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clock enable during sleep mode Set and reset by software.
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<'_, APB2LPENRrs> {
        SAI2LPEN_W::new(self, 22)
    }
    ///Bit 24 - USB clock enable during sleep mode
    #[inline(always)]
    pub fn usblpen(&mut self) -> USBLPEN_W<'_, APB2LPENRrs> {
        USBLPEN_W::new(self, 24)
    }
}
/**RCC APB2 sleep clock register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#RCC:APB2LPENR)*/
pub struct APB2LPENRrs;
impl crate::RegisterSpec for APB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2lpenr::R`](R) reader structure
impl crate::Readable for APB2LPENRrs {}
///`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure
impl crate::Writable for APB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2LPENR to value 0x017f_7800
impl crate::Resettable for APB2LPENRrs {
    const RESET_VALUE: u32 = 0x017f_7800;
}
