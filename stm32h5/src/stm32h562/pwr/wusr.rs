///Register `WUSR` reader
pub type R = crate::R<WUSRrs>;
/**wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1R {
    ///0: No wakeup event occurred
    NoEventOccurred = 0,
    ///1: A wakeup event received from WUFx pin
    EventOccurred = 1,
}
impl From<WUF1R> for bool {
    #[inline(always)]
    fn from(variant: WUF1R) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF1` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub type WUF1_R = crate::BitReader<WUF1R>;
impl WUF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF1R {
        match self.bits {
            false => WUF1R::NoEventOccurred,
            true => WUF1R::EventOccurred,
        }
    }
    ///No wakeup event occurred
    #[inline(always)]
    pub fn is_no_event_occurred(&self) -> bool {
        *self == WUF1R::NoEventOccurred
    }
    ///A wakeup event received from WUFx pin
    #[inline(always)]
    pub fn is_event_occurred(&self) -> bool {
        *self == WUF1R::EventOccurred
    }
}
///Field `WUF2` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF2_R;
///Field `WUF3` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF3_R;
///Field `WUF4` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF4_R;
///Field `WUF5` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF5_R;
///Field `WUF6` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF6_R;
///Field `WUF7` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF7_R;
///Field `WUF8` reader - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
pub use WUF1_R as WUF8_R;
impl R {
    ///Bit 0 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUSR")
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("wuf6", &self.wuf6())
            .field("wuf7", &self.wuf7())
            .field("wuf8", &self.wuf8())
            .finish()
    }
}
/**PWR wakeup status register

You can [`read`](crate::Reg::read) this register and get [`wusr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#PWR:WUSR)*/
pub struct WUSRrs;
impl crate::RegisterSpec for WUSRrs {
    type Ux = u32;
}
///`read()` method returns [`wusr::R`](R) reader structure
impl crate::Readable for WUSRrs {}
///`reset()` method sets WUSR to value 0
impl crate::Resettable for WUSRrs {}
