///Register `AHB2ENR` reader
pub type R = crate::R<AHB2ENRrs>;
///Register `AHB2ENR` writer
pub type W = crate::W<AHB2ENRrs>;
/**IO port A clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
///Field `GPIOBEN` reader - IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - IO port D clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - IO port E clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - IO port F clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - IO port G clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `ADC12EN` reader - ADC12 clock enable
pub use GPIOAEN_R as ADC12EN_R;
///Field `ADC345EN` reader - ADC345 clock enable
pub use GPIOAEN_R as ADC345EN_R;
///Field `DAC1EN` reader - DAC1 clock enable
pub use GPIOAEN_R as DAC1EN_R;
///Field `DAC2EN` reader - DAC2 clock enable
pub use GPIOAEN_R as DAC2EN_R;
///Field `DAC3EN` reader - DAC3 clock enable
pub use GPIOAEN_R as DAC3EN_R;
///Field `DAC4EN` reader - DAC4 clock enable
pub use GPIOAEN_R as DAC4EN_R;
///Field `AESEN` reader - AES clock enable
pub use GPIOAEN_R as AESEN_R;
///Field `RNGEN` reader - RNG enable
pub use GPIOAEN_R as RNGEN_R;
///Field `GPIOBEN` writer - IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - IO port D clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - IO port E clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - IO port F clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - IO port G clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `ADC12EN` writer - ADC12 clock enable
pub use GPIOAEN_W as ADC12EN_W;
///Field `ADC345EN` writer - ADC345 clock enable
pub use GPIOAEN_W as ADC345EN_W;
///Field `DAC1EN` writer - DAC1 clock enable
pub use GPIOAEN_W as DAC1EN_W;
///Field `DAC2EN` writer - DAC2 clock enable
pub use GPIOAEN_W as DAC2EN_W;
///Field `DAC3EN` writer - DAC3 clock enable
pub use GPIOAEN_W as DAC3EN_W;
///Field `DAC4EN` writer - DAC4 clock enable
pub use GPIOAEN_W as DAC4EN_W;
///Field `AESEN` writer - AES clock enable
pub use GPIOAEN_W as AESEN_W;
///Field `RNGEN` writer - RNG enable
pub use GPIOAEN_W as RNGEN_W;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - ADC12 clock enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC345 clock enable
    #[inline(always)]
    pub fn adc345en(&self) -> ADC345EN_R {
        ADC345EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DAC1 clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC2 clock enable
    #[inline(always)]
    pub fn dac2en(&self) -> DAC2EN_R {
        DAC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DAC3 clock enable
    #[inline(always)]
    pub fn dac3en(&self) -> DAC3EN_R {
        DAC3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DAC4 clock enable
    #[inline(always)]
    pub fn dac4en(&self) -> DAC4EN_R {
        DAC4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - AES clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - RNG enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2ENR")
            .field("gpioaen", &self.gpioaen())
            .field("gpioben", &self.gpioben())
            .field("gpiocen", &self.gpiocen())
            .field("gpioden", &self.gpioden())
            .field("gpioeen", &self.gpioeen())
            .field("gpiofen", &self.gpiofen())
            .field("gpiogen", &self.gpiogen())
            .field("adc12en", &self.adc12en())
            .field("adc345en", &self.adc345en())
            .field("dac1en", &self.dac1en())
            .field("dac2en", &self.dac2en())
            .field("dac3en", &self.dac3en())
            .field("dac4en", &self.dac4en())
            .field("aesen", &self.aesen())
            .field("rngen", &self.rngen())
            .finish()
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<'_, AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<'_, AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<'_, AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<'_, AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<'_, AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<'_, AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<'_, AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    ///Bit 13 - ADC12 clock enable
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<'_, AHB2ENRrs> {
        ADC12EN_W::new(self, 13)
    }
    ///Bit 14 - ADC345 clock enable
    #[inline(always)]
    pub fn adc345en(&mut self) -> ADC345EN_W<'_, AHB2ENRrs> {
        ADC345EN_W::new(self, 14)
    }
    ///Bit 16 - DAC1 clock enable
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<'_, AHB2ENRrs> {
        DAC1EN_W::new(self, 16)
    }
    ///Bit 17 - DAC2 clock enable
    #[inline(always)]
    pub fn dac2en(&mut self) -> DAC2EN_W<'_, AHB2ENRrs> {
        DAC2EN_W::new(self, 17)
    }
    ///Bit 18 - DAC3 clock enable
    #[inline(always)]
    pub fn dac3en(&mut self) -> DAC3EN_W<'_, AHB2ENRrs> {
        DAC3EN_W::new(self, 18)
    }
    ///Bit 19 - DAC4 clock enable
    #[inline(always)]
    pub fn dac4en(&mut self) -> DAC4EN_W<'_, AHB2ENRrs> {
        DAC4EN_W::new(self, 19)
    }
    ///Bit 24 - AES clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<'_, AHB2ENRrs> {
        AESEN_W::new(self, 24)
    }
    ///Bit 26 - RNG enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<'_, AHB2ENRrs> {
        RNGEN_W::new(self, 26)
    }
}
/**AHB2 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431.html#RCC:AHB2ENR)*/
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb2enr::R`](R) reader structure
impl crate::Readable for AHB2ENRrs {}
///`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENRrs {}
