#[doc = "Reader of register LTDC_LCR"]
pub type R = crate::R<u32, super::LTDC_LCR>;
#[doc = "Reader of field `LNBR`"]
pub type LNBR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - LNBR"]
    #[inline(always)]
    pub fn lnbr(&self) -> LNBR_R {
        LNBR_R::new((self.bits & 0xff) as u8)
    }
}
