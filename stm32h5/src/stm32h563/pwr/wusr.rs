#[doc = "Register `WUSR` reader"]
pub type R = crate::R<WUSRrs>;
#[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1R {
    #[doc = "0: No wakeup event occurred"]
    NoEventOccurred = 0,
    #[doc = "1: A wakeup event received from WUFx pin"]
    EventOccurred = 1,
}
impl From<WUF1R> for bool {
    #[inline(always)]
    fn from(variant: WUF1R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF1` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub type WUF1_R = crate::BitReader<WUF1R>;
impl WUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF1R {
        match self.bits {
            false => WUF1R::NoEventOccurred,
            true => WUF1R::EventOccurred,
        }
    }
    #[doc = "No wakeup event occurred"]
    #[inline(always)]
    pub fn is_no_event_occurred(&self) -> bool {
        *self == WUF1R::NoEventOccurred
    }
    #[doc = "A wakeup event received from WUFx pin"]
    #[inline(always)]
    pub fn is_event_occurred(&self) -> bool {
        *self == WUF1R::EventOccurred
    }
}
#[doc = "Field `WUF2` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF2_R;
#[doc = "Field `WUF3` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF3_R;
#[doc = "Field `WUF4` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF4_R;
#[doc = "Field `WUF5` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF5_R;
#[doc = "Field `WUF6` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF6_R;
#[doc = "Field `WUF7` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF7_R;
#[doc = "Field `WUF8` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
pub use WUF1_R as WUF8_R;
impl R {
    #[doc = "Bit 0 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PWR wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wusr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUSRrs;
impl crate::RegisterSpec for WUSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wusr::R`](R) reader structure"]
impl crate::Readable for WUSRrs {}
#[doc = "`reset()` method sets WUSR to value 0"]
impl crate::Resettable for WUSRrs {
    const RESET_VALUE: u32 = 0;
}
