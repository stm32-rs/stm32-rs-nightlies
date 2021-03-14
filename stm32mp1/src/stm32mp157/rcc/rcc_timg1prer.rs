#[doc = "Reader of register RCC_TIMG1PRER"]
pub type R = crate::R<u32, super::RCC_TIMG1PRER>;
#[doc = "Writer for register RCC_TIMG1PRER"]
pub type W = crate::W<u32, super::RCC_TIMG1PRER>;
#[doc = "Register RCC_TIMG1PRER `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_TIMG1PRER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `TIMG1PRE`"]
pub type TIMG1PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMG1PRE`"]
pub struct TIMG1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG1PRE_W<'a> {
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
#[doc = "Reader of field `TIMG1PRERDY`"]
pub type TIMG1PRERDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIMG1PRE"]
    #[inline(always)]
    pub fn timg1pre(&self) -> TIMG1PRE_R {
        TIMG1PRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - TIMG1PRERDY"]
    #[inline(always)]
    pub fn timg1prerdy(&self) -> TIMG1PRERDY_R {
        TIMG1PRERDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG1PRE"]
    #[inline(always)]
    pub fn timg1pre(&mut self) -> TIMG1PRE_W {
        TIMG1PRE_W { w: self }
    }
}
