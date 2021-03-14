#[doc = "Reader of register RCC_MP_SREQCLRR"]
pub type R = crate::R<u32, super::RCC_MP_SREQCLRR>;
#[doc = "Writer for register RCC_MP_SREQCLRR"]
pub type W = crate::W<u32, super::RCC_MP_SREQCLRR>;
#[doc = "Register RCC_MP_SREQCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_SREQCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STPREQ_P0`"]
pub type STPREQ_P0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPREQ_P0`"]
pub struct STPREQ_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> STPREQ_P0_W<'a> {
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
#[doc = "Reader of field `STPREQ_P1`"]
pub type STPREQ_P1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPREQ_P1`"]
pub struct STPREQ_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> STPREQ_P1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&self) -> STPREQ_P0_R {
        STPREQ_P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&self) -> STPREQ_P1_R {
        STPREQ_P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&mut self) -> STPREQ_P0_W {
        STPREQ_P0_W { w: self }
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&mut self) -> STPREQ_P1_W {
        STPREQ_P1_W { w: self }
    }
}
