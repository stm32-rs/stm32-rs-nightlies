#[doc = "Reader of register MPCBB2_VCTR37"]
pub type R = crate::R<u32, super::MPCBB2_VCTR37>;
#[doc = "Writer for register MPCBB2_VCTR37"]
pub type W = crate::W<u32, super::MPCBB2_VCTR37>;
#[doc = "Register MPCBB2_VCTR37 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR37 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1184`"]
pub type B1184_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1184`"]
pub struct B1184_W<'a> {
    w: &'a mut W,
}
impl<'a> B1184_W<'a> {
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
#[doc = "Reader of field `B1185`"]
pub type B1185_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1185`"]
pub struct B1185_W<'a> {
    w: &'a mut W,
}
impl<'a> B1185_W<'a> {
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
#[doc = "Reader of field `B1186`"]
pub type B1186_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1186`"]
pub struct B1186_W<'a> {
    w: &'a mut W,
}
impl<'a> B1186_W<'a> {
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
#[doc = "Reader of field `B1187`"]
pub type B1187_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1187`"]
pub struct B1187_W<'a> {
    w: &'a mut W,
}
impl<'a> B1187_W<'a> {
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
#[doc = "Reader of field `B1188`"]
pub type B1188_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1188`"]
pub struct B1188_W<'a> {
    w: &'a mut W,
}
impl<'a> B1188_W<'a> {
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
#[doc = "Reader of field `B1189`"]
pub type B1189_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1189`"]
pub struct B1189_W<'a> {
    w: &'a mut W,
}
impl<'a> B1189_W<'a> {
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
#[doc = "Reader of field `B1190`"]
pub type B1190_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1190`"]
pub struct B1190_W<'a> {
    w: &'a mut W,
}
impl<'a> B1190_W<'a> {
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
#[doc = "Reader of field `B1191`"]
pub type B1191_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1191`"]
pub struct B1191_W<'a> {
    w: &'a mut W,
}
impl<'a> B1191_W<'a> {
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
#[doc = "Reader of field `B1192`"]
pub type B1192_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1192`"]
pub struct B1192_W<'a> {
    w: &'a mut W,
}
impl<'a> B1192_W<'a> {
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
#[doc = "Reader of field `B1193`"]
pub type B1193_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1193`"]
pub struct B1193_W<'a> {
    w: &'a mut W,
}
impl<'a> B1193_W<'a> {
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
#[doc = "Reader of field `B1194`"]
pub type B1194_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1194`"]
pub struct B1194_W<'a> {
    w: &'a mut W,
}
impl<'a> B1194_W<'a> {
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
#[doc = "Reader of field `B1195`"]
pub type B1195_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1195`"]
pub struct B1195_W<'a> {
    w: &'a mut W,
}
impl<'a> B1195_W<'a> {
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
#[doc = "Reader of field `B1196`"]
pub type B1196_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1196`"]
pub struct B1196_W<'a> {
    w: &'a mut W,
}
impl<'a> B1196_W<'a> {
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
#[doc = "Reader of field `B1197`"]
pub type B1197_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1197`"]
pub struct B1197_W<'a> {
    w: &'a mut W,
}
impl<'a> B1197_W<'a> {
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
#[doc = "Reader of field `B1198`"]
pub type B1198_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1198`"]
pub struct B1198_W<'a> {
    w: &'a mut W,
}
impl<'a> B1198_W<'a> {
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
#[doc = "Reader of field `B1199`"]
pub type B1199_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1199`"]
pub struct B1199_W<'a> {
    w: &'a mut W,
}
impl<'a> B1199_W<'a> {
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
#[doc = "Reader of field `B1200`"]
pub type B1200_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1200`"]
pub struct B1200_W<'a> {
    w: &'a mut W,
}
impl<'a> B1200_W<'a> {
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
#[doc = "Reader of field `B1201`"]
pub type B1201_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1201`"]
pub struct B1201_W<'a> {
    w: &'a mut W,
}
impl<'a> B1201_W<'a> {
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
#[doc = "Reader of field `B1202`"]
pub type B1202_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1202`"]
pub struct B1202_W<'a> {
    w: &'a mut W,
}
impl<'a> B1202_W<'a> {
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
#[doc = "Reader of field `B1203`"]
pub type B1203_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1203`"]
pub struct B1203_W<'a> {
    w: &'a mut W,
}
impl<'a> B1203_W<'a> {
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
#[doc = "Reader of field `B1204`"]
pub type B1204_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1204`"]
pub struct B1204_W<'a> {
    w: &'a mut W,
}
impl<'a> B1204_W<'a> {
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
#[doc = "Reader of field `B1205`"]
pub type B1205_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1205`"]
pub struct B1205_W<'a> {
    w: &'a mut W,
}
impl<'a> B1205_W<'a> {
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
#[doc = "Reader of field `B1206`"]
pub type B1206_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1206`"]
pub struct B1206_W<'a> {
    w: &'a mut W,
}
impl<'a> B1206_W<'a> {
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
#[doc = "Reader of field `B1207`"]
pub type B1207_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1207`"]
pub struct B1207_W<'a> {
    w: &'a mut W,
}
impl<'a> B1207_W<'a> {
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
#[doc = "Reader of field `B1208`"]
pub type B1208_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1208`"]
pub struct B1208_W<'a> {
    w: &'a mut W,
}
impl<'a> B1208_W<'a> {
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
#[doc = "Reader of field `B1209`"]
pub type B1209_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1209`"]
pub struct B1209_W<'a> {
    w: &'a mut W,
}
impl<'a> B1209_W<'a> {
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
#[doc = "Reader of field `B1210`"]
pub type B1210_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1210`"]
pub struct B1210_W<'a> {
    w: &'a mut W,
}
impl<'a> B1210_W<'a> {
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
#[doc = "Reader of field `B1211`"]
pub type B1211_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1211`"]
pub struct B1211_W<'a> {
    w: &'a mut W,
}
impl<'a> B1211_W<'a> {
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
#[doc = "Reader of field `B1212`"]
pub type B1212_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1212`"]
pub struct B1212_W<'a> {
    w: &'a mut W,
}
impl<'a> B1212_W<'a> {
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
#[doc = "Reader of field `B1213`"]
pub type B1213_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1213`"]
pub struct B1213_W<'a> {
    w: &'a mut W,
}
impl<'a> B1213_W<'a> {
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
#[doc = "Reader of field `B1214`"]
pub type B1214_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1214`"]
pub struct B1214_W<'a> {
    w: &'a mut W,
}
impl<'a> B1214_W<'a> {
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
#[doc = "Reader of field `B1215`"]
pub type B1215_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1215`"]
pub struct B1215_W<'a> {
    w: &'a mut W,
}
impl<'a> B1215_W<'a> {
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
    #[doc = "Bit 0 - B1184"]
    #[inline(always)]
    pub fn b1184(&self) -> B1184_R {
        B1184_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1185"]
    #[inline(always)]
    pub fn b1185(&self) -> B1185_R {
        B1185_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1186"]
    #[inline(always)]
    pub fn b1186(&self) -> B1186_R {
        B1186_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1187"]
    #[inline(always)]
    pub fn b1187(&self) -> B1187_R {
        B1187_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1188"]
    #[inline(always)]
    pub fn b1188(&self) -> B1188_R {
        B1188_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1189"]
    #[inline(always)]
    pub fn b1189(&self) -> B1189_R {
        B1189_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1190"]
    #[inline(always)]
    pub fn b1190(&self) -> B1190_R {
        B1190_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1191"]
    #[inline(always)]
    pub fn b1191(&self) -> B1191_R {
        B1191_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1192"]
    #[inline(always)]
    pub fn b1192(&self) -> B1192_R {
        B1192_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1193"]
    #[inline(always)]
    pub fn b1193(&self) -> B1193_R {
        B1193_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1194"]
    #[inline(always)]
    pub fn b1194(&self) -> B1194_R {
        B1194_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1195"]
    #[inline(always)]
    pub fn b1195(&self) -> B1195_R {
        B1195_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1196"]
    #[inline(always)]
    pub fn b1196(&self) -> B1196_R {
        B1196_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1197"]
    #[inline(always)]
    pub fn b1197(&self) -> B1197_R {
        B1197_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1198"]
    #[inline(always)]
    pub fn b1198(&self) -> B1198_R {
        B1198_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1199"]
    #[inline(always)]
    pub fn b1199(&self) -> B1199_R {
        B1199_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1200"]
    #[inline(always)]
    pub fn b1200(&self) -> B1200_R {
        B1200_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1201"]
    #[inline(always)]
    pub fn b1201(&self) -> B1201_R {
        B1201_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1202"]
    #[inline(always)]
    pub fn b1202(&self) -> B1202_R {
        B1202_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1203"]
    #[inline(always)]
    pub fn b1203(&self) -> B1203_R {
        B1203_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1204"]
    #[inline(always)]
    pub fn b1204(&self) -> B1204_R {
        B1204_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1205"]
    #[inline(always)]
    pub fn b1205(&self) -> B1205_R {
        B1205_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1206"]
    #[inline(always)]
    pub fn b1206(&self) -> B1206_R {
        B1206_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1207"]
    #[inline(always)]
    pub fn b1207(&self) -> B1207_R {
        B1207_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1208"]
    #[inline(always)]
    pub fn b1208(&self) -> B1208_R {
        B1208_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1209"]
    #[inline(always)]
    pub fn b1209(&self) -> B1209_R {
        B1209_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1210"]
    #[inline(always)]
    pub fn b1210(&self) -> B1210_R {
        B1210_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1211"]
    #[inline(always)]
    pub fn b1211(&self) -> B1211_R {
        B1211_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1212"]
    #[inline(always)]
    pub fn b1212(&self) -> B1212_R {
        B1212_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1213"]
    #[inline(always)]
    pub fn b1213(&self) -> B1213_R {
        B1213_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1214"]
    #[inline(always)]
    pub fn b1214(&self) -> B1214_R {
        B1214_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1215"]
    #[inline(always)]
    pub fn b1215(&self) -> B1215_R {
        B1215_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1184"]
    #[inline(always)]
    pub fn b1184(&mut self) -> B1184_W {
        B1184_W { w: self }
    }
    #[doc = "Bit 1 - B1185"]
    #[inline(always)]
    pub fn b1185(&mut self) -> B1185_W {
        B1185_W { w: self }
    }
    #[doc = "Bit 2 - B1186"]
    #[inline(always)]
    pub fn b1186(&mut self) -> B1186_W {
        B1186_W { w: self }
    }
    #[doc = "Bit 3 - B1187"]
    #[inline(always)]
    pub fn b1187(&mut self) -> B1187_W {
        B1187_W { w: self }
    }
    #[doc = "Bit 4 - B1188"]
    #[inline(always)]
    pub fn b1188(&mut self) -> B1188_W {
        B1188_W { w: self }
    }
    #[doc = "Bit 5 - B1189"]
    #[inline(always)]
    pub fn b1189(&mut self) -> B1189_W {
        B1189_W { w: self }
    }
    #[doc = "Bit 6 - B1190"]
    #[inline(always)]
    pub fn b1190(&mut self) -> B1190_W {
        B1190_W { w: self }
    }
    #[doc = "Bit 7 - B1191"]
    #[inline(always)]
    pub fn b1191(&mut self) -> B1191_W {
        B1191_W { w: self }
    }
    #[doc = "Bit 8 - B1192"]
    #[inline(always)]
    pub fn b1192(&mut self) -> B1192_W {
        B1192_W { w: self }
    }
    #[doc = "Bit 9 - B1193"]
    #[inline(always)]
    pub fn b1193(&mut self) -> B1193_W {
        B1193_W { w: self }
    }
    #[doc = "Bit 10 - B1194"]
    #[inline(always)]
    pub fn b1194(&mut self) -> B1194_W {
        B1194_W { w: self }
    }
    #[doc = "Bit 11 - B1195"]
    #[inline(always)]
    pub fn b1195(&mut self) -> B1195_W {
        B1195_W { w: self }
    }
    #[doc = "Bit 12 - B1196"]
    #[inline(always)]
    pub fn b1196(&mut self) -> B1196_W {
        B1196_W { w: self }
    }
    #[doc = "Bit 13 - B1197"]
    #[inline(always)]
    pub fn b1197(&mut self) -> B1197_W {
        B1197_W { w: self }
    }
    #[doc = "Bit 14 - B1198"]
    #[inline(always)]
    pub fn b1198(&mut self) -> B1198_W {
        B1198_W { w: self }
    }
    #[doc = "Bit 15 - B1199"]
    #[inline(always)]
    pub fn b1199(&mut self) -> B1199_W {
        B1199_W { w: self }
    }
    #[doc = "Bit 16 - B1200"]
    #[inline(always)]
    pub fn b1200(&mut self) -> B1200_W {
        B1200_W { w: self }
    }
    #[doc = "Bit 17 - B1201"]
    #[inline(always)]
    pub fn b1201(&mut self) -> B1201_W {
        B1201_W { w: self }
    }
    #[doc = "Bit 18 - B1202"]
    #[inline(always)]
    pub fn b1202(&mut self) -> B1202_W {
        B1202_W { w: self }
    }
    #[doc = "Bit 19 - B1203"]
    #[inline(always)]
    pub fn b1203(&mut self) -> B1203_W {
        B1203_W { w: self }
    }
    #[doc = "Bit 20 - B1204"]
    #[inline(always)]
    pub fn b1204(&mut self) -> B1204_W {
        B1204_W { w: self }
    }
    #[doc = "Bit 21 - B1205"]
    #[inline(always)]
    pub fn b1205(&mut self) -> B1205_W {
        B1205_W { w: self }
    }
    #[doc = "Bit 22 - B1206"]
    #[inline(always)]
    pub fn b1206(&mut self) -> B1206_W {
        B1206_W { w: self }
    }
    #[doc = "Bit 23 - B1207"]
    #[inline(always)]
    pub fn b1207(&mut self) -> B1207_W {
        B1207_W { w: self }
    }
    #[doc = "Bit 24 - B1208"]
    #[inline(always)]
    pub fn b1208(&mut self) -> B1208_W {
        B1208_W { w: self }
    }
    #[doc = "Bit 25 - B1209"]
    #[inline(always)]
    pub fn b1209(&mut self) -> B1209_W {
        B1209_W { w: self }
    }
    #[doc = "Bit 26 - B1210"]
    #[inline(always)]
    pub fn b1210(&mut self) -> B1210_W {
        B1210_W { w: self }
    }
    #[doc = "Bit 27 - B1211"]
    #[inline(always)]
    pub fn b1211(&mut self) -> B1211_W {
        B1211_W { w: self }
    }
    #[doc = "Bit 28 - B1212"]
    #[inline(always)]
    pub fn b1212(&mut self) -> B1212_W {
        B1212_W { w: self }
    }
    #[doc = "Bit 29 - B1213"]
    #[inline(always)]
    pub fn b1213(&mut self) -> B1213_W {
        B1213_W { w: self }
    }
    #[doc = "Bit 30 - B1214"]
    #[inline(always)]
    pub fn b1214(&mut self) -> B1214_W {
        B1214_W { w: self }
    }
    #[doc = "Bit 31 - B1215"]
    #[inline(always)]
    pub fn b1215(&mut self) -> B1215_W {
        B1215_W { w: self }
    }
}
