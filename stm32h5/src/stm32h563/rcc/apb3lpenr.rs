#[doc = "Register `APB3LPENR` reader"]
pub type R = crate::R<APB3LPENRrs>;
#[doc = "Register `APB3LPENR` writer"]
pub type W = crate::W<APB3LPENRrs>;
#[doc = "SBS clock enable during sleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSLPEN {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<SBSLPEN> for bool {
    #[inline(always)]
    fn from(variant: SBSLPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSLPEN` reader - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_R = crate::BitReader<SBSLPEN>;
impl SBSLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBSLPEN {
        match self.bits {
            false => SBSLPEN::Disabled,
            true => SBSLPEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBSLPEN::Disabled
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBSLPEN::Enabled
    }
}
#[doc = "Field `SBSLPEN` writer - SBS clock enable during sleep mode Set and reset by software."]
pub type SBSLPEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSLPEN>;
impl<'a, REG> SBSLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSLPEN::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSLPEN::Enabled)
    }
}
#[doc = "Field `SPI5LPEN` reader - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
pub use SBSLPEN_R as SPI5LPEN_R;
#[doc = "Field `LPUART1LPEN` reader - LPUART1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPUART1LPEN_R;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as I2C3LPEN_R;
#[doc = "Field `I2C4LPEN` reader - I2C4 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as I2C4LPEN_R;
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPTIM1LPEN_R;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPTIM3LPEN_R;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPTIM4LPEN_R;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPTIM5LPEN_R;
#[doc = "Field `LPTIM6LPEN` reader - LPTIM6 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPTIM6LPEN_R;
#[doc = "Field `VREFLPEN` reader - VREF clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as VREFLPEN_R;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as RTCAPBLPEN_R;
#[doc = "Field `SPI5LPEN` writer - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
pub use SBSLPEN_W as SPI5LPEN_W;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPUART1LPEN_W;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as I2C3LPEN_W;
#[doc = "Field `I2C4LPEN` writer - I2C4 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as I2C4LPEN_W;
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPTIM1LPEN_W;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPTIM3LPEN_W;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPTIM4LPEN_W;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPTIM5LPEN_W;
#[doc = "Field `LPTIM6LPEN` writer - LPTIM6 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPTIM6LPEN_W;
#[doc = "Field `VREFLPEN` writer - VREF clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as VREFLPEN_W;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as RTCAPBLPEN_W;
impl R {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn sbslpen(&self) -> SBSLPEN_R {
        SBSLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim6lpen(&self) -> LPTIM6LPEN_R {
        LPTIM6LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbslpen(&mut self) -> SBSLPEN_W<APB3LPENRrs> {
        SBSLPEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - SPI5 clock enable during Slsleepeep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<APB3LPENRrs> {
        SPI5LPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<APB3LPENRrs> {
        LPUART1LPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<APB3LPENRrs> {
        I2C3LPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - I2C4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<APB3LPENRrs> {
        I2C4LPEN_W::new(self, 8)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<APB3LPENRrs> {
        LPTIM1LPEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM3 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<APB3LPENRrs> {
        LPTIM3LPEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - LPTIM4 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<APB3LPENRrs> {
        LPTIM4LPEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - LPTIM5 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<APB3LPENRrs> {
        LPTIM5LPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - LPTIM6 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim6lpen(&mut self) -> LPTIM6LPEN_W<APB3LPENRrs> {
        LPTIM6LPEN_W::new(self, 15)
    }
    #[doc = "Bit 20 - VREF clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<APB3LPENRrs> {
        VREFLPEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<APB3LPENRrs> {
        RTCAPBLPEN_W::new(self, 21)
    }
}
#[doc = "RCC APB4 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3LPENRrs;
impl crate::RegisterSpec for APB3LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3lpenr::R`](R) reader structure"]
impl crate::Readable for APB3LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3lpenr::W`](W) writer structure"]
impl crate::Writable for APB3LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3LPENR to value 0x0030_f9e2"]
impl crate::Resettable for APB3LPENRrs {
    const RESET_VALUE: u32 = 0x0030_f9e2;
}
