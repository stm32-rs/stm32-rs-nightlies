#[doc = "Reader of register MPCBB2_VCTR46"]
pub type R = crate::R<u32, super::MPCBB2_VCTR46>;
#[doc = "Writer for register MPCBB2_VCTR46"]
pub type W = crate::W<u32, super::MPCBB2_VCTR46>;
#[doc = "Register MPCBB2_VCTR46 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR46 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1472`"]
pub type B1472_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1472`"]
pub struct B1472_W<'a> {
    w: &'a mut W,
}
impl<'a> B1472_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1473`"]
pub type B1473_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1473`"]
pub struct B1473_W<'a> {
    w: &'a mut W,
}
impl<'a> B1473_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1474`"]
pub type B1474_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1474`"]
pub struct B1474_W<'a> {
    w: &'a mut W,
}
impl<'a> B1474_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1475`"]
pub type B1475_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1475`"]
pub struct B1475_W<'a> {
    w: &'a mut W,
}
impl<'a> B1475_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1476`"]
pub type B1476_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1476`"]
pub struct B1476_W<'a> {
    w: &'a mut W,
}
impl<'a> B1476_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1477`"]
pub type B1477_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1477`"]
pub struct B1477_W<'a> {
    w: &'a mut W,
}
impl<'a> B1477_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1478`"]
pub type B1478_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1478`"]
pub struct B1478_W<'a> {
    w: &'a mut W,
}
impl<'a> B1478_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1479`"]
pub type B1479_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1479`"]
pub struct B1479_W<'a> {
    w: &'a mut W,
}
impl<'a> B1479_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1480`"]
pub type B1480_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1480`"]
pub struct B1480_W<'a> {
    w: &'a mut W,
}
impl<'a> B1480_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1481`"]
pub type B1481_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1481`"]
pub struct B1481_W<'a> {
    w: &'a mut W,
}
impl<'a> B1481_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1482`"]
pub type B1482_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1482`"]
pub struct B1482_W<'a> {
    w: &'a mut W,
}
impl<'a> B1482_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1483`"]
pub type B1483_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1483`"]
pub struct B1483_W<'a> {
    w: &'a mut W,
}
impl<'a> B1483_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1484`"]
pub type B1484_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1484`"]
pub struct B1484_W<'a> {
    w: &'a mut W,
}
impl<'a> B1484_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1485`"]
pub type B1485_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1485`"]
pub struct B1485_W<'a> {
    w: &'a mut W,
}
impl<'a> B1485_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1486`"]
pub type B1486_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1486`"]
pub struct B1486_W<'a> {
    w: &'a mut W,
}
impl<'a> B1486_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1487`"]
pub type B1487_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1487`"]
pub struct B1487_W<'a> {
    w: &'a mut W,
}
impl<'a> B1487_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1488`"]
pub type B1488_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1488`"]
pub struct B1488_W<'a> {
    w: &'a mut W,
}
impl<'a> B1488_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1489`"]
pub type B1489_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1489`"]
pub struct B1489_W<'a> {
    w: &'a mut W,
}
impl<'a> B1489_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1490`"]
pub type B1490_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1490`"]
pub struct B1490_W<'a> {
    w: &'a mut W,
}
impl<'a> B1490_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1491`"]
pub type B1491_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1491`"]
pub struct B1491_W<'a> {
    w: &'a mut W,
}
impl<'a> B1491_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1492`"]
pub type B1492_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1492`"]
pub struct B1492_W<'a> {
    w: &'a mut W,
}
impl<'a> B1492_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1493`"]
pub type B1493_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1493`"]
pub struct B1493_W<'a> {
    w: &'a mut W,
}
impl<'a> B1493_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1494`"]
pub type B1494_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1494`"]
pub struct B1494_W<'a> {
    w: &'a mut W,
}
impl<'a> B1494_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1495`"]
pub type B1495_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1495`"]
pub struct B1495_W<'a> {
    w: &'a mut W,
}
impl<'a> B1495_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1496`"]
pub type B1496_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1496`"]
pub struct B1496_W<'a> {
    w: &'a mut W,
}
impl<'a> B1496_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1497`"]
pub type B1497_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1497`"]
pub struct B1497_W<'a> {
    w: &'a mut W,
}
impl<'a> B1497_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1498`"]
pub type B1498_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1498`"]
pub struct B1498_W<'a> {
    w: &'a mut W,
}
impl<'a> B1498_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1499`"]
pub type B1499_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1499`"]
pub struct B1499_W<'a> {
    w: &'a mut W,
}
impl<'a> B1499_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1500`"]
pub type B1500_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1500`"]
pub struct B1500_W<'a> {
    w: &'a mut W,
}
impl<'a> B1500_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1501`"]
pub type B1501_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1501`"]
pub struct B1501_W<'a> {
    w: &'a mut W,
}
impl<'a> B1501_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1502`"]
pub type B1502_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1502`"]
pub struct B1502_W<'a> {
    w: &'a mut W,
}
impl<'a> B1502_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1503`"]
pub type B1503_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1503`"]
pub struct B1503_W<'a> {
    w: &'a mut W,
}
impl<'a> B1503_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1472"]
    #[inline(always)]
    pub fn b1472(&self) -> B1472_R {
        B1472_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1473"]
    #[inline(always)]
    pub fn b1473(&self) -> B1473_R {
        B1473_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1474"]
    #[inline(always)]
    pub fn b1474(&self) -> B1474_R {
        B1474_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1475"]
    #[inline(always)]
    pub fn b1475(&self) -> B1475_R {
        B1475_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1476"]
    #[inline(always)]
    pub fn b1476(&self) -> B1476_R {
        B1476_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1477"]
    #[inline(always)]
    pub fn b1477(&self) -> B1477_R {
        B1477_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1478"]
    #[inline(always)]
    pub fn b1478(&self) -> B1478_R {
        B1478_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1479"]
    #[inline(always)]
    pub fn b1479(&self) -> B1479_R {
        B1479_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1480"]
    #[inline(always)]
    pub fn b1480(&self) -> B1480_R {
        B1480_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1481"]
    #[inline(always)]
    pub fn b1481(&self) -> B1481_R {
        B1481_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1482"]
    #[inline(always)]
    pub fn b1482(&self) -> B1482_R {
        B1482_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1483"]
    #[inline(always)]
    pub fn b1483(&self) -> B1483_R {
        B1483_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1484"]
    #[inline(always)]
    pub fn b1484(&self) -> B1484_R {
        B1484_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1485"]
    #[inline(always)]
    pub fn b1485(&self) -> B1485_R {
        B1485_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1486"]
    #[inline(always)]
    pub fn b1486(&self) -> B1486_R {
        B1486_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1487"]
    #[inline(always)]
    pub fn b1487(&self) -> B1487_R {
        B1487_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1488"]
    #[inline(always)]
    pub fn b1488(&self) -> B1488_R {
        B1488_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1489"]
    #[inline(always)]
    pub fn b1489(&self) -> B1489_R {
        B1489_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1490"]
    #[inline(always)]
    pub fn b1490(&self) -> B1490_R {
        B1490_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1491"]
    #[inline(always)]
    pub fn b1491(&self) -> B1491_R {
        B1491_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1492"]
    #[inline(always)]
    pub fn b1492(&self) -> B1492_R {
        B1492_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1493"]
    #[inline(always)]
    pub fn b1493(&self) -> B1493_R {
        B1493_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1494"]
    #[inline(always)]
    pub fn b1494(&self) -> B1494_R {
        B1494_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1495"]
    #[inline(always)]
    pub fn b1495(&self) -> B1495_R {
        B1495_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1496"]
    #[inline(always)]
    pub fn b1496(&self) -> B1496_R {
        B1496_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1497"]
    #[inline(always)]
    pub fn b1497(&self) -> B1497_R {
        B1497_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1498"]
    #[inline(always)]
    pub fn b1498(&self) -> B1498_R {
        B1498_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1499"]
    #[inline(always)]
    pub fn b1499(&self) -> B1499_R {
        B1499_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1500"]
    #[inline(always)]
    pub fn b1500(&self) -> B1500_R {
        B1500_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1501"]
    #[inline(always)]
    pub fn b1501(&self) -> B1501_R {
        B1501_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1502"]
    #[inline(always)]
    pub fn b1502(&self) -> B1502_R {
        B1502_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1503"]
    #[inline(always)]
    pub fn b1503(&self) -> B1503_R {
        B1503_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1472"]
    #[inline(always)]
    pub fn b1472(&mut self) -> B1472_W {
        B1472_W { w: self }
    }
    #[doc = "Bit 1 - B1473"]
    #[inline(always)]
    pub fn b1473(&mut self) -> B1473_W {
        B1473_W { w: self }
    }
    #[doc = "Bit 2 - B1474"]
    #[inline(always)]
    pub fn b1474(&mut self) -> B1474_W {
        B1474_W { w: self }
    }
    #[doc = "Bit 3 - B1475"]
    #[inline(always)]
    pub fn b1475(&mut self) -> B1475_W {
        B1475_W { w: self }
    }
    #[doc = "Bit 4 - B1476"]
    #[inline(always)]
    pub fn b1476(&mut self) -> B1476_W {
        B1476_W { w: self }
    }
    #[doc = "Bit 5 - B1477"]
    #[inline(always)]
    pub fn b1477(&mut self) -> B1477_W {
        B1477_W { w: self }
    }
    #[doc = "Bit 6 - B1478"]
    #[inline(always)]
    pub fn b1478(&mut self) -> B1478_W {
        B1478_W { w: self }
    }
    #[doc = "Bit 7 - B1479"]
    #[inline(always)]
    pub fn b1479(&mut self) -> B1479_W {
        B1479_W { w: self }
    }
    #[doc = "Bit 8 - B1480"]
    #[inline(always)]
    pub fn b1480(&mut self) -> B1480_W {
        B1480_W { w: self }
    }
    #[doc = "Bit 9 - B1481"]
    #[inline(always)]
    pub fn b1481(&mut self) -> B1481_W {
        B1481_W { w: self }
    }
    #[doc = "Bit 10 - B1482"]
    #[inline(always)]
    pub fn b1482(&mut self) -> B1482_W {
        B1482_W { w: self }
    }
    #[doc = "Bit 11 - B1483"]
    #[inline(always)]
    pub fn b1483(&mut self) -> B1483_W {
        B1483_W { w: self }
    }
    #[doc = "Bit 12 - B1484"]
    #[inline(always)]
    pub fn b1484(&mut self) -> B1484_W {
        B1484_W { w: self }
    }
    #[doc = "Bit 13 - B1485"]
    #[inline(always)]
    pub fn b1485(&mut self) -> B1485_W {
        B1485_W { w: self }
    }
    #[doc = "Bit 14 - B1486"]
    #[inline(always)]
    pub fn b1486(&mut self) -> B1486_W {
        B1486_W { w: self }
    }
    #[doc = "Bit 15 - B1487"]
    #[inline(always)]
    pub fn b1487(&mut self) -> B1487_W {
        B1487_W { w: self }
    }
    #[doc = "Bit 16 - B1488"]
    #[inline(always)]
    pub fn b1488(&mut self) -> B1488_W {
        B1488_W { w: self }
    }
    #[doc = "Bit 17 - B1489"]
    #[inline(always)]
    pub fn b1489(&mut self) -> B1489_W {
        B1489_W { w: self }
    }
    #[doc = "Bit 18 - B1490"]
    #[inline(always)]
    pub fn b1490(&mut self) -> B1490_W {
        B1490_W { w: self }
    }
    #[doc = "Bit 19 - B1491"]
    #[inline(always)]
    pub fn b1491(&mut self) -> B1491_W {
        B1491_W { w: self }
    }
    #[doc = "Bit 20 - B1492"]
    #[inline(always)]
    pub fn b1492(&mut self) -> B1492_W {
        B1492_W { w: self }
    }
    #[doc = "Bit 21 - B1493"]
    #[inline(always)]
    pub fn b1493(&mut self) -> B1493_W {
        B1493_W { w: self }
    }
    #[doc = "Bit 22 - B1494"]
    #[inline(always)]
    pub fn b1494(&mut self) -> B1494_W {
        B1494_W { w: self }
    }
    #[doc = "Bit 23 - B1495"]
    #[inline(always)]
    pub fn b1495(&mut self) -> B1495_W {
        B1495_W { w: self }
    }
    #[doc = "Bit 24 - B1496"]
    #[inline(always)]
    pub fn b1496(&mut self) -> B1496_W {
        B1496_W { w: self }
    }
    #[doc = "Bit 25 - B1497"]
    #[inline(always)]
    pub fn b1497(&mut self) -> B1497_W {
        B1497_W { w: self }
    }
    #[doc = "Bit 26 - B1498"]
    #[inline(always)]
    pub fn b1498(&mut self) -> B1498_W {
        B1498_W { w: self }
    }
    #[doc = "Bit 27 - B1499"]
    #[inline(always)]
    pub fn b1499(&mut self) -> B1499_W {
        B1499_W { w: self }
    }
    #[doc = "Bit 28 - B1500"]
    #[inline(always)]
    pub fn b1500(&mut self) -> B1500_W {
        B1500_W { w: self }
    }
    #[doc = "Bit 29 - B1501"]
    #[inline(always)]
    pub fn b1501(&mut self) -> B1501_W {
        B1501_W { w: self }
    }
    #[doc = "Bit 30 - B1502"]
    #[inline(always)]
    pub fn b1502(&mut self) -> B1502_W {
        B1502_W { w: self }
    }
    #[doc = "Bit 31 - B1503"]
    #[inline(always)]
    pub fn b1503(&mut self) -> B1503_W {
        B1503_W { w: self }
    }
}
