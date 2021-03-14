#[doc = "Reader of register JDR4"]
pub type R = crate::R<u32, super::JDR4>;
#[doc = "Reader of field `JDATA4`"]
pub type JDATA4_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - JDATA4"]
    #[inline(always)]
    pub fn jdata4(&self) -> JDATA4_R {
        JDATA4_R::new((self.bits & 0xffff) as u16)
    }
}
