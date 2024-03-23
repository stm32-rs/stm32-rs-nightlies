#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<AHB2LPENRrs>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<AHB2LPENRrs>;
#[doc = "GPIOA clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<GPIOALPEN> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIOALPEN` reader - GPIOA clock enable during sleep mode Set and reset by software."]
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN>;
impl GPIOALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GPIOALPEN {
        match self.bits {
            false => GPIOALPEN::Disabled,
            true => GPIOALPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOALPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOALPEN::Enabled
    }
}
#[doc = "Field `GPIOALPEN` writer - GPIOA clock enable during sleep mode Set and reset by software."]
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG, GPIOALPEN>;
impl<'a, REG> GPIOALPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GPIOALPEN::Enabled)
    }
}
#[doc = "Field `GPIOBLPEN` reader - GPIOB clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOBLPEN_R;
#[doc = "Field `GPIOCLPEN` reader - GPIOC clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOCLPEN_R;
#[doc = "Field `GPIODLPEN` reader - GPIOD clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIODLPEN_R;
#[doc = "Field `GPIOELPEN` reader - GPIOE clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOELPEN_R;
#[doc = "Field `GPIOFLPEN` reader - GPIOF clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOFLPEN_R;
#[doc = "Field `GPIOGLPEN` reader - GPIOG clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOGLPEN_R;
#[doc = "Field `GPIOHLPEN` reader - GPIOH clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOHLPEN_R;
#[doc = "Field `GPIOILPEN` reader - GPIOI clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOILPEN_R;
#[doc = "Field `ADC12LPEN` reader - ADC1 and 2 peripherals clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as ADC12LPEN_R;
#[doc = "Field `DAC12LPEN` reader - DAC clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as DAC12LPEN_R;
#[doc = "Field `DCMI_PSSILPEN` reader - digital camera interface clock enable during sleep mode (DCMI or PSSI depending which interface is active) Set and reset by software."]
pub use GPIOALPEN_R as DCMI_PSSILPEN_R;
#[doc = "Field `HASHLPEN` reader - HASH clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as HASHLPEN_R;
#[doc = "Field `RNGLPEN` reader - RNG clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as RNGLPEN_R;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as SRAM2LPEN_R;
#[doc = "Field `SRAM3LPEN` reader - SRAM3 clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_R as SRAM3LPEN_R;
#[doc = "Field `GPIOBLPEN` writer - GPIOB clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOBLPEN_W;
#[doc = "Field `GPIOCLPEN` writer - GPIOC clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOCLPEN_W;
#[doc = "Field `GPIODLPEN` writer - GPIOD clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIODLPEN_W;
#[doc = "Field `GPIOELPEN` writer - GPIOE clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOELPEN_W;
#[doc = "Field `GPIOFLPEN` writer - GPIOF clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOFLPEN_W;
#[doc = "Field `GPIOGLPEN` writer - GPIOG clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOGLPEN_W;
#[doc = "Field `GPIOHLPEN` writer - GPIOH clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOHLPEN_W;
#[doc = "Field `GPIOILPEN` writer - GPIOI clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOILPEN_W;
#[doc = "Field `ADC12LPEN` writer - ADC1 and 2 peripherals clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as ADC12LPEN_W;
#[doc = "Field `DAC12LPEN` writer - DAC clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as DAC12LPEN_W;
#[doc = "Field `DCMI_PSSILPEN` writer - digital camera interface clock enable during sleep mode (DCMI or PSSI depending which interface is active) Set and reset by software."]
pub use GPIOALPEN_W as DCMI_PSSILPEN_W;
#[doc = "Field `HASHLPEN` writer - HASH clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as HASHLPEN_W;
#[doc = "Field `RNGLPEN` writer - RNG clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as RNGLPEN_W;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as SRAM2LPEN_W;
#[doc = "Field `SRAM3LPEN` writer - SRAM3 clock enable during sleep mode Set and reset by software."]
pub use GPIOALPEN_W as SRAM3LPEN_W;
impl R {
    #[doc = "Bit 0 - GPIOA clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOI clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC1 and 2 peripherals clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn dac12lpen(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - digital camera interface clock enable during sleep mode (DCMI or PSSI depending which interface is active) Set and reset by software."]
    #[inline(always)]
    pub fn dcmi_pssilpen(&self) -> DCMI_PSSILPEN_R {
        DCMI_PSSILPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RNG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<AHB2LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOB clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<AHB2LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<AHB2LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIOD clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<AHB2LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOE clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<AHB2LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHB2LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<AHB2LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<AHB2LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOI clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<AHB2LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    #[doc = "Bit 10 - ADC1 and 2 peripherals clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<AHB2LPENRrs> {
        ADC12LPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DAC clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dac12lpen(&mut self) -> DAC12LPEN_W<AHB2LPENRrs> {
        DAC12LPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - digital camera interface clock enable during sleep mode (DCMI or PSSI depending which interface is active) Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssilpen(&mut self) -> DCMI_PSSILPEN_W<AHB2LPENRrs> {
        DCMI_PSSILPEN_W::new(self, 12)
    }
    #[doc = "Bit 17 - HASH clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<AHB2LPENRrs> {
        HASHLPEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RNG clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<AHB2LPENRrs> {
        RNGLPEN_W::new(self, 18)
    }
    #[doc = "Bit 30 - SRAM2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<AHB2LPENRrs> {
        SRAM2LPEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<AHB2LPENRrs> {
        SRAM3LPEN_W::new(self, 31)
    }
}
#[doc = "RCC AHB2 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2LPENRrs;
impl crate::RegisterSpec for AHB2LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2lpenr::R`](R) reader structure"]
impl crate::Readable for AHB2LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure"]
impl crate::Writable for AHB2LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0xc01f_1dff"]
impl crate::Resettable for AHB2LPENRrs {
    const RESET_VALUE: u32 = 0xc01f_1dff;
}
