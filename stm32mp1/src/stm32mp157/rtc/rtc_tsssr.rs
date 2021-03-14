#[doc = "Reader of register RTC_TSSSR"]
pub type R = crate::R<u32, super::RTC_TSSSR>;
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SS"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
