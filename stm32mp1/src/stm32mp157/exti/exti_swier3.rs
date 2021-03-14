#[doc = "Reader of register EXTI_SWIER3"]
pub type R = crate::R<u32, super::EXTI_SWIER3>;
#[doc = "Writer for register EXTI_SWIER3"]
pub type W = crate::W<u32, super::EXTI_SWIER3>;
#[doc = "Register EXTI_SWIER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_SWIER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWI65`"]
pub type SWI65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI65`"]
pub struct SWI65_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI65_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SWI66`"]
pub type SWI66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI66`"]
pub struct SWI66_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI66_W<'a> {
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
#[doc = "Reader of field `SWI68`"]
pub type SWI68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI68`"]
pub struct SWI68_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI68_W<'a> {
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
#[doc = "Reader of field `SWI73`"]
pub type SWI73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI73`"]
pub struct SWI73_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI73_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SWI74`"]
pub type SWI74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWI74`"]
pub struct SWI74_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI74_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    pub fn swi65(&self) -> SWI65_R {
        SWI65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    pub fn swi66(&self) -> SWI66_R {
        SWI66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    pub fn swi68(&self) -> SWI68_R {
        SWI68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    pub fn swi73(&self) -> SWI73_R {
        SWI73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    pub fn swi74(&self) -> SWI74_R {
        SWI74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    pub fn swi65(&mut self) -> SWI65_W {
        SWI65_W { w: self }
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    pub fn swi66(&mut self) -> SWI66_W {
        SWI66_W { w: self }
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    pub fn swi68(&mut self) -> SWI68_W {
        SWI68_W { w: self }
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    pub fn swi73(&mut self) -> SWI73_W {
        SWI73_W { w: self }
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    pub fn swi74(&mut self) -> SWI74_W {
        SWI74_W { w: self }
    }
}
