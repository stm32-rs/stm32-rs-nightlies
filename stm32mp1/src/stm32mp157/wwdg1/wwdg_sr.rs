#[doc = "Reader of register WWDG_SR"]
pub type R = crate::R<u32, super::WWDG_SR>;
#[doc = "Writer for register WWDG_SR"]
pub type W = crate::W<u32, super::WWDG_SR>;
#[doc = "Register WWDG_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::WWDG_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EWIF`"]
pub type EWIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWIF`"]
pub struct EWIF_W<'a> {
    w: &'a mut W,
}
impl<'a> EWIF_W<'a> {
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
    #[doc = "Bit 0 - EWIF"]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWIF"]
    #[inline(always)]
    pub fn ewif(&mut self) -> EWIF_W {
        EWIF_W { w: self }
    }
}
