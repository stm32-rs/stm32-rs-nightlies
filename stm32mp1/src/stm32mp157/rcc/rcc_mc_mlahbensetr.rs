#[doc = "Reader of register RCC_MC_MLAHBENSETR"]
pub type R = crate::R<u32, super::RCC_MC_MLAHBENSETR>;
#[doc = "Writer for register RCC_MC_MLAHBENSETR"]
pub type W = crate::W<u32, super::RCC_MC_MLAHBENSETR>;
#[doc = "Register RCC_MC_MLAHBENSETR `reset()`'s with value 0x10"]
impl crate::ResetValue for super::RCC_MC_MLAHBENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `RETRAMEN`"]
pub type RETRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETRAMEN`"]
pub struct RETRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    pub fn retramen(&self) -> RETRAMEN_R {
        RETRAMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RETRAMEN"]
    #[inline(always)]
    pub fn retramen(&mut self) -> RETRAMEN_W {
        RETRAMEN_W { w: self }
    }
}
