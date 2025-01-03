///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**masked interrupt status of channel x (x = 7 to 0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS0R {
    ///0: No interrupt has occurred on channel
    NoTrigger = 0,
    ///1: An interrupt has occurred on channel
    Trigger = 1,
}
impl From<MIS0R> for bool {
    #[inline(always)]
    fn from(variant: MIS0R) -> Self {
        variant as u8 != 0
    }
}
///Field `MIS0` reader - masked interrupt status of channel x (x = 7 to 0)
pub type MIS0_R = crate::BitReader<MIS0R>;
impl MIS0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MIS0R {
        match self.bits {
            false => MIS0R::NoTrigger,
            true => MIS0R::Trigger,
        }
    }
    ///No interrupt has occurred on channel
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == MIS0R::NoTrigger
    }
    ///An interrupt has occurred on channel
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MIS0R::Trigger
    }
}
///Field `MIS1` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS1_R;
///Field `MIS2` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS2_R;
///Field `MIS3` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS3_R;
///Field `MIS4` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS4_R;
///Field `MIS5` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS5_R;
///Field `MIS6` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS6_R;
///Field `MIS7` reader - masked interrupt status of channel x (x = 7 to 0)
pub use MIS0_R as MIS7_R;
impl R {
    ///Bit 0 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis4(&self) -> MIS4_R {
        MIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis5(&self) -> MIS5_R {
        MIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis6(&self) -> MIS6_R {
        MIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - masked interrupt status of channel x (x = 7 to 0)
    #[inline(always)]
    pub fn mis7(&self) -> MIS7_R {
        MIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("mis0", &self.mis0())
            .field("mis1", &self.mis1())
            .field("mis2", &self.mis2())
            .field("mis3", &self.mis3())
            .field("mis4", &self.mis4())
            .field("mis5", &self.mis5())
            .field("mis6", &self.mis6())
            .field("mis7", &self.mis7())
            .finish()
    }
}
/**GPDMA non-secure masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#GPDMA1:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
