#[doc = "Reader of register RCC_TIMG2PRER"]
pub type R = crate::R<u32, super::RCC_TIMG2PRER>;
#[doc = "Writer for register RCC_TIMG2PRER"]
pub type W = crate::W<u32, super::RCC_TIMG2PRER>;
#[doc = "Register RCC_TIMG2PRER `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_TIMG2PRER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `TIMG2PRE`"]
pub type TIMG2PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG2PRE`"]
pub struct TIMG2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG2PRE_W<'a> {
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
#[doc = "Reader of field `TIMG2PRERDY`"]
pub type TIMG2PRERDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&self) -> TIMG2PRE_R {
        TIMG2PRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - TIMG2PRERDY"]
    #[inline(always)]
    pub fn timg2prerdy(&self) -> TIMG2PRERDY_R {
        TIMG2PRERDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&mut self) -> TIMG2PRE_W {
        TIMG2PRE_W { w: self }
    }
}
