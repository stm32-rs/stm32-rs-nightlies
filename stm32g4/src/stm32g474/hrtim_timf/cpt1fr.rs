#[doc = "Reader of register CPT1FR"]
pub type R = crate::R<u32, super::CPT1FR>;
#[doc = "Reader of field `CPT1x`"]
pub type CPT1X_R = crate::R<u16, u16>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timerx Capture 1 Direction status"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
