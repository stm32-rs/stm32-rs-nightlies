#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<GPIOAEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN>;
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOAEN {
        match self.bits {
            false => GPIOAEN::Disabled,
            true => GPIOAEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOAEN>;
impl<'a, REG> GPIOAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOAEN::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `ADC12EN` reader - ADC clock enable"]
pub use GPIOAEN_R as ADC12EN_R;
#[doc = "Field `ADC345EN` reader - DCMI clock enable"]
pub use GPIOAEN_R as ADC345EN_R;
#[doc = "Field `DAC1EN` reader - AES accelerator clock enable"]
pub use GPIOAEN_R as DAC1EN_R;
#[doc = "Field `DAC2EN` reader - HASH clock enable"]
pub use GPIOAEN_R as DAC2EN_R;
#[doc = "Field `DAC3EN` reader - Random Number Generator clock enable"]
pub use GPIOAEN_R as DAC3EN_R;
#[doc = "Field `DAC4EN` reader - DAC4 clock enable"]
pub use GPIOAEN_R as DAC4EN_R;
#[doc = "Field `AESEN` reader - AES clock enable"]
pub use GPIOAEN_R as AESEN_R;
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub use GPIOAEN_R as RNGEN_R;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `ADC12EN` writer - ADC clock enable"]
pub use GPIOAEN_W as ADC12EN_W;
#[doc = "Field `ADC345EN` writer - DCMI clock enable"]
pub use GPIOAEN_W as ADC345EN_W;
#[doc = "Field `DAC1EN` writer - AES accelerator clock enable"]
pub use GPIOAEN_W as DAC1EN_W;
#[doc = "Field `DAC2EN` writer - HASH clock enable"]
pub use GPIOAEN_W as DAC2EN_W;
#[doc = "Field `DAC3EN` writer - Random Number Generator clock enable"]
pub use GPIOAEN_W as DAC3EN_W;
#[doc = "Field `DAC4EN` writer - DAC4 clock enable"]
pub use GPIOAEN_W as DAC4EN_W;
#[doc = "Field `AESEN` writer - AES clock enable"]
pub use GPIOAEN_W as AESEN_W;
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub use GPIOAEN_W as RNGEN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn adc345en(&self) -> ADC345EN_R {
        ADC345EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn dac2en(&self) -> DAC2EN_R {
        DAC2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn dac3en(&self) -> DAC3EN_R {
        DAC3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DAC4 clock enable"]
    #[inline(always)]
    pub fn dac4en(&self) -> DAC4EN_R {
        DAC4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - AES clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<AHB2ENRrs> {
        ADC12EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc345en(&mut self) -> ADC345EN_W<AHB2ENRrs> {
        ADC345EN_W::new(self, 14)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac1en(&mut self) -> DAC1EN_W<AHB2ENRrs> {
        DAC1EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac2en(&mut self) -> DAC2EN_W<AHB2ENRrs> {
        DAC2EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac3en(&mut self) -> DAC3EN_W<AHB2ENRrs> {
        DAC3EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - DAC4 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac4en(&mut self) -> DAC4EN_W<AHB2ENRrs> {
        DAC4EN_W::new(self, 19)
    }
    #[doc = "Bit 24 - AES clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<AHB2ENRrs> {
        AESEN_W::new(self, 24)
    }
    #[doc = "Bit 26 - Random Number Generator clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENRrs> {
        RNGEN_W::new(self, 26)
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2ENRrs;
impl crate::RegisterSpec for AHB2ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for AHB2ENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for AHB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
