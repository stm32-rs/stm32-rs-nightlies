#[doc = "Reader of register DIVH"]
pub type R = crate::R<u32, super::DIVH>;
#[doc = "Reader of field `DIVH`"]
pub type DIVH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - RTC prescaler divider register high"]
    #[inline(always)]
    pub fn divh(&self) -> DIVH_R {
        DIVH_R::new((self.bits & 0x0f) as u8)
    }
}
