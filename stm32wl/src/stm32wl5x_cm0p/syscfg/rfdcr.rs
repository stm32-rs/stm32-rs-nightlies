#[doc = "Reader of register RFDCR"]
pub type R = crate::R<u32, super::RFDCR>;
#[doc = "Writer for register RFDCR"]
pub type W = crate::W<u32, super::RFDCR>;
#[doc = "Register RFDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RFDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFTBSEL`"]
pub type RFTBSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFTBSEL`"]
pub struct RFTBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTBSEL_W<'a> {
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
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    pub fn rftbsel(&self) -> RFTBSEL_R {
        RFTBSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - radio debug test bus selection"]
    #[inline(always)]
    pub fn rftbsel(&mut self) -> RFTBSEL_W {
        RFTBSEL_W { w: self }
    }
}
