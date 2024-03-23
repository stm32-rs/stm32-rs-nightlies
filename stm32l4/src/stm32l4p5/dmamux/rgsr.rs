#[doc = "Register `RGSR` reader"]
pub type R = crate::R<RGSRrs>;
#[doc = "Trigger overrun event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF0 {
    #[doc = "0: No new trigger event occured on DMA request generator channel x, before the request counter underrun"]
    NoTrigger = 0,
    #[doc = "1: New trigger event occured on DMA request generator channel x, before the request counter underrun"]
    Trigger = 1,
}
impl From<OF0> for bool {
    #[inline(always)]
    fn from(variant: OF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OF0` reader - Trigger overrun event flag"]
pub type OF0_R = crate::BitReader<OF0>;
impl OF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OF0 {
        match self.bits {
            false => OF0::NoTrigger,
            true => OF0::Trigger,
        }
    }
    #[doc = "No new trigger event occured on DMA request generator channel x, before the request counter underrun"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == OF0::NoTrigger
    }
    #[doc = "New trigger event occured on DMA request generator channel x, before the request counter underrun"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OF0::Trigger
    }
}
#[doc = "Field `OF1` reader - Trigger overrun event flag"]
pub use OF0_R as OF1_R;
#[doc = "Field `OF2` reader - Trigger overrun event flag"]
pub use OF0_R as OF2_R;
#[doc = "Field `OF3` reader - Trigger overrun event flag"]
pub use OF0_R as OF3_R;
impl R {
    #[doc = "Bit 0 - Trigger overrun event flag"]
    #[inline(always)]
    pub fn of0(&self) -> OF0_R {
        OF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger overrun event flag"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger overrun event flag"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun event flag"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "request generator interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGSRrs;
impl crate::RegisterSpec for RGSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgsr::R`](R) reader structure"]
impl crate::Readable for RGSRrs {}
#[doc = "`reset()` method sets RGSR to value 0"]
impl crate::Resettable for RGSRrs {
    const RESET_VALUE: u32 = 0;
}
