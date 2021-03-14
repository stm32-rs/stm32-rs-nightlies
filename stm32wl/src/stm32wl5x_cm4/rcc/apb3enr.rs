#[doc = "Reader of register APB3ENR"]
pub type R = crate::R<u32, super::APB3ENR>;
#[doc = "Writer for register APB3ENR"]
pub type W = crate::W<u32, super::APB3ENR>;
#[doc = "Register APB3ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::APB3ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBGHZSPIEN`"]
pub type SUBGHZSPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBGHZSPIEN`"]
pub struct SUBGHZSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPIEN_W<'a> {
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
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable"]
    #[inline(always)]
    pub fn subghzspien(&self) -> SUBGHZSPIEN_R {
        SUBGHZSPIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sub-GHz radio SPI clock enable"]
    #[inline(always)]
    pub fn subghzspien(&mut self) -> SUBGHZSPIEN_W {
        SUBGHZSPIEN_W { w: self }
    }
}
