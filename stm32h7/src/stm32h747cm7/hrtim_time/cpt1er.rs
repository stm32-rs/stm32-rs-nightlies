#[doc = "Reader of register CPT1ER"]
pub type R = crate::R<u32, super::CPT1ER>;
#[doc = "Reader of field `CPT1x`"]
pub type CPT1X_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
}
