#[doc = "Reader of register GICD_PIDR4"]
pub type R = crate::R<u32, super::GICD_PIDR4>;
#[doc = "Reader of field `PIDR4`"]
pub type PIDR4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR4"]
    #[inline(always)]
    pub fn pidr4(&self) -> PIDR4_R {
        PIDR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
