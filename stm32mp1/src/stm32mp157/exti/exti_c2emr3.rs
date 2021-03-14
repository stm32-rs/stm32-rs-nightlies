#[doc = "Reader of register EXTI_C2EMR3"]
pub type R = crate::R<u32, super::EXTI_C2EMR3>;
#[doc = "Writer for register EXTI_C2EMR3"]
pub type W = crate::W<u32, super::EXTI_C2EMR3>;
#[doc = "Register EXTI_C2EMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_C2EMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM66`"]
pub type EM66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM66`"]
pub struct EM66_W<'a> {
    w: &'a mut W,
}
impl<'a> EM66_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - EM66"]
    #[inline(always)]
    pub fn em66(&self) -> EM66_R {
        EM66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - EM66"]
    #[inline(always)]
    pub fn em66(&mut self) -> EM66_W {
        EM66_W { w: self }
    }
}
