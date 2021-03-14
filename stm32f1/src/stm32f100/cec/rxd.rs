#[doc = "Reader of register RXD"]
pub type R = crate::R<u32, super::RXD>;
#[doc = "Reader of field `RXD`"]
pub type RXD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Rx data"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
