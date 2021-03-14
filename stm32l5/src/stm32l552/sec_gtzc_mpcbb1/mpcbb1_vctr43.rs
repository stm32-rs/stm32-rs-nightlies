#[doc = "Reader of register MPCBB1_VCTR43"]
pub type R = crate::R<u32, super::MPCBB1_VCTR43>;
#[doc = "Writer for register MPCBB1_VCTR43"]
pub type W = crate::W<u32, super::MPCBB1_VCTR43>;
#[doc = "Register MPCBB1_VCTR43 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR43 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1376`"]
pub type B1376_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1376`"]
pub struct B1376_W<'a> {
    w: &'a mut W,
}
impl<'a> B1376_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1377`"]
pub type B1377_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1377`"]
pub struct B1377_W<'a> {
    w: &'a mut W,
}
impl<'a> B1377_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1378`"]
pub type B1378_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1378`"]
pub struct B1378_W<'a> {
    w: &'a mut W,
}
impl<'a> B1378_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1379`"]
pub type B1379_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1379`"]
pub struct B1379_W<'a> {
    w: &'a mut W,
}
impl<'a> B1379_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1380`"]
pub type B1380_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1380`"]
pub struct B1380_W<'a> {
    w: &'a mut W,
}
impl<'a> B1380_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1381`"]
pub type B1381_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1381`"]
pub struct B1381_W<'a> {
    w: &'a mut W,
}
impl<'a> B1381_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1382`"]
pub type B1382_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1382`"]
pub struct B1382_W<'a> {
    w: &'a mut W,
}
impl<'a> B1382_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1383`"]
pub type B1383_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1383`"]
pub struct B1383_W<'a> {
    w: &'a mut W,
}
impl<'a> B1383_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1384`"]
pub type B1384_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1384`"]
pub struct B1384_W<'a> {
    w: &'a mut W,
}
impl<'a> B1384_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1385`"]
pub type B1385_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1385`"]
pub struct B1385_W<'a> {
    w: &'a mut W,
}
impl<'a> B1385_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1386`"]
pub type B1386_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1386`"]
pub struct B1386_W<'a> {
    w: &'a mut W,
}
impl<'a> B1386_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1387`"]
pub type B1387_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1387`"]
pub struct B1387_W<'a> {
    w: &'a mut W,
}
impl<'a> B1387_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1388`"]
pub type B1388_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1388`"]
pub struct B1388_W<'a> {
    w: &'a mut W,
}
impl<'a> B1388_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1389`"]
pub type B1389_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1389`"]
pub struct B1389_W<'a> {
    w: &'a mut W,
}
impl<'a> B1389_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1390`"]
pub type B1390_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1390`"]
pub struct B1390_W<'a> {
    w: &'a mut W,
}
impl<'a> B1390_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1391`"]
pub type B1391_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1391`"]
pub struct B1391_W<'a> {
    w: &'a mut W,
}
impl<'a> B1391_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1392`"]
pub type B1392_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1392`"]
pub struct B1392_W<'a> {
    w: &'a mut W,
}
impl<'a> B1392_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1393`"]
pub type B1393_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1393`"]
pub struct B1393_W<'a> {
    w: &'a mut W,
}
impl<'a> B1393_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1394`"]
pub type B1394_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1394`"]
pub struct B1394_W<'a> {
    w: &'a mut W,
}
impl<'a> B1394_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1395`"]
pub type B1395_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1395`"]
pub struct B1395_W<'a> {
    w: &'a mut W,
}
impl<'a> B1395_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1396`"]
pub type B1396_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1396`"]
pub struct B1396_W<'a> {
    w: &'a mut W,
}
impl<'a> B1396_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1397`"]
pub type B1397_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1397`"]
pub struct B1397_W<'a> {
    w: &'a mut W,
}
impl<'a> B1397_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1398`"]
pub type B1398_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1398`"]
pub struct B1398_W<'a> {
    w: &'a mut W,
}
impl<'a> B1398_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1399`"]
pub type B1399_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1399`"]
pub struct B1399_W<'a> {
    w: &'a mut W,
}
impl<'a> B1399_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1400`"]
pub type B1400_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1400`"]
pub struct B1400_W<'a> {
    w: &'a mut W,
}
impl<'a> B1400_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1401`"]
pub type B1401_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1401`"]
pub struct B1401_W<'a> {
    w: &'a mut W,
}
impl<'a> B1401_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1402`"]
pub type B1402_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1402`"]
pub struct B1402_W<'a> {
    w: &'a mut W,
}
impl<'a> B1402_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1403`"]
pub type B1403_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1403`"]
pub struct B1403_W<'a> {
    w: &'a mut W,
}
impl<'a> B1403_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1404`"]
pub type B1404_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1404`"]
pub struct B1404_W<'a> {
    w: &'a mut W,
}
impl<'a> B1404_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1405`"]
pub type B1405_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1405`"]
pub struct B1405_W<'a> {
    w: &'a mut W,
}
impl<'a> B1405_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1406`"]
pub type B1406_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1406`"]
pub struct B1406_W<'a> {
    w: &'a mut W,
}
impl<'a> B1406_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1407`"]
pub type B1407_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1407`"]
pub struct B1407_W<'a> {
    w: &'a mut W,
}
impl<'a> B1407_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1376"]
    #[inline(always)]
    pub fn b1376(&self) -> B1376_R {
        B1376_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1377"]
    #[inline(always)]
    pub fn b1377(&self) -> B1377_R {
        B1377_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1378"]
    #[inline(always)]
    pub fn b1378(&self) -> B1378_R {
        B1378_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1379"]
    #[inline(always)]
    pub fn b1379(&self) -> B1379_R {
        B1379_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1380"]
    #[inline(always)]
    pub fn b1380(&self) -> B1380_R {
        B1380_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1381"]
    #[inline(always)]
    pub fn b1381(&self) -> B1381_R {
        B1381_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1382"]
    #[inline(always)]
    pub fn b1382(&self) -> B1382_R {
        B1382_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1383"]
    #[inline(always)]
    pub fn b1383(&self) -> B1383_R {
        B1383_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1384"]
    #[inline(always)]
    pub fn b1384(&self) -> B1384_R {
        B1384_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1385"]
    #[inline(always)]
    pub fn b1385(&self) -> B1385_R {
        B1385_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1386"]
    #[inline(always)]
    pub fn b1386(&self) -> B1386_R {
        B1386_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1387"]
    #[inline(always)]
    pub fn b1387(&self) -> B1387_R {
        B1387_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1388"]
    #[inline(always)]
    pub fn b1388(&self) -> B1388_R {
        B1388_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1389"]
    #[inline(always)]
    pub fn b1389(&self) -> B1389_R {
        B1389_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1390"]
    #[inline(always)]
    pub fn b1390(&self) -> B1390_R {
        B1390_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1391"]
    #[inline(always)]
    pub fn b1391(&self) -> B1391_R {
        B1391_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1392"]
    #[inline(always)]
    pub fn b1392(&self) -> B1392_R {
        B1392_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1393"]
    #[inline(always)]
    pub fn b1393(&self) -> B1393_R {
        B1393_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1394"]
    #[inline(always)]
    pub fn b1394(&self) -> B1394_R {
        B1394_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1395"]
    #[inline(always)]
    pub fn b1395(&self) -> B1395_R {
        B1395_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1396"]
    #[inline(always)]
    pub fn b1396(&self) -> B1396_R {
        B1396_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1397"]
    #[inline(always)]
    pub fn b1397(&self) -> B1397_R {
        B1397_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1398"]
    #[inline(always)]
    pub fn b1398(&self) -> B1398_R {
        B1398_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1399"]
    #[inline(always)]
    pub fn b1399(&self) -> B1399_R {
        B1399_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1400"]
    #[inline(always)]
    pub fn b1400(&self) -> B1400_R {
        B1400_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1401"]
    #[inline(always)]
    pub fn b1401(&self) -> B1401_R {
        B1401_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1402"]
    #[inline(always)]
    pub fn b1402(&self) -> B1402_R {
        B1402_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1403"]
    #[inline(always)]
    pub fn b1403(&self) -> B1403_R {
        B1403_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1404"]
    #[inline(always)]
    pub fn b1404(&self) -> B1404_R {
        B1404_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1405"]
    #[inline(always)]
    pub fn b1405(&self) -> B1405_R {
        B1405_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1406"]
    #[inline(always)]
    pub fn b1406(&self) -> B1406_R {
        B1406_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1407"]
    #[inline(always)]
    pub fn b1407(&self) -> B1407_R {
        B1407_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1376"]
    #[inline(always)]
    pub fn b1376(&mut self) -> B1376_W {
        B1376_W { w: self }
    }
    #[doc = "Bit 1 - B1377"]
    #[inline(always)]
    pub fn b1377(&mut self) -> B1377_W {
        B1377_W { w: self }
    }
    #[doc = "Bit 2 - B1378"]
    #[inline(always)]
    pub fn b1378(&mut self) -> B1378_W {
        B1378_W { w: self }
    }
    #[doc = "Bit 3 - B1379"]
    #[inline(always)]
    pub fn b1379(&mut self) -> B1379_W {
        B1379_W { w: self }
    }
    #[doc = "Bit 4 - B1380"]
    #[inline(always)]
    pub fn b1380(&mut self) -> B1380_W {
        B1380_W { w: self }
    }
    #[doc = "Bit 5 - B1381"]
    #[inline(always)]
    pub fn b1381(&mut self) -> B1381_W {
        B1381_W { w: self }
    }
    #[doc = "Bit 6 - B1382"]
    #[inline(always)]
    pub fn b1382(&mut self) -> B1382_W {
        B1382_W { w: self }
    }
    #[doc = "Bit 7 - B1383"]
    #[inline(always)]
    pub fn b1383(&mut self) -> B1383_W {
        B1383_W { w: self }
    }
    #[doc = "Bit 8 - B1384"]
    #[inline(always)]
    pub fn b1384(&mut self) -> B1384_W {
        B1384_W { w: self }
    }
    #[doc = "Bit 9 - B1385"]
    #[inline(always)]
    pub fn b1385(&mut self) -> B1385_W {
        B1385_W { w: self }
    }
    #[doc = "Bit 10 - B1386"]
    #[inline(always)]
    pub fn b1386(&mut self) -> B1386_W {
        B1386_W { w: self }
    }
    #[doc = "Bit 11 - B1387"]
    #[inline(always)]
    pub fn b1387(&mut self) -> B1387_W {
        B1387_W { w: self }
    }
    #[doc = "Bit 12 - B1388"]
    #[inline(always)]
    pub fn b1388(&mut self) -> B1388_W {
        B1388_W { w: self }
    }
    #[doc = "Bit 13 - B1389"]
    #[inline(always)]
    pub fn b1389(&mut self) -> B1389_W {
        B1389_W { w: self }
    }
    #[doc = "Bit 14 - B1390"]
    #[inline(always)]
    pub fn b1390(&mut self) -> B1390_W {
        B1390_W { w: self }
    }
    #[doc = "Bit 15 - B1391"]
    #[inline(always)]
    pub fn b1391(&mut self) -> B1391_W {
        B1391_W { w: self }
    }
    #[doc = "Bit 16 - B1392"]
    #[inline(always)]
    pub fn b1392(&mut self) -> B1392_W {
        B1392_W { w: self }
    }
    #[doc = "Bit 17 - B1393"]
    #[inline(always)]
    pub fn b1393(&mut self) -> B1393_W {
        B1393_W { w: self }
    }
    #[doc = "Bit 18 - B1394"]
    #[inline(always)]
    pub fn b1394(&mut self) -> B1394_W {
        B1394_W { w: self }
    }
    #[doc = "Bit 19 - B1395"]
    #[inline(always)]
    pub fn b1395(&mut self) -> B1395_W {
        B1395_W { w: self }
    }
    #[doc = "Bit 20 - B1396"]
    #[inline(always)]
    pub fn b1396(&mut self) -> B1396_W {
        B1396_W { w: self }
    }
    #[doc = "Bit 21 - B1397"]
    #[inline(always)]
    pub fn b1397(&mut self) -> B1397_W {
        B1397_W { w: self }
    }
    #[doc = "Bit 22 - B1398"]
    #[inline(always)]
    pub fn b1398(&mut self) -> B1398_W {
        B1398_W { w: self }
    }
    #[doc = "Bit 23 - B1399"]
    #[inline(always)]
    pub fn b1399(&mut self) -> B1399_W {
        B1399_W { w: self }
    }
    #[doc = "Bit 24 - B1400"]
    #[inline(always)]
    pub fn b1400(&mut self) -> B1400_W {
        B1400_W { w: self }
    }
    #[doc = "Bit 25 - B1401"]
    #[inline(always)]
    pub fn b1401(&mut self) -> B1401_W {
        B1401_W { w: self }
    }
    #[doc = "Bit 26 - B1402"]
    #[inline(always)]
    pub fn b1402(&mut self) -> B1402_W {
        B1402_W { w: self }
    }
    #[doc = "Bit 27 - B1403"]
    #[inline(always)]
    pub fn b1403(&mut self) -> B1403_W {
        B1403_W { w: self }
    }
    #[doc = "Bit 28 - B1404"]
    #[inline(always)]
    pub fn b1404(&mut self) -> B1404_W {
        B1404_W { w: self }
    }
    #[doc = "Bit 29 - B1405"]
    #[inline(always)]
    pub fn b1405(&mut self) -> B1405_W {
        B1405_W { w: self }
    }
    #[doc = "Bit 30 - B1406"]
    #[inline(always)]
    pub fn b1406(&mut self) -> B1406_W {
        B1406_W { w: self }
    }
    #[doc = "Bit 31 - B1407"]
    #[inline(always)]
    pub fn b1407(&mut self) -> B1407_W {
        B1407_W { w: self }
    }
}
