#[doc = "Reader of register FR2"]
pub type R = crate::R<u32, super::FR2>;
#[doc = "Writer for register FR2"]
pub type W = crate::W<u32, super::FR2>;
#[doc = "Register FR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FB0`"]
pub type FB0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB0`"]
pub struct FB0_W<'a> {
    w: &'a mut W,
}
impl<'a> FB0_W<'a> {
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
#[doc = "Reader of field `FB1`"]
pub type FB1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB1`"]
pub struct FB1_W<'a> {
    w: &'a mut W,
}
impl<'a> FB1_W<'a> {
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
#[doc = "Reader of field `FB2`"]
pub type FB2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB2`"]
pub struct FB2_W<'a> {
    w: &'a mut W,
}
impl<'a> FB2_W<'a> {
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
#[doc = "Reader of field `FB3`"]
pub type FB3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB3`"]
pub struct FB3_W<'a> {
    w: &'a mut W,
}
impl<'a> FB3_W<'a> {
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
#[doc = "Reader of field `FB4`"]
pub type FB4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB4`"]
pub struct FB4_W<'a> {
    w: &'a mut W,
}
impl<'a> FB4_W<'a> {
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
#[doc = "Reader of field `FB5`"]
pub type FB5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB5`"]
pub struct FB5_W<'a> {
    w: &'a mut W,
}
impl<'a> FB5_W<'a> {
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
#[doc = "Reader of field `FB6`"]
pub type FB6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB6`"]
pub struct FB6_W<'a> {
    w: &'a mut W,
}
impl<'a> FB6_W<'a> {
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
#[doc = "Reader of field `FB7`"]
pub type FB7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB7`"]
pub struct FB7_W<'a> {
    w: &'a mut W,
}
impl<'a> FB7_W<'a> {
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
#[doc = "Reader of field `FB8`"]
pub type FB8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB8`"]
pub struct FB8_W<'a> {
    w: &'a mut W,
}
impl<'a> FB8_W<'a> {
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
#[doc = "Reader of field `FB9`"]
pub type FB9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB9`"]
pub struct FB9_W<'a> {
    w: &'a mut W,
}
impl<'a> FB9_W<'a> {
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
#[doc = "Reader of field `FB10`"]
pub type FB10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB10`"]
pub struct FB10_W<'a> {
    w: &'a mut W,
}
impl<'a> FB10_W<'a> {
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
#[doc = "Reader of field `FB11`"]
pub type FB11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB11`"]
pub struct FB11_W<'a> {
    w: &'a mut W,
}
impl<'a> FB11_W<'a> {
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
#[doc = "Reader of field `FB12`"]
pub type FB12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB12`"]
pub struct FB12_W<'a> {
    w: &'a mut W,
}
impl<'a> FB12_W<'a> {
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
#[doc = "Reader of field `FB13`"]
pub type FB13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB13`"]
pub struct FB13_W<'a> {
    w: &'a mut W,
}
impl<'a> FB13_W<'a> {
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
#[doc = "Reader of field `FB14`"]
pub type FB14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB14`"]
pub struct FB14_W<'a> {
    w: &'a mut W,
}
impl<'a> FB14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FB15`"]
pub type FB15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB15`"]
pub struct FB15_W<'a> {
    w: &'a mut W,
}
impl<'a> FB15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FB16`"]
pub type FB16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB16`"]
pub struct FB16_W<'a> {
    w: &'a mut W,
}
impl<'a> FB16_W<'a> {
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
#[doc = "Reader of field `FB17`"]
pub type FB17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB17`"]
pub struct FB17_W<'a> {
    w: &'a mut W,
}
impl<'a> FB17_W<'a> {
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
#[doc = "Reader of field `FB18`"]
pub type FB18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB18`"]
pub struct FB18_W<'a> {
    w: &'a mut W,
}
impl<'a> FB18_W<'a> {
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
#[doc = "Reader of field `FB19`"]
pub type FB19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB19`"]
pub struct FB19_W<'a> {
    w: &'a mut W,
}
impl<'a> FB19_W<'a> {
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
#[doc = "Reader of field `FB20`"]
pub type FB20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB20`"]
pub struct FB20_W<'a> {
    w: &'a mut W,
}
impl<'a> FB20_W<'a> {
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
#[doc = "Reader of field `FB21`"]
pub type FB21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB21`"]
pub struct FB21_W<'a> {
    w: &'a mut W,
}
impl<'a> FB21_W<'a> {
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
#[doc = "Reader of field `FB22`"]
pub type FB22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB22`"]
pub struct FB22_W<'a> {
    w: &'a mut W,
}
impl<'a> FB22_W<'a> {
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
#[doc = "Reader of field `FB23`"]
pub type FB23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB23`"]
pub struct FB23_W<'a> {
    w: &'a mut W,
}
impl<'a> FB23_W<'a> {
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
#[doc = "Reader of field `FB24`"]
pub type FB24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB24`"]
pub struct FB24_W<'a> {
    w: &'a mut W,
}
impl<'a> FB24_W<'a> {
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
#[doc = "Reader of field `FB25`"]
pub type FB25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB25`"]
pub struct FB25_W<'a> {
    w: &'a mut W,
}
impl<'a> FB25_W<'a> {
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
#[doc = "Reader of field `FB26`"]
pub type FB26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB26`"]
pub struct FB26_W<'a> {
    w: &'a mut W,
}
impl<'a> FB26_W<'a> {
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
#[doc = "Reader of field `FB27`"]
pub type FB27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB27`"]
pub struct FB27_W<'a> {
    w: &'a mut W,
}
impl<'a> FB27_W<'a> {
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
#[doc = "Reader of field `FB28`"]
pub type FB28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB28`"]
pub struct FB28_W<'a> {
    w: &'a mut W,
}
impl<'a> FB28_W<'a> {
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
#[doc = "Reader of field `FB29`"]
pub type FB29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB29`"]
pub struct FB29_W<'a> {
    w: &'a mut W,
}
impl<'a> FB29_W<'a> {
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
#[doc = "Reader of field `FB30`"]
pub type FB30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB30`"]
pub struct FB30_W<'a> {
    w: &'a mut W,
}
impl<'a> FB30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `FB31`"]
pub type FB31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB31`"]
pub struct FB31_W<'a> {
    w: &'a mut W,
}
impl<'a> FB31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&self) -> FB0_R {
        FB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&self) -> FB1_R {
        FB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&self) -> FB2_R {
        FB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&self) -> FB3_R {
        FB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&self) -> FB4_R {
        FB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&self) -> FB5_R {
        FB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&self) -> FB6_R {
        FB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&self) -> FB7_R {
        FB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&self) -> FB8_R {
        FB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&self) -> FB9_R {
        FB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&self) -> FB10_R {
        FB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&self) -> FB11_R {
        FB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&self) -> FB12_R {
        FB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&self) -> FB13_R {
        FB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&self) -> FB14_R {
        FB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&self) -> FB15_R {
        FB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&self) -> FB16_R {
        FB16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&self) -> FB17_R {
        FB17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&self) -> FB18_R {
        FB18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&self) -> FB19_R {
        FB19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&self) -> FB20_R {
        FB20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&self) -> FB21_R {
        FB21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&self) -> FB22_R {
        FB22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&self) -> FB23_R {
        FB23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&self) -> FB24_R {
        FB24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&self) -> FB25_R {
        FB25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&self) -> FB26_R {
        FB26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&self) -> FB27_R {
        FB27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&self) -> FB28_R {
        FB28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&self) -> FB29_R {
        FB29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&self) -> FB30_R {
        FB30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&self) -> FB31_R {
        FB31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&mut self) -> FB0_W {
        FB0_W { w: self }
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&mut self) -> FB1_W {
        FB1_W { w: self }
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&mut self) -> FB2_W {
        FB2_W { w: self }
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&mut self) -> FB3_W {
        FB3_W { w: self }
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&mut self) -> FB4_W {
        FB4_W { w: self }
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&mut self) -> FB5_W {
        FB5_W { w: self }
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&mut self) -> FB6_W {
        FB6_W { w: self }
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&mut self) -> FB7_W {
        FB7_W { w: self }
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&mut self) -> FB8_W {
        FB8_W { w: self }
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&mut self) -> FB9_W {
        FB9_W { w: self }
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&mut self) -> FB10_W {
        FB10_W { w: self }
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&mut self) -> FB11_W {
        FB11_W { w: self }
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&mut self) -> FB12_W {
        FB12_W { w: self }
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&mut self) -> FB13_W {
        FB13_W { w: self }
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&mut self) -> FB14_W {
        FB14_W { w: self }
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&mut self) -> FB15_W {
        FB15_W { w: self }
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&mut self) -> FB16_W {
        FB16_W { w: self }
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&mut self) -> FB17_W {
        FB17_W { w: self }
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&mut self) -> FB18_W {
        FB18_W { w: self }
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&mut self) -> FB19_W {
        FB19_W { w: self }
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&mut self) -> FB20_W {
        FB20_W { w: self }
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&mut self) -> FB21_W {
        FB21_W { w: self }
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&mut self) -> FB22_W {
        FB22_W { w: self }
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&mut self) -> FB23_W {
        FB23_W { w: self }
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&mut self) -> FB24_W {
        FB24_W { w: self }
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&mut self) -> FB25_W {
        FB25_W { w: self }
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&mut self) -> FB26_W {
        FB26_W { w: self }
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&mut self) -> FB27_W {
        FB27_W { w: self }
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&mut self) -> FB28_W {
        FB28_W { w: self }
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&mut self) -> FB29_W {
        FB29_W { w: self }
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&mut self) -> FB30_W {
        FB30_W { w: self }
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&mut self) -> FB31_W {
        FB31_W { w: self }
    }
}
