///Register `RGSR` reader
pub type R = crate::R<RGSRrs>;
/**Trigger overrun event flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF0 {
    ///0: No new trigger event occured on DMA request generator channel x, before the request counter underrun
    NoTrigger = 0,
    ///1: New trigger event occured on DMA request generator channel x, before the request counter underrun
    Trigger = 1,
}
impl From<OF0> for bool {
    #[inline(always)]
    fn from(variant: OF0) -> Self {
        variant as u8 != 0
    }
}
///Field `OF0` reader - Trigger overrun event flag
pub type OF0_R = crate::BitReader<OF0>;
impl OF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OF0 {
        match self.bits {
            false => OF0::NoTrigger,
            true => OF0::Trigger,
        }
    }
    ///No new trigger event occured on DMA request generator channel x, before the request counter underrun
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == OF0::NoTrigger
    }
    ///New trigger event occured on DMA request generator channel x, before the request counter underrun
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OF0::Trigger
    }
}
///Field `OF1` reader - Trigger overrun event flag
pub use OF0_R as OF1_R;
///Field `OF2` reader - Trigger overrun event flag
pub use OF0_R as OF2_R;
///Field `OF3` reader - Trigger overrun event flag
pub use OF0_R as OF3_R;
impl R {
    ///Bit 0 - Trigger overrun event flag
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Trigger overrun event flag
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Trigger overrun event flag
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Trigger overrun event flag
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGSR")
            .field("of0", &self.of0())
            .field("of3", &self.of3())
            .field("of2", &self.of2())
            .field("of1", &self.of1())
            .finish()
    }
}
/**request generator interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#DMAMUX:RGSR)*/
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
///`read()` method returns [`rgsr::R`](R) reader structure
impl crate::Readable for RGSRrs {}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSRrs {
    const RESET_VALUE: u32 = 0;
}
