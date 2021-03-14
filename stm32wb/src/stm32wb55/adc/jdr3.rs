#[doc = "Reader of register JDR3"]
pub type R = crate::R<u32, super::JDR3>;
#[doc = "Reader of field `JDATA3`"]
pub type JDATA3_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADC group injected sequencer rank 3 conversion data"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new((self.bits & 0xffff) as u16)
    }
}
