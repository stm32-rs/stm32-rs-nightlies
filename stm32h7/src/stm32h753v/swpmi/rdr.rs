#[doc = "Reader of register RDR"]
pub type R = crate::R<u32, super::RDR>;
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - received data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
