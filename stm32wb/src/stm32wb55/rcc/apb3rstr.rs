#[doc = "Reader of register APB3RSTR"]
pub type R = crate::R<u32, super::APB3RSTR>;
#[doc = "Writer for register APB3RSTR"]
pub type W = crate::W<u32, super::APB3RSTR>;
#[doc = "Register APB3RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB3RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFRST`"]
pub type RFRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFRST`"]
pub struct RFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRST_W<'a> {
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
    #[doc = "Bit 0 - Radio system BLE reset"]
    #[inline(always)]
    pub fn rfrst(&self) -> RFRST_R {
        RFRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Radio system BLE reset"]
    #[inline(always)]
    pub fn rfrst(&mut self) -> RFRST_W {
        RFRST_W { w: self }
    }
}
