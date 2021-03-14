#[doc = "Reader of register MPCBB2_VCTR33"]
pub type R = crate::R<u32, super::MPCBB2_VCTR33>;
#[doc = "Writer for register MPCBB2_VCTR33"]
pub type W = crate::W<u32, super::MPCBB2_VCTR33>;
#[doc = "Register MPCBB2_VCTR33 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR33 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1056`"]
pub type B1056_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1056`"]
pub struct B1056_W<'a> {
    w: &'a mut W,
}
impl<'a> B1056_W<'a> {
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
#[doc = "Reader of field `B1057`"]
pub type B1057_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1057`"]
pub struct B1057_W<'a> {
    w: &'a mut W,
}
impl<'a> B1057_W<'a> {
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
#[doc = "Reader of field `B1058`"]
pub type B1058_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1058`"]
pub struct B1058_W<'a> {
    w: &'a mut W,
}
impl<'a> B1058_W<'a> {
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
#[doc = "Reader of field `B1059`"]
pub type B1059_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1059`"]
pub struct B1059_W<'a> {
    w: &'a mut W,
}
impl<'a> B1059_W<'a> {
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
#[doc = "Reader of field `B1060`"]
pub type B1060_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1060`"]
pub struct B1060_W<'a> {
    w: &'a mut W,
}
impl<'a> B1060_W<'a> {
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
#[doc = "Reader of field `B1061`"]
pub type B1061_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1061`"]
pub struct B1061_W<'a> {
    w: &'a mut W,
}
impl<'a> B1061_W<'a> {
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
#[doc = "Reader of field `B1062`"]
pub type B1062_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1062`"]
pub struct B1062_W<'a> {
    w: &'a mut W,
}
impl<'a> B1062_W<'a> {
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
#[doc = "Reader of field `B1063`"]
pub type B1063_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1063`"]
pub struct B1063_W<'a> {
    w: &'a mut W,
}
impl<'a> B1063_W<'a> {
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
#[doc = "Reader of field `B1064`"]
pub type B1064_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1064`"]
pub struct B1064_W<'a> {
    w: &'a mut W,
}
impl<'a> B1064_W<'a> {
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
#[doc = "Reader of field `B1065`"]
pub type B1065_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1065`"]
pub struct B1065_W<'a> {
    w: &'a mut W,
}
impl<'a> B1065_W<'a> {
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
#[doc = "Reader of field `B1066`"]
pub type B1066_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1066`"]
pub struct B1066_W<'a> {
    w: &'a mut W,
}
impl<'a> B1066_W<'a> {
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
#[doc = "Reader of field `B1067`"]
pub type B1067_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1067`"]
pub struct B1067_W<'a> {
    w: &'a mut W,
}
impl<'a> B1067_W<'a> {
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
#[doc = "Reader of field `B1068`"]
pub type B1068_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1068`"]
pub struct B1068_W<'a> {
    w: &'a mut W,
}
impl<'a> B1068_W<'a> {
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
#[doc = "Reader of field `B1069`"]
pub type B1069_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1069`"]
pub struct B1069_W<'a> {
    w: &'a mut W,
}
impl<'a> B1069_W<'a> {
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
#[doc = "Reader of field `B1070`"]
pub type B1070_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1070`"]
pub struct B1070_W<'a> {
    w: &'a mut W,
}
impl<'a> B1070_W<'a> {
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
#[doc = "Reader of field `B1071`"]
pub type B1071_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1071`"]
pub struct B1071_W<'a> {
    w: &'a mut W,
}
impl<'a> B1071_W<'a> {
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
#[doc = "Reader of field `B1072`"]
pub type B1072_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1072`"]
pub struct B1072_W<'a> {
    w: &'a mut W,
}
impl<'a> B1072_W<'a> {
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
#[doc = "Reader of field `B1073`"]
pub type B1073_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1073`"]
pub struct B1073_W<'a> {
    w: &'a mut W,
}
impl<'a> B1073_W<'a> {
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
#[doc = "Reader of field `B1074`"]
pub type B1074_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1074`"]
pub struct B1074_W<'a> {
    w: &'a mut W,
}
impl<'a> B1074_W<'a> {
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
#[doc = "Reader of field `B1075`"]
pub type B1075_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1075`"]
pub struct B1075_W<'a> {
    w: &'a mut W,
}
impl<'a> B1075_W<'a> {
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
#[doc = "Reader of field `B1076`"]
pub type B1076_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1076`"]
pub struct B1076_W<'a> {
    w: &'a mut W,
}
impl<'a> B1076_W<'a> {
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
#[doc = "Reader of field `B1077`"]
pub type B1077_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1077`"]
pub struct B1077_W<'a> {
    w: &'a mut W,
}
impl<'a> B1077_W<'a> {
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
#[doc = "Reader of field `B1078`"]
pub type B1078_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1078`"]
pub struct B1078_W<'a> {
    w: &'a mut W,
}
impl<'a> B1078_W<'a> {
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
#[doc = "Reader of field `B1079`"]
pub type B1079_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1079`"]
pub struct B1079_W<'a> {
    w: &'a mut W,
}
impl<'a> B1079_W<'a> {
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
#[doc = "Reader of field `B1080`"]
pub type B1080_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1080`"]
pub struct B1080_W<'a> {
    w: &'a mut W,
}
impl<'a> B1080_W<'a> {
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
#[doc = "Reader of field `B1081`"]
pub type B1081_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1081`"]
pub struct B1081_W<'a> {
    w: &'a mut W,
}
impl<'a> B1081_W<'a> {
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
#[doc = "Reader of field `B1082`"]
pub type B1082_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1082`"]
pub struct B1082_W<'a> {
    w: &'a mut W,
}
impl<'a> B1082_W<'a> {
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
#[doc = "Reader of field `B1083`"]
pub type B1083_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1083`"]
pub struct B1083_W<'a> {
    w: &'a mut W,
}
impl<'a> B1083_W<'a> {
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
#[doc = "Reader of field `B1084`"]
pub type B1084_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1084`"]
pub struct B1084_W<'a> {
    w: &'a mut W,
}
impl<'a> B1084_W<'a> {
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
#[doc = "Reader of field `B1085`"]
pub type B1085_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1085`"]
pub struct B1085_W<'a> {
    w: &'a mut W,
}
impl<'a> B1085_W<'a> {
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
#[doc = "Reader of field `B1086`"]
pub type B1086_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1086`"]
pub struct B1086_W<'a> {
    w: &'a mut W,
}
impl<'a> B1086_W<'a> {
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
#[doc = "Reader of field `B1087`"]
pub type B1087_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1087`"]
pub struct B1087_W<'a> {
    w: &'a mut W,
}
impl<'a> B1087_W<'a> {
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
    #[doc = "Bit 0 - B1056"]
    #[inline(always)]
    pub fn b1056(&self) -> B1056_R {
        B1056_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1057"]
    #[inline(always)]
    pub fn b1057(&self) -> B1057_R {
        B1057_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1058"]
    #[inline(always)]
    pub fn b1058(&self) -> B1058_R {
        B1058_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1059"]
    #[inline(always)]
    pub fn b1059(&self) -> B1059_R {
        B1059_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1060"]
    #[inline(always)]
    pub fn b1060(&self) -> B1060_R {
        B1060_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1061"]
    #[inline(always)]
    pub fn b1061(&self) -> B1061_R {
        B1061_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1062"]
    #[inline(always)]
    pub fn b1062(&self) -> B1062_R {
        B1062_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1063"]
    #[inline(always)]
    pub fn b1063(&self) -> B1063_R {
        B1063_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1064"]
    #[inline(always)]
    pub fn b1064(&self) -> B1064_R {
        B1064_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1065"]
    #[inline(always)]
    pub fn b1065(&self) -> B1065_R {
        B1065_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1066"]
    #[inline(always)]
    pub fn b1066(&self) -> B1066_R {
        B1066_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1067"]
    #[inline(always)]
    pub fn b1067(&self) -> B1067_R {
        B1067_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1068"]
    #[inline(always)]
    pub fn b1068(&self) -> B1068_R {
        B1068_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1069"]
    #[inline(always)]
    pub fn b1069(&self) -> B1069_R {
        B1069_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1070"]
    #[inline(always)]
    pub fn b1070(&self) -> B1070_R {
        B1070_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1071"]
    #[inline(always)]
    pub fn b1071(&self) -> B1071_R {
        B1071_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1072"]
    #[inline(always)]
    pub fn b1072(&self) -> B1072_R {
        B1072_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1073"]
    #[inline(always)]
    pub fn b1073(&self) -> B1073_R {
        B1073_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1074"]
    #[inline(always)]
    pub fn b1074(&self) -> B1074_R {
        B1074_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1075"]
    #[inline(always)]
    pub fn b1075(&self) -> B1075_R {
        B1075_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1076"]
    #[inline(always)]
    pub fn b1076(&self) -> B1076_R {
        B1076_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1077"]
    #[inline(always)]
    pub fn b1077(&self) -> B1077_R {
        B1077_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1078"]
    #[inline(always)]
    pub fn b1078(&self) -> B1078_R {
        B1078_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1079"]
    #[inline(always)]
    pub fn b1079(&self) -> B1079_R {
        B1079_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1080"]
    #[inline(always)]
    pub fn b1080(&self) -> B1080_R {
        B1080_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1081"]
    #[inline(always)]
    pub fn b1081(&self) -> B1081_R {
        B1081_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1082"]
    #[inline(always)]
    pub fn b1082(&self) -> B1082_R {
        B1082_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1083"]
    #[inline(always)]
    pub fn b1083(&self) -> B1083_R {
        B1083_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1084"]
    #[inline(always)]
    pub fn b1084(&self) -> B1084_R {
        B1084_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1085"]
    #[inline(always)]
    pub fn b1085(&self) -> B1085_R {
        B1085_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1086"]
    #[inline(always)]
    pub fn b1086(&self) -> B1086_R {
        B1086_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1087"]
    #[inline(always)]
    pub fn b1087(&self) -> B1087_R {
        B1087_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1056"]
    #[inline(always)]
    pub fn b1056(&mut self) -> B1056_W {
        B1056_W { w: self }
    }
    #[doc = "Bit 1 - B1057"]
    #[inline(always)]
    pub fn b1057(&mut self) -> B1057_W {
        B1057_W { w: self }
    }
    #[doc = "Bit 2 - B1058"]
    #[inline(always)]
    pub fn b1058(&mut self) -> B1058_W {
        B1058_W { w: self }
    }
    #[doc = "Bit 3 - B1059"]
    #[inline(always)]
    pub fn b1059(&mut self) -> B1059_W {
        B1059_W { w: self }
    }
    #[doc = "Bit 4 - B1060"]
    #[inline(always)]
    pub fn b1060(&mut self) -> B1060_W {
        B1060_W { w: self }
    }
    #[doc = "Bit 5 - B1061"]
    #[inline(always)]
    pub fn b1061(&mut self) -> B1061_W {
        B1061_W { w: self }
    }
    #[doc = "Bit 6 - B1062"]
    #[inline(always)]
    pub fn b1062(&mut self) -> B1062_W {
        B1062_W { w: self }
    }
    #[doc = "Bit 7 - B1063"]
    #[inline(always)]
    pub fn b1063(&mut self) -> B1063_W {
        B1063_W { w: self }
    }
    #[doc = "Bit 8 - B1064"]
    #[inline(always)]
    pub fn b1064(&mut self) -> B1064_W {
        B1064_W { w: self }
    }
    #[doc = "Bit 9 - B1065"]
    #[inline(always)]
    pub fn b1065(&mut self) -> B1065_W {
        B1065_W { w: self }
    }
    #[doc = "Bit 10 - B1066"]
    #[inline(always)]
    pub fn b1066(&mut self) -> B1066_W {
        B1066_W { w: self }
    }
    #[doc = "Bit 11 - B1067"]
    #[inline(always)]
    pub fn b1067(&mut self) -> B1067_W {
        B1067_W { w: self }
    }
    #[doc = "Bit 12 - B1068"]
    #[inline(always)]
    pub fn b1068(&mut self) -> B1068_W {
        B1068_W { w: self }
    }
    #[doc = "Bit 13 - B1069"]
    #[inline(always)]
    pub fn b1069(&mut self) -> B1069_W {
        B1069_W { w: self }
    }
    #[doc = "Bit 14 - B1070"]
    #[inline(always)]
    pub fn b1070(&mut self) -> B1070_W {
        B1070_W { w: self }
    }
    #[doc = "Bit 15 - B1071"]
    #[inline(always)]
    pub fn b1071(&mut self) -> B1071_W {
        B1071_W { w: self }
    }
    #[doc = "Bit 16 - B1072"]
    #[inline(always)]
    pub fn b1072(&mut self) -> B1072_W {
        B1072_W { w: self }
    }
    #[doc = "Bit 17 - B1073"]
    #[inline(always)]
    pub fn b1073(&mut self) -> B1073_W {
        B1073_W { w: self }
    }
    #[doc = "Bit 18 - B1074"]
    #[inline(always)]
    pub fn b1074(&mut self) -> B1074_W {
        B1074_W { w: self }
    }
    #[doc = "Bit 19 - B1075"]
    #[inline(always)]
    pub fn b1075(&mut self) -> B1075_W {
        B1075_W { w: self }
    }
    #[doc = "Bit 20 - B1076"]
    #[inline(always)]
    pub fn b1076(&mut self) -> B1076_W {
        B1076_W { w: self }
    }
    #[doc = "Bit 21 - B1077"]
    #[inline(always)]
    pub fn b1077(&mut self) -> B1077_W {
        B1077_W { w: self }
    }
    #[doc = "Bit 22 - B1078"]
    #[inline(always)]
    pub fn b1078(&mut self) -> B1078_W {
        B1078_W { w: self }
    }
    #[doc = "Bit 23 - B1079"]
    #[inline(always)]
    pub fn b1079(&mut self) -> B1079_W {
        B1079_W { w: self }
    }
    #[doc = "Bit 24 - B1080"]
    #[inline(always)]
    pub fn b1080(&mut self) -> B1080_W {
        B1080_W { w: self }
    }
    #[doc = "Bit 25 - B1081"]
    #[inline(always)]
    pub fn b1081(&mut self) -> B1081_W {
        B1081_W { w: self }
    }
    #[doc = "Bit 26 - B1082"]
    #[inline(always)]
    pub fn b1082(&mut self) -> B1082_W {
        B1082_W { w: self }
    }
    #[doc = "Bit 27 - B1083"]
    #[inline(always)]
    pub fn b1083(&mut self) -> B1083_W {
        B1083_W { w: self }
    }
    #[doc = "Bit 28 - B1084"]
    #[inline(always)]
    pub fn b1084(&mut self) -> B1084_W {
        B1084_W { w: self }
    }
    #[doc = "Bit 29 - B1085"]
    #[inline(always)]
    pub fn b1085(&mut self) -> B1085_W {
        B1085_W { w: self }
    }
    #[doc = "Bit 30 - B1086"]
    #[inline(always)]
    pub fn b1086(&mut self) -> B1086_W {
        B1086_W { w: self }
    }
    #[doc = "Bit 31 - B1087"]
    #[inline(always)]
    pub fn b1087(&mut self) -> B1087_W {
        B1087_W { w: self }
    }
}
