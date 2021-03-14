#[doc = "Reader of register GICH_ELSR0"]
pub type R = crate::R<u32, super::GICH_ELSR0>;
#[doc = "Reader of field `ELSR0`"]
pub type ELSR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELSR0"]
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
