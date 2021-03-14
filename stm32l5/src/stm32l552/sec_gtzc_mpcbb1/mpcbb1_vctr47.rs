#[doc = "Reader of register MPCBB1_VCTR47"]
pub type R = crate::R<u32, super::MPCBB1_VCTR47>;
#[doc = "Writer for register MPCBB1_VCTR47"]
pub type W = crate::W<u32, super::MPCBB1_VCTR47>;
#[doc = "Register MPCBB1_VCTR47 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR47 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1504`"]
pub type B1504_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1504`"]
pub struct B1504_W<'a> {
    w: &'a mut W,
}
impl<'a> B1504_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1505`"]
pub type B1505_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1505`"]
pub struct B1505_W<'a> {
    w: &'a mut W,
}
impl<'a> B1505_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1506`"]
pub type B1506_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1506`"]
pub struct B1506_W<'a> {
    w: &'a mut W,
}
impl<'a> B1506_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1507`"]
pub type B1507_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1507`"]
pub struct B1507_W<'a> {
    w: &'a mut W,
}
impl<'a> B1507_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1508`"]
pub type B1508_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1508`"]
pub struct B1508_W<'a> {
    w: &'a mut W,
}
impl<'a> B1508_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1509`"]
pub type B1509_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1509`"]
pub struct B1509_W<'a> {
    w: &'a mut W,
}
impl<'a> B1509_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1510`"]
pub type B1510_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1510`"]
pub struct B1510_W<'a> {
    w: &'a mut W,
}
impl<'a> B1510_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1511`"]
pub type B1511_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1511`"]
pub struct B1511_W<'a> {
    w: &'a mut W,
}
impl<'a> B1511_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1512`"]
pub type B1512_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1512`"]
pub struct B1512_W<'a> {
    w: &'a mut W,
}
impl<'a> B1512_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1513`"]
pub type B1513_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1513`"]
pub struct B1513_W<'a> {
    w: &'a mut W,
}
impl<'a> B1513_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1514`"]
pub type B1514_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1514`"]
pub struct B1514_W<'a> {
    w: &'a mut W,
}
impl<'a> B1514_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1515`"]
pub type B1515_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1515`"]
pub struct B1515_W<'a> {
    w: &'a mut W,
}
impl<'a> B1515_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1516`"]
pub type B1516_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1516`"]
pub struct B1516_W<'a> {
    w: &'a mut W,
}
impl<'a> B1516_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1517`"]
pub type B1517_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1517`"]
pub struct B1517_W<'a> {
    w: &'a mut W,
}
impl<'a> B1517_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1518`"]
pub type B1518_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1518`"]
pub struct B1518_W<'a> {
    w: &'a mut W,
}
impl<'a> B1518_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1519`"]
pub type B1519_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1519`"]
pub struct B1519_W<'a> {
    w: &'a mut W,
}
impl<'a> B1519_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1520`"]
pub type B1520_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1520`"]
pub struct B1520_W<'a> {
    w: &'a mut W,
}
impl<'a> B1520_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1521`"]
pub type B1521_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1521`"]
pub struct B1521_W<'a> {
    w: &'a mut W,
}
impl<'a> B1521_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1522`"]
pub type B1522_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1522`"]
pub struct B1522_W<'a> {
    w: &'a mut W,
}
impl<'a> B1522_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1523`"]
pub type B1523_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1523`"]
pub struct B1523_W<'a> {
    w: &'a mut W,
}
impl<'a> B1523_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1524`"]
pub type B1524_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1524`"]
pub struct B1524_W<'a> {
    w: &'a mut W,
}
impl<'a> B1524_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1525`"]
pub type B1525_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1525`"]
pub struct B1525_W<'a> {
    w: &'a mut W,
}
impl<'a> B1525_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1526`"]
pub type B1526_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1526`"]
pub struct B1526_W<'a> {
    w: &'a mut W,
}
impl<'a> B1526_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1527`"]
pub type B1527_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1527`"]
pub struct B1527_W<'a> {
    w: &'a mut W,
}
impl<'a> B1527_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1528`"]
pub type B1528_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1528`"]
pub struct B1528_W<'a> {
    w: &'a mut W,
}
impl<'a> B1528_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1529`"]
pub type B1529_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1529`"]
pub struct B1529_W<'a> {
    w: &'a mut W,
}
impl<'a> B1529_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1530`"]
pub type B1530_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1530`"]
pub struct B1530_W<'a> {
    w: &'a mut W,
}
impl<'a> B1530_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1531`"]
pub type B1531_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1531`"]
pub struct B1531_W<'a> {
    w: &'a mut W,
}
impl<'a> B1531_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1532`"]
pub type B1532_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1532`"]
pub struct B1532_W<'a> {
    w: &'a mut W,
}
impl<'a> B1532_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1533`"]
pub type B1533_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1533`"]
pub struct B1533_W<'a> {
    w: &'a mut W,
}
impl<'a> B1533_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1534`"]
pub type B1534_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1534`"]
pub struct B1534_W<'a> {
    w: &'a mut W,
}
impl<'a> B1534_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1535`"]
pub type B1535_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1535`"]
pub struct B1535_W<'a> {
    w: &'a mut W,
}
impl<'a> B1535_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1504"]
    #[inline(always)]
    pub fn b1504(&self) -> B1504_R {
        B1504_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1505"]
    #[inline(always)]
    pub fn b1505(&self) -> B1505_R {
        B1505_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1506"]
    #[inline(always)]
    pub fn b1506(&self) -> B1506_R {
        B1506_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1507"]
    #[inline(always)]
    pub fn b1507(&self) -> B1507_R {
        B1507_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1508"]
    #[inline(always)]
    pub fn b1508(&self) -> B1508_R {
        B1508_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1509"]
    #[inline(always)]
    pub fn b1509(&self) -> B1509_R {
        B1509_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1510"]
    #[inline(always)]
    pub fn b1510(&self) -> B1510_R {
        B1510_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1511"]
    #[inline(always)]
    pub fn b1511(&self) -> B1511_R {
        B1511_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1512"]
    #[inline(always)]
    pub fn b1512(&self) -> B1512_R {
        B1512_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1513"]
    #[inline(always)]
    pub fn b1513(&self) -> B1513_R {
        B1513_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1514"]
    #[inline(always)]
    pub fn b1514(&self) -> B1514_R {
        B1514_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1515"]
    #[inline(always)]
    pub fn b1515(&self) -> B1515_R {
        B1515_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1516"]
    #[inline(always)]
    pub fn b1516(&self) -> B1516_R {
        B1516_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1517"]
    #[inline(always)]
    pub fn b1517(&self) -> B1517_R {
        B1517_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1518"]
    #[inline(always)]
    pub fn b1518(&self) -> B1518_R {
        B1518_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1519"]
    #[inline(always)]
    pub fn b1519(&self) -> B1519_R {
        B1519_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1520"]
    #[inline(always)]
    pub fn b1520(&self) -> B1520_R {
        B1520_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1521"]
    #[inline(always)]
    pub fn b1521(&self) -> B1521_R {
        B1521_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1522"]
    #[inline(always)]
    pub fn b1522(&self) -> B1522_R {
        B1522_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1523"]
    #[inline(always)]
    pub fn b1523(&self) -> B1523_R {
        B1523_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1524"]
    #[inline(always)]
    pub fn b1524(&self) -> B1524_R {
        B1524_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1525"]
    #[inline(always)]
    pub fn b1525(&self) -> B1525_R {
        B1525_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1526"]
    #[inline(always)]
    pub fn b1526(&self) -> B1526_R {
        B1526_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1527"]
    #[inline(always)]
    pub fn b1527(&self) -> B1527_R {
        B1527_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1528"]
    #[inline(always)]
    pub fn b1528(&self) -> B1528_R {
        B1528_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1529"]
    #[inline(always)]
    pub fn b1529(&self) -> B1529_R {
        B1529_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1530"]
    #[inline(always)]
    pub fn b1530(&self) -> B1530_R {
        B1530_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1531"]
    #[inline(always)]
    pub fn b1531(&self) -> B1531_R {
        B1531_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1532"]
    #[inline(always)]
    pub fn b1532(&self) -> B1532_R {
        B1532_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1533"]
    #[inline(always)]
    pub fn b1533(&self) -> B1533_R {
        B1533_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1534"]
    #[inline(always)]
    pub fn b1534(&self) -> B1534_R {
        B1534_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1535"]
    #[inline(always)]
    pub fn b1535(&self) -> B1535_R {
        B1535_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1504"]
    #[inline(always)]
    pub fn b1504(&mut self) -> B1504_W {
        B1504_W { w: self }
    }
    #[doc = "Bit 1 - B1505"]
    #[inline(always)]
    pub fn b1505(&mut self) -> B1505_W {
        B1505_W { w: self }
    }
    #[doc = "Bit 2 - B1506"]
    #[inline(always)]
    pub fn b1506(&mut self) -> B1506_W {
        B1506_W { w: self }
    }
    #[doc = "Bit 3 - B1507"]
    #[inline(always)]
    pub fn b1507(&mut self) -> B1507_W {
        B1507_W { w: self }
    }
    #[doc = "Bit 4 - B1508"]
    #[inline(always)]
    pub fn b1508(&mut self) -> B1508_W {
        B1508_W { w: self }
    }
    #[doc = "Bit 5 - B1509"]
    #[inline(always)]
    pub fn b1509(&mut self) -> B1509_W {
        B1509_W { w: self }
    }
    #[doc = "Bit 6 - B1510"]
    #[inline(always)]
    pub fn b1510(&mut self) -> B1510_W {
        B1510_W { w: self }
    }
    #[doc = "Bit 7 - B1511"]
    #[inline(always)]
    pub fn b1511(&mut self) -> B1511_W {
        B1511_W { w: self }
    }
    #[doc = "Bit 8 - B1512"]
    #[inline(always)]
    pub fn b1512(&mut self) -> B1512_W {
        B1512_W { w: self }
    }
    #[doc = "Bit 9 - B1513"]
    #[inline(always)]
    pub fn b1513(&mut self) -> B1513_W {
        B1513_W { w: self }
    }
    #[doc = "Bit 10 - B1514"]
    #[inline(always)]
    pub fn b1514(&mut self) -> B1514_W {
        B1514_W { w: self }
    }
    #[doc = "Bit 11 - B1515"]
    #[inline(always)]
    pub fn b1515(&mut self) -> B1515_W {
        B1515_W { w: self }
    }
    #[doc = "Bit 12 - B1516"]
    #[inline(always)]
    pub fn b1516(&mut self) -> B1516_W {
        B1516_W { w: self }
    }
    #[doc = "Bit 13 - B1517"]
    #[inline(always)]
    pub fn b1517(&mut self) -> B1517_W {
        B1517_W { w: self }
    }
    #[doc = "Bit 14 - B1518"]
    #[inline(always)]
    pub fn b1518(&mut self) -> B1518_W {
        B1518_W { w: self }
    }
    #[doc = "Bit 15 - B1519"]
    #[inline(always)]
    pub fn b1519(&mut self) -> B1519_W {
        B1519_W { w: self }
    }
    #[doc = "Bit 16 - B1520"]
    #[inline(always)]
    pub fn b1520(&mut self) -> B1520_W {
        B1520_W { w: self }
    }
    #[doc = "Bit 17 - B1521"]
    #[inline(always)]
    pub fn b1521(&mut self) -> B1521_W {
        B1521_W { w: self }
    }
    #[doc = "Bit 18 - B1522"]
    #[inline(always)]
    pub fn b1522(&mut self) -> B1522_W {
        B1522_W { w: self }
    }
    #[doc = "Bit 19 - B1523"]
    #[inline(always)]
    pub fn b1523(&mut self) -> B1523_W {
        B1523_W { w: self }
    }
    #[doc = "Bit 20 - B1524"]
    #[inline(always)]
    pub fn b1524(&mut self) -> B1524_W {
        B1524_W { w: self }
    }
    #[doc = "Bit 21 - B1525"]
    #[inline(always)]
    pub fn b1525(&mut self) -> B1525_W {
        B1525_W { w: self }
    }
    #[doc = "Bit 22 - B1526"]
    #[inline(always)]
    pub fn b1526(&mut self) -> B1526_W {
        B1526_W { w: self }
    }
    #[doc = "Bit 23 - B1527"]
    #[inline(always)]
    pub fn b1527(&mut self) -> B1527_W {
        B1527_W { w: self }
    }
    #[doc = "Bit 24 - B1528"]
    #[inline(always)]
    pub fn b1528(&mut self) -> B1528_W {
        B1528_W { w: self }
    }
    #[doc = "Bit 25 - B1529"]
    #[inline(always)]
    pub fn b1529(&mut self) -> B1529_W {
        B1529_W { w: self }
    }
    #[doc = "Bit 26 - B1530"]
    #[inline(always)]
    pub fn b1530(&mut self) -> B1530_W {
        B1530_W { w: self }
    }
    #[doc = "Bit 27 - B1531"]
    #[inline(always)]
    pub fn b1531(&mut self) -> B1531_W {
        B1531_W { w: self }
    }
    #[doc = "Bit 28 - B1532"]
    #[inline(always)]
    pub fn b1532(&mut self) -> B1532_W {
        B1532_W { w: self }
    }
    #[doc = "Bit 29 - B1533"]
    #[inline(always)]
    pub fn b1533(&mut self) -> B1533_W {
        B1533_W { w: self }
    }
    #[doc = "Bit 30 - B1534"]
    #[inline(always)]
    pub fn b1534(&mut self) -> B1534_W {
        B1534_W { w: self }
    }
    #[doc = "Bit 31 - B1535"]
    #[inline(always)]
    pub fn b1535(&mut self) -> B1535_W {
        B1535_W { w: self }
    }
}
