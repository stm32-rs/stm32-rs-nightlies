#[doc = "Reader of register CSR2"]
pub type R = crate::R<u32, super::CSR2>;
#[doc = "Writer for register CSR2"]
pub type W = crate::W<u32, super::CSR2>;
#[doc = "Register CSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WUPF1`"]
pub type WUPF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUPF2`"]
pub type WUPF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUPF3`"]
pub type WUPF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUPF4`"]
pub type WUPF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUPF5`"]
pub type WUPF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `WUPF6`"]
pub type WUPF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EWUP1`"]
pub type EWUP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP1`"]
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EWUP2`"]
pub type EWUP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP2`"]
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
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
#[doc = "Reader of field `EWUP3`"]
pub type EWUP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP3`"]
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
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
#[doc = "Reader of field `EWUP4`"]
pub type EWUP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP4`"]
pub struct EWUP4_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `EWUP5`"]
pub type EWUP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP5`"]
pub struct EWUP5_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `EWUP6`"]
pub type EWUP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP6`"]
pub struct EWUP6_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Pin flag for PA0"]
    #[inline(always)]
    pub fn wupf1(&self) -> WUPF1_R {
        WUPF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Pin flag for PA2"]
    #[inline(always)]
    pub fn wupf2(&self) -> WUPF2_R {
        WUPF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Pin flag for PC1"]
    #[inline(always)]
    pub fn wupf3(&self) -> WUPF3_R {
        WUPF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Pin flag for PC13"]
    #[inline(always)]
    pub fn wupf4(&self) -> WUPF4_R {
        WUPF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Pin flag for PI8"]
    #[inline(always)]
    pub fn wupf5(&self) -> WUPF5_R {
        WUPF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Pin flag for PI11"]
    #[inline(always)]
    pub fn wupf6(&self) -> WUPF6_R {
        WUPF6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Wakeup pin for PA0"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Wakeup pin for PA2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Wakeup pin for PC1"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Wakeup pin for PC13"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Wakeup pin for PI8"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Wakeup pin for PI11"]
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable Wakeup pin for PA0"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
    #[doc = "Bit 9 - Enable Wakeup pin for PA2"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    #[doc = "Bit 10 - Enable Wakeup pin for PC1"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    #[doc = "Bit 11 - Enable Wakeup pin for PC13"]
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W {
        EWUP4_W { w: self }
    }
    #[doc = "Bit 12 - Enable Wakeup pin for PI8"]
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W {
        EWUP5_W { w: self }
    }
    #[doc = "Bit 13 - Enable Wakeup pin for PI11"]
    #[inline(always)]
    pub fn ewup6(&mut self) -> EWUP6_W {
        EWUP6_W { w: self }
    }
}
