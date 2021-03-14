#[doc = "Reader of register CPT2CR"]
pub type R = crate::R<u32, super::CPT2CR>;
#[doc = "Reader of field `CPT2x`"]
pub type CPT2X_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
