#[doc = "Reader of register JDR1"]
pub type R = crate::R<u32, super::JDR1>;
#[doc = "Reader of field `JDATA1`"]
pub type JDATA1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 1 conversion data"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
