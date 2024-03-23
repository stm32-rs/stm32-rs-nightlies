#[doc = "Register `APB3ENR` reader"]
pub type R = crate::R<APB3ENRrs>;
#[doc = "Register `APB3ENR` writer"]
pub type W = crate::W<APB3ENRrs>;
#[doc = "SBS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBSEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<SBSEN> for bool {
    #[inline(always)]
    fn from(variant: SBSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBSEN` reader - SBS clock enable Set and reset by software."]
pub type SBSEN_R = crate::BitReader<SBSEN>;
impl SBSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBSEN {
        match self.bits {
            false => SBSEN::Disabled,
            true => SBSEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBSEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBSEN::Enabled
    }
}
#[doc = "Field `SBSEN` writer - SBS clock enable Set and reset by software."]
pub type SBSEN_W<'a, REG> = crate::BitWriter<'a, REG, SBSEN>;
impl<'a, REG> SBSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBSEN::Enabled)
    }
}
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable Set and reset by software."]
pub use SBSEN_R as LPUART1EN_R;
#[doc = "Field `I3C2EN` reader - I3C2EN clock enable Set and reset by software."]
pub use SBSEN_R as I3C2EN_R;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable Set and reset by software."]
pub use SBSEN_R as LPTIM1EN_R;
#[doc = "Field `VREFEN` reader - VREF clock enable Set and reset by software."]
pub use SBSEN_R as VREFEN_R;
#[doc = "Field `RTCAPBEN` reader - RTC APB interface clock enable Set and reset by software."]
pub use SBSEN_R as RTCAPBEN_R;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable Set and reset by software."]
pub use SBSEN_W as LPUART1EN_W;
#[doc = "Field `I3C2EN` writer - I3C2EN clock enable Set and reset by software."]
pub use SBSEN_W as I3C2EN_W;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable Set and reset by software."]
pub use SBSEN_W as LPTIM1EN_W;
#[doc = "Field `VREFEN` writer - VREF clock enable Set and reset by software."]
pub use SBSEN_W as VREFEN_W;
#[doc = "Field `RTCAPBEN` writer - RTC APB interface clock enable Set and reset by software."]
pub use SBSEN_W as RTCAPBEN_W;
impl R {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn sbsen(&self) -> SBSEN_R {
        SBSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2EN clock enable Set and reset by software."]
    #[inline(always)]
    pub fn i3c2en(&self) -> I3C2EN_R {
        I3C2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsen(&mut self) -> SBSEN_W<APB3ENRrs> {
        SBSEN_W::new(self, 1)
    }
    #[doc = "Bit 6 - LPUART1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB3ENRrs> {
        LPUART1EN_W::new(self, 6)
    }
    #[doc = "Bit 9 - I3C2EN clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c2en(&mut self) -> I3C2EN_W<APB3ENRrs> {
        I3C2EN_W::new(self, 9)
    }
    #[doc = "Bit 11 - LPTIM1 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB3ENRrs> {
        LPTIM1EN_W::new(self, 11)
    }
    #[doc = "Bit 20 - VREF clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<APB3ENRrs> {
        VREFEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - RTC APB interface clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB3ENRrs> {
        RTCAPBEN_W::new(self, 21)
    }
}
#[doc = "RCC APB3 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3ENRrs;
impl crate::RegisterSpec for APB3ENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3enr::R`](R) reader structure"]
impl crate::Readable for APB3ENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3enr::W`](W) writer structure"]
impl crate::Writable for APB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3ENR to value 0"]
impl crate::Resettable for APB3ENRrs {
    const RESET_VALUE: u32 = 0;
}
