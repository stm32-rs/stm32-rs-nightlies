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
#[doc = "Field `LPUART1LPEN` reader - LPUART1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPUART1LPEN_R;
#[doc = "Field `I3C2LPEN` reader - I3C2 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as I3C2LPEN_R;
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as LPTIM1LPEN_R;
#[doc = "Field `VREFLPEN` reader - VREF clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as VREFLPEN_R;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB interface clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_R as RTCAPBLPEN_R;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPUART1LPEN_W;
#[doc = "Field `I3C2LPEN` writer - I3C2 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as I3C2LPEN_W;
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 clock enable during sleep mode Set and reset by software."]
pub use SBSLPEN_W as LPTIM1LPEN_W;
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
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn i3c2lpen(&self) -> I3C2LPEN_R {
        I3C2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 11) & 1) != 0)
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
    #[doc = "Bit 6 - LPUART1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<APB3LPENRrs> {
        LPUART1LPEN_W::new(self, 6)
    }
    #[doc = "Bit 9 - I3C2 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c2lpen(&mut self) -> I3C2LPEN_W<APB3LPENRrs> {
        I3C2LPEN_W::new(self, 9)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable during sleep mode Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<APB3LPENRrs> {
        LPTIM1LPEN_W::new(self, 11)
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
#[doc = "RCC APB3 sleep clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets APB3LPENR to value 0x0030_fa42"]
impl crate::Resettable for APB3LPENRrs {
    const RESET_VALUE: u32 = 0x0030_fa42;
}
