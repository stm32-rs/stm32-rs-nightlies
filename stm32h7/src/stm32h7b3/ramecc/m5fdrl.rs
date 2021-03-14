#[doc = "Reader of register M5FDRL"]
pub type R = crate::R<u32, super::M5FDRL>;
#[doc = "Reader of field `FEC`"]
pub type FEC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
