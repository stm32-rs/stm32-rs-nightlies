#[doc = "Reader of register CRRCR"]
pub type R = crate::R<u32, super::CRRCR>;
#[doc = "Reader of field `HSI48CAL`"]
pub type HSI48CAL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Internal RC 48 MHz clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new((self.bits & 0x03ff) as u16)
    }
}
