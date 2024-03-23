#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<AHB2ENRrs>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<AHB2ENRrs>;
#[doc = "GPIOA clock enable Set and reset by software.\n\nValue on reset: 0"]
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
#[doc = "Field `GPIOAEN` reader - GPIOA clock enable Set and reset by software."]
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
#[doc = "Field `GPIOAEN` writer - GPIOA clock enable Set and reset by software."]
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
#[doc = "Field `GPIOBEN` reader - GPIOB clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - GPIOC clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - GPIOD clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - GPIOE clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - GPIOF clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - GPIOG clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - GPIOH clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOIEN` reader - GPIOI clock enable Set and reset by software."]
pub use GPIOAEN_R as GPIOIEN_R;
#[doc = "Field `ADC12EN` reader - ADC1 and 2 peripherals clock enabled Set and reset by software."]
pub use GPIOAEN_R as ADC12EN_R;
#[doc = "Field `DAC12EN` reader - DAC clock enable Set and reset by software."]
pub use GPIOAEN_R as DAC12EN_R;
#[doc = "Field `DCMI_PSSIEN` reader - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software."]
pub use GPIOAEN_R as DCMI_PSSIEN_R;
#[doc = "Field `HASHEN` reader - HASH clock enable Set and reset by software."]
pub use GPIOAEN_R as HASHEN_R;
#[doc = "Field `RNGEN` reader - RNG clock enable Set and reset by software."]
pub use GPIOAEN_R as RNGEN_R;
#[doc = "Field `SRAM3EN` reader - SRAM3 clock enable Set and reset by software."]
pub use GPIOAEN_R as SRAM3EN_R;
#[doc = "Field `SRAM2EN` reader - SRAM2 clock enable Set and reset by software."]
pub use GPIOAEN_R as SRAM2EN_R;
#[doc = "Field `GPIOBEN` writer - GPIOB clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - GPIOC clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - GPIOD clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - GPIOE clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - GPIOF clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - GPIOG clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - GPIOH clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOIEN` writer - GPIOI clock enable Set and reset by software."]
pub use GPIOAEN_W as GPIOIEN_W;
#[doc = "Field `ADC12EN` writer - ADC1 and 2 peripherals clock enabled Set and reset by software."]
pub use GPIOAEN_W as ADC12EN_W;
#[doc = "Field `DAC12EN` writer - DAC clock enable Set and reset by software."]
pub use GPIOAEN_W as DAC12EN_W;
#[doc = "Field `DCMI_PSSIEN` writer - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software."]
pub use GPIOAEN_W as DCMI_PSSIEN_W;
#[doc = "Field `HASHEN` writer - HASH clock enable Set and reset by software."]
pub use GPIOAEN_W as HASHEN_W;
#[doc = "Field `RNGEN` writer - RNG clock enable Set and reset by software."]
pub use GPIOAEN_W as RNGEN_W;
#[doc = "Field `SRAM3EN` writer - SRAM3 clock enable Set and reset by software."]
pub use GPIOAEN_W as SRAM3EN_W;
#[doc = "Field `SRAM2EN` writer - SRAM2 clock enable Set and reset by software."]
pub use GPIOAEN_W as SRAM2EN_W;
impl R {
    #[doc = "Bit 0 - GPIOA clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOI clock enable Set and reset by software."]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 and 2 peripherals clock enabled Set and reset by software."]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssien(&self) -> DCMI_PSSIEN_R {
        DCMI_PSSIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable Set and reset by software."]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM3 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB2ENRrs> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB2ENRrs> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB2ENRrs> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB2ENRrs> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOE clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB2ENRrs> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB2ENRrs> {
        GPIOFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB2ENRrs> {
        GPIOGEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOH clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB2ENRrs> {
        GPIOHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOI clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<AHB2ENRrs> {
        GPIOIEN_W::new(self, 8)
    }
    #[doc = "Bit 10 - ADC1 and 2 peripherals clock enabled Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<AHB2ENRrs> {
        ADC12EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac12en(&mut self) -> DAC12EN_W<AHB2ENRrs> {
        DAC12EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - digital camera interface clock enable (DCMI or PSSI depending which interface is active) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssien(&mut self) -> DCMI_PSSIEN_W<AHB2ENRrs> {
        DCMI_PSSIEN_W::new(self, 12)
    }
    #[doc = "Bit 17 - HASH clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<AHB2ENRrs> {
        HASHEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB2ENRrs> {
        RNGEN_W::new(self, 18)
    }
    #[doc = "Bit 30 - SRAM3 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<AHB2ENRrs> {
        SRAM3EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<AHB2ENRrs> {
        SRAM2EN_W::new(self, 31)
    }
}
#[doc = "RCC AHB2 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets AHB2ENR to value 0xc000_0000"]
impl crate::Resettable for AHB2ENRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
