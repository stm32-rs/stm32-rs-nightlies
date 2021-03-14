#[doc = "Reader of register MPCBB2_VCTR50"]
pub type R = crate::R<u32, super::MPCBB2_VCTR50>;
#[doc = "Writer for register MPCBB2_VCTR50"]
pub type W = crate::W<u32, super::MPCBB2_VCTR50>;
#[doc = "Register MPCBB2_VCTR50 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR50 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1600`"]
pub type B1600_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1600`"]
pub struct B1600_W<'a> {
    w: &'a mut W,
}
impl<'a> B1600_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1601`"]
pub type B1601_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1601`"]
pub struct B1601_W<'a> {
    w: &'a mut W,
}
impl<'a> B1601_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1602`"]
pub type B1602_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1602`"]
pub struct B1602_W<'a> {
    w: &'a mut W,
}
impl<'a> B1602_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1603`"]
pub type B1603_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1603`"]
pub struct B1603_W<'a> {
    w: &'a mut W,
}
impl<'a> B1603_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1604`"]
pub type B1604_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1604`"]
pub struct B1604_W<'a> {
    w: &'a mut W,
}
impl<'a> B1604_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1605`"]
pub type B1605_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1605`"]
pub struct B1605_W<'a> {
    w: &'a mut W,
}
impl<'a> B1605_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1606`"]
pub type B1606_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1606`"]
pub struct B1606_W<'a> {
    w: &'a mut W,
}
impl<'a> B1606_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1607`"]
pub type B1607_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1607`"]
pub struct B1607_W<'a> {
    w: &'a mut W,
}
impl<'a> B1607_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1608`"]
pub type B1608_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1608`"]
pub struct B1608_W<'a> {
    w: &'a mut W,
}
impl<'a> B1608_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1609`"]
pub type B1609_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1609`"]
pub struct B1609_W<'a> {
    w: &'a mut W,
}
impl<'a> B1609_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1610`"]
pub type B1610_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1610`"]
pub struct B1610_W<'a> {
    w: &'a mut W,
}
impl<'a> B1610_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1611`"]
pub type B1611_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1611`"]
pub struct B1611_W<'a> {
    w: &'a mut W,
}
impl<'a> B1611_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1612`"]
pub type B1612_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1612`"]
pub struct B1612_W<'a> {
    w: &'a mut W,
}
impl<'a> B1612_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1613`"]
pub type B1613_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1613`"]
pub struct B1613_W<'a> {
    w: &'a mut W,
}
impl<'a> B1613_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1614`"]
pub type B1614_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1614`"]
pub struct B1614_W<'a> {
    w: &'a mut W,
}
impl<'a> B1614_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1615`"]
pub type B1615_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1615`"]
pub struct B1615_W<'a> {
    w: &'a mut W,
}
impl<'a> B1615_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1616`"]
pub type B1616_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1616`"]
pub struct B1616_W<'a> {
    w: &'a mut W,
}
impl<'a> B1616_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1617`"]
pub type B1617_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1617`"]
pub struct B1617_W<'a> {
    w: &'a mut W,
}
impl<'a> B1617_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1618`"]
pub type B1618_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1618`"]
pub struct B1618_W<'a> {
    w: &'a mut W,
}
impl<'a> B1618_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1619`"]
pub type B1619_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1619`"]
pub struct B1619_W<'a> {
    w: &'a mut W,
}
impl<'a> B1619_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1620`"]
pub type B1620_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1620`"]
pub struct B1620_W<'a> {
    w: &'a mut W,
}
impl<'a> B1620_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1621`"]
pub type B1621_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1621`"]
pub struct B1621_W<'a> {
    w: &'a mut W,
}
impl<'a> B1621_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1622`"]
pub type B1622_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1622`"]
pub struct B1622_W<'a> {
    w: &'a mut W,
}
impl<'a> B1622_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1623`"]
pub type B1623_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1623`"]
pub struct B1623_W<'a> {
    w: &'a mut W,
}
impl<'a> B1623_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1624`"]
pub type B1624_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1624`"]
pub struct B1624_W<'a> {
    w: &'a mut W,
}
impl<'a> B1624_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1625`"]
pub type B1625_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1625`"]
pub struct B1625_W<'a> {
    w: &'a mut W,
}
impl<'a> B1625_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1626`"]
pub type B1626_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1626`"]
pub struct B1626_W<'a> {
    w: &'a mut W,
}
impl<'a> B1626_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1627`"]
pub type B1627_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1627`"]
pub struct B1627_W<'a> {
    w: &'a mut W,
}
impl<'a> B1627_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1628`"]
pub type B1628_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1628`"]
pub struct B1628_W<'a> {
    w: &'a mut W,
}
impl<'a> B1628_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1629`"]
pub type B1629_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1629`"]
pub struct B1629_W<'a> {
    w: &'a mut W,
}
impl<'a> B1629_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1630`"]
pub type B1630_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1630`"]
pub struct B1630_W<'a> {
    w: &'a mut W,
}
impl<'a> B1630_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1631`"]
pub type B1631_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1631`"]
pub struct B1631_W<'a> {
    w: &'a mut W,
}
impl<'a> B1631_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1600"]
    #[inline(always)]
    pub fn b1600(&self) -> B1600_R {
        B1600_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1601"]
    #[inline(always)]
    pub fn b1601(&self) -> B1601_R {
        B1601_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1602"]
    #[inline(always)]
    pub fn b1602(&self) -> B1602_R {
        B1602_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1603"]
    #[inline(always)]
    pub fn b1603(&self) -> B1603_R {
        B1603_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1604"]
    #[inline(always)]
    pub fn b1604(&self) -> B1604_R {
        B1604_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1605"]
    #[inline(always)]
    pub fn b1605(&self) -> B1605_R {
        B1605_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1606"]
    #[inline(always)]
    pub fn b1606(&self) -> B1606_R {
        B1606_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1607"]
    #[inline(always)]
    pub fn b1607(&self) -> B1607_R {
        B1607_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1608"]
    #[inline(always)]
    pub fn b1608(&self) -> B1608_R {
        B1608_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1609"]
    #[inline(always)]
    pub fn b1609(&self) -> B1609_R {
        B1609_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1610"]
    #[inline(always)]
    pub fn b1610(&self) -> B1610_R {
        B1610_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1611"]
    #[inline(always)]
    pub fn b1611(&self) -> B1611_R {
        B1611_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1612"]
    #[inline(always)]
    pub fn b1612(&self) -> B1612_R {
        B1612_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1613"]
    #[inline(always)]
    pub fn b1613(&self) -> B1613_R {
        B1613_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1614"]
    #[inline(always)]
    pub fn b1614(&self) -> B1614_R {
        B1614_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1615"]
    #[inline(always)]
    pub fn b1615(&self) -> B1615_R {
        B1615_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1616"]
    #[inline(always)]
    pub fn b1616(&self) -> B1616_R {
        B1616_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1617"]
    #[inline(always)]
    pub fn b1617(&self) -> B1617_R {
        B1617_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1618"]
    #[inline(always)]
    pub fn b1618(&self) -> B1618_R {
        B1618_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1619"]
    #[inline(always)]
    pub fn b1619(&self) -> B1619_R {
        B1619_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1620"]
    #[inline(always)]
    pub fn b1620(&self) -> B1620_R {
        B1620_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1621"]
    #[inline(always)]
    pub fn b1621(&self) -> B1621_R {
        B1621_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1622"]
    #[inline(always)]
    pub fn b1622(&self) -> B1622_R {
        B1622_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1623"]
    #[inline(always)]
    pub fn b1623(&self) -> B1623_R {
        B1623_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1624"]
    #[inline(always)]
    pub fn b1624(&self) -> B1624_R {
        B1624_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1625"]
    #[inline(always)]
    pub fn b1625(&self) -> B1625_R {
        B1625_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1626"]
    #[inline(always)]
    pub fn b1626(&self) -> B1626_R {
        B1626_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1627"]
    #[inline(always)]
    pub fn b1627(&self) -> B1627_R {
        B1627_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1628"]
    #[inline(always)]
    pub fn b1628(&self) -> B1628_R {
        B1628_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1629"]
    #[inline(always)]
    pub fn b1629(&self) -> B1629_R {
        B1629_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1630"]
    #[inline(always)]
    pub fn b1630(&self) -> B1630_R {
        B1630_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1631"]
    #[inline(always)]
    pub fn b1631(&self) -> B1631_R {
        B1631_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1600"]
    #[inline(always)]
    pub fn b1600(&mut self) -> B1600_W {
        B1600_W { w: self }
    }
    #[doc = "Bit 1 - B1601"]
    #[inline(always)]
    pub fn b1601(&mut self) -> B1601_W {
        B1601_W { w: self }
    }
    #[doc = "Bit 2 - B1602"]
    #[inline(always)]
    pub fn b1602(&mut self) -> B1602_W {
        B1602_W { w: self }
    }
    #[doc = "Bit 3 - B1603"]
    #[inline(always)]
    pub fn b1603(&mut self) -> B1603_W {
        B1603_W { w: self }
    }
    #[doc = "Bit 4 - B1604"]
    #[inline(always)]
    pub fn b1604(&mut self) -> B1604_W {
        B1604_W { w: self }
    }
    #[doc = "Bit 5 - B1605"]
    #[inline(always)]
    pub fn b1605(&mut self) -> B1605_W {
        B1605_W { w: self }
    }
    #[doc = "Bit 6 - B1606"]
    #[inline(always)]
    pub fn b1606(&mut self) -> B1606_W {
        B1606_W { w: self }
    }
    #[doc = "Bit 7 - B1607"]
    #[inline(always)]
    pub fn b1607(&mut self) -> B1607_W {
        B1607_W { w: self }
    }
    #[doc = "Bit 8 - B1608"]
    #[inline(always)]
    pub fn b1608(&mut self) -> B1608_W {
        B1608_W { w: self }
    }
    #[doc = "Bit 9 - B1609"]
    #[inline(always)]
    pub fn b1609(&mut self) -> B1609_W {
        B1609_W { w: self }
    }
    #[doc = "Bit 10 - B1610"]
    #[inline(always)]
    pub fn b1610(&mut self) -> B1610_W {
        B1610_W { w: self }
    }
    #[doc = "Bit 11 - B1611"]
    #[inline(always)]
    pub fn b1611(&mut self) -> B1611_W {
        B1611_W { w: self }
    }
    #[doc = "Bit 12 - B1612"]
    #[inline(always)]
    pub fn b1612(&mut self) -> B1612_W {
        B1612_W { w: self }
    }
    #[doc = "Bit 13 - B1613"]
    #[inline(always)]
    pub fn b1613(&mut self) -> B1613_W {
        B1613_W { w: self }
    }
    #[doc = "Bit 14 - B1614"]
    #[inline(always)]
    pub fn b1614(&mut self) -> B1614_W {
        B1614_W { w: self }
    }
    #[doc = "Bit 15 - B1615"]
    #[inline(always)]
    pub fn b1615(&mut self) -> B1615_W {
        B1615_W { w: self }
    }
    #[doc = "Bit 16 - B1616"]
    #[inline(always)]
    pub fn b1616(&mut self) -> B1616_W {
        B1616_W { w: self }
    }
    #[doc = "Bit 17 - B1617"]
    #[inline(always)]
    pub fn b1617(&mut self) -> B1617_W {
        B1617_W { w: self }
    }
    #[doc = "Bit 18 - B1618"]
    #[inline(always)]
    pub fn b1618(&mut self) -> B1618_W {
        B1618_W { w: self }
    }
    #[doc = "Bit 19 - B1619"]
    #[inline(always)]
    pub fn b1619(&mut self) -> B1619_W {
        B1619_W { w: self }
    }
    #[doc = "Bit 20 - B1620"]
    #[inline(always)]
    pub fn b1620(&mut self) -> B1620_W {
        B1620_W { w: self }
    }
    #[doc = "Bit 21 - B1621"]
    #[inline(always)]
    pub fn b1621(&mut self) -> B1621_W {
        B1621_W { w: self }
    }
    #[doc = "Bit 22 - B1622"]
    #[inline(always)]
    pub fn b1622(&mut self) -> B1622_W {
        B1622_W { w: self }
    }
    #[doc = "Bit 23 - B1623"]
    #[inline(always)]
    pub fn b1623(&mut self) -> B1623_W {
        B1623_W { w: self }
    }
    #[doc = "Bit 24 - B1624"]
    #[inline(always)]
    pub fn b1624(&mut self) -> B1624_W {
        B1624_W { w: self }
    }
    #[doc = "Bit 25 - B1625"]
    #[inline(always)]
    pub fn b1625(&mut self) -> B1625_W {
        B1625_W { w: self }
    }
    #[doc = "Bit 26 - B1626"]
    #[inline(always)]
    pub fn b1626(&mut self) -> B1626_W {
        B1626_W { w: self }
    }
    #[doc = "Bit 27 - B1627"]
    #[inline(always)]
    pub fn b1627(&mut self) -> B1627_W {
        B1627_W { w: self }
    }
    #[doc = "Bit 28 - B1628"]
    #[inline(always)]
    pub fn b1628(&mut self) -> B1628_W {
        B1628_W { w: self }
    }
    #[doc = "Bit 29 - B1629"]
    #[inline(always)]
    pub fn b1629(&mut self) -> B1629_W {
        B1629_W { w: self }
    }
    #[doc = "Bit 30 - B1630"]
    #[inline(always)]
    pub fn b1630(&mut self) -> B1630_W {
        B1630_W { w: self }
    }
    #[doc = "Bit 31 - B1631"]
    #[inline(always)]
    pub fn b1631(&mut self) -> B1631_W {
        B1631_W { w: self }
    }
}
