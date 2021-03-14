#[doc = "Reader of register DDRPERFM_IER"]
pub type R = crate::R<u32, super::DDRPERFM_IER>;
#[doc = "Writer for register DDRPERFM_IER"]
pub type W = crate::W<u32, super::DDRPERFM_IER>;
#[doc = "Register DDRPERFM_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPERFM_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVFIE`"]
pub type OVFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVFIE`"]
pub struct OVFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    pub fn ovfie(&self) -> OVFIE_R {
        OVFIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OVFIE"]
    #[inline(always)]
    pub fn ovfie(&mut self) -> OVFIE_W {
        OVFIE_W { w: self }
    }
}
