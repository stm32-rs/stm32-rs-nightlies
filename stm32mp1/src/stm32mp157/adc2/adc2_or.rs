#[doc = "Reader of register ADC2_OR"]
pub type R = crate::R<u32, super::ADC2_OR>;
#[doc = "Writer for register ADC2_OR"]
pub type W = crate::W<u32, super::ADC2_OR>;
#[doc = "Register ADC2_OR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC2_OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VDDCOREEN`"]
pub type VDDCOREEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDCOREEN`"]
pub struct VDDCOREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCOREEN_W<'a> {
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
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    pub fn vddcoreen(&self) -> VDDCOREEN_R {
        VDDCOREEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    pub fn vddcoreen(&mut self) -> VDDCOREEN_W {
        VDDCOREEN_W { w: self }
    }
}
