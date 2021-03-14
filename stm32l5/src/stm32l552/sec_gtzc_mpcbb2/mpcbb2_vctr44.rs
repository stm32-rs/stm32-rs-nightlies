#[doc = "Reader of register MPCBB2_VCTR44"]
pub type R = crate::R<u32, super::MPCBB2_VCTR44>;
#[doc = "Writer for register MPCBB2_VCTR44"]
pub type W = crate::W<u32, super::MPCBB2_VCTR44>;
#[doc = "Register MPCBB2_VCTR44 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR44 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1408`"]
pub type B1408_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1408`"]
pub struct B1408_W<'a> {
    w: &'a mut W,
}
impl<'a> B1408_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1409`"]
pub type B1409_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1409`"]
pub struct B1409_W<'a> {
    w: &'a mut W,
}
impl<'a> B1409_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1410`"]
pub type B1410_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1410`"]
pub struct B1410_W<'a> {
    w: &'a mut W,
}
impl<'a> B1410_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1411`"]
pub type B1411_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1411`"]
pub struct B1411_W<'a> {
    w: &'a mut W,
}
impl<'a> B1411_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1412`"]
pub type B1412_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1412`"]
pub struct B1412_W<'a> {
    w: &'a mut W,
}
impl<'a> B1412_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1413`"]
pub type B1413_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1413`"]
pub struct B1413_W<'a> {
    w: &'a mut W,
}
impl<'a> B1413_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1414`"]
pub type B1414_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1414`"]
pub struct B1414_W<'a> {
    w: &'a mut W,
}
impl<'a> B1414_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1415`"]
pub type B1415_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1415`"]
pub struct B1415_W<'a> {
    w: &'a mut W,
}
impl<'a> B1415_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1416`"]
pub type B1416_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1416`"]
pub struct B1416_W<'a> {
    w: &'a mut W,
}
impl<'a> B1416_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1417`"]
pub type B1417_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1417`"]
pub struct B1417_W<'a> {
    w: &'a mut W,
}
impl<'a> B1417_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1418`"]
pub type B1418_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1418`"]
pub struct B1418_W<'a> {
    w: &'a mut W,
}
impl<'a> B1418_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1419`"]
pub type B1419_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1419`"]
pub struct B1419_W<'a> {
    w: &'a mut W,
}
impl<'a> B1419_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1420`"]
pub type B1420_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1420`"]
pub struct B1420_W<'a> {
    w: &'a mut W,
}
impl<'a> B1420_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1421`"]
pub type B1421_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1421`"]
pub struct B1421_W<'a> {
    w: &'a mut W,
}
impl<'a> B1421_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1422`"]
pub type B1422_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1422`"]
pub struct B1422_W<'a> {
    w: &'a mut W,
}
impl<'a> B1422_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1423`"]
pub type B1423_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1423`"]
pub struct B1423_W<'a> {
    w: &'a mut W,
}
impl<'a> B1423_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1424`"]
pub type B1424_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1424`"]
pub struct B1424_W<'a> {
    w: &'a mut W,
}
impl<'a> B1424_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1425`"]
pub type B1425_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1425`"]
pub struct B1425_W<'a> {
    w: &'a mut W,
}
impl<'a> B1425_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1426`"]
pub type B1426_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1426`"]
pub struct B1426_W<'a> {
    w: &'a mut W,
}
impl<'a> B1426_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1427`"]
pub type B1427_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1427`"]
pub struct B1427_W<'a> {
    w: &'a mut W,
}
impl<'a> B1427_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1428`"]
pub type B1428_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1428`"]
pub struct B1428_W<'a> {
    w: &'a mut W,
}
impl<'a> B1428_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1429`"]
pub type B1429_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1429`"]
pub struct B1429_W<'a> {
    w: &'a mut W,
}
impl<'a> B1429_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1430`"]
pub type B1430_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1430`"]
pub struct B1430_W<'a> {
    w: &'a mut W,
}
impl<'a> B1430_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1431`"]
pub type B1431_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1431`"]
pub struct B1431_W<'a> {
    w: &'a mut W,
}
impl<'a> B1431_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1432`"]
pub type B1432_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1432`"]
pub struct B1432_W<'a> {
    w: &'a mut W,
}
impl<'a> B1432_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1433`"]
pub type B1433_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1433`"]
pub struct B1433_W<'a> {
    w: &'a mut W,
}
impl<'a> B1433_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1434`"]
pub type B1434_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1434`"]
pub struct B1434_W<'a> {
    w: &'a mut W,
}
impl<'a> B1434_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1435`"]
pub type B1435_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1435`"]
pub struct B1435_W<'a> {
    w: &'a mut W,
}
impl<'a> B1435_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1436`"]
pub type B1436_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1436`"]
pub struct B1436_W<'a> {
    w: &'a mut W,
}
impl<'a> B1436_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1437`"]
pub type B1437_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1437`"]
pub struct B1437_W<'a> {
    w: &'a mut W,
}
impl<'a> B1437_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1438`"]
pub type B1438_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1438`"]
pub struct B1438_W<'a> {
    w: &'a mut W,
}
impl<'a> B1438_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1439`"]
pub type B1439_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1439`"]
pub struct B1439_W<'a> {
    w: &'a mut W,
}
impl<'a> B1439_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1408"]
    #[inline(always)]
    pub fn b1408(&self) -> B1408_R {
        B1408_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1409"]
    #[inline(always)]
    pub fn b1409(&self) -> B1409_R {
        B1409_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1410"]
    #[inline(always)]
    pub fn b1410(&self) -> B1410_R {
        B1410_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1411"]
    #[inline(always)]
    pub fn b1411(&self) -> B1411_R {
        B1411_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1412"]
    #[inline(always)]
    pub fn b1412(&self) -> B1412_R {
        B1412_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1413"]
    #[inline(always)]
    pub fn b1413(&self) -> B1413_R {
        B1413_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1414"]
    #[inline(always)]
    pub fn b1414(&self) -> B1414_R {
        B1414_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1415"]
    #[inline(always)]
    pub fn b1415(&self) -> B1415_R {
        B1415_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1416"]
    #[inline(always)]
    pub fn b1416(&self) -> B1416_R {
        B1416_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1417"]
    #[inline(always)]
    pub fn b1417(&self) -> B1417_R {
        B1417_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1418"]
    #[inline(always)]
    pub fn b1418(&self) -> B1418_R {
        B1418_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1419"]
    #[inline(always)]
    pub fn b1419(&self) -> B1419_R {
        B1419_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1420"]
    #[inline(always)]
    pub fn b1420(&self) -> B1420_R {
        B1420_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1421"]
    #[inline(always)]
    pub fn b1421(&self) -> B1421_R {
        B1421_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1422"]
    #[inline(always)]
    pub fn b1422(&self) -> B1422_R {
        B1422_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1423"]
    #[inline(always)]
    pub fn b1423(&self) -> B1423_R {
        B1423_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1424"]
    #[inline(always)]
    pub fn b1424(&self) -> B1424_R {
        B1424_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1425"]
    #[inline(always)]
    pub fn b1425(&self) -> B1425_R {
        B1425_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1426"]
    #[inline(always)]
    pub fn b1426(&self) -> B1426_R {
        B1426_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1427"]
    #[inline(always)]
    pub fn b1427(&self) -> B1427_R {
        B1427_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1428"]
    #[inline(always)]
    pub fn b1428(&self) -> B1428_R {
        B1428_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1429"]
    #[inline(always)]
    pub fn b1429(&self) -> B1429_R {
        B1429_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1430"]
    #[inline(always)]
    pub fn b1430(&self) -> B1430_R {
        B1430_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1431"]
    #[inline(always)]
    pub fn b1431(&self) -> B1431_R {
        B1431_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1432"]
    #[inline(always)]
    pub fn b1432(&self) -> B1432_R {
        B1432_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1433"]
    #[inline(always)]
    pub fn b1433(&self) -> B1433_R {
        B1433_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1434"]
    #[inline(always)]
    pub fn b1434(&self) -> B1434_R {
        B1434_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1435"]
    #[inline(always)]
    pub fn b1435(&self) -> B1435_R {
        B1435_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1436"]
    #[inline(always)]
    pub fn b1436(&self) -> B1436_R {
        B1436_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1437"]
    #[inline(always)]
    pub fn b1437(&self) -> B1437_R {
        B1437_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1438"]
    #[inline(always)]
    pub fn b1438(&self) -> B1438_R {
        B1438_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1439"]
    #[inline(always)]
    pub fn b1439(&self) -> B1439_R {
        B1439_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1408"]
    #[inline(always)]
    pub fn b1408(&mut self) -> B1408_W {
        B1408_W { w: self }
    }
    #[doc = "Bit 1 - B1409"]
    #[inline(always)]
    pub fn b1409(&mut self) -> B1409_W {
        B1409_W { w: self }
    }
    #[doc = "Bit 2 - B1410"]
    #[inline(always)]
    pub fn b1410(&mut self) -> B1410_W {
        B1410_W { w: self }
    }
    #[doc = "Bit 3 - B1411"]
    #[inline(always)]
    pub fn b1411(&mut self) -> B1411_W {
        B1411_W { w: self }
    }
    #[doc = "Bit 4 - B1412"]
    #[inline(always)]
    pub fn b1412(&mut self) -> B1412_W {
        B1412_W { w: self }
    }
    #[doc = "Bit 5 - B1413"]
    #[inline(always)]
    pub fn b1413(&mut self) -> B1413_W {
        B1413_W { w: self }
    }
    #[doc = "Bit 6 - B1414"]
    #[inline(always)]
    pub fn b1414(&mut self) -> B1414_W {
        B1414_W { w: self }
    }
    #[doc = "Bit 7 - B1415"]
    #[inline(always)]
    pub fn b1415(&mut self) -> B1415_W {
        B1415_W { w: self }
    }
    #[doc = "Bit 8 - B1416"]
    #[inline(always)]
    pub fn b1416(&mut self) -> B1416_W {
        B1416_W { w: self }
    }
    #[doc = "Bit 9 - B1417"]
    #[inline(always)]
    pub fn b1417(&mut self) -> B1417_W {
        B1417_W { w: self }
    }
    #[doc = "Bit 10 - B1418"]
    #[inline(always)]
    pub fn b1418(&mut self) -> B1418_W {
        B1418_W { w: self }
    }
    #[doc = "Bit 11 - B1419"]
    #[inline(always)]
    pub fn b1419(&mut self) -> B1419_W {
        B1419_W { w: self }
    }
    #[doc = "Bit 12 - B1420"]
    #[inline(always)]
    pub fn b1420(&mut self) -> B1420_W {
        B1420_W { w: self }
    }
    #[doc = "Bit 13 - B1421"]
    #[inline(always)]
    pub fn b1421(&mut self) -> B1421_W {
        B1421_W { w: self }
    }
    #[doc = "Bit 14 - B1422"]
    #[inline(always)]
    pub fn b1422(&mut self) -> B1422_W {
        B1422_W { w: self }
    }
    #[doc = "Bit 15 - B1423"]
    #[inline(always)]
    pub fn b1423(&mut self) -> B1423_W {
        B1423_W { w: self }
    }
    #[doc = "Bit 16 - B1424"]
    #[inline(always)]
    pub fn b1424(&mut self) -> B1424_W {
        B1424_W { w: self }
    }
    #[doc = "Bit 17 - B1425"]
    #[inline(always)]
    pub fn b1425(&mut self) -> B1425_W {
        B1425_W { w: self }
    }
    #[doc = "Bit 18 - B1426"]
    #[inline(always)]
    pub fn b1426(&mut self) -> B1426_W {
        B1426_W { w: self }
    }
    #[doc = "Bit 19 - B1427"]
    #[inline(always)]
    pub fn b1427(&mut self) -> B1427_W {
        B1427_W { w: self }
    }
    #[doc = "Bit 20 - B1428"]
    #[inline(always)]
    pub fn b1428(&mut self) -> B1428_W {
        B1428_W { w: self }
    }
    #[doc = "Bit 21 - B1429"]
    #[inline(always)]
    pub fn b1429(&mut self) -> B1429_W {
        B1429_W { w: self }
    }
    #[doc = "Bit 22 - B1430"]
    #[inline(always)]
    pub fn b1430(&mut self) -> B1430_W {
        B1430_W { w: self }
    }
    #[doc = "Bit 23 - B1431"]
    #[inline(always)]
    pub fn b1431(&mut self) -> B1431_W {
        B1431_W { w: self }
    }
    #[doc = "Bit 24 - B1432"]
    #[inline(always)]
    pub fn b1432(&mut self) -> B1432_W {
        B1432_W { w: self }
    }
    #[doc = "Bit 25 - B1433"]
    #[inline(always)]
    pub fn b1433(&mut self) -> B1433_W {
        B1433_W { w: self }
    }
    #[doc = "Bit 26 - B1434"]
    #[inline(always)]
    pub fn b1434(&mut self) -> B1434_W {
        B1434_W { w: self }
    }
    #[doc = "Bit 27 - B1435"]
    #[inline(always)]
    pub fn b1435(&mut self) -> B1435_W {
        B1435_W { w: self }
    }
    #[doc = "Bit 28 - B1436"]
    #[inline(always)]
    pub fn b1436(&mut self) -> B1436_W {
        B1436_W { w: self }
    }
    #[doc = "Bit 29 - B1437"]
    #[inline(always)]
    pub fn b1437(&mut self) -> B1437_W {
        B1437_W { w: self }
    }
    #[doc = "Bit 30 - B1438"]
    #[inline(always)]
    pub fn b1438(&mut self) -> B1438_W {
        B1438_W { w: self }
    }
    #[doc = "Bit 31 - B1439"]
    #[inline(always)]
    pub fn b1439(&mut self) -> B1439_W {
        B1439_W { w: self }
    }
}
