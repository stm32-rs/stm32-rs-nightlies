///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**TIM1 clock enable

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
///Field `TIM1EN` reader - TIM1 clock enable
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
///Field `TIM1EN` writer - TIM1 clock enable
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
///Field `TIM8EN` reader - TIM8 clock enable
pub use TIM1EN_R as TIM8EN_R;
///Field `USART1EN` reader - USART1 clock enable
pub use TIM1EN_R as USART1EN_R;
///Field `USART6EN` reader - USART6 clock enable
pub use TIM1EN_R as USART6EN_R;
///Field `ADC1EN` reader - ADC1 clock enable
pub use TIM1EN_R as ADC1EN_R;
///Field `ADC2EN` reader - ADC2 clock enable
pub use TIM1EN_R as ADC2EN_R;
///Field `ADC3EN` reader - ADC3 clock enable
pub use TIM1EN_R as ADC3EN_R;
///Field `SDIOEN` reader - SDIO clock enable
pub use TIM1EN_R as SDIOEN_R;
///Field `SPI1EN` reader - SPI1 clock enable
pub use TIM1EN_R as SPI1EN_R;
///Field `SYSCFGEN` reader - System configuration controller clock enable
pub use TIM1EN_R as SYSCFGEN_R;
///Field `TIM9EN` reader - TIM9 clock enable
pub use TIM1EN_R as TIM9EN_R;
///Field `TIM10EN` reader - TIM10 clock enable
pub use TIM1EN_R as TIM10EN_R;
///Field `TIM11EN` reader - TIM11 clock enable
pub use TIM1EN_R as TIM11EN_R;
///Field `TIM8EN` writer - TIM8 clock enable
pub use TIM1EN_W as TIM8EN_W;
///Field `USART1EN` writer - USART1 clock enable
pub use TIM1EN_W as USART1EN_W;
///Field `USART6EN` writer - USART6 clock enable
pub use TIM1EN_W as USART6EN_W;
///Field `ADC1EN` writer - ADC1 clock enable
pub use TIM1EN_W as ADC1EN_W;
///Field `ADC2EN` writer - ADC2 clock enable
pub use TIM1EN_W as ADC2EN_W;
///Field `ADC3EN` writer - ADC3 clock enable
pub use TIM1EN_W as ADC3EN_W;
///Field `SDIOEN` writer - SDIO clock enable
pub use TIM1EN_W as SDIOEN_W;
///Field `SPI1EN` writer - SPI1 clock enable
pub use TIM1EN_W as SPI1EN_W;
///Field `SYSCFGEN` writer - System configuration controller clock enable
pub use TIM1EN_W as SYSCFGEN_W;
///Field `TIM9EN` writer - TIM9 clock enable
pub use TIM1EN_W as TIM9EN_W;
///Field `TIM10EN` writer - TIM10 clock enable
pub use TIM1EN_W as TIM10EN_W;
///Field `TIM11EN` writer - TIM11 clock enable
pub use TIM1EN_W as TIM11EN_W;
impl R {
    ///Bit 0 - TIM1 clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 clock enable
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 clock enable
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - ADC1 clock enable
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC2 clock enable
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC3 clock enable
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SDIO clock enable
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM9 clock enable
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 clock enable
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 clock enable
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("tim1en", &self.tim1en())
            .field("tim11en", &self.tim11en())
            .field("tim10en", &self.tim10en())
            .field("tim9en", &self.tim9en())
            .field("syscfgen", &self.syscfgen())
            .field("spi1en", &self.spi1en())
            .field("sdioen", &self.sdioen())
            .field("adc3en", &self.adc3en())
            .field("adc2en", &self.adc2en())
            .field("adc1en", &self.adc1en())
            .field("usart6en", &self.usart6en())
            .field("usart1en", &self.usart1en())
            .field("tim8en", &self.tim8en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 clock enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APB2ENRrs> {
        TIM1EN_W::new(self, 0)
    }
    ///Bit 1 - TIM8 clock enable
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<'_, APB2ENRrs> {
        TIM8EN_W::new(self, 1)
    }
    ///Bit 4 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 4)
    }
    ///Bit 5 - USART6 clock enable
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W<'_, APB2ENRrs> {
        USART6EN_W::new(self, 5)
    }
    ///Bit 8 - ADC1 clock enable
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W<'_, APB2ENRrs> {
        ADC1EN_W::new(self, 8)
    }
    ///Bit 9 - ADC2 clock enable
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W<'_, APB2ENRrs> {
        ADC2EN_W::new(self, 9)
    }
    ///Bit 10 - ADC3 clock enable
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W<'_, APB2ENRrs> {
        ADC3EN_W::new(self, 10)
    }
    ///Bit 11 - SDIO clock enable
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<'_, APB2ENRrs> {
        SDIOEN_W::new(self, 11)
    }
    ///Bit 12 - SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB2ENRrs> {
        SYSCFGEN_W::new(self, 14)
    }
    ///Bit 16 - TIM9 clock enable
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<'_, APB2ENRrs> {
        TIM9EN_W::new(self, 16)
    }
    ///Bit 17 - TIM10 clock enable
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W<'_, APB2ENRrs> {
        TIM10EN_W::new(self, 17)
    }
    ///Bit 18 - TIM11 clock enable
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W<'_, APB2ENRrs> {
        TIM11EN_W::new(self, 18)
    }
}
/**APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F407.html#RCC:APB2ENR)*/
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
