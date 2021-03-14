#[doc = "Reader of register MPCBB1_VCTR45"]
pub type R = crate::R<u32, super::MPCBB1_VCTR45>;
#[doc = "Writer for register MPCBB1_VCTR45"]
pub type W = crate::W<u32, super::MPCBB1_VCTR45>;
#[doc = "Register MPCBB1_VCTR45 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR45 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1440`"]
pub type B1440_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1440`"]
pub struct B1440_W<'a> {
    w: &'a mut W,
}
impl<'a> B1440_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1441`"]
pub type B1441_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1441`"]
pub struct B1441_W<'a> {
    w: &'a mut W,
}
impl<'a> B1441_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1442`"]
pub type B1442_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1442`"]
pub struct B1442_W<'a> {
    w: &'a mut W,
}
impl<'a> B1442_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1443`"]
pub type B1443_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1443`"]
pub struct B1443_W<'a> {
    w: &'a mut W,
}
impl<'a> B1443_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1444`"]
pub type B1444_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1444`"]
pub struct B1444_W<'a> {
    w: &'a mut W,
}
impl<'a> B1444_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1445`"]
pub type B1445_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1445`"]
pub struct B1445_W<'a> {
    w: &'a mut W,
}
impl<'a> B1445_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1446`"]
pub type B1446_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1446`"]
pub struct B1446_W<'a> {
    w: &'a mut W,
}
impl<'a> B1446_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1447`"]
pub type B1447_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1447`"]
pub struct B1447_W<'a> {
    w: &'a mut W,
}
impl<'a> B1447_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1448`"]
pub type B1448_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1448`"]
pub struct B1448_W<'a> {
    w: &'a mut W,
}
impl<'a> B1448_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1449`"]
pub type B1449_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1449`"]
pub struct B1449_W<'a> {
    w: &'a mut W,
}
impl<'a> B1449_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1450`"]
pub type B1450_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1450`"]
pub struct B1450_W<'a> {
    w: &'a mut W,
}
impl<'a> B1450_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1451`"]
pub type B1451_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1451`"]
pub struct B1451_W<'a> {
    w: &'a mut W,
}
impl<'a> B1451_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1452`"]
pub type B1452_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1452`"]
pub struct B1452_W<'a> {
    w: &'a mut W,
}
impl<'a> B1452_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1453`"]
pub type B1453_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1453`"]
pub struct B1453_W<'a> {
    w: &'a mut W,
}
impl<'a> B1453_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1454`"]
pub type B1454_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1454`"]
pub struct B1454_W<'a> {
    w: &'a mut W,
}
impl<'a> B1454_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1455`"]
pub type B1455_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1455`"]
pub struct B1455_W<'a> {
    w: &'a mut W,
}
impl<'a> B1455_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1456`"]
pub type B1456_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1456`"]
pub struct B1456_W<'a> {
    w: &'a mut W,
}
impl<'a> B1456_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1457`"]
pub type B1457_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1457`"]
pub struct B1457_W<'a> {
    w: &'a mut W,
}
impl<'a> B1457_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1458`"]
pub type B1458_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1458`"]
pub struct B1458_W<'a> {
    w: &'a mut W,
}
impl<'a> B1458_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1459`"]
pub type B1459_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1459`"]
pub struct B1459_W<'a> {
    w: &'a mut W,
}
impl<'a> B1459_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1460`"]
pub type B1460_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1460`"]
pub struct B1460_W<'a> {
    w: &'a mut W,
}
impl<'a> B1460_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1461`"]
pub type B1461_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1461`"]
pub struct B1461_W<'a> {
    w: &'a mut W,
}
impl<'a> B1461_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1462`"]
pub type B1462_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1462`"]
pub struct B1462_W<'a> {
    w: &'a mut W,
}
impl<'a> B1462_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1463`"]
pub type B1463_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1463`"]
pub struct B1463_W<'a> {
    w: &'a mut W,
}
impl<'a> B1463_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1464`"]
pub type B1464_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1464`"]
pub struct B1464_W<'a> {
    w: &'a mut W,
}
impl<'a> B1464_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1465`"]
pub type B1465_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1465`"]
pub struct B1465_W<'a> {
    w: &'a mut W,
}
impl<'a> B1465_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1466`"]
pub type B1466_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1466`"]
pub struct B1466_W<'a> {
    w: &'a mut W,
}
impl<'a> B1466_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1467`"]
pub type B1467_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1467`"]
pub struct B1467_W<'a> {
    w: &'a mut W,
}
impl<'a> B1467_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1468`"]
pub type B1468_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1468`"]
pub struct B1468_W<'a> {
    w: &'a mut W,
}
impl<'a> B1468_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1469`"]
pub type B1469_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1469`"]
pub struct B1469_W<'a> {
    w: &'a mut W,
}
impl<'a> B1469_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1470`"]
pub type B1470_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1470`"]
pub struct B1470_W<'a> {
    w: &'a mut W,
}
impl<'a> B1470_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1471`"]
pub type B1471_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1471`"]
pub struct B1471_W<'a> {
    w: &'a mut W,
}
impl<'a> B1471_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1440"]
    #[inline(always)]
    pub fn b1440(&self) -> B1440_R {
        B1440_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1441"]
    #[inline(always)]
    pub fn b1441(&self) -> B1441_R {
        B1441_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1442"]
    #[inline(always)]
    pub fn b1442(&self) -> B1442_R {
        B1442_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1443"]
    #[inline(always)]
    pub fn b1443(&self) -> B1443_R {
        B1443_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1444"]
    #[inline(always)]
    pub fn b1444(&self) -> B1444_R {
        B1444_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1445"]
    #[inline(always)]
    pub fn b1445(&self) -> B1445_R {
        B1445_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1446"]
    #[inline(always)]
    pub fn b1446(&self) -> B1446_R {
        B1446_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1447"]
    #[inline(always)]
    pub fn b1447(&self) -> B1447_R {
        B1447_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1448"]
    #[inline(always)]
    pub fn b1448(&self) -> B1448_R {
        B1448_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1449"]
    #[inline(always)]
    pub fn b1449(&self) -> B1449_R {
        B1449_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1450"]
    #[inline(always)]
    pub fn b1450(&self) -> B1450_R {
        B1450_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1451"]
    #[inline(always)]
    pub fn b1451(&self) -> B1451_R {
        B1451_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1452"]
    #[inline(always)]
    pub fn b1452(&self) -> B1452_R {
        B1452_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1453"]
    #[inline(always)]
    pub fn b1453(&self) -> B1453_R {
        B1453_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1454"]
    #[inline(always)]
    pub fn b1454(&self) -> B1454_R {
        B1454_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1455"]
    #[inline(always)]
    pub fn b1455(&self) -> B1455_R {
        B1455_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1456"]
    #[inline(always)]
    pub fn b1456(&self) -> B1456_R {
        B1456_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1457"]
    #[inline(always)]
    pub fn b1457(&self) -> B1457_R {
        B1457_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1458"]
    #[inline(always)]
    pub fn b1458(&self) -> B1458_R {
        B1458_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1459"]
    #[inline(always)]
    pub fn b1459(&self) -> B1459_R {
        B1459_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1460"]
    #[inline(always)]
    pub fn b1460(&self) -> B1460_R {
        B1460_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1461"]
    #[inline(always)]
    pub fn b1461(&self) -> B1461_R {
        B1461_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1462"]
    #[inline(always)]
    pub fn b1462(&self) -> B1462_R {
        B1462_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1463"]
    #[inline(always)]
    pub fn b1463(&self) -> B1463_R {
        B1463_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1464"]
    #[inline(always)]
    pub fn b1464(&self) -> B1464_R {
        B1464_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1465"]
    #[inline(always)]
    pub fn b1465(&self) -> B1465_R {
        B1465_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1466"]
    #[inline(always)]
    pub fn b1466(&self) -> B1466_R {
        B1466_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1467"]
    #[inline(always)]
    pub fn b1467(&self) -> B1467_R {
        B1467_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1468"]
    #[inline(always)]
    pub fn b1468(&self) -> B1468_R {
        B1468_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1469"]
    #[inline(always)]
    pub fn b1469(&self) -> B1469_R {
        B1469_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1470"]
    #[inline(always)]
    pub fn b1470(&self) -> B1470_R {
        B1470_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1471"]
    #[inline(always)]
    pub fn b1471(&self) -> B1471_R {
        B1471_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1440"]
    #[inline(always)]
    pub fn b1440(&mut self) -> B1440_W {
        B1440_W { w: self }
    }
    #[doc = "Bit 1 - B1441"]
    #[inline(always)]
    pub fn b1441(&mut self) -> B1441_W {
        B1441_W { w: self }
    }
    #[doc = "Bit 2 - B1442"]
    #[inline(always)]
    pub fn b1442(&mut self) -> B1442_W {
        B1442_W { w: self }
    }
    #[doc = "Bit 3 - B1443"]
    #[inline(always)]
    pub fn b1443(&mut self) -> B1443_W {
        B1443_W { w: self }
    }
    #[doc = "Bit 4 - B1444"]
    #[inline(always)]
    pub fn b1444(&mut self) -> B1444_W {
        B1444_W { w: self }
    }
    #[doc = "Bit 5 - B1445"]
    #[inline(always)]
    pub fn b1445(&mut self) -> B1445_W {
        B1445_W { w: self }
    }
    #[doc = "Bit 6 - B1446"]
    #[inline(always)]
    pub fn b1446(&mut self) -> B1446_W {
        B1446_W { w: self }
    }
    #[doc = "Bit 7 - B1447"]
    #[inline(always)]
    pub fn b1447(&mut self) -> B1447_W {
        B1447_W { w: self }
    }
    #[doc = "Bit 8 - B1448"]
    #[inline(always)]
    pub fn b1448(&mut self) -> B1448_W {
        B1448_W { w: self }
    }
    #[doc = "Bit 9 - B1449"]
    #[inline(always)]
    pub fn b1449(&mut self) -> B1449_W {
        B1449_W { w: self }
    }
    #[doc = "Bit 10 - B1450"]
    #[inline(always)]
    pub fn b1450(&mut self) -> B1450_W {
        B1450_W { w: self }
    }
    #[doc = "Bit 11 - B1451"]
    #[inline(always)]
    pub fn b1451(&mut self) -> B1451_W {
        B1451_W { w: self }
    }
    #[doc = "Bit 12 - B1452"]
    #[inline(always)]
    pub fn b1452(&mut self) -> B1452_W {
        B1452_W { w: self }
    }
    #[doc = "Bit 13 - B1453"]
    #[inline(always)]
    pub fn b1453(&mut self) -> B1453_W {
        B1453_W { w: self }
    }
    #[doc = "Bit 14 - B1454"]
    #[inline(always)]
    pub fn b1454(&mut self) -> B1454_W {
        B1454_W { w: self }
    }
    #[doc = "Bit 15 - B1455"]
    #[inline(always)]
    pub fn b1455(&mut self) -> B1455_W {
        B1455_W { w: self }
    }
    #[doc = "Bit 16 - B1456"]
    #[inline(always)]
    pub fn b1456(&mut self) -> B1456_W {
        B1456_W { w: self }
    }
    #[doc = "Bit 17 - B1457"]
    #[inline(always)]
    pub fn b1457(&mut self) -> B1457_W {
        B1457_W { w: self }
    }
    #[doc = "Bit 18 - B1458"]
    #[inline(always)]
    pub fn b1458(&mut self) -> B1458_W {
        B1458_W { w: self }
    }
    #[doc = "Bit 19 - B1459"]
    #[inline(always)]
    pub fn b1459(&mut self) -> B1459_W {
        B1459_W { w: self }
    }
    #[doc = "Bit 20 - B1460"]
    #[inline(always)]
    pub fn b1460(&mut self) -> B1460_W {
        B1460_W { w: self }
    }
    #[doc = "Bit 21 - B1461"]
    #[inline(always)]
    pub fn b1461(&mut self) -> B1461_W {
        B1461_W { w: self }
    }
    #[doc = "Bit 22 - B1462"]
    #[inline(always)]
    pub fn b1462(&mut self) -> B1462_W {
        B1462_W { w: self }
    }
    #[doc = "Bit 23 - B1463"]
    #[inline(always)]
    pub fn b1463(&mut self) -> B1463_W {
        B1463_W { w: self }
    }
    #[doc = "Bit 24 - B1464"]
    #[inline(always)]
    pub fn b1464(&mut self) -> B1464_W {
        B1464_W { w: self }
    }
    #[doc = "Bit 25 - B1465"]
    #[inline(always)]
    pub fn b1465(&mut self) -> B1465_W {
        B1465_W { w: self }
    }
    #[doc = "Bit 26 - B1466"]
    #[inline(always)]
    pub fn b1466(&mut self) -> B1466_W {
        B1466_W { w: self }
    }
    #[doc = "Bit 27 - B1467"]
    #[inline(always)]
    pub fn b1467(&mut self) -> B1467_W {
        B1467_W { w: self }
    }
    #[doc = "Bit 28 - B1468"]
    #[inline(always)]
    pub fn b1468(&mut self) -> B1468_W {
        B1468_W { w: self }
    }
    #[doc = "Bit 29 - B1469"]
    #[inline(always)]
    pub fn b1469(&mut self) -> B1469_W {
        B1469_W { w: self }
    }
    #[doc = "Bit 30 - B1470"]
    #[inline(always)]
    pub fn b1470(&mut self) -> B1470_W {
        B1470_W { w: self }
    }
    #[doc = "Bit 31 - B1471"]
    #[inline(always)]
    pub fn b1471(&mut self) -> B1471_W {
        B1471_W { w: self }
    }
}
