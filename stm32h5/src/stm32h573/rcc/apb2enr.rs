///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**TIM1 clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM1EN> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM1EN` reader - TIM1 clock enable Set and reset by software.
pub type TIM1EN_R = crate::BitReader<TIM1EN>;
impl TIM1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1EN {
        match self.bits {
            false => TIM1EN::Disabled,
            true => TIM1EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN::Enabled
    }
}
///Field `TIM1EN` writer - TIM1 clock enable Set and reset by software.
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM1EN>;
impl<'a, REG> TIM1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1EN::Enabled)
    }
}
///Field `SPI1EN` reader - SPI1 clock enable Set and reset by software.
pub use TIM1EN_R as SPI1EN_R;
///Field `TIM8EN` reader - TIM8 clock enable Set and reset by software.
pub use TIM1EN_R as TIM8EN_R;
///Field `USART1EN` reader - USART1 clock enable Set and reset by software.
pub use TIM1EN_R as USART1EN_R;
///Field `TIM15EN` reader - TIM15 clock enable Set and reset by software.
pub use TIM1EN_R as TIM15EN_R;
///Field `TIM16EN` reader - TIM16 clock enable Set and reset by software.
pub use TIM1EN_R as TIM16EN_R;
///Field `TIM17EN` reader - TIM17 clock enable Set and reset by software.
pub use TIM1EN_R as TIM17EN_R;
///Field `SPI4EN` reader - SPI4 clock enable Set and reset by software.
pub use TIM1EN_R as SPI4EN_R;
///Field `SPI6EN` reader - SPI6 clock enable Set and reset by software.
pub use TIM1EN_R as SPI6EN_R;
///Field `SAI1EN` reader - SAI1 clock enable Set and reset by software.
pub use TIM1EN_R as SAI1EN_R;
///Field `SAI2EN` reader - SAI2 clock enable Set and cleared by software.
pub use TIM1EN_R as SAI2EN_R;
///Field `USBEN` reader - USB clock enable
pub use TIM1EN_R as USBEN_R;
///Field `SPI1EN` writer - SPI1 clock enable Set and reset by software.
pub use TIM1EN_W as SPI1EN_W;
///Field `TIM8EN` writer - TIM8 clock enable Set and reset by software.
pub use TIM1EN_W as TIM8EN_W;
///Field `USART1EN` writer - USART1 clock enable Set and reset by software.
pub use TIM1EN_W as USART1EN_W;
///Field `TIM15EN` writer - TIM15 clock enable Set and reset by software.
pub use TIM1EN_W as TIM15EN_W;
///Field `TIM16EN` writer - TIM16 clock enable Set and reset by software.
pub use TIM1EN_W as TIM16EN_W;
///Field `TIM17EN` writer - TIM17 clock enable Set and reset by software.
pub use TIM1EN_W as TIM17EN_W;
///Field `SPI4EN` writer - SPI4 clock enable Set and reset by software.
pub use TIM1EN_W as SPI4EN_W;
///Field `SPI6EN` writer - SPI6 clock enable Set and reset by software.
pub use TIM1EN_W as SPI6EN_W;
///Field `SAI1EN` writer - SAI1 clock enable Set and reset by software.
pub use TIM1EN_W as SAI1EN_W;
///Field `SAI2EN` writer - SAI2 clock enable Set and cleared by software.
pub use TIM1EN_W as SAI2EN_W;
///Field `USBEN` writer - USB clock enable
pub use TIM1EN_W as USBEN_W;
impl R {
    ///Bit 11 - TIM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SPI4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPI6 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SAI1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - USB clock enable
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("tim1en", &self.tim1en())
            .field("spi1en", &self.spi1en())
            .field("tim8en", &self.tim8en())
            .field("usart1en", &self.usart1en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("spi4en", &self.spi4en())
            .field("spi6en", &self.spi6en())
            .field("sai1en", &self.sai1en())
            .field("sai2en", &self.sai2en())
            .field("usben", &self.usben())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 13 - TIM8 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<'_, APB2ENRrs> {
        TIM8EN_W::new(self, 13)
    }
    ///Bit 14 - USART1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 16 - TIM15 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<'_, APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 clock enable Set and reset by software.
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 19 - SPI4 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi4en(&mut self) -> SPI4EN_W<'_, APB2ENRrs> {
        SPI4EN_W::new(self, 19)
    }
    ///Bit 20 - SPI6 clock enable Set and reset by software.
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W<'_, APB2ENRrs> {
        SPI6EN_W::new(self, 20)
    }
    ///Bit 21 - SAI1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<'_, APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W<'_, APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
    ///Bit 24 - USB clock enable
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<'_, APB2ENRrs> {
        USBEN_W::new(self, 24)
    }
}
/**RCC APB2 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
