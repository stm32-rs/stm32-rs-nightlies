#[doc = "Reader of register MPCBB1_VCTR34"]
pub type R = crate::R<u32, super::MPCBB1_VCTR34>;
#[doc = "Writer for register MPCBB1_VCTR34"]
pub type W = crate::W<u32, super::MPCBB1_VCTR34>;
#[doc = "Register MPCBB1_VCTR34 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR34 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1088`"]
pub type B1088_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1088`"]
pub struct B1088_W<'a> {
    w: &'a mut W,
}
impl<'a> B1088_W<'a> {
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
#[doc = "Reader of field `B1089`"]
pub type B1089_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1089`"]
pub struct B1089_W<'a> {
    w: &'a mut W,
}
impl<'a> B1089_W<'a> {
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
#[doc = "Reader of field `B1090`"]
pub type B1090_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1090`"]
pub struct B1090_W<'a> {
    w: &'a mut W,
}
impl<'a> B1090_W<'a> {
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
#[doc = "Reader of field `B1091`"]
pub type B1091_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1091`"]
pub struct B1091_W<'a> {
    w: &'a mut W,
}
impl<'a> B1091_W<'a> {
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
#[doc = "Reader of field `B1092`"]
pub type B1092_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1092`"]
pub struct B1092_W<'a> {
    w: &'a mut W,
}
impl<'a> B1092_W<'a> {
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
#[doc = "Reader of field `B1093`"]
pub type B1093_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1093`"]
pub struct B1093_W<'a> {
    w: &'a mut W,
}
impl<'a> B1093_W<'a> {
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
#[doc = "Reader of field `B1094`"]
pub type B1094_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1094`"]
pub struct B1094_W<'a> {
    w: &'a mut W,
}
impl<'a> B1094_W<'a> {
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
#[doc = "Reader of field `B1095`"]
pub type B1095_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1095`"]
pub struct B1095_W<'a> {
    w: &'a mut W,
}
impl<'a> B1095_W<'a> {
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
#[doc = "Reader of field `B1096`"]
pub type B1096_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1096`"]
pub struct B1096_W<'a> {
    w: &'a mut W,
}
impl<'a> B1096_W<'a> {
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
#[doc = "Reader of field `B1097`"]
pub type B1097_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1097`"]
pub struct B1097_W<'a> {
    w: &'a mut W,
}
impl<'a> B1097_W<'a> {
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
#[doc = "Reader of field `B1098`"]
pub type B1098_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1098`"]
pub struct B1098_W<'a> {
    w: &'a mut W,
}
impl<'a> B1098_W<'a> {
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
#[doc = "Reader of field `B1099`"]
pub type B1099_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1099`"]
pub struct B1099_W<'a> {
    w: &'a mut W,
}
impl<'a> B1099_W<'a> {
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
#[doc = "Reader of field `B1100`"]
pub type B1100_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1100`"]
pub struct B1100_W<'a> {
    w: &'a mut W,
}
impl<'a> B1100_W<'a> {
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
#[doc = "Reader of field `B1101`"]
pub type B1101_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1101`"]
pub struct B1101_W<'a> {
    w: &'a mut W,
}
impl<'a> B1101_W<'a> {
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
#[doc = "Reader of field `B1102`"]
pub type B1102_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1102`"]
pub struct B1102_W<'a> {
    w: &'a mut W,
}
impl<'a> B1102_W<'a> {
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
#[doc = "Reader of field `B1103`"]
pub type B1103_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1103`"]
pub struct B1103_W<'a> {
    w: &'a mut W,
}
impl<'a> B1103_W<'a> {
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
#[doc = "Reader of field `B1104`"]
pub type B1104_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1104`"]
pub struct B1104_W<'a> {
    w: &'a mut W,
}
impl<'a> B1104_W<'a> {
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
#[doc = "Reader of field `B1105`"]
pub type B1105_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1105`"]
pub struct B1105_W<'a> {
    w: &'a mut W,
}
impl<'a> B1105_W<'a> {
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
#[doc = "Reader of field `B1106`"]
pub type B1106_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1106`"]
pub struct B1106_W<'a> {
    w: &'a mut W,
}
impl<'a> B1106_W<'a> {
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
#[doc = "Reader of field `B1107`"]
pub type B1107_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1107`"]
pub struct B1107_W<'a> {
    w: &'a mut W,
}
impl<'a> B1107_W<'a> {
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
#[doc = "Reader of field `B1108`"]
pub type B1108_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1108`"]
pub struct B1108_W<'a> {
    w: &'a mut W,
}
impl<'a> B1108_W<'a> {
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
#[doc = "Reader of field `B1109`"]
pub type B1109_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1109`"]
pub struct B1109_W<'a> {
    w: &'a mut W,
}
impl<'a> B1109_W<'a> {
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
#[doc = "Reader of field `B1110`"]
pub type B1110_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1110`"]
pub struct B1110_W<'a> {
    w: &'a mut W,
}
impl<'a> B1110_W<'a> {
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
#[doc = "Reader of field `B1111`"]
pub type B1111_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1111`"]
pub struct B1111_W<'a> {
    w: &'a mut W,
}
impl<'a> B1111_W<'a> {
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
#[doc = "Reader of field `B1112`"]
pub type B1112_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1112`"]
pub struct B1112_W<'a> {
    w: &'a mut W,
}
impl<'a> B1112_W<'a> {
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
#[doc = "Reader of field `B1113`"]
pub type B1113_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1113`"]
pub struct B1113_W<'a> {
    w: &'a mut W,
}
impl<'a> B1113_W<'a> {
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
#[doc = "Reader of field `B1114`"]
pub type B1114_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1114`"]
pub struct B1114_W<'a> {
    w: &'a mut W,
}
impl<'a> B1114_W<'a> {
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
#[doc = "Reader of field `B1115`"]
pub type B1115_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1115`"]
pub struct B1115_W<'a> {
    w: &'a mut W,
}
impl<'a> B1115_W<'a> {
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
#[doc = "Reader of field `B1116`"]
pub type B1116_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1116`"]
pub struct B1116_W<'a> {
    w: &'a mut W,
}
impl<'a> B1116_W<'a> {
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
#[doc = "Reader of field `B1117`"]
pub type B1117_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1117`"]
pub struct B1117_W<'a> {
    w: &'a mut W,
}
impl<'a> B1117_W<'a> {
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
#[doc = "Reader of field `B1118`"]
pub type B1118_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1118`"]
pub struct B1118_W<'a> {
    w: &'a mut W,
}
impl<'a> B1118_W<'a> {
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
#[doc = "Reader of field `B1119`"]
pub type B1119_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1119`"]
pub struct B1119_W<'a> {
    w: &'a mut W,
}
impl<'a> B1119_W<'a> {
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
    #[doc = "Bit 0 - B1088"]
    #[inline(always)]
    pub fn b1088(&self) -> B1088_R {
        B1088_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1089"]
    #[inline(always)]
    pub fn b1089(&self) -> B1089_R {
        B1089_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1090"]
    #[inline(always)]
    pub fn b1090(&self) -> B1090_R {
        B1090_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1091"]
    #[inline(always)]
    pub fn b1091(&self) -> B1091_R {
        B1091_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1092"]
    #[inline(always)]
    pub fn b1092(&self) -> B1092_R {
        B1092_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1093"]
    #[inline(always)]
    pub fn b1093(&self) -> B1093_R {
        B1093_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1094"]
    #[inline(always)]
    pub fn b1094(&self) -> B1094_R {
        B1094_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1095"]
    #[inline(always)]
    pub fn b1095(&self) -> B1095_R {
        B1095_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1096"]
    #[inline(always)]
    pub fn b1096(&self) -> B1096_R {
        B1096_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1097"]
    #[inline(always)]
    pub fn b1097(&self) -> B1097_R {
        B1097_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1098"]
    #[inline(always)]
    pub fn b1098(&self) -> B1098_R {
        B1098_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1099"]
    #[inline(always)]
    pub fn b1099(&self) -> B1099_R {
        B1099_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1100"]
    #[inline(always)]
    pub fn b1100(&self) -> B1100_R {
        B1100_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1101"]
    #[inline(always)]
    pub fn b1101(&self) -> B1101_R {
        B1101_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1102"]
    #[inline(always)]
    pub fn b1102(&self) -> B1102_R {
        B1102_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1103"]
    #[inline(always)]
    pub fn b1103(&self) -> B1103_R {
        B1103_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1104"]
    #[inline(always)]
    pub fn b1104(&self) -> B1104_R {
        B1104_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1105"]
    #[inline(always)]
    pub fn b1105(&self) -> B1105_R {
        B1105_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1106"]
    #[inline(always)]
    pub fn b1106(&self) -> B1106_R {
        B1106_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1107"]
    #[inline(always)]
    pub fn b1107(&self) -> B1107_R {
        B1107_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1108"]
    #[inline(always)]
    pub fn b1108(&self) -> B1108_R {
        B1108_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1109"]
    #[inline(always)]
    pub fn b1109(&self) -> B1109_R {
        B1109_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1110"]
    #[inline(always)]
    pub fn b1110(&self) -> B1110_R {
        B1110_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1111"]
    #[inline(always)]
    pub fn b1111(&self) -> B1111_R {
        B1111_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1112"]
    #[inline(always)]
    pub fn b1112(&self) -> B1112_R {
        B1112_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1113"]
    #[inline(always)]
    pub fn b1113(&self) -> B1113_R {
        B1113_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1114"]
    #[inline(always)]
    pub fn b1114(&self) -> B1114_R {
        B1114_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1115"]
    #[inline(always)]
    pub fn b1115(&self) -> B1115_R {
        B1115_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1116"]
    #[inline(always)]
    pub fn b1116(&self) -> B1116_R {
        B1116_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1117"]
    #[inline(always)]
    pub fn b1117(&self) -> B1117_R {
        B1117_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1118"]
    #[inline(always)]
    pub fn b1118(&self) -> B1118_R {
        B1118_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1119"]
    #[inline(always)]
    pub fn b1119(&self) -> B1119_R {
        B1119_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1088"]
    #[inline(always)]
    pub fn b1088(&mut self) -> B1088_W {
        B1088_W { w: self }
    }
    #[doc = "Bit 1 - B1089"]
    #[inline(always)]
    pub fn b1089(&mut self) -> B1089_W {
        B1089_W { w: self }
    }
    #[doc = "Bit 2 - B1090"]
    #[inline(always)]
    pub fn b1090(&mut self) -> B1090_W {
        B1090_W { w: self }
    }
    #[doc = "Bit 3 - B1091"]
    #[inline(always)]
    pub fn b1091(&mut self) -> B1091_W {
        B1091_W { w: self }
    }
    #[doc = "Bit 4 - B1092"]
    #[inline(always)]
    pub fn b1092(&mut self) -> B1092_W {
        B1092_W { w: self }
    }
    #[doc = "Bit 5 - B1093"]
    #[inline(always)]
    pub fn b1093(&mut self) -> B1093_W {
        B1093_W { w: self }
    }
    #[doc = "Bit 6 - B1094"]
    #[inline(always)]
    pub fn b1094(&mut self) -> B1094_W {
        B1094_W { w: self }
    }
    #[doc = "Bit 7 - B1095"]
    #[inline(always)]
    pub fn b1095(&mut self) -> B1095_W {
        B1095_W { w: self }
    }
    #[doc = "Bit 8 - B1096"]
    #[inline(always)]
    pub fn b1096(&mut self) -> B1096_W {
        B1096_W { w: self }
    }
    #[doc = "Bit 9 - B1097"]
    #[inline(always)]
    pub fn b1097(&mut self) -> B1097_W {
        B1097_W { w: self }
    }
    #[doc = "Bit 10 - B1098"]
    #[inline(always)]
    pub fn b1098(&mut self) -> B1098_W {
        B1098_W { w: self }
    }
    #[doc = "Bit 11 - B1099"]
    #[inline(always)]
    pub fn b1099(&mut self) -> B1099_W {
        B1099_W { w: self }
    }
    #[doc = "Bit 12 - B1100"]
    #[inline(always)]
    pub fn b1100(&mut self) -> B1100_W {
        B1100_W { w: self }
    }
    #[doc = "Bit 13 - B1101"]
    #[inline(always)]
    pub fn b1101(&mut self) -> B1101_W {
        B1101_W { w: self }
    }
    #[doc = "Bit 14 - B1102"]
    #[inline(always)]
    pub fn b1102(&mut self) -> B1102_W {
        B1102_W { w: self }
    }
    #[doc = "Bit 15 - B1103"]
    #[inline(always)]
    pub fn b1103(&mut self) -> B1103_W {
        B1103_W { w: self }
    }
    #[doc = "Bit 16 - B1104"]
    #[inline(always)]
    pub fn b1104(&mut self) -> B1104_W {
        B1104_W { w: self }
    }
    #[doc = "Bit 17 - B1105"]
    #[inline(always)]
    pub fn b1105(&mut self) -> B1105_W {
        B1105_W { w: self }
    }
    #[doc = "Bit 18 - B1106"]
    #[inline(always)]
    pub fn b1106(&mut self) -> B1106_W {
        B1106_W { w: self }
    }
    #[doc = "Bit 19 - B1107"]
    #[inline(always)]
    pub fn b1107(&mut self) -> B1107_W {
        B1107_W { w: self }
    }
    #[doc = "Bit 20 - B1108"]
    #[inline(always)]
    pub fn b1108(&mut self) -> B1108_W {
        B1108_W { w: self }
    }
    #[doc = "Bit 21 - B1109"]
    #[inline(always)]
    pub fn b1109(&mut self) -> B1109_W {
        B1109_W { w: self }
    }
    #[doc = "Bit 22 - B1110"]
    #[inline(always)]
    pub fn b1110(&mut self) -> B1110_W {
        B1110_W { w: self }
    }
    #[doc = "Bit 23 - B1111"]
    #[inline(always)]
    pub fn b1111(&mut self) -> B1111_W {
        B1111_W { w: self }
    }
    #[doc = "Bit 24 - B1112"]
    #[inline(always)]
    pub fn b1112(&mut self) -> B1112_W {
        B1112_W { w: self }
    }
    #[doc = "Bit 25 - B1113"]
    #[inline(always)]
    pub fn b1113(&mut self) -> B1113_W {
        B1113_W { w: self }
    }
    #[doc = "Bit 26 - B1114"]
    #[inline(always)]
    pub fn b1114(&mut self) -> B1114_W {
        B1114_W { w: self }
    }
    #[doc = "Bit 27 - B1115"]
    #[inline(always)]
    pub fn b1115(&mut self) -> B1115_W {
        B1115_W { w: self }
    }
    #[doc = "Bit 28 - B1116"]
    #[inline(always)]
    pub fn b1116(&mut self) -> B1116_W {
        B1116_W { w: self }
    }
    #[doc = "Bit 29 - B1117"]
    #[inline(always)]
    pub fn b1117(&mut self) -> B1117_W {
        B1117_W { w: self }
    }
    #[doc = "Bit 30 - B1118"]
    #[inline(always)]
    pub fn b1118(&mut self) -> B1118_W {
        B1118_W { w: self }
    }
    #[doc = "Bit 31 - B1119"]
    #[inline(always)]
    pub fn b1119(&mut self) -> B1119_W {
        B1119_W { w: self }
    }
}
