#[doc = "Reader of register DAC_DOR1"]
pub type R = crate::R<u32, super::DAC_DOR1>;
#[doc = "Reader of field `DACC1DOR`"]
pub type DACC1DOR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
