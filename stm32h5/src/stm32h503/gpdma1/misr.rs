#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "masked interrupt status of channel x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIS0R {
    #[doc = "0: No interrupt has occurred on channel"]
    NoTrigger = 0,
    #[doc = "1: An interrupt has occurred on channel"]
    Trigger = 1,
}
impl From<MIS0R> for bool {
    #[inline(always)]
    fn from(variant: MIS0R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS0` reader - masked interrupt status of channel x"]
pub type MIS0_R = crate::BitReader<MIS0R>;
impl MIS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIS0R {
        match self.bits {
            false => MIS0R::NoTrigger,
            true => MIS0R::Trigger,
        }
    }
    #[doc = "No interrupt has occurred on channel"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == MIS0R::NoTrigger
    }
    #[doc = "An interrupt has occurred on channel"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MIS0R::Trigger
    }
}
#[doc = "Field `MIS1` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS1_R;
#[doc = "Field `MIS2` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS2_R;
#[doc = "Field `MIS3` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS3_R;
#[doc = "Field `MIS4` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS4_R;
#[doc = "Field `MIS5` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS5_R;
#[doc = "Field `MIS6` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS6_R;
#[doc = "Field `MIS7` reader - masked interrupt status of channel x"]
pub use MIS0_R as MIS7_R;
impl R {
    #[doc = "Bit 0 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis0(&self) -> MIS0_R {
        MIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis1(&self) -> MIS1_R {
        MIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis2(&self) -> MIS2_R {
        MIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis3(&self) -> MIS3_R {
        MIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis4(&self) -> MIS4_R {
        MIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis5(&self) -> MIS5_R {
        MIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis6(&self) -> MIS6_R {
        MIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - masked interrupt status of channel x"]
    #[inline(always)]
    pub fn mis7(&self) -> MIS7_R {
        MIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "GPDMA masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
