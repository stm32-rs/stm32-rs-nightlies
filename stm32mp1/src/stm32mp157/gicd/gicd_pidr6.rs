#[doc = "Reader of register GICD_PIDR6"]
pub type R = crate::R<u32, super::GICD_PIDR6>;
#[doc = "Reader of field `PIDR6`"]
pub type PIDR6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR6"]
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
