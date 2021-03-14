#[doc = "Reader of register DIVL"]
pub type R = crate::R<u32, super::DIVL>;
#[doc = "Reader of field `DIVL`"]
pub type DIVL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC prescaler divider register Low"]
    #[inline(always)]
    pub fn divl(&self) -> DIVL_R {
        DIVL_R::new((self.bits & 0xffff) as u16)
    }
}
