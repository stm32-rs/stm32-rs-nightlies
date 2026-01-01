///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
/**SYSCFG clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<SYSCFGEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGEN` reader - SYSCFG clock enable
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
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN::Enabled
    }
}
///Field `SYSCFGEN` writer - SYSCFG clock enable
pub type SYSCFGEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGEN>;
impl<'a, REG> SYSCFGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGEN::Enabled)
    }
}
///Field `ADCEN` reader - ADC 1 interface clock enable
pub use SYSCFGEN_R as ADCEN_R;
///Field `SPI1EN` reader - SPI 1 clock enable
pub use SYSCFGEN_R as SPI1EN_R;
///Field `USART1EN` reader - USART1 clock enable
pub use SYSCFGEN_R as USART1EN_R;
///Field `TIM15EN` reader - TIM15 timer clock enable
pub use SYSCFGEN_R as TIM15EN_R;
///Field `TIM16EN` reader - TIM16 timer clock enable
pub use SYSCFGEN_R as TIM16EN_R;
///Field `TIM17EN` reader - TIM17 timer clock enable
pub use SYSCFGEN_R as TIM17EN_R;
///Field `TIM19EN` reader - TIM19 timer clock enable
pub use SYSCFGEN_R as TIM19EN_R;
///Field `DBGMCUEN` reader - MCU debug module clock enable
pub use SYSCFGEN_R as DBGMCUEN_R;
///Field `SDADC1EN` reader - SDADC1 (Sigma Delta ADC 1) clock enable
pub use SYSCFGEN_R as SDADC1EN_R;
///Field `SDADC2EN` reader - SDADC2 (Sigma Delta ADC 2) clock enable
pub use SYSCFGEN_R as SDADC2EN_R;
///Field `SDADC3EN` reader - SDADC3 (Sigma Delta ADC 3) clock enable
pub use SYSCFGEN_R as SDADC3EN_R;
///Field `ADCEN` writer - ADC 1 interface clock enable
pub use SYSCFGEN_W as ADCEN_W;
///Field `SPI1EN` writer - SPI 1 clock enable
pub use SYSCFGEN_W as SPI1EN_W;
///Field `USART1EN` writer - USART1 clock enable
pub use SYSCFGEN_W as USART1EN_W;
///Field `TIM15EN` writer - TIM15 timer clock enable
pub use SYSCFGEN_W as TIM15EN_W;
///Field `TIM16EN` writer - TIM16 timer clock enable
pub use SYSCFGEN_W as TIM16EN_W;
///Field `TIM17EN` writer - TIM17 timer clock enable
pub use SYSCFGEN_W as TIM17EN_W;
///Field `TIM19EN` writer - TIM19 timer clock enable
pub use SYSCFGEN_W as TIM19EN_W;
///Field `DBGMCUEN` writer - MCU debug module clock enable
pub use SYSCFGEN_W as DBGMCUEN_W;
///Field `SDADC1EN` writer - SDADC1 (Sigma Delta ADC 1) clock enable
pub use SYSCFGEN_W as SDADC1EN_W;
///Field `SDADC2EN` writer - SDADC2 (Sigma Delta ADC 2) clock enable
pub use SYSCFGEN_W as SDADC2EN_W;
///Field `SDADC3EN` writer - SDADC3 (Sigma Delta ADC 3) clock enable
pub use SYSCFGEN_W as SDADC3EN_W;
impl R {
    ///Bit 0 - SYSCFG clock enable
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - ADC 1 interface clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
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
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM19 timer clock enable
    #[inline(always)]
    pub fn tim19en(&self) -> TIM19EN_R {
        TIM19EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - MCU debug module clock enable
    #[inline(always)]
    pub fn dbgmcuen(&self) -> DBGMCUEN_R {
        DBGMCUEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - SDADC1 (Sigma Delta ADC 1) clock enable
    #[inline(always)]
    pub fn sdadc1en(&self) -> SDADC1EN_R {
        SDADC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SDADC2 (Sigma Delta ADC 2) clock enable
    #[inline(always)]
    pub fn sdadc2en(&self) -> SDADC2EN_R {
        SDADC2EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SDADC3 (Sigma Delta ADC 3) clock enable
    #[inline(always)]
    pub fn sdadc3en(&self) -> SDADC3EN_R {
        SDADC3EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("syscfgen", &self.syscfgen())
            .field("adcen", &self.adcen())
            .field("spi1en", &self.spi1en())
            .field("usart1en", &self.usart1en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("tim19en", &self.tim19en())
            .field("dbgmcuen", &self.dbgmcuen())
            .field("sdadc1en", &self.sdadc1en())
            .field("sdadc2en", &self.sdadc2en())
            .field("sdadc3en", &self.sdadc3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - SYSCFG clock enable
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<'_, APB2ENRrs> {
        SYSCFGEN_W::new(self, 0)
    }
    ///Bit 9 - ADC 1 interface clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<'_, APB2ENRrs> {
        ADCEN_W::new(self, 9)
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
    ///Bit 16 - TIM15 timer clock enable
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<'_, APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 19 - TIM19 timer clock enable
    #[inline(always)]
    pub fn tim19en(&mut self) -> TIM19EN_W<'_, APB2ENRrs> {
        TIM19EN_W::new(self, 19)
    }
    ///Bit 22 - MCU debug module clock enable
    #[inline(always)]
    pub fn dbgmcuen(&mut self) -> DBGMCUEN_W<'_, APB2ENRrs> {
        DBGMCUEN_W::new(self, 22)
    }
    ///Bit 24 - SDADC1 (Sigma Delta ADC 1) clock enable
    #[inline(always)]
    pub fn sdadc1en(&mut self) -> SDADC1EN_W<'_, APB2ENRrs> {
        SDADC1EN_W::new(self, 24)
    }
    ///Bit 25 - SDADC2 (Sigma Delta ADC 2) clock enable
    #[inline(always)]
    pub fn sdadc2en(&mut self) -> SDADC2EN_W<'_, APB2ENRrs> {
        SDADC2EN_W::new(self, 25)
    }
    ///Bit 26 - SDADC3 (Sigma Delta ADC 3) clock enable
    #[inline(always)]
    pub fn sdadc3en(&mut self) -> SDADC3EN_W<'_, APB2ENRrs> {
        SDADC3EN_W::new(self, 26)
    }
}
/**APB2 peripheral clock enable register (RCC_APB2ENR)

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#RCC:APB2ENR)*/
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
