///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**System configuration controller clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - System configuration controller clock enable
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN>;
impl SYSCFGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGEN {
        match self.bits {
            false => SYSCFGEN::Disabled,
            true => SYSCFGEN::Enabled,
        }
    }
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
///Field `SYSCFGEN` writer - System configuration controller clock enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
///Field `TIM9EN` reader - TIM9 timer clock enable
pub use SYSCFGEN_R as TIM9EN_R;
///Field `TIM10EN` reader - TIM10 timer clock enable
pub use SYSCFGEN_R as TIM10EN_R;
///Field `TIM11EN` reader - TIM11 timer clock enable
pub use SYSCFGEN_R as TIM11EN_R;
///Field `ADC1EN` reader - ADC1 interface clock enable
pub use SYSCFGEN_R as ADC1EN_R;
///Field `SDIOEN` reader - SDIO clock enable
pub use SYSCFGEN_R as SDIOEN_R;
///Field `SPI1EN` reader - SPI 1 clock enable
pub use SYSCFGEN_R as SPI1EN_R;
///Field `USART1EN` reader - USART1 clock enable
pub use SYSCFGEN_R as USART1EN_R;
///Field `TIM9EN` writer - TIM9 timer clock enable
pub use SYSCFGEN_W as TIM9EN_W;
///Field `TIM10EN` writer - TIM10 timer clock enable
pub use SYSCFGEN_W as TIM10EN_W;
///Field `TIM11EN` writer - TIM11 timer clock enable
pub use SYSCFGEN_W as TIM11EN_W;
///Field `ADC1EN` writer - ADC1 interface clock enable
pub use SYSCFGEN_W as ADC1EN_W;
///Field `SDIOEN` writer - SDIO clock enable
pub use SYSCFGEN_W as SDIOEN_W;
///Field `SPI1EN` writer - SPI 1 clock enable
pub use SYSCFGEN_W as SPI1EN_W;
///Field `USART1EN` writer - USART1 clock enable
pub use SYSCFGEN_W as USART1EN_W;
impl R {
    ///Bit 0 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - TIM9 timer clock enable
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM10 timer clock enable
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM11 timer clock enable
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - ADC1 interface clock enable
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - SDIO clock enable
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("syscfgen", &self.syscfgen())
            .field("usart1en", &self.usart1en())
            .field("spi1en", &self.spi1en())
            .field("sdioen", &self.sdioen())
            .field("adc1en", &self.adc1en())
            .field("tim11en", &self.tim11en())
            .field("tim10en", &self.tim10en())
            .field("tim9en", &self.tim9en())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration controller clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 2 - TIM9 timer clock enable
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<'_, APB2ENRrs> {
        TIM9EN_W::new(self, 2)
    }
    ///Bit 3 - TIM10 timer clock enable
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W<'_, APB2ENRrs> {
        TIM10EN_W::new(self, 3)
    }
    ///Bit 4 - TIM11 timer clock enable
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W<'_, APB2ENRrs> {
        TIM11EN_W::new(self, 4)
    }
    ///Bit 9 - ADC1 interface clock enable
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W<'_, APB2ENRrs> {
        ADC1EN_W::new(self, 9)
    }
    ///Bit 11 - SDIO clock enable
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<'_, APB2ENRrs> {
        SDIOEN_W::new(self, 11)
    }
    ///Bit 12 - SPI 1 clock enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - USART1 clock enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
}
/**APB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#RCC:APB2ENR)*/
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
