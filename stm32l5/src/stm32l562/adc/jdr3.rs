#[doc = "Reader of register JDR3"]
pub type R = crate::R<u32, super::JDR3>;
#[doc = "Reader of field `JDATA`"]
pub type JDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA3"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
