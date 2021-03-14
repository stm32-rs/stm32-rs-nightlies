#[doc = "Reader of register JDR2"]
pub type R = crate::R<u32, super::JDR2>;
#[doc = "Reader of field `JDATA2`"]
pub type JDATA2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group injected sequencer rank 2 conversion data"]
    #[inline(always)]
    pub fn jdata2(&self) -> JDATA2_R {
        JDATA2_R::new((self.bits & 0xffff) as u16)
    }
}
