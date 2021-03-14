#[doc = "Reader of register MPCBB2_VCTR35"]
pub type R = crate::R<u32, super::MPCBB2_VCTR35>;
#[doc = "Writer for register MPCBB2_VCTR35"]
pub type W = crate::W<u32, super::MPCBB2_VCTR35>;
#[doc = "Register MPCBB2_VCTR35 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR35 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1120`"]
pub type B1120_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1120`"]
pub struct B1120_W<'a> {
    w: &'a mut W,
}
impl<'a> B1120_W<'a> {
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
#[doc = "Reader of field `B1121`"]
pub type B1121_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1121`"]
pub struct B1121_W<'a> {
    w: &'a mut W,
}
impl<'a> B1121_W<'a> {
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
#[doc = "Reader of field `B1122`"]
pub type B1122_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1122`"]
pub struct B1122_W<'a> {
    w: &'a mut W,
}
impl<'a> B1122_W<'a> {
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
#[doc = "Reader of field `B1123`"]
pub type B1123_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1123`"]
pub struct B1123_W<'a> {
    w: &'a mut W,
}
impl<'a> B1123_W<'a> {
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
#[doc = "Reader of field `B1124`"]
pub type B1124_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1124`"]
pub struct B1124_W<'a> {
    w: &'a mut W,
}
impl<'a> B1124_W<'a> {
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
#[doc = "Reader of field `B1125`"]
pub type B1125_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1125`"]
pub struct B1125_W<'a> {
    w: &'a mut W,
}
impl<'a> B1125_W<'a> {
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
#[doc = "Reader of field `B1126`"]
pub type B1126_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1126`"]
pub struct B1126_W<'a> {
    w: &'a mut W,
}
impl<'a> B1126_W<'a> {
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
#[doc = "Reader of field `B1127`"]
pub type B1127_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1127`"]
pub struct B1127_W<'a> {
    w: &'a mut W,
}
impl<'a> B1127_W<'a> {
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
#[doc = "Reader of field `B1128`"]
pub type B1128_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1128`"]
pub struct B1128_W<'a> {
    w: &'a mut W,
}
impl<'a> B1128_W<'a> {
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
#[doc = "Reader of field `B1129`"]
pub type B1129_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1129`"]
pub struct B1129_W<'a> {
    w: &'a mut W,
}
impl<'a> B1129_W<'a> {
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
#[doc = "Reader of field `B1130`"]
pub type B1130_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1130`"]
pub struct B1130_W<'a> {
    w: &'a mut W,
}
impl<'a> B1130_W<'a> {
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
#[doc = "Reader of field `B1131`"]
pub type B1131_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1131`"]
pub struct B1131_W<'a> {
    w: &'a mut W,
}
impl<'a> B1131_W<'a> {
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
#[doc = "Reader of field `B1132`"]
pub type B1132_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1132`"]
pub struct B1132_W<'a> {
    w: &'a mut W,
}
impl<'a> B1132_W<'a> {
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
#[doc = "Reader of field `B1133`"]
pub type B1133_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1133`"]
pub struct B1133_W<'a> {
    w: &'a mut W,
}
impl<'a> B1133_W<'a> {
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
#[doc = "Reader of field `B1134`"]
pub type B1134_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1134`"]
pub struct B1134_W<'a> {
    w: &'a mut W,
}
impl<'a> B1134_W<'a> {
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
#[doc = "Reader of field `B1135`"]
pub type B1135_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1135`"]
pub struct B1135_W<'a> {
    w: &'a mut W,
}
impl<'a> B1135_W<'a> {
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
#[doc = "Reader of field `B1136`"]
pub type B1136_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1136`"]
pub struct B1136_W<'a> {
    w: &'a mut W,
}
impl<'a> B1136_W<'a> {
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
#[doc = "Reader of field `B1137`"]
pub type B1137_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1137`"]
pub struct B1137_W<'a> {
    w: &'a mut W,
}
impl<'a> B1137_W<'a> {
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
#[doc = "Reader of field `B1138`"]
pub type B1138_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1138`"]
pub struct B1138_W<'a> {
    w: &'a mut W,
}
impl<'a> B1138_W<'a> {
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
#[doc = "Reader of field `B1139`"]
pub type B1139_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1139`"]
pub struct B1139_W<'a> {
    w: &'a mut W,
}
impl<'a> B1139_W<'a> {
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
#[doc = "Reader of field `B1140`"]
pub type B1140_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1140`"]
pub struct B1140_W<'a> {
    w: &'a mut W,
}
impl<'a> B1140_W<'a> {
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
#[doc = "Reader of field `B1141`"]
pub type B1141_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1141`"]
pub struct B1141_W<'a> {
    w: &'a mut W,
}
impl<'a> B1141_W<'a> {
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
#[doc = "Reader of field `B1142`"]
pub type B1142_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1142`"]
pub struct B1142_W<'a> {
    w: &'a mut W,
}
impl<'a> B1142_W<'a> {
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
#[doc = "Reader of field `B1143`"]
pub type B1143_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1143`"]
pub struct B1143_W<'a> {
    w: &'a mut W,
}
impl<'a> B1143_W<'a> {
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
#[doc = "Reader of field `B1144`"]
pub type B1144_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1144`"]
pub struct B1144_W<'a> {
    w: &'a mut W,
}
impl<'a> B1144_W<'a> {
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
#[doc = "Reader of field `B1145`"]
pub type B1145_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1145`"]
pub struct B1145_W<'a> {
    w: &'a mut W,
}
impl<'a> B1145_W<'a> {
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
#[doc = "Reader of field `B1146`"]
pub type B1146_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1146`"]
pub struct B1146_W<'a> {
    w: &'a mut W,
}
impl<'a> B1146_W<'a> {
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
#[doc = "Reader of field `B1147`"]
pub type B1147_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1147`"]
pub struct B1147_W<'a> {
    w: &'a mut W,
}
impl<'a> B1147_W<'a> {
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
#[doc = "Reader of field `B1148`"]
pub type B1148_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1148`"]
pub struct B1148_W<'a> {
    w: &'a mut W,
}
impl<'a> B1148_W<'a> {
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
#[doc = "Reader of field `B1149`"]
pub type B1149_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1149`"]
pub struct B1149_W<'a> {
    w: &'a mut W,
}
impl<'a> B1149_W<'a> {
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
#[doc = "Reader of field `B1150`"]
pub type B1150_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1150`"]
pub struct B1150_W<'a> {
    w: &'a mut W,
}
impl<'a> B1150_W<'a> {
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
#[doc = "Reader of field `B1151`"]
pub type B1151_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1151`"]
pub struct B1151_W<'a> {
    w: &'a mut W,
}
impl<'a> B1151_W<'a> {
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
    #[doc = "Bit 0 - B1120"]
    #[inline(always)]
    pub fn b1120(&self) -> B1120_R {
        B1120_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1121"]
    #[inline(always)]
    pub fn b1121(&self) -> B1121_R {
        B1121_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1122"]
    #[inline(always)]
    pub fn b1122(&self) -> B1122_R {
        B1122_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1123"]
    #[inline(always)]
    pub fn b1123(&self) -> B1123_R {
        B1123_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1124"]
    #[inline(always)]
    pub fn b1124(&self) -> B1124_R {
        B1124_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1125"]
    #[inline(always)]
    pub fn b1125(&self) -> B1125_R {
        B1125_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1126"]
    #[inline(always)]
    pub fn b1126(&self) -> B1126_R {
        B1126_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1127"]
    #[inline(always)]
    pub fn b1127(&self) -> B1127_R {
        B1127_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1128"]
    #[inline(always)]
    pub fn b1128(&self) -> B1128_R {
        B1128_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1129"]
    #[inline(always)]
    pub fn b1129(&self) -> B1129_R {
        B1129_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1130"]
    #[inline(always)]
    pub fn b1130(&self) -> B1130_R {
        B1130_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1131"]
    #[inline(always)]
    pub fn b1131(&self) -> B1131_R {
        B1131_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1132"]
    #[inline(always)]
    pub fn b1132(&self) -> B1132_R {
        B1132_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1133"]
    #[inline(always)]
    pub fn b1133(&self) -> B1133_R {
        B1133_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1134"]
    #[inline(always)]
    pub fn b1134(&self) -> B1134_R {
        B1134_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1135"]
    #[inline(always)]
    pub fn b1135(&self) -> B1135_R {
        B1135_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1136"]
    #[inline(always)]
    pub fn b1136(&self) -> B1136_R {
        B1136_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1137"]
    #[inline(always)]
    pub fn b1137(&self) -> B1137_R {
        B1137_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1138"]
    #[inline(always)]
    pub fn b1138(&self) -> B1138_R {
        B1138_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1139"]
    #[inline(always)]
    pub fn b1139(&self) -> B1139_R {
        B1139_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1140"]
    #[inline(always)]
    pub fn b1140(&self) -> B1140_R {
        B1140_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1141"]
    #[inline(always)]
    pub fn b1141(&self) -> B1141_R {
        B1141_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1142"]
    #[inline(always)]
    pub fn b1142(&self) -> B1142_R {
        B1142_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1143"]
    #[inline(always)]
    pub fn b1143(&self) -> B1143_R {
        B1143_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1144"]
    #[inline(always)]
    pub fn b1144(&self) -> B1144_R {
        B1144_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1145"]
    #[inline(always)]
    pub fn b1145(&self) -> B1145_R {
        B1145_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1146"]
    #[inline(always)]
    pub fn b1146(&self) -> B1146_R {
        B1146_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1147"]
    #[inline(always)]
    pub fn b1147(&self) -> B1147_R {
        B1147_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1148"]
    #[inline(always)]
    pub fn b1148(&self) -> B1148_R {
        B1148_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1149"]
    #[inline(always)]
    pub fn b1149(&self) -> B1149_R {
        B1149_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1150"]
    #[inline(always)]
    pub fn b1150(&self) -> B1150_R {
        B1150_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1151"]
    #[inline(always)]
    pub fn b1151(&self) -> B1151_R {
        B1151_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1120"]
    #[inline(always)]
    pub fn b1120(&mut self) -> B1120_W {
        B1120_W { w: self }
    }
    #[doc = "Bit 1 - B1121"]
    #[inline(always)]
    pub fn b1121(&mut self) -> B1121_W {
        B1121_W { w: self }
    }
    #[doc = "Bit 2 - B1122"]
    #[inline(always)]
    pub fn b1122(&mut self) -> B1122_W {
        B1122_W { w: self }
    }
    #[doc = "Bit 3 - B1123"]
    #[inline(always)]
    pub fn b1123(&mut self) -> B1123_W {
        B1123_W { w: self }
    }
    #[doc = "Bit 4 - B1124"]
    #[inline(always)]
    pub fn b1124(&mut self) -> B1124_W {
        B1124_W { w: self }
    }
    #[doc = "Bit 5 - B1125"]
    #[inline(always)]
    pub fn b1125(&mut self) -> B1125_W {
        B1125_W { w: self }
    }
    #[doc = "Bit 6 - B1126"]
    #[inline(always)]
    pub fn b1126(&mut self) -> B1126_W {
        B1126_W { w: self }
    }
    #[doc = "Bit 7 - B1127"]
    #[inline(always)]
    pub fn b1127(&mut self) -> B1127_W {
        B1127_W { w: self }
    }
    #[doc = "Bit 8 - B1128"]
    #[inline(always)]
    pub fn b1128(&mut self) -> B1128_W {
        B1128_W { w: self }
    }
    #[doc = "Bit 9 - B1129"]
    #[inline(always)]
    pub fn b1129(&mut self) -> B1129_W {
        B1129_W { w: self }
    }
    #[doc = "Bit 10 - B1130"]
    #[inline(always)]
    pub fn b1130(&mut self) -> B1130_W {
        B1130_W { w: self }
    }
    #[doc = "Bit 11 - B1131"]
    #[inline(always)]
    pub fn b1131(&mut self) -> B1131_W {
        B1131_W { w: self }
    }
    #[doc = "Bit 12 - B1132"]
    #[inline(always)]
    pub fn b1132(&mut self) -> B1132_W {
        B1132_W { w: self }
    }
    #[doc = "Bit 13 - B1133"]
    #[inline(always)]
    pub fn b1133(&mut self) -> B1133_W {
        B1133_W { w: self }
    }
    #[doc = "Bit 14 - B1134"]
    #[inline(always)]
    pub fn b1134(&mut self) -> B1134_W {
        B1134_W { w: self }
    }
    #[doc = "Bit 15 - B1135"]
    #[inline(always)]
    pub fn b1135(&mut self) -> B1135_W {
        B1135_W { w: self }
    }
    #[doc = "Bit 16 - B1136"]
    #[inline(always)]
    pub fn b1136(&mut self) -> B1136_W {
        B1136_W { w: self }
    }
    #[doc = "Bit 17 - B1137"]
    #[inline(always)]
    pub fn b1137(&mut self) -> B1137_W {
        B1137_W { w: self }
    }
    #[doc = "Bit 18 - B1138"]
    #[inline(always)]
    pub fn b1138(&mut self) -> B1138_W {
        B1138_W { w: self }
    }
    #[doc = "Bit 19 - B1139"]
    #[inline(always)]
    pub fn b1139(&mut self) -> B1139_W {
        B1139_W { w: self }
    }
    #[doc = "Bit 20 - B1140"]
    #[inline(always)]
    pub fn b1140(&mut self) -> B1140_W {
        B1140_W { w: self }
    }
    #[doc = "Bit 21 - B1141"]
    #[inline(always)]
    pub fn b1141(&mut self) -> B1141_W {
        B1141_W { w: self }
    }
    #[doc = "Bit 22 - B1142"]
    #[inline(always)]
    pub fn b1142(&mut self) -> B1142_W {
        B1142_W { w: self }
    }
    #[doc = "Bit 23 - B1143"]
    #[inline(always)]
    pub fn b1143(&mut self) -> B1143_W {
        B1143_W { w: self }
    }
    #[doc = "Bit 24 - B1144"]
    #[inline(always)]
    pub fn b1144(&mut self) -> B1144_W {
        B1144_W { w: self }
    }
    #[doc = "Bit 25 - B1145"]
    #[inline(always)]
    pub fn b1145(&mut self) -> B1145_W {
        B1145_W { w: self }
    }
    #[doc = "Bit 26 - B1146"]
    #[inline(always)]
    pub fn b1146(&mut self) -> B1146_W {
        B1146_W { w: self }
    }
    #[doc = "Bit 27 - B1147"]
    #[inline(always)]
    pub fn b1147(&mut self) -> B1147_W {
        B1147_W { w: self }
    }
    #[doc = "Bit 28 - B1148"]
    #[inline(always)]
    pub fn b1148(&mut self) -> B1148_W {
        B1148_W { w: self }
    }
    #[doc = "Bit 29 - B1149"]
    #[inline(always)]
    pub fn b1149(&mut self) -> B1149_W {
        B1149_W { w: self }
    }
    #[doc = "Bit 30 - B1150"]
    #[inline(always)]
    pub fn b1150(&mut self) -> B1150_W {
        B1150_W { w: self }
    }
    #[doc = "Bit 31 - B1151"]
    #[inline(always)]
    pub fn b1151(&mut self) -> B1151_W {
        B1151_W { w: self }
    }
}
