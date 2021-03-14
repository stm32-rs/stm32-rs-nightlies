#[doc = "Reader of register SSR"]
pub type R = crate::R<u32, super::SSR>;
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value SS\\[15:0\\]
is the value in the synchronous prescaler counter. The fraction of a second is given by the formula below: Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1) Note: SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by RTC_TR/RTC_DR."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
