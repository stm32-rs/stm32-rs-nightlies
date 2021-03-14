#[doc = "Reader of register APB3SMENR"]
pub type R = crate::R<u32, super::APB3SMENR>;
#[doc = "Writer for register APB3SMENR"]
pub type W = crate::W<u32, super::APB3SMENR>;
#[doc = "Register APB3SMENR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::APB3SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SUBGHZSPISMEN`"]
pub type SUBGHZSPISMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBGHZSPISMEN`"]
pub struct SUBGHZSPISMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPISMEN_W<'a> {
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
    #[doc = "Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn subghzspismen(&self) -> SUBGHZSPISMEN_R {
        SUBGHZSPISMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub-GHz radio SPI clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn subghzspismen(&mut self) -> SUBGHZSPISMEN_W {
        SUBGHZSPISMEN_W { w: self }
    }
}
