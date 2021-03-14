#[doc = "Reader of register MPCBB2_VCTR38"]
pub type R = crate::R<u32, super::MPCBB2_VCTR38>;
#[doc = "Writer for register MPCBB2_VCTR38"]
pub type W = crate::W<u32, super::MPCBB2_VCTR38>;
#[doc = "Register MPCBB2_VCTR38 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1216`"]
pub type B1216_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1216`"]
pub struct B1216_W<'a> {
    w: &'a mut W,
}
impl<'a> B1216_W<'a> {
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
#[doc = "Reader of field `B1217`"]
pub type B1217_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1217`"]
pub struct B1217_W<'a> {
    w: &'a mut W,
}
impl<'a> B1217_W<'a> {
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
#[doc = "Reader of field `B1218`"]
pub type B1218_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1218`"]
pub struct B1218_W<'a> {
    w: &'a mut W,
}
impl<'a> B1218_W<'a> {
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
#[doc = "Reader of field `B1219`"]
pub type B1219_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1219`"]
pub struct B1219_W<'a> {
    w: &'a mut W,
}
impl<'a> B1219_W<'a> {
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
#[doc = "Reader of field `B1220`"]
pub type B1220_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1220`"]
pub struct B1220_W<'a> {
    w: &'a mut W,
}
impl<'a> B1220_W<'a> {
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
#[doc = "Reader of field `B1221`"]
pub type B1221_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1221`"]
pub struct B1221_W<'a> {
    w: &'a mut W,
}
impl<'a> B1221_W<'a> {
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
#[doc = "Reader of field `B1222`"]
pub type B1222_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1222`"]
pub struct B1222_W<'a> {
    w: &'a mut W,
}
impl<'a> B1222_W<'a> {
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
#[doc = "Reader of field `B1223`"]
pub type B1223_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1223`"]
pub struct B1223_W<'a> {
    w: &'a mut W,
}
impl<'a> B1223_W<'a> {
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
#[doc = "Reader of field `B1224`"]
pub type B1224_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1224`"]
pub struct B1224_W<'a> {
    w: &'a mut W,
}
impl<'a> B1224_W<'a> {
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
#[doc = "Reader of field `B1225`"]
pub type B1225_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1225`"]
pub struct B1225_W<'a> {
    w: &'a mut W,
}
impl<'a> B1225_W<'a> {
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
#[doc = "Reader of field `B1226`"]
pub type B1226_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1226`"]
pub struct B1226_W<'a> {
    w: &'a mut W,
}
impl<'a> B1226_W<'a> {
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
#[doc = "Reader of field `B1227`"]
pub type B1227_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1227`"]
pub struct B1227_W<'a> {
    w: &'a mut W,
}
impl<'a> B1227_W<'a> {
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
#[doc = "Reader of field `B1228`"]
pub type B1228_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1228`"]
pub struct B1228_W<'a> {
    w: &'a mut W,
}
impl<'a> B1228_W<'a> {
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
#[doc = "Reader of field `B1229`"]
pub type B1229_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1229`"]
pub struct B1229_W<'a> {
    w: &'a mut W,
}
impl<'a> B1229_W<'a> {
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
#[doc = "Reader of field `B1230`"]
pub type B1230_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1230`"]
pub struct B1230_W<'a> {
    w: &'a mut W,
}
impl<'a> B1230_W<'a> {
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
#[doc = "Reader of field `B1231`"]
pub type B1231_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1231`"]
pub struct B1231_W<'a> {
    w: &'a mut W,
}
impl<'a> B1231_W<'a> {
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
#[doc = "Reader of field `B1232`"]
pub type B1232_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1232`"]
pub struct B1232_W<'a> {
    w: &'a mut W,
}
impl<'a> B1232_W<'a> {
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
#[doc = "Reader of field `B1233`"]
pub type B1233_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1233`"]
pub struct B1233_W<'a> {
    w: &'a mut W,
}
impl<'a> B1233_W<'a> {
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
#[doc = "Reader of field `B1234`"]
pub type B1234_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1234`"]
pub struct B1234_W<'a> {
    w: &'a mut W,
}
impl<'a> B1234_W<'a> {
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
#[doc = "Reader of field `B1235`"]
pub type B1235_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1235`"]
pub struct B1235_W<'a> {
    w: &'a mut W,
}
impl<'a> B1235_W<'a> {
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
#[doc = "Reader of field `B1236`"]
pub type B1236_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1236`"]
pub struct B1236_W<'a> {
    w: &'a mut W,
}
impl<'a> B1236_W<'a> {
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
#[doc = "Reader of field `B1237`"]
pub type B1237_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1237`"]
pub struct B1237_W<'a> {
    w: &'a mut W,
}
impl<'a> B1237_W<'a> {
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
#[doc = "Reader of field `B1238`"]
pub type B1238_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1238`"]
pub struct B1238_W<'a> {
    w: &'a mut W,
}
impl<'a> B1238_W<'a> {
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
#[doc = "Reader of field `B1239`"]
pub type B1239_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1239`"]
pub struct B1239_W<'a> {
    w: &'a mut W,
}
impl<'a> B1239_W<'a> {
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
#[doc = "Reader of field `B1240`"]
pub type B1240_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1240`"]
pub struct B1240_W<'a> {
    w: &'a mut W,
}
impl<'a> B1240_W<'a> {
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
#[doc = "Reader of field `B1241`"]
pub type B1241_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1241`"]
pub struct B1241_W<'a> {
    w: &'a mut W,
}
impl<'a> B1241_W<'a> {
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
#[doc = "Reader of field `B1242`"]
pub type B1242_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1242`"]
pub struct B1242_W<'a> {
    w: &'a mut W,
}
impl<'a> B1242_W<'a> {
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
#[doc = "Reader of field `B1243`"]
pub type B1243_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1243`"]
pub struct B1243_W<'a> {
    w: &'a mut W,
}
impl<'a> B1243_W<'a> {
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
#[doc = "Reader of field `B1244`"]
pub type B1244_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1244`"]
pub struct B1244_W<'a> {
    w: &'a mut W,
}
impl<'a> B1244_W<'a> {
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
#[doc = "Reader of field `B1245`"]
pub type B1245_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1245`"]
pub struct B1245_W<'a> {
    w: &'a mut W,
}
impl<'a> B1245_W<'a> {
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
#[doc = "Reader of field `B1246`"]
pub type B1246_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1246`"]
pub struct B1246_W<'a> {
    w: &'a mut W,
}
impl<'a> B1246_W<'a> {
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
#[doc = "Reader of field `B1247`"]
pub type B1247_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1247`"]
pub struct B1247_W<'a> {
    w: &'a mut W,
}
impl<'a> B1247_W<'a> {
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
    #[doc = "Bit 0 - B1216"]
    #[inline(always)]
    pub fn b1216(&self) -> B1216_R {
        B1216_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1217"]
    #[inline(always)]
    pub fn b1217(&self) -> B1217_R {
        B1217_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1218"]
    #[inline(always)]
    pub fn b1218(&self) -> B1218_R {
        B1218_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1219"]
    #[inline(always)]
    pub fn b1219(&self) -> B1219_R {
        B1219_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1220"]
    #[inline(always)]
    pub fn b1220(&self) -> B1220_R {
        B1220_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1221"]
    #[inline(always)]
    pub fn b1221(&self) -> B1221_R {
        B1221_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1222"]
    #[inline(always)]
    pub fn b1222(&self) -> B1222_R {
        B1222_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1223"]
    #[inline(always)]
    pub fn b1223(&self) -> B1223_R {
        B1223_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1224"]
    #[inline(always)]
    pub fn b1224(&self) -> B1224_R {
        B1224_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1225"]
    #[inline(always)]
    pub fn b1225(&self) -> B1225_R {
        B1225_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1226"]
    #[inline(always)]
    pub fn b1226(&self) -> B1226_R {
        B1226_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1227"]
    #[inline(always)]
    pub fn b1227(&self) -> B1227_R {
        B1227_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1228"]
    #[inline(always)]
    pub fn b1228(&self) -> B1228_R {
        B1228_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1229"]
    #[inline(always)]
    pub fn b1229(&self) -> B1229_R {
        B1229_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1230"]
    #[inline(always)]
    pub fn b1230(&self) -> B1230_R {
        B1230_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1231"]
    #[inline(always)]
    pub fn b1231(&self) -> B1231_R {
        B1231_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1232"]
    #[inline(always)]
    pub fn b1232(&self) -> B1232_R {
        B1232_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1233"]
    #[inline(always)]
    pub fn b1233(&self) -> B1233_R {
        B1233_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1234"]
    #[inline(always)]
    pub fn b1234(&self) -> B1234_R {
        B1234_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1235"]
    #[inline(always)]
    pub fn b1235(&self) -> B1235_R {
        B1235_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1236"]
    #[inline(always)]
    pub fn b1236(&self) -> B1236_R {
        B1236_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1237"]
    #[inline(always)]
    pub fn b1237(&self) -> B1237_R {
        B1237_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1238"]
    #[inline(always)]
    pub fn b1238(&self) -> B1238_R {
        B1238_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1239"]
    #[inline(always)]
    pub fn b1239(&self) -> B1239_R {
        B1239_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1240"]
    #[inline(always)]
    pub fn b1240(&self) -> B1240_R {
        B1240_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1241"]
    #[inline(always)]
    pub fn b1241(&self) -> B1241_R {
        B1241_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1242"]
    #[inline(always)]
    pub fn b1242(&self) -> B1242_R {
        B1242_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1243"]
    #[inline(always)]
    pub fn b1243(&self) -> B1243_R {
        B1243_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1244"]
    #[inline(always)]
    pub fn b1244(&self) -> B1244_R {
        B1244_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1245"]
    #[inline(always)]
    pub fn b1245(&self) -> B1245_R {
        B1245_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1246"]
    #[inline(always)]
    pub fn b1246(&self) -> B1246_R {
        B1246_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1247"]
    #[inline(always)]
    pub fn b1247(&self) -> B1247_R {
        B1247_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1216"]
    #[inline(always)]
    pub fn b1216(&mut self) -> B1216_W {
        B1216_W { w: self }
    }
    #[doc = "Bit 1 - B1217"]
    #[inline(always)]
    pub fn b1217(&mut self) -> B1217_W {
        B1217_W { w: self }
    }
    #[doc = "Bit 2 - B1218"]
    #[inline(always)]
    pub fn b1218(&mut self) -> B1218_W {
        B1218_W { w: self }
    }
    #[doc = "Bit 3 - B1219"]
    #[inline(always)]
    pub fn b1219(&mut self) -> B1219_W {
        B1219_W { w: self }
    }
    #[doc = "Bit 4 - B1220"]
    #[inline(always)]
    pub fn b1220(&mut self) -> B1220_W {
        B1220_W { w: self }
    }
    #[doc = "Bit 5 - B1221"]
    #[inline(always)]
    pub fn b1221(&mut self) -> B1221_W {
        B1221_W { w: self }
    }
    #[doc = "Bit 6 - B1222"]
    #[inline(always)]
    pub fn b1222(&mut self) -> B1222_W {
        B1222_W { w: self }
    }
    #[doc = "Bit 7 - B1223"]
    #[inline(always)]
    pub fn b1223(&mut self) -> B1223_W {
        B1223_W { w: self }
    }
    #[doc = "Bit 8 - B1224"]
    #[inline(always)]
    pub fn b1224(&mut self) -> B1224_W {
        B1224_W { w: self }
    }
    #[doc = "Bit 9 - B1225"]
    #[inline(always)]
    pub fn b1225(&mut self) -> B1225_W {
        B1225_W { w: self }
    }
    #[doc = "Bit 10 - B1226"]
    #[inline(always)]
    pub fn b1226(&mut self) -> B1226_W {
        B1226_W { w: self }
    }
    #[doc = "Bit 11 - B1227"]
    #[inline(always)]
    pub fn b1227(&mut self) -> B1227_W {
        B1227_W { w: self }
    }
    #[doc = "Bit 12 - B1228"]
    #[inline(always)]
    pub fn b1228(&mut self) -> B1228_W {
        B1228_W { w: self }
    }
    #[doc = "Bit 13 - B1229"]
    #[inline(always)]
    pub fn b1229(&mut self) -> B1229_W {
        B1229_W { w: self }
    }
    #[doc = "Bit 14 - B1230"]
    #[inline(always)]
    pub fn b1230(&mut self) -> B1230_W {
        B1230_W { w: self }
    }
    #[doc = "Bit 15 - B1231"]
    #[inline(always)]
    pub fn b1231(&mut self) -> B1231_W {
        B1231_W { w: self }
    }
    #[doc = "Bit 16 - B1232"]
    #[inline(always)]
    pub fn b1232(&mut self) -> B1232_W {
        B1232_W { w: self }
    }
    #[doc = "Bit 17 - B1233"]
    #[inline(always)]
    pub fn b1233(&mut self) -> B1233_W {
        B1233_W { w: self }
    }
    #[doc = "Bit 18 - B1234"]
    #[inline(always)]
    pub fn b1234(&mut self) -> B1234_W {
        B1234_W { w: self }
    }
    #[doc = "Bit 19 - B1235"]
    #[inline(always)]
    pub fn b1235(&mut self) -> B1235_W {
        B1235_W { w: self }
    }
    #[doc = "Bit 20 - B1236"]
    #[inline(always)]
    pub fn b1236(&mut self) -> B1236_W {
        B1236_W { w: self }
    }
    #[doc = "Bit 21 - B1237"]
    #[inline(always)]
    pub fn b1237(&mut self) -> B1237_W {
        B1237_W { w: self }
    }
    #[doc = "Bit 22 - B1238"]
    #[inline(always)]
    pub fn b1238(&mut self) -> B1238_W {
        B1238_W { w: self }
    }
    #[doc = "Bit 23 - B1239"]
    #[inline(always)]
    pub fn b1239(&mut self) -> B1239_W {
        B1239_W { w: self }
    }
    #[doc = "Bit 24 - B1240"]
    #[inline(always)]
    pub fn b1240(&mut self) -> B1240_W {
        B1240_W { w: self }
    }
    #[doc = "Bit 25 - B1241"]
    #[inline(always)]
    pub fn b1241(&mut self) -> B1241_W {
        B1241_W { w: self }
    }
    #[doc = "Bit 26 - B1242"]
    #[inline(always)]
    pub fn b1242(&mut self) -> B1242_W {
        B1242_W { w: self }
    }
    #[doc = "Bit 27 - B1243"]
    #[inline(always)]
    pub fn b1243(&mut self) -> B1243_W {
        B1243_W { w: self }
    }
    #[doc = "Bit 28 - B1244"]
    #[inline(always)]
    pub fn b1244(&mut self) -> B1244_W {
        B1244_W { w: self }
    }
    #[doc = "Bit 29 - B1245"]
    #[inline(always)]
    pub fn b1245(&mut self) -> B1245_W {
        B1245_W { w: self }
    }
    #[doc = "Bit 30 - B1246"]
    #[inline(always)]
    pub fn b1246(&mut self) -> B1246_W {
        B1246_W { w: self }
    }
    #[doc = "Bit 31 - B1247"]
    #[inline(always)]
    pub fn b1247(&mut self) -> B1247_W {
        B1247_W { w: self }
    }
}
