#[doc = "Reader of register CPT2FR"]
pub type R = crate::R<u32, super::CPT2FR>;
#[doc = "Reader of field `CPT2x`"]
pub type CPT2X_R = crate::R<u16, u16>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timerx Capture 1 Direction status"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
