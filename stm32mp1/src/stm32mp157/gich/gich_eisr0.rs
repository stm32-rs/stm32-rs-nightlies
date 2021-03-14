#[doc = "Reader of register GICH_EISR0"]
pub type R = crate::R<u32, super::GICH_EISR0>;
#[doc = "Reader of field `EISR0`"]
pub type EISR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EISR0"]
    #[inline(always)]
    pub fn eisr0(&self) -> EISR0_R {
        EISR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
