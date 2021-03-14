#[doc = "Reader of register HDP_VAL"]
pub type R = crate::R<u32, super::HDP_VAL>;
#[doc = "Reader of field `HDPVAL`"]
pub type HDPVAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - HDPVAL"]
    #[inline(always)]
    pub fn hdpval(&self) -> HDPVAL_R {
        HDPVAL_R::new((self.bits & 0xff) as u8)
    }
}
