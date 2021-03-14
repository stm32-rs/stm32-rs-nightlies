#[doc = "Reader of register GICD_PIDR1"]
pub type R = crate::R<u32, super::GICD_PIDR1>;
#[doc = "Reader of field `PIDR1`"]
pub type PIDR1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR1"]
    #[inline(always)]
    pub fn pidr1(&self) -> PIDR1_R {
        PIDR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
