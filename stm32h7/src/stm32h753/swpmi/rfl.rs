#[doc = "Reader of register RFL"]
pub type R = crate::R<u32, super::RFL>;
#[doc = "Reader of field `RFL`"]
pub type RFL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Receive frame length"]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x1f) as u8)
    }
}
