#[doc = "Reader of register MPCBB2_VCTR53"]
pub type R = crate::R<u32, super::MPCBB2_VCTR53>;
#[doc = "Writer for register MPCBB2_VCTR53"]
pub type W = crate::W<u32, super::MPCBB2_VCTR53>;
#[doc = "Register MPCBB2_VCTR53 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR53 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1696`"]
pub type B1696_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1696`"]
pub struct B1696_W<'a> {
    w: &'a mut W,
}
impl<'a> B1696_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1697`"]
pub type B1697_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1697`"]
pub struct B1697_W<'a> {
    w: &'a mut W,
}
impl<'a> B1697_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1698`"]
pub type B1698_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1698`"]
pub struct B1698_W<'a> {
    w: &'a mut W,
}
impl<'a> B1698_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1699`"]
pub type B1699_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1699`"]
pub struct B1699_W<'a> {
    w: &'a mut W,
}
impl<'a> B1699_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1700`"]
pub type B1700_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1700`"]
pub struct B1700_W<'a> {
    w: &'a mut W,
}
impl<'a> B1700_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1701`"]
pub type B1701_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1701`"]
pub struct B1701_W<'a> {
    w: &'a mut W,
}
impl<'a> B1701_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1702`"]
pub type B1702_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1702`"]
pub struct B1702_W<'a> {
    w: &'a mut W,
}
impl<'a> B1702_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1703`"]
pub type B1703_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1703`"]
pub struct B1703_W<'a> {
    w: &'a mut W,
}
impl<'a> B1703_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1704`"]
pub type B1704_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1704`"]
pub struct B1704_W<'a> {
    w: &'a mut W,
}
impl<'a> B1704_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1705`"]
pub type B1705_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1705`"]
pub struct B1705_W<'a> {
    w: &'a mut W,
}
impl<'a> B1705_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1706`"]
pub type B1706_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1706`"]
pub struct B1706_W<'a> {
    w: &'a mut W,
}
impl<'a> B1706_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1707`"]
pub type B1707_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1707`"]
pub struct B1707_W<'a> {
    w: &'a mut W,
}
impl<'a> B1707_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1708`"]
pub type B1708_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1708`"]
pub struct B1708_W<'a> {
    w: &'a mut W,
}
impl<'a> B1708_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1709`"]
pub type B1709_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1709`"]
pub struct B1709_W<'a> {
    w: &'a mut W,
}
impl<'a> B1709_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1710`"]
pub type B1710_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1710`"]
pub struct B1710_W<'a> {
    w: &'a mut W,
}
impl<'a> B1710_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1711`"]
pub type B1711_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1711`"]
pub struct B1711_W<'a> {
    w: &'a mut W,
}
impl<'a> B1711_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1712`"]
pub type B1712_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1712`"]
pub struct B1712_W<'a> {
    w: &'a mut W,
}
impl<'a> B1712_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1713`"]
pub type B1713_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1713`"]
pub struct B1713_W<'a> {
    w: &'a mut W,
}
impl<'a> B1713_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1714`"]
pub type B1714_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1714`"]
pub struct B1714_W<'a> {
    w: &'a mut W,
}
impl<'a> B1714_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1715`"]
pub type B1715_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1715`"]
pub struct B1715_W<'a> {
    w: &'a mut W,
}
impl<'a> B1715_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1716`"]
pub type B1716_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1716`"]
pub struct B1716_W<'a> {
    w: &'a mut W,
}
impl<'a> B1716_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1717`"]
pub type B1717_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1717`"]
pub struct B1717_W<'a> {
    w: &'a mut W,
}
impl<'a> B1717_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1718`"]
pub type B1718_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1718`"]
pub struct B1718_W<'a> {
    w: &'a mut W,
}
impl<'a> B1718_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1719`"]
pub type B1719_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1719`"]
pub struct B1719_W<'a> {
    w: &'a mut W,
}
impl<'a> B1719_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1720`"]
pub type B1720_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1720`"]
pub struct B1720_W<'a> {
    w: &'a mut W,
}
impl<'a> B1720_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1721`"]
pub type B1721_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1721`"]
pub struct B1721_W<'a> {
    w: &'a mut W,
}
impl<'a> B1721_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1722`"]
pub type B1722_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1722`"]
pub struct B1722_W<'a> {
    w: &'a mut W,
}
impl<'a> B1722_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1723`"]
pub type B1723_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1723`"]
pub struct B1723_W<'a> {
    w: &'a mut W,
}
impl<'a> B1723_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1724`"]
pub type B1724_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1724`"]
pub struct B1724_W<'a> {
    w: &'a mut W,
}
impl<'a> B1724_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1725`"]
pub type B1725_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1725`"]
pub struct B1725_W<'a> {
    w: &'a mut W,
}
impl<'a> B1725_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1726`"]
pub type B1726_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1726`"]
pub struct B1726_W<'a> {
    w: &'a mut W,
}
impl<'a> B1726_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1727`"]
pub type B1727_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1727`"]
pub struct B1727_W<'a> {
    w: &'a mut W,
}
impl<'a> B1727_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1696"]
    #[inline(always)]
    pub fn b1696(&self) -> B1696_R {
        B1696_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1697"]
    #[inline(always)]
    pub fn b1697(&self) -> B1697_R {
        B1697_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1698"]
    #[inline(always)]
    pub fn b1698(&self) -> B1698_R {
        B1698_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1699"]
    #[inline(always)]
    pub fn b1699(&self) -> B1699_R {
        B1699_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1700"]
    #[inline(always)]
    pub fn b1700(&self) -> B1700_R {
        B1700_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1701"]
    #[inline(always)]
    pub fn b1701(&self) -> B1701_R {
        B1701_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1702"]
    #[inline(always)]
    pub fn b1702(&self) -> B1702_R {
        B1702_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1703"]
    #[inline(always)]
    pub fn b1703(&self) -> B1703_R {
        B1703_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1704"]
    #[inline(always)]
    pub fn b1704(&self) -> B1704_R {
        B1704_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1705"]
    #[inline(always)]
    pub fn b1705(&self) -> B1705_R {
        B1705_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1706"]
    #[inline(always)]
    pub fn b1706(&self) -> B1706_R {
        B1706_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1707"]
    #[inline(always)]
    pub fn b1707(&self) -> B1707_R {
        B1707_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1708"]
    #[inline(always)]
    pub fn b1708(&self) -> B1708_R {
        B1708_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1709"]
    #[inline(always)]
    pub fn b1709(&self) -> B1709_R {
        B1709_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1710"]
    #[inline(always)]
    pub fn b1710(&self) -> B1710_R {
        B1710_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1711"]
    #[inline(always)]
    pub fn b1711(&self) -> B1711_R {
        B1711_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1712"]
    #[inline(always)]
    pub fn b1712(&self) -> B1712_R {
        B1712_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1713"]
    #[inline(always)]
    pub fn b1713(&self) -> B1713_R {
        B1713_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1714"]
    #[inline(always)]
    pub fn b1714(&self) -> B1714_R {
        B1714_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1715"]
    #[inline(always)]
    pub fn b1715(&self) -> B1715_R {
        B1715_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1716"]
    #[inline(always)]
    pub fn b1716(&self) -> B1716_R {
        B1716_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1717"]
    #[inline(always)]
    pub fn b1717(&self) -> B1717_R {
        B1717_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1718"]
    #[inline(always)]
    pub fn b1718(&self) -> B1718_R {
        B1718_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1719"]
    #[inline(always)]
    pub fn b1719(&self) -> B1719_R {
        B1719_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1720"]
    #[inline(always)]
    pub fn b1720(&self) -> B1720_R {
        B1720_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1721"]
    #[inline(always)]
    pub fn b1721(&self) -> B1721_R {
        B1721_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1722"]
    #[inline(always)]
    pub fn b1722(&self) -> B1722_R {
        B1722_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1723"]
    #[inline(always)]
    pub fn b1723(&self) -> B1723_R {
        B1723_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1724"]
    #[inline(always)]
    pub fn b1724(&self) -> B1724_R {
        B1724_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1725"]
    #[inline(always)]
    pub fn b1725(&self) -> B1725_R {
        B1725_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1726"]
    #[inline(always)]
    pub fn b1726(&self) -> B1726_R {
        B1726_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1727"]
    #[inline(always)]
    pub fn b1727(&self) -> B1727_R {
        B1727_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1696"]
    #[inline(always)]
    pub fn b1696(&mut self) -> B1696_W {
        B1696_W { w: self }
    }
    #[doc = "Bit 1 - B1697"]
    #[inline(always)]
    pub fn b1697(&mut self) -> B1697_W {
        B1697_W { w: self }
    }
    #[doc = "Bit 2 - B1698"]
    #[inline(always)]
    pub fn b1698(&mut self) -> B1698_W {
        B1698_W { w: self }
    }
    #[doc = "Bit 3 - B1699"]
    #[inline(always)]
    pub fn b1699(&mut self) -> B1699_W {
        B1699_W { w: self }
    }
    #[doc = "Bit 4 - B1700"]
    #[inline(always)]
    pub fn b1700(&mut self) -> B1700_W {
        B1700_W { w: self }
    }
    #[doc = "Bit 5 - B1701"]
    #[inline(always)]
    pub fn b1701(&mut self) -> B1701_W {
        B1701_W { w: self }
    }
    #[doc = "Bit 6 - B1702"]
    #[inline(always)]
    pub fn b1702(&mut self) -> B1702_W {
        B1702_W { w: self }
    }
    #[doc = "Bit 7 - B1703"]
    #[inline(always)]
    pub fn b1703(&mut self) -> B1703_W {
        B1703_W { w: self }
    }
    #[doc = "Bit 8 - B1704"]
    #[inline(always)]
    pub fn b1704(&mut self) -> B1704_W {
        B1704_W { w: self }
    }
    #[doc = "Bit 9 - B1705"]
    #[inline(always)]
    pub fn b1705(&mut self) -> B1705_W {
        B1705_W { w: self }
    }
    #[doc = "Bit 10 - B1706"]
    #[inline(always)]
    pub fn b1706(&mut self) -> B1706_W {
        B1706_W { w: self }
    }
    #[doc = "Bit 11 - B1707"]
    #[inline(always)]
    pub fn b1707(&mut self) -> B1707_W {
        B1707_W { w: self }
    }
    #[doc = "Bit 12 - B1708"]
    #[inline(always)]
    pub fn b1708(&mut self) -> B1708_W {
        B1708_W { w: self }
    }
    #[doc = "Bit 13 - B1709"]
    #[inline(always)]
    pub fn b1709(&mut self) -> B1709_W {
        B1709_W { w: self }
    }
    #[doc = "Bit 14 - B1710"]
    #[inline(always)]
    pub fn b1710(&mut self) -> B1710_W {
        B1710_W { w: self }
    }
    #[doc = "Bit 15 - B1711"]
    #[inline(always)]
    pub fn b1711(&mut self) -> B1711_W {
        B1711_W { w: self }
    }
    #[doc = "Bit 16 - B1712"]
    #[inline(always)]
    pub fn b1712(&mut self) -> B1712_W {
        B1712_W { w: self }
    }
    #[doc = "Bit 17 - B1713"]
    #[inline(always)]
    pub fn b1713(&mut self) -> B1713_W {
        B1713_W { w: self }
    }
    #[doc = "Bit 18 - B1714"]
    #[inline(always)]
    pub fn b1714(&mut self) -> B1714_W {
        B1714_W { w: self }
    }
    #[doc = "Bit 19 - B1715"]
    #[inline(always)]
    pub fn b1715(&mut self) -> B1715_W {
        B1715_W { w: self }
    }
    #[doc = "Bit 20 - B1716"]
    #[inline(always)]
    pub fn b1716(&mut self) -> B1716_W {
        B1716_W { w: self }
    }
    #[doc = "Bit 21 - B1717"]
    #[inline(always)]
    pub fn b1717(&mut self) -> B1717_W {
        B1717_W { w: self }
    }
    #[doc = "Bit 22 - B1718"]
    #[inline(always)]
    pub fn b1718(&mut self) -> B1718_W {
        B1718_W { w: self }
    }
    #[doc = "Bit 23 - B1719"]
    #[inline(always)]
    pub fn b1719(&mut self) -> B1719_W {
        B1719_W { w: self }
    }
    #[doc = "Bit 24 - B1720"]
    #[inline(always)]
    pub fn b1720(&mut self) -> B1720_W {
        B1720_W { w: self }
    }
    #[doc = "Bit 25 - B1721"]
    #[inline(always)]
    pub fn b1721(&mut self) -> B1721_W {
        B1721_W { w: self }
    }
    #[doc = "Bit 26 - B1722"]
    #[inline(always)]
    pub fn b1722(&mut self) -> B1722_W {
        B1722_W { w: self }
    }
    #[doc = "Bit 27 - B1723"]
    #[inline(always)]
    pub fn b1723(&mut self) -> B1723_W {
        B1723_W { w: self }
    }
    #[doc = "Bit 28 - B1724"]
    #[inline(always)]
    pub fn b1724(&mut self) -> B1724_W {
        B1724_W { w: self }
    }
    #[doc = "Bit 29 - B1725"]
    #[inline(always)]
    pub fn b1725(&mut self) -> B1725_W {
        B1725_W { w: self }
    }
    #[doc = "Bit 30 - B1726"]
    #[inline(always)]
    pub fn b1726(&mut self) -> B1726_W {
        B1726_W { w: self }
    }
    #[doc = "Bit 31 - B1727"]
    #[inline(always)]
    pub fn b1727(&mut self) -> B1727_W {
        B1727_W { w: self }
    }
}
