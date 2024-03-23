#[doc = "Register `AHB4LPENR` reader"]
pub type R = crate::R<AHB4LPENRrs>;
#[doc = "Register `AHB4LPENR` writer"]
pub type W = crate::W<AHB4LPENRrs>;
#[doc = "GPIO peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
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
#[doc = "Field `GPIOALPEN` reader - GPIO peripheral clock enable during CSleep mode"]
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
#[doc = "Field `GPIOALPEN` writer - GPIO peripheral clock enable during CSleep mode"]
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
#[doc = "Field `GPIOBLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOBLPEN_R;
#[doc = "Field `GPIOCLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOCLPEN_R;
#[doc = "Field `GPIODLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIODLPEN_R;
#[doc = "Field `GPIOELPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOELPEN_R;
#[doc = "Field `GPIOFLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOFLPEN_R;
#[doc = "Field `GPIOGLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOGLPEN_R;
#[doc = "Field `GPIOHLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOHLPEN_R;
#[doc = "Field `GPIOILPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOILPEN_R;
#[doc = "Field `GPIOJLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOJLPEN_R;
#[doc = "Field `GPIOKLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as GPIOKLPEN_R;
#[doc = "Field `CRCLPEN` reader - CRC peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_R as CRCLPEN_R;
#[doc = "Field `BDMALPEN` reader - BDMA Clock Enable During CSleep Mode"]
pub use GPIOALPEN_R as BDMALPEN_R;
#[doc = "Field `ADC3LPEN` reader - ADC3 Peripheral Clocks Enable During CSleep Mode"]
pub use GPIOALPEN_R as ADC3LPEN_R;
#[doc = "Field `BKPRAMLPEN` reader - Backup RAM Clock Enable During CSleep Mode"]
pub use GPIOALPEN_R as BKPRAMLPEN_R;
#[doc = "Field `SRAM4LPEN` reader - SRAM4 Clock Enable During CSleep Mode"]
pub use GPIOALPEN_R as SRAM4LPEN_R;
#[doc = "Field `GPIOBLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOBLPEN_W;
#[doc = "Field `GPIOCLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOCLPEN_W;
#[doc = "Field `GPIODLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIODLPEN_W;
#[doc = "Field `GPIOELPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOELPEN_W;
#[doc = "Field `GPIOFLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOFLPEN_W;
#[doc = "Field `GPIOGLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOGLPEN_W;
#[doc = "Field `GPIOHLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOHLPEN_W;
#[doc = "Field `GPIOILPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOILPEN_W;
#[doc = "Field `GPIOJLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOJLPEN_W;
#[doc = "Field `GPIOKLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as GPIOKLPEN_W;
#[doc = "Field `CRCLPEN` writer - CRC peripheral clock enable during CSleep mode"]
pub use GPIOALPEN_W as CRCLPEN_W;
#[doc = "Field `BDMALPEN` writer - BDMA Clock Enable During CSleep Mode"]
pub use GPIOALPEN_W as BDMALPEN_W;
#[doc = "Field `ADC3LPEN` writer - ADC3 Peripheral Clocks Enable During CSleep Mode"]
pub use GPIOALPEN_W as ADC3LPEN_W;
#[doc = "Field `BKPRAMLPEN` writer - Backup RAM Clock Enable During CSleep Mode"]
pub use GPIOALPEN_W as BKPRAMLPEN_W;
#[doc = "Field `SRAM4LPEN` writer - SRAM4 Clock Enable During CSleep Mode"]
pub use GPIOALPEN_W as SRAM4LPEN_W;
impl R {
    #[doc = "Bit 0 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn bdmalpen(&self) -> BDMALPEN_R {
        BDMALPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc3lpen(&self) -> ADC3LPEN_R {
        ADC3LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM4 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram4lpen(&self) -> SRAM4LPEN_R {
        SRAM4LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<AHB4LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<AHB4LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<AHB4LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<AHB4LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<AHB4LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<AHB4LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<AHB4LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<AHB4LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<AHB4LPENRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<AHB4LPENRrs> {
        GPIOJLPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<AHB4LPENRrs> {
        GPIOKLPEN_W::new(self, 10)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<AHB4LPENRrs> {
        CRCLPEN_W::new(self, 19)
    }
    #[doc = "Bit 21 - BDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bdmalpen(&mut self) -> BDMALPEN_W<AHB4LPENRrs> {
        BDMALPEN_W::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc3lpen(&mut self) -> ADC3LPEN_W<AHB4LPENRrs> {
        ADC3LPEN_W::new(self, 24)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<AHB4LPENRrs> {
        BKPRAMLPEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - SRAM4 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram4lpen(&mut self) -> SRAM4LPEN_W<AHB4LPENRrs> {
        SRAM4LPEN_W::new(self, 29)
    }
}
#[doc = "RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4LPENRrs;
impl crate::RegisterSpec for AHB4LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4lpenr::R`](R) reader structure"]
impl crate::Readable for AHB4LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`ahb4lpenr::W`](W) writer structure"]
impl crate::Writable for AHB4LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB4LPENR to value 0"]
impl crate::Resettable for AHB4LPENRrs {
    const RESET_VALUE: u32 = 0;
}
