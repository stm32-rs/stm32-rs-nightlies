#[doc = "Reader of register ADC_JDR3"]
pub type R = crate::R<u32, super::ADC_JDR3>;
#[doc = "Reader of field `JDATA`"]
pub type JDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
