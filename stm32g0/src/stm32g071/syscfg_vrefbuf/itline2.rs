#[doc = "Reader of register ITLINE2"]
pub type R = crate::R<u32, super::ITLINE2>;
#[doc = "Reader of field `TAMP`"]
pub type TAMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TAMP"]
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
