#[doc = "Register `C1_APB4LPENR` reader"]
pub type R = crate::R<C1_APB4LPENRrs>;
#[doc = "Register `C1_APB4LPENR` writer"]
pub type W = crate::W<C1_APB4LPENRrs>;
#[doc = "SYSCFG peripheral clock enable during CSleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGLPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<SYSCFGLPEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode"]
pub type SYSCFGLPEN_R = crate::BitReader<SYSCFGLPEN>;
impl SYSCFGLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGLPEN {
        match self.bits {
            false => SYSCFGLPEN::Disabled,
            true => SYSCFGLPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGLPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGLPEN::Enabled
    }
}
#[doc = "Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode"]
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGLPEN>;
impl<'a, REG> SYSCFGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGLPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGLPEN::Enabled)
    }
}
#[doc = "Field `LPUART1LPEN` reader - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as LPUART1LPEN_R;
#[doc = "Field `SPI6LPEN` reader - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as SPI6LPEN_R;
#[doc = "Field `I2C4LPEN` reader - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as I2C4LPEN_R;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as LPTIM2LPEN_R;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as LPTIM3LPEN_R;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as LPTIM4LPEN_R;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as LPTIM5LPEN_R;
#[doc = "Field `COMP12LPEN` reader - COMP1/2 peripheral clock enable during CSleep mode"]
pub use SYSCFGLPEN_R as COMP12LPEN_R;
#[doc = "Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode"]
pub use SYSCFGLPEN_R as VREFLPEN_R;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB Clock Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as RTCAPBLPEN_R;
#[doc = "Field `SAI4LPEN` reader - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_R as SAI4LPEN_R;
#[doc = "Field `DTSLPEN` reader - Digital temperature sensor block enable during CSleep Mode"]
pub use SYSCFGLPEN_R as DTSLPEN_R;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as LPUART1LPEN_W;
#[doc = "Field `SPI6LPEN` writer - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as SPI6LPEN_W;
#[doc = "Field `I2C4LPEN` writer - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as I2C4LPEN_W;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as LPTIM2LPEN_W;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as LPTIM3LPEN_W;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as LPTIM4LPEN_W;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as LPTIM5LPEN_W;
#[doc = "Field `COMP12LPEN` writer - COMP1/2 peripheral clock enable during CSleep mode"]
pub use SYSCFGLPEN_W as COMP12LPEN_W;
#[doc = "Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode"]
pub use SYSCFGLPEN_W as VREFLPEN_W;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB Clock Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as RTCAPBLPEN_W;
#[doc = "Field `SAI4LPEN` writer - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub use SYSCFGLPEN_W as SAI4LPEN_W;
#[doc = "Field `DTSLPEN` writer - Digital temperature sensor block enable during CSleep Mode"]
pub use SYSCFGLPEN_W as DTSLPEN_W;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - Digital temperature sensor block enable during CSleep Mode"]
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<C1_APB4LPENRrs> {
        SYSCFGLPEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<C1_APB4LPENRrs> {
        LPUART1LPEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<C1_APB4LPENRrs> {
        SPI6LPEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<C1_APB4LPENRrs> {
        I2C4LPEN_W::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<C1_APB4LPENRrs> {
        LPTIM2LPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<C1_APB4LPENRrs> {
        LPTIM3LPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<C1_APB4LPENRrs> {
        LPTIM4LPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<C1_APB4LPENRrs> {
        LPTIM5LPEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W<C1_APB4LPENRrs> {
        COMP12LPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<C1_APB4LPENRrs> {
        VREFLPEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<C1_APB4LPENRrs> {
        RTCAPBLPEN_W::new(self, 16)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<C1_APB4LPENRrs> {
        SAI4LPEN_W::new(self, 21)
    }
    #[doc = "Bit 26 - Digital temperature sensor block enable during CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<C1_APB4LPENRrs> {
        DTSLPEN_W::new(self, 26)
    }
}
#[doc = "RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb4lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb4lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_APB4LPENRrs;
impl crate::RegisterSpec for C1_APB4LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb4lpenr::R`](R) reader structure"]
impl crate::Readable for C1_APB4LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`c1_apb4lpenr::W`](W) writer structure"]
impl crate::Writable for C1_APB4LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1_APB4LPENR to value 0x0421_deaa"]
impl crate::Resettable for C1_APB4LPENRrs {
    const RESET_VALUE: u32 = 0x0421_deaa;
}
