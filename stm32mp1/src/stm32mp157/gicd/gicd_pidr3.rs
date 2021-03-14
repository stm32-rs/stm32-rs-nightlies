#[doc = "Reader of register GICD_PIDR3"]
pub type R = crate::R<u32, super::GICD_PIDR3>;
#[doc = "Reader of field `PIDR3`"]
pub type PIDR3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR3"]
    #[inline(always)]
    pub fn pidr3(&self) -> PIDR3_R {
        PIDR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
