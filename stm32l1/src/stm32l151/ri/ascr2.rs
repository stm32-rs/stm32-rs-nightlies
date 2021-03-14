#[doc = "Reader of register ASCR2"]
pub type R = crate::R<u32, super::ASCR2>;
#[doc = "Writer for register ASCR2"]
pub type W = crate::W<u32, super::ASCR2>;
#[doc = "Register ASCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ASCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GR5_4`"]
pub type GR5_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR5_4`"]
pub struct GR5_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GR5_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `GR6_4`"]
pub type GR6_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR6_4`"]
pub struct GR6_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GR6_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `GR6_3`"]
pub type GR6_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR6_3`"]
pub struct GR6_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR6_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `GR7_7`"]
pub type GR7_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR7_7`"]
pub struct GR7_7_W<'a> {
    w: &'a mut W,
}
impl<'a> GR7_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `GR7_6`"]
pub type GR7_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR7_6`"]
pub struct GR7_6_W<'a> {
    w: &'a mut W,
}
impl<'a> GR7_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `GR7_5`"]
pub type GR7_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR7_5`"]
pub struct GR7_5_W<'a> {
    w: &'a mut W,
}
impl<'a> GR7_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `GR2_5`"]
pub type GR2_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR2_5`"]
pub struct GR2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> GR2_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `GR2_4`"]
pub type GR2_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR2_4`"]
pub struct GR2_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GR2_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `GR2_3`"]
pub type GR2_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR2_3`"]
pub struct GR2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR2_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `GR9_4`"]
pub type GR9_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR9_4`"]
pub struct GR9_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GR9_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `GR9_3`"]
pub type GR9_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR9_3`"]
pub struct GR9_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR9_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `GR3_5`"]
pub type GR3_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR3_5`"]
pub struct GR3_5_W<'a> {
    w: &'a mut W,
}
impl<'a> GR3_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `GR3_4`"]
pub type GR3_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR3_4`"]
pub struct GR3_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GR3_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `GR3_3`"]
pub type GR3_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR3_3`"]
pub struct GR3_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR3_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `GR4_3`"]
pub type GR4_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR4_3`"]
pub struct GR4_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR4_3_W<'a> {
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
#[doc = "Reader of field `GR4_2`"]
pub type GR4_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR4_2`"]
pub struct GR4_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GR4_2_W<'a> {
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
#[doc = "Reader of field `GR4_1`"]
pub type GR4_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR4_1`"]
pub struct GR4_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GR4_1_W<'a> {
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
#[doc = "Reader of field `GR5_3`"]
pub type GR5_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR5_3`"]
pub struct GR5_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR5_3_W<'a> {
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
#[doc = "Reader of field `GR5_2`"]
pub type GR5_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR5_2`"]
pub struct GR5_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GR5_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `GR5_1`"]
pub type GR5_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR5_1`"]
pub struct GR5_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GR5_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GR6_2`"]
pub type GR6_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR6_2`"]
pub struct GR6_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GR6_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `GR6_1`"]
pub type GR6_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR6_1`"]
pub struct GR6_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GR6_1_W<'a> {
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
#[doc = "Reader of field `GR10_4`"]
pub type GR10_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR10_4`"]
pub struct GR10_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GR10_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `GR10_3`"]
pub type GR10_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR10_3`"]
pub struct GR10_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GR10_3_W<'a> {
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
#[doc = "Reader of field `GR10_2`"]
pub type GR10_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR10_2`"]
pub struct GR10_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GR10_2_W<'a> {
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
#[doc = "Reader of field `GR10_1`"]
pub type GR10_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GR10_1`"]
pub struct GR10_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GR10_1_W<'a> {
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
    #[doc = "Bit 29 - GR5_4 analog switch control"]
    #[inline(always)]
    pub fn gr5_4(&self) -> GR5_4_R {
        GR5_4_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GR6_4 analog switch control"]
    #[inline(always)]
    pub fn gr6_4(&self) -> GR6_4_R {
        GR6_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GR6_3 analog switch control"]
    #[inline(always)]
    pub fn gr6_3(&self) -> GR6_3_R {
        GR6_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - GR7_7 analog switch control"]
    #[inline(always)]
    pub fn gr7_7(&self) -> GR7_7_R {
        GR7_7_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GR7_6 analog switch control"]
    #[inline(always)]
    pub fn gr7_6(&self) -> GR7_6_R {
        GR7_6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GR7_5 analog switch control"]
    #[inline(always)]
    pub fn gr7_5(&self) -> GR7_5_R {
        GR7_5_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GR2_5 analog switch control"]
    #[inline(always)]
    pub fn gr2_5(&self) -> GR2_5_R {
        GR2_5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GR2_4 analog switch control"]
    #[inline(always)]
    pub fn gr2_4(&self) -> GR2_4_R {
        GR2_4_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GR2_3 analog switch control"]
    #[inline(always)]
    pub fn gr2_3(&self) -> GR2_3_R {
        GR2_3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GR9_4 analog switch control"]
    #[inline(always)]
    pub fn gr9_4(&self) -> GR9_4_R {
        GR9_4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GR9_3 analog switch control"]
    #[inline(always)]
    pub fn gr9_3(&self) -> GR9_3_R {
        GR9_3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GR3_5 analog switch control"]
    #[inline(always)]
    pub fn gr3_5(&self) -> GR3_5_R {
        GR3_5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GR3_4 analog switch control"]
    #[inline(always)]
    pub fn gr3_4(&self) -> GR3_4_R {
        GR3_4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GR3_3 analog switch control"]
    #[inline(always)]
    pub fn gr3_3(&self) -> GR3_3_R {
        GR3_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GR4_3 analog switch control"]
    #[inline(always)]
    pub fn gr4_3(&self) -> GR4_3_R {
        GR4_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GR4_2 analog switch control"]
    #[inline(always)]
    pub fn gr4_2(&self) -> GR4_2_R {
        GR4_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GR4_1 analog switch control"]
    #[inline(always)]
    pub fn gr4_1(&self) -> GR4_1_R {
        GR4_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GR5_3 analog switch control"]
    #[inline(always)]
    pub fn gr5_3(&self) -> GR5_3_R {
        GR5_3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GR5_2 analog switch control"]
    #[inline(always)]
    pub fn gr5_2(&self) -> GR5_2_R {
        GR5_2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GR5_1 analog switch control"]
    #[inline(always)]
    pub fn gr5_1(&self) -> GR5_1_R {
        GR5_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GR6_2 analog switch control"]
    #[inline(always)]
    pub fn gr6_2(&self) -> GR6_2_R {
        GR6_2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GR6_1 analog switch control"]
    #[inline(always)]
    pub fn gr6_1(&self) -> GR6_1_R {
        GR6_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GR10_4 analog switch control"]
    #[inline(always)]
    pub fn gr10_4(&self) -> GR10_4_R {
        GR10_4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GR10_3 analog switch control"]
    #[inline(always)]
    pub fn gr10_3(&self) -> GR10_3_R {
        GR10_3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GR10_2 analog switch control"]
    #[inline(always)]
    pub fn gr10_2(&self) -> GR10_2_R {
        GR10_2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GR10_1 analog switch control"]
    #[inline(always)]
    pub fn gr10_1(&self) -> GR10_1_R {
        GR10_1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - GR5_4 analog switch control"]
    #[inline(always)]
    pub fn gr5_4(&mut self) -> GR5_4_W {
        GR5_4_W { w: self }
    }
    #[doc = "Bit 28 - GR6_4 analog switch control"]
    #[inline(always)]
    pub fn gr6_4(&mut self) -> GR6_4_W {
        GR6_4_W { w: self }
    }
    #[doc = "Bit 27 - GR6_3 analog switch control"]
    #[inline(always)]
    pub fn gr6_3(&mut self) -> GR6_3_W {
        GR6_3_W { w: self }
    }
    #[doc = "Bit 26 - GR7_7 analog switch control"]
    #[inline(always)]
    pub fn gr7_7(&mut self) -> GR7_7_W {
        GR7_7_W { w: self }
    }
    #[doc = "Bit 25 - GR7_6 analog switch control"]
    #[inline(always)]
    pub fn gr7_6(&mut self) -> GR7_6_W {
        GR7_6_W { w: self }
    }
    #[doc = "Bit 24 - GR7_5 analog switch control"]
    #[inline(always)]
    pub fn gr7_5(&mut self) -> GR7_5_W {
        GR7_5_W { w: self }
    }
    #[doc = "Bit 23 - GR2_5 analog switch control"]
    #[inline(always)]
    pub fn gr2_5(&mut self) -> GR2_5_W {
        GR2_5_W { w: self }
    }
    #[doc = "Bit 22 - GR2_4 analog switch control"]
    #[inline(always)]
    pub fn gr2_4(&mut self) -> GR2_4_W {
        GR2_4_W { w: self }
    }
    #[doc = "Bit 21 - GR2_3 analog switch control"]
    #[inline(always)]
    pub fn gr2_3(&mut self) -> GR2_3_W {
        GR2_3_W { w: self }
    }
    #[doc = "Bit 20 - GR9_4 analog switch control"]
    #[inline(always)]
    pub fn gr9_4(&mut self) -> GR9_4_W {
        GR9_4_W { w: self }
    }
    #[doc = "Bit 19 - GR9_3 analog switch control"]
    #[inline(always)]
    pub fn gr9_3(&mut self) -> GR9_3_W {
        GR9_3_W { w: self }
    }
    #[doc = "Bit 18 - GR3_5 analog switch control"]
    #[inline(always)]
    pub fn gr3_5(&mut self) -> GR3_5_W {
        GR3_5_W { w: self }
    }
    #[doc = "Bit 17 - GR3_4 analog switch control"]
    #[inline(always)]
    pub fn gr3_4(&mut self) -> GR3_4_W {
        GR3_4_W { w: self }
    }
    #[doc = "Bit 16 - GR3_3 analog switch control"]
    #[inline(always)]
    pub fn gr3_3(&mut self) -> GR3_3_W {
        GR3_3_W { w: self }
    }
    #[doc = "Bit 11 - GR4_3 analog switch control"]
    #[inline(always)]
    pub fn gr4_3(&mut self) -> GR4_3_W {
        GR4_3_W { w: self }
    }
    #[doc = "Bit 10 - GR4_2 analog switch control"]
    #[inline(always)]
    pub fn gr4_2(&mut self) -> GR4_2_W {
        GR4_2_W { w: self }
    }
    #[doc = "Bit 9 - GR4_1 analog switch control"]
    #[inline(always)]
    pub fn gr4_1(&mut self) -> GR4_1_W {
        GR4_1_W { w: self }
    }
    #[doc = "Bit 8 - GR5_3 analog switch control"]
    #[inline(always)]
    pub fn gr5_3(&mut self) -> GR5_3_W {
        GR5_3_W { w: self }
    }
    #[doc = "Bit 7 - GR5_2 analog switch control"]
    #[inline(always)]
    pub fn gr5_2(&mut self) -> GR5_2_W {
        GR5_2_W { w: self }
    }
    #[doc = "Bit 6 - GR5_1 analog switch control"]
    #[inline(always)]
    pub fn gr5_1(&mut self) -> GR5_1_W {
        GR5_1_W { w: self }
    }
    #[doc = "Bit 5 - GR6_2 analog switch control"]
    #[inline(always)]
    pub fn gr6_2(&mut self) -> GR6_2_W {
        GR6_2_W { w: self }
    }
    #[doc = "Bit 4 - GR6_1 analog switch control"]
    #[inline(always)]
    pub fn gr6_1(&mut self) -> GR6_1_W {
        GR6_1_W { w: self }
    }
    #[doc = "Bit 3 - GR10_4 analog switch control"]
    #[inline(always)]
    pub fn gr10_4(&mut self) -> GR10_4_W {
        GR10_4_W { w: self }
    }
    #[doc = "Bit 2 - GR10_3 analog switch control"]
    #[inline(always)]
    pub fn gr10_3(&mut self) -> GR10_3_W {
        GR10_3_W { w: self }
    }
    #[doc = "Bit 1 - GR10_2 analog switch control"]
    #[inline(always)]
    pub fn gr10_2(&mut self) -> GR10_2_W {
        GR10_2_W { w: self }
    }
    #[doc = "Bit 0 - GR10_1 analog switch control"]
    #[inline(always)]
    pub fn gr10_1(&mut self) -> GR10_1_W {
        GR10_1_W { w: self }
    }
}
