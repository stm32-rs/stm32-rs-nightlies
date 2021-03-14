#[doc = "Reader of register JDR1"]
pub type R = crate::R<u32, super::JDR1>;
#[doc = "Reader of field `JDATA1`"]
pub type JDATA1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA1"]
    #[inline(always)]
    pub fn jdata1(&self) -> JDATA1_R {
        JDATA1_R::new((self.bits & 0xffff) as u16)
    }
}
