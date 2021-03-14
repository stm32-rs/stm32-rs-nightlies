#[doc = "Reader of register MPCBB2_VCTR48"]
pub type R = crate::R<u32, super::MPCBB2_VCTR48>;
#[doc = "Writer for register MPCBB2_VCTR48"]
pub type W = crate::W<u32, super::MPCBB2_VCTR48>;
#[doc = "Register MPCBB2_VCTR48 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR48 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1536`"]
pub type B1536_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1536`"]
pub struct B1536_W<'a> {
    w: &'a mut W,
}
impl<'a> B1536_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1537`"]
pub type B1537_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1537`"]
pub struct B1537_W<'a> {
    w: &'a mut W,
}
impl<'a> B1537_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1538`"]
pub type B1538_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1538`"]
pub struct B1538_W<'a> {
    w: &'a mut W,
}
impl<'a> B1538_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1539`"]
pub type B1539_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1539`"]
pub struct B1539_W<'a> {
    w: &'a mut W,
}
impl<'a> B1539_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1540`"]
pub type B1540_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1540`"]
pub struct B1540_W<'a> {
    w: &'a mut W,
}
impl<'a> B1540_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1541`"]
pub type B1541_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1541`"]
pub struct B1541_W<'a> {
    w: &'a mut W,
}
impl<'a> B1541_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1542`"]
pub type B1542_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1542`"]
pub struct B1542_W<'a> {
    w: &'a mut W,
}
impl<'a> B1542_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1543`"]
pub type B1543_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1543`"]
pub struct B1543_W<'a> {
    w: &'a mut W,
}
impl<'a> B1543_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1544`"]
pub type B1544_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1544`"]
pub struct B1544_W<'a> {
    w: &'a mut W,
}
impl<'a> B1544_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1545`"]
pub type B1545_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1545`"]
pub struct B1545_W<'a> {
    w: &'a mut W,
}
impl<'a> B1545_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1546`"]
pub type B1546_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1546`"]
pub struct B1546_W<'a> {
    w: &'a mut W,
}
impl<'a> B1546_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1547`"]
pub type B1547_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1547`"]
pub struct B1547_W<'a> {
    w: &'a mut W,
}
impl<'a> B1547_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1548`"]
pub type B1548_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1548`"]
pub struct B1548_W<'a> {
    w: &'a mut W,
}
impl<'a> B1548_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1549`"]
pub type B1549_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1549`"]
pub struct B1549_W<'a> {
    w: &'a mut W,
}
impl<'a> B1549_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1550`"]
pub type B1550_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1550`"]
pub struct B1550_W<'a> {
    w: &'a mut W,
}
impl<'a> B1550_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1551`"]
pub type B1551_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1551`"]
pub struct B1551_W<'a> {
    w: &'a mut W,
}
impl<'a> B1551_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1552`"]
pub type B1552_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1552`"]
pub struct B1552_W<'a> {
    w: &'a mut W,
}
impl<'a> B1552_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1553`"]
pub type B1553_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1553`"]
pub struct B1553_W<'a> {
    w: &'a mut W,
}
impl<'a> B1553_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1554`"]
pub type B1554_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1554`"]
pub struct B1554_W<'a> {
    w: &'a mut W,
}
impl<'a> B1554_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1555`"]
pub type B1555_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1555`"]
pub struct B1555_W<'a> {
    w: &'a mut W,
}
impl<'a> B1555_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1556`"]
pub type B1556_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1556`"]
pub struct B1556_W<'a> {
    w: &'a mut W,
}
impl<'a> B1556_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1557`"]
pub type B1557_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1557`"]
pub struct B1557_W<'a> {
    w: &'a mut W,
}
impl<'a> B1557_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1558`"]
pub type B1558_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1558`"]
pub struct B1558_W<'a> {
    w: &'a mut W,
}
impl<'a> B1558_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1559`"]
pub type B1559_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1559`"]
pub struct B1559_W<'a> {
    w: &'a mut W,
}
impl<'a> B1559_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1560`"]
pub type B1560_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1560`"]
pub struct B1560_W<'a> {
    w: &'a mut W,
}
impl<'a> B1560_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1561`"]
pub type B1561_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1561`"]
pub struct B1561_W<'a> {
    w: &'a mut W,
}
impl<'a> B1561_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1562`"]
pub type B1562_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1562`"]
pub struct B1562_W<'a> {
    w: &'a mut W,
}
impl<'a> B1562_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1563`"]
pub type B1563_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1563`"]
pub struct B1563_W<'a> {
    w: &'a mut W,
}
impl<'a> B1563_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1564`"]
pub type B1564_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1564`"]
pub struct B1564_W<'a> {
    w: &'a mut W,
}
impl<'a> B1564_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1565`"]
pub type B1565_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1565`"]
pub struct B1565_W<'a> {
    w: &'a mut W,
}
impl<'a> B1565_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1566`"]
pub type B1566_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1566`"]
pub struct B1566_W<'a> {
    w: &'a mut W,
}
impl<'a> B1566_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1567`"]
pub type B1567_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1567`"]
pub struct B1567_W<'a> {
    w: &'a mut W,
}
impl<'a> B1567_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1536"]
    #[inline(always)]
    pub fn b1536(&self) -> B1536_R {
        B1536_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1537"]
    #[inline(always)]
    pub fn b1537(&self) -> B1537_R {
        B1537_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1538"]
    #[inline(always)]
    pub fn b1538(&self) -> B1538_R {
        B1538_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1539"]
    #[inline(always)]
    pub fn b1539(&self) -> B1539_R {
        B1539_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1540"]
    #[inline(always)]
    pub fn b1540(&self) -> B1540_R {
        B1540_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1541"]
    #[inline(always)]
    pub fn b1541(&self) -> B1541_R {
        B1541_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1542"]
    #[inline(always)]
    pub fn b1542(&self) -> B1542_R {
        B1542_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1543"]
    #[inline(always)]
    pub fn b1543(&self) -> B1543_R {
        B1543_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1544"]
    #[inline(always)]
    pub fn b1544(&self) -> B1544_R {
        B1544_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1545"]
    #[inline(always)]
    pub fn b1545(&self) -> B1545_R {
        B1545_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1546"]
    #[inline(always)]
    pub fn b1546(&self) -> B1546_R {
        B1546_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1547"]
    #[inline(always)]
    pub fn b1547(&self) -> B1547_R {
        B1547_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1548"]
    #[inline(always)]
    pub fn b1548(&self) -> B1548_R {
        B1548_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1549"]
    #[inline(always)]
    pub fn b1549(&self) -> B1549_R {
        B1549_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1550"]
    #[inline(always)]
    pub fn b1550(&self) -> B1550_R {
        B1550_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1551"]
    #[inline(always)]
    pub fn b1551(&self) -> B1551_R {
        B1551_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1552"]
    #[inline(always)]
    pub fn b1552(&self) -> B1552_R {
        B1552_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1553"]
    #[inline(always)]
    pub fn b1553(&self) -> B1553_R {
        B1553_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1554"]
    #[inline(always)]
    pub fn b1554(&self) -> B1554_R {
        B1554_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1555"]
    #[inline(always)]
    pub fn b1555(&self) -> B1555_R {
        B1555_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1556"]
    #[inline(always)]
    pub fn b1556(&self) -> B1556_R {
        B1556_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1557"]
    #[inline(always)]
    pub fn b1557(&self) -> B1557_R {
        B1557_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1558"]
    #[inline(always)]
    pub fn b1558(&self) -> B1558_R {
        B1558_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1559"]
    #[inline(always)]
    pub fn b1559(&self) -> B1559_R {
        B1559_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1560"]
    #[inline(always)]
    pub fn b1560(&self) -> B1560_R {
        B1560_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1561"]
    #[inline(always)]
    pub fn b1561(&self) -> B1561_R {
        B1561_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1562"]
    #[inline(always)]
    pub fn b1562(&self) -> B1562_R {
        B1562_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1563"]
    #[inline(always)]
    pub fn b1563(&self) -> B1563_R {
        B1563_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1564"]
    #[inline(always)]
    pub fn b1564(&self) -> B1564_R {
        B1564_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1565"]
    #[inline(always)]
    pub fn b1565(&self) -> B1565_R {
        B1565_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1566"]
    #[inline(always)]
    pub fn b1566(&self) -> B1566_R {
        B1566_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1567"]
    #[inline(always)]
    pub fn b1567(&self) -> B1567_R {
        B1567_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1536"]
    #[inline(always)]
    pub fn b1536(&mut self) -> B1536_W {
        B1536_W { w: self }
    }
    #[doc = "Bit 1 - B1537"]
    #[inline(always)]
    pub fn b1537(&mut self) -> B1537_W {
        B1537_W { w: self }
    }
    #[doc = "Bit 2 - B1538"]
    #[inline(always)]
    pub fn b1538(&mut self) -> B1538_W {
        B1538_W { w: self }
    }
    #[doc = "Bit 3 - B1539"]
    #[inline(always)]
    pub fn b1539(&mut self) -> B1539_W {
        B1539_W { w: self }
    }
    #[doc = "Bit 4 - B1540"]
    #[inline(always)]
    pub fn b1540(&mut self) -> B1540_W {
        B1540_W { w: self }
    }
    #[doc = "Bit 5 - B1541"]
    #[inline(always)]
    pub fn b1541(&mut self) -> B1541_W {
        B1541_W { w: self }
    }
    #[doc = "Bit 6 - B1542"]
    #[inline(always)]
    pub fn b1542(&mut self) -> B1542_W {
        B1542_W { w: self }
    }
    #[doc = "Bit 7 - B1543"]
    #[inline(always)]
    pub fn b1543(&mut self) -> B1543_W {
        B1543_W { w: self }
    }
    #[doc = "Bit 8 - B1544"]
    #[inline(always)]
    pub fn b1544(&mut self) -> B1544_W {
        B1544_W { w: self }
    }
    #[doc = "Bit 9 - B1545"]
    #[inline(always)]
    pub fn b1545(&mut self) -> B1545_W {
        B1545_W { w: self }
    }
    #[doc = "Bit 10 - B1546"]
    #[inline(always)]
    pub fn b1546(&mut self) -> B1546_W {
        B1546_W { w: self }
    }
    #[doc = "Bit 11 - B1547"]
    #[inline(always)]
    pub fn b1547(&mut self) -> B1547_W {
        B1547_W { w: self }
    }
    #[doc = "Bit 12 - B1548"]
    #[inline(always)]
    pub fn b1548(&mut self) -> B1548_W {
        B1548_W { w: self }
    }
    #[doc = "Bit 13 - B1549"]
    #[inline(always)]
    pub fn b1549(&mut self) -> B1549_W {
        B1549_W { w: self }
    }
    #[doc = "Bit 14 - B1550"]
    #[inline(always)]
    pub fn b1550(&mut self) -> B1550_W {
        B1550_W { w: self }
    }
    #[doc = "Bit 15 - B1551"]
    #[inline(always)]
    pub fn b1551(&mut self) -> B1551_W {
        B1551_W { w: self }
    }
    #[doc = "Bit 16 - B1552"]
    #[inline(always)]
    pub fn b1552(&mut self) -> B1552_W {
        B1552_W { w: self }
    }
    #[doc = "Bit 17 - B1553"]
    #[inline(always)]
    pub fn b1553(&mut self) -> B1553_W {
        B1553_W { w: self }
    }
    #[doc = "Bit 18 - B1554"]
    #[inline(always)]
    pub fn b1554(&mut self) -> B1554_W {
        B1554_W { w: self }
    }
    #[doc = "Bit 19 - B1555"]
    #[inline(always)]
    pub fn b1555(&mut self) -> B1555_W {
        B1555_W { w: self }
    }
    #[doc = "Bit 20 - B1556"]
    #[inline(always)]
    pub fn b1556(&mut self) -> B1556_W {
        B1556_W { w: self }
    }
    #[doc = "Bit 21 - B1557"]
    #[inline(always)]
    pub fn b1557(&mut self) -> B1557_W {
        B1557_W { w: self }
    }
    #[doc = "Bit 22 - B1558"]
    #[inline(always)]
    pub fn b1558(&mut self) -> B1558_W {
        B1558_W { w: self }
    }
    #[doc = "Bit 23 - B1559"]
    #[inline(always)]
    pub fn b1559(&mut self) -> B1559_W {
        B1559_W { w: self }
    }
    #[doc = "Bit 24 - B1560"]
    #[inline(always)]
    pub fn b1560(&mut self) -> B1560_W {
        B1560_W { w: self }
    }
    #[doc = "Bit 25 - B1561"]
    #[inline(always)]
    pub fn b1561(&mut self) -> B1561_W {
        B1561_W { w: self }
    }
    #[doc = "Bit 26 - B1562"]
    #[inline(always)]
    pub fn b1562(&mut self) -> B1562_W {
        B1562_W { w: self }
    }
    #[doc = "Bit 27 - B1563"]
    #[inline(always)]
    pub fn b1563(&mut self) -> B1563_W {
        B1563_W { w: self }
    }
    #[doc = "Bit 28 - B1564"]
    #[inline(always)]
    pub fn b1564(&mut self) -> B1564_W {
        B1564_W { w: self }
    }
    #[doc = "Bit 29 - B1565"]
    #[inline(always)]
    pub fn b1565(&mut self) -> B1565_W {
        B1565_W { w: self }
    }
    #[doc = "Bit 30 - B1566"]
    #[inline(always)]
    pub fn b1566(&mut self) -> B1566_W {
        B1566_W { w: self }
    }
    #[doc = "Bit 31 - B1567"]
    #[inline(always)]
    pub fn b1567(&mut self) -> B1567_W {
        B1567_W { w: self }
    }
}
