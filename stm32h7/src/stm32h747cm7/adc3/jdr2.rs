#[doc = "Reader of register JDR2"]
pub type R = crate::R<u32, super::JDR2>;
#[doc = "Reader of field `JDATA2`"]
pub type JDATA2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 2 conversion data"]
    #[inline(always)]
    pub fn jdata2(&self) -> JDATA2_R {
        JDATA2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
