#[doc = "Reader of register MPCBB2_VCTR36"]
pub type R = crate::R<u32, super::MPCBB2_VCTR36>;
#[doc = "Writer for register MPCBB2_VCTR36"]
pub type W = crate::W<u32, super::MPCBB2_VCTR36>;
#[doc = "Register MPCBB2_VCTR36 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR36 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1152`"]
pub type B1152_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1152`"]
pub struct B1152_W<'a> {
    w: &'a mut W,
}
impl<'a> B1152_W<'a> {
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
#[doc = "Reader of field `B1153`"]
pub type B1153_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1153`"]
pub struct B1153_W<'a> {
    w: &'a mut W,
}
impl<'a> B1153_W<'a> {
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
#[doc = "Reader of field `B1154`"]
pub type B1154_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1154`"]
pub struct B1154_W<'a> {
    w: &'a mut W,
}
impl<'a> B1154_W<'a> {
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
#[doc = "Reader of field `B1155`"]
pub type B1155_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1155`"]
pub struct B1155_W<'a> {
    w: &'a mut W,
}
impl<'a> B1155_W<'a> {
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
#[doc = "Reader of field `B1156`"]
pub type B1156_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1156`"]
pub struct B1156_W<'a> {
    w: &'a mut W,
}
impl<'a> B1156_W<'a> {
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
#[doc = "Reader of field `B1157`"]
pub type B1157_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1157`"]
pub struct B1157_W<'a> {
    w: &'a mut W,
}
impl<'a> B1157_W<'a> {
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
#[doc = "Reader of field `B1158`"]
pub type B1158_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1158`"]
pub struct B1158_W<'a> {
    w: &'a mut W,
}
impl<'a> B1158_W<'a> {
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
#[doc = "Reader of field `B1159`"]
pub type B1159_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1159`"]
pub struct B1159_W<'a> {
    w: &'a mut W,
}
impl<'a> B1159_W<'a> {
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
#[doc = "Reader of field `B1160`"]
pub type B1160_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1160`"]
pub struct B1160_W<'a> {
    w: &'a mut W,
}
impl<'a> B1160_W<'a> {
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
#[doc = "Reader of field `B1161`"]
pub type B1161_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1161`"]
pub struct B1161_W<'a> {
    w: &'a mut W,
}
impl<'a> B1161_W<'a> {
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
#[doc = "Reader of field `B1162`"]
pub type B1162_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1162`"]
pub struct B1162_W<'a> {
    w: &'a mut W,
}
impl<'a> B1162_W<'a> {
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
#[doc = "Reader of field `B1163`"]
pub type B1163_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1163`"]
pub struct B1163_W<'a> {
    w: &'a mut W,
}
impl<'a> B1163_W<'a> {
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
#[doc = "Reader of field `B1164`"]
pub type B1164_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1164`"]
pub struct B1164_W<'a> {
    w: &'a mut W,
}
impl<'a> B1164_W<'a> {
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
#[doc = "Reader of field `B1165`"]
pub type B1165_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1165`"]
pub struct B1165_W<'a> {
    w: &'a mut W,
}
impl<'a> B1165_W<'a> {
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
#[doc = "Reader of field `B1166`"]
pub type B1166_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1166`"]
pub struct B1166_W<'a> {
    w: &'a mut W,
}
impl<'a> B1166_W<'a> {
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
#[doc = "Reader of field `B1167`"]
pub type B1167_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1167`"]
pub struct B1167_W<'a> {
    w: &'a mut W,
}
impl<'a> B1167_W<'a> {
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
#[doc = "Reader of field `B1168`"]
pub type B1168_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1168`"]
pub struct B1168_W<'a> {
    w: &'a mut W,
}
impl<'a> B1168_W<'a> {
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
#[doc = "Reader of field `B1169`"]
pub type B1169_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1169`"]
pub struct B1169_W<'a> {
    w: &'a mut W,
}
impl<'a> B1169_W<'a> {
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
#[doc = "Reader of field `B1170`"]
pub type B1170_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1170`"]
pub struct B1170_W<'a> {
    w: &'a mut W,
}
impl<'a> B1170_W<'a> {
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
#[doc = "Reader of field `B1171`"]
pub type B1171_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1171`"]
pub struct B1171_W<'a> {
    w: &'a mut W,
}
impl<'a> B1171_W<'a> {
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
#[doc = "Reader of field `B1172`"]
pub type B1172_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1172`"]
pub struct B1172_W<'a> {
    w: &'a mut W,
}
impl<'a> B1172_W<'a> {
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
#[doc = "Reader of field `B1173`"]
pub type B1173_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1173`"]
pub struct B1173_W<'a> {
    w: &'a mut W,
}
impl<'a> B1173_W<'a> {
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
#[doc = "Reader of field `B1174`"]
pub type B1174_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1174`"]
pub struct B1174_W<'a> {
    w: &'a mut W,
}
impl<'a> B1174_W<'a> {
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
#[doc = "Reader of field `B1175`"]
pub type B1175_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1175`"]
pub struct B1175_W<'a> {
    w: &'a mut W,
}
impl<'a> B1175_W<'a> {
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
#[doc = "Reader of field `B1176`"]
pub type B1176_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1176`"]
pub struct B1176_W<'a> {
    w: &'a mut W,
}
impl<'a> B1176_W<'a> {
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
#[doc = "Reader of field `B1177`"]
pub type B1177_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1177`"]
pub struct B1177_W<'a> {
    w: &'a mut W,
}
impl<'a> B1177_W<'a> {
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
#[doc = "Reader of field `B1178`"]
pub type B1178_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1178`"]
pub struct B1178_W<'a> {
    w: &'a mut W,
}
impl<'a> B1178_W<'a> {
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
#[doc = "Reader of field `B1179`"]
pub type B1179_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1179`"]
pub struct B1179_W<'a> {
    w: &'a mut W,
}
impl<'a> B1179_W<'a> {
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
#[doc = "Reader of field `B1180`"]
pub type B1180_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1180`"]
pub struct B1180_W<'a> {
    w: &'a mut W,
}
impl<'a> B1180_W<'a> {
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
#[doc = "Reader of field `B1181`"]
pub type B1181_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1181`"]
pub struct B1181_W<'a> {
    w: &'a mut W,
}
impl<'a> B1181_W<'a> {
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
#[doc = "Reader of field `B1182`"]
pub type B1182_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1182`"]
pub struct B1182_W<'a> {
    w: &'a mut W,
}
impl<'a> B1182_W<'a> {
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
#[doc = "Reader of field `B1183`"]
pub type B1183_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1183`"]
pub struct B1183_W<'a> {
    w: &'a mut W,
}
impl<'a> B1183_W<'a> {
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
    #[doc = "Bit 0 - B1152"]
    #[inline(always)]
    pub fn b1152(&self) -> B1152_R {
        B1152_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1153"]
    #[inline(always)]
    pub fn b1153(&self) -> B1153_R {
        B1153_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1154"]
    #[inline(always)]
    pub fn b1154(&self) -> B1154_R {
        B1154_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1155"]
    #[inline(always)]
    pub fn b1155(&self) -> B1155_R {
        B1155_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1156"]
    #[inline(always)]
    pub fn b1156(&self) -> B1156_R {
        B1156_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1157"]
    #[inline(always)]
    pub fn b1157(&self) -> B1157_R {
        B1157_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1158"]
    #[inline(always)]
    pub fn b1158(&self) -> B1158_R {
        B1158_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1159"]
    #[inline(always)]
    pub fn b1159(&self) -> B1159_R {
        B1159_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1160"]
    #[inline(always)]
    pub fn b1160(&self) -> B1160_R {
        B1160_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1161"]
    #[inline(always)]
    pub fn b1161(&self) -> B1161_R {
        B1161_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1162"]
    #[inline(always)]
    pub fn b1162(&self) -> B1162_R {
        B1162_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1163"]
    #[inline(always)]
    pub fn b1163(&self) -> B1163_R {
        B1163_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1164"]
    #[inline(always)]
    pub fn b1164(&self) -> B1164_R {
        B1164_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1165"]
    #[inline(always)]
    pub fn b1165(&self) -> B1165_R {
        B1165_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1166"]
    #[inline(always)]
    pub fn b1166(&self) -> B1166_R {
        B1166_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1167"]
    #[inline(always)]
    pub fn b1167(&self) -> B1167_R {
        B1167_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1168"]
    #[inline(always)]
    pub fn b1168(&self) -> B1168_R {
        B1168_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1169"]
    #[inline(always)]
    pub fn b1169(&self) -> B1169_R {
        B1169_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1170"]
    #[inline(always)]
    pub fn b1170(&self) -> B1170_R {
        B1170_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1171"]
    #[inline(always)]
    pub fn b1171(&self) -> B1171_R {
        B1171_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1172"]
    #[inline(always)]
    pub fn b1172(&self) -> B1172_R {
        B1172_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1173"]
    #[inline(always)]
    pub fn b1173(&self) -> B1173_R {
        B1173_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1174"]
    #[inline(always)]
    pub fn b1174(&self) -> B1174_R {
        B1174_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1175"]
    #[inline(always)]
    pub fn b1175(&self) -> B1175_R {
        B1175_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1176"]
    #[inline(always)]
    pub fn b1176(&self) -> B1176_R {
        B1176_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1177"]
    #[inline(always)]
    pub fn b1177(&self) -> B1177_R {
        B1177_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1178"]
    #[inline(always)]
    pub fn b1178(&self) -> B1178_R {
        B1178_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1179"]
    #[inline(always)]
    pub fn b1179(&self) -> B1179_R {
        B1179_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1180"]
    #[inline(always)]
    pub fn b1180(&self) -> B1180_R {
        B1180_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1181"]
    #[inline(always)]
    pub fn b1181(&self) -> B1181_R {
        B1181_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1182"]
    #[inline(always)]
    pub fn b1182(&self) -> B1182_R {
        B1182_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1183"]
    #[inline(always)]
    pub fn b1183(&self) -> B1183_R {
        B1183_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1152"]
    #[inline(always)]
    pub fn b1152(&mut self) -> B1152_W {
        B1152_W { w: self }
    }
    #[doc = "Bit 1 - B1153"]
    #[inline(always)]
    pub fn b1153(&mut self) -> B1153_W {
        B1153_W { w: self }
    }
    #[doc = "Bit 2 - B1154"]
    #[inline(always)]
    pub fn b1154(&mut self) -> B1154_W {
        B1154_W { w: self }
    }
    #[doc = "Bit 3 - B1155"]
    #[inline(always)]
    pub fn b1155(&mut self) -> B1155_W {
        B1155_W { w: self }
    }
    #[doc = "Bit 4 - B1156"]
    #[inline(always)]
    pub fn b1156(&mut self) -> B1156_W {
        B1156_W { w: self }
    }
    #[doc = "Bit 5 - B1157"]
    #[inline(always)]
    pub fn b1157(&mut self) -> B1157_W {
        B1157_W { w: self }
    }
    #[doc = "Bit 6 - B1158"]
    #[inline(always)]
    pub fn b1158(&mut self) -> B1158_W {
        B1158_W { w: self }
    }
    #[doc = "Bit 7 - B1159"]
    #[inline(always)]
    pub fn b1159(&mut self) -> B1159_W {
        B1159_W { w: self }
    }
    #[doc = "Bit 8 - B1160"]
    #[inline(always)]
    pub fn b1160(&mut self) -> B1160_W {
        B1160_W { w: self }
    }
    #[doc = "Bit 9 - B1161"]
    #[inline(always)]
    pub fn b1161(&mut self) -> B1161_W {
        B1161_W { w: self }
    }
    #[doc = "Bit 10 - B1162"]
    #[inline(always)]
    pub fn b1162(&mut self) -> B1162_W {
        B1162_W { w: self }
    }
    #[doc = "Bit 11 - B1163"]
    #[inline(always)]
    pub fn b1163(&mut self) -> B1163_W {
        B1163_W { w: self }
    }
    #[doc = "Bit 12 - B1164"]
    #[inline(always)]
    pub fn b1164(&mut self) -> B1164_W {
        B1164_W { w: self }
    }
    #[doc = "Bit 13 - B1165"]
    #[inline(always)]
    pub fn b1165(&mut self) -> B1165_W {
        B1165_W { w: self }
    }
    #[doc = "Bit 14 - B1166"]
    #[inline(always)]
    pub fn b1166(&mut self) -> B1166_W {
        B1166_W { w: self }
    }
    #[doc = "Bit 15 - B1167"]
    #[inline(always)]
    pub fn b1167(&mut self) -> B1167_W {
        B1167_W { w: self }
    }
    #[doc = "Bit 16 - B1168"]
    #[inline(always)]
    pub fn b1168(&mut self) -> B1168_W {
        B1168_W { w: self }
    }
    #[doc = "Bit 17 - B1169"]
    #[inline(always)]
    pub fn b1169(&mut self) -> B1169_W {
        B1169_W { w: self }
    }
    #[doc = "Bit 18 - B1170"]
    #[inline(always)]
    pub fn b1170(&mut self) -> B1170_W {
        B1170_W { w: self }
    }
    #[doc = "Bit 19 - B1171"]
    #[inline(always)]
    pub fn b1171(&mut self) -> B1171_W {
        B1171_W { w: self }
    }
    #[doc = "Bit 20 - B1172"]
    #[inline(always)]
    pub fn b1172(&mut self) -> B1172_W {
        B1172_W { w: self }
    }
    #[doc = "Bit 21 - B1173"]
    #[inline(always)]
    pub fn b1173(&mut self) -> B1173_W {
        B1173_W { w: self }
    }
    #[doc = "Bit 22 - B1174"]
    #[inline(always)]
    pub fn b1174(&mut self) -> B1174_W {
        B1174_W { w: self }
    }
    #[doc = "Bit 23 - B1175"]
    #[inline(always)]
    pub fn b1175(&mut self) -> B1175_W {
        B1175_W { w: self }
    }
    #[doc = "Bit 24 - B1176"]
    #[inline(always)]
    pub fn b1176(&mut self) -> B1176_W {
        B1176_W { w: self }
    }
    #[doc = "Bit 25 - B1177"]
    #[inline(always)]
    pub fn b1177(&mut self) -> B1177_W {
        B1177_W { w: self }
    }
    #[doc = "Bit 26 - B1178"]
    #[inline(always)]
    pub fn b1178(&mut self) -> B1178_W {
        B1178_W { w: self }
    }
    #[doc = "Bit 27 - B1179"]
    #[inline(always)]
    pub fn b1179(&mut self) -> B1179_W {
        B1179_W { w: self }
    }
    #[doc = "Bit 28 - B1180"]
    #[inline(always)]
    pub fn b1180(&mut self) -> B1180_W {
        B1180_W { w: self }
    }
    #[doc = "Bit 29 - B1181"]
    #[inline(always)]
    pub fn b1181(&mut self) -> B1181_W {
        B1181_W { w: self }
    }
    #[doc = "Bit 30 - B1182"]
    #[inline(always)]
    pub fn b1182(&mut self) -> B1182_W {
        B1182_W { w: self }
    }
    #[doc = "Bit 31 - B1183"]
    #[inline(always)]
    pub fn b1183(&mut self) -> B1183_W {
        B1183_W { w: self }
    }
}
