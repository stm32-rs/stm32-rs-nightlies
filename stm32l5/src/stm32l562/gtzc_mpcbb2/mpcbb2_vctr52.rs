#[doc = "Reader of register MPCBB2_VCTR52"]
pub type R = crate::R<u32, super::MPCBB2_VCTR52>;
#[doc = "Writer for register MPCBB2_VCTR52"]
pub type W = crate::W<u32, super::MPCBB2_VCTR52>;
#[doc = "Register MPCBB2_VCTR52 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR52 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B1664`"]
pub type B1664_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1664`"]
pub struct B1664_W<'a> {
    w: &'a mut W,
}
impl<'a> B1664_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1665`"]
pub type B1665_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1665`"]
pub struct B1665_W<'a> {
    w: &'a mut W,
}
impl<'a> B1665_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1666`"]
pub type B1666_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1666`"]
pub struct B1666_W<'a> {
    w: &'a mut W,
}
impl<'a> B1666_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1667`"]
pub type B1667_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1667`"]
pub struct B1667_W<'a> {
    w: &'a mut W,
}
impl<'a> B1667_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1668`"]
pub type B1668_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1668`"]
pub struct B1668_W<'a> {
    w: &'a mut W,
}
impl<'a> B1668_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1669`"]
pub type B1669_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1669`"]
pub struct B1669_W<'a> {
    w: &'a mut W,
}
impl<'a> B1669_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1670`"]
pub type B1670_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1670`"]
pub struct B1670_W<'a> {
    w: &'a mut W,
}
impl<'a> B1670_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1671`"]
pub type B1671_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1671`"]
pub struct B1671_W<'a> {
    w: &'a mut W,
}
impl<'a> B1671_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1672`"]
pub type B1672_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1672`"]
pub struct B1672_W<'a> {
    w: &'a mut W,
}
impl<'a> B1672_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1673`"]
pub type B1673_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1673`"]
pub struct B1673_W<'a> {
    w: &'a mut W,
}
impl<'a> B1673_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1674`"]
pub type B1674_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1674`"]
pub struct B1674_W<'a> {
    w: &'a mut W,
}
impl<'a> B1674_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1675`"]
pub type B1675_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1675`"]
pub struct B1675_W<'a> {
    w: &'a mut W,
}
impl<'a> B1675_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1676`"]
pub type B1676_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1676`"]
pub struct B1676_W<'a> {
    w: &'a mut W,
}
impl<'a> B1676_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1677`"]
pub type B1677_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1677`"]
pub struct B1677_W<'a> {
    w: &'a mut W,
}
impl<'a> B1677_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1678`"]
pub type B1678_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1678`"]
pub struct B1678_W<'a> {
    w: &'a mut W,
}
impl<'a> B1678_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1679`"]
pub type B1679_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1679`"]
pub struct B1679_W<'a> {
    w: &'a mut W,
}
impl<'a> B1679_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1680`"]
pub type B1680_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1680`"]
pub struct B1680_W<'a> {
    w: &'a mut W,
}
impl<'a> B1680_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1681`"]
pub type B1681_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1681`"]
pub struct B1681_W<'a> {
    w: &'a mut W,
}
impl<'a> B1681_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1682`"]
pub type B1682_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1682`"]
pub struct B1682_W<'a> {
    w: &'a mut W,
}
impl<'a> B1682_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1683`"]
pub type B1683_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1683`"]
pub struct B1683_W<'a> {
    w: &'a mut W,
}
impl<'a> B1683_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1684`"]
pub type B1684_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1684`"]
pub struct B1684_W<'a> {
    w: &'a mut W,
}
impl<'a> B1684_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1685`"]
pub type B1685_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1685`"]
pub struct B1685_W<'a> {
    w: &'a mut W,
}
impl<'a> B1685_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1686`"]
pub type B1686_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1686`"]
pub struct B1686_W<'a> {
    w: &'a mut W,
}
impl<'a> B1686_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1687`"]
pub type B1687_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1687`"]
pub struct B1687_W<'a> {
    w: &'a mut W,
}
impl<'a> B1687_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1688`"]
pub type B1688_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1688`"]
pub struct B1688_W<'a> {
    w: &'a mut W,
}
impl<'a> B1688_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1689`"]
pub type B1689_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1689`"]
pub struct B1689_W<'a> {
    w: &'a mut W,
}
impl<'a> B1689_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1690`"]
pub type B1690_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1690`"]
pub struct B1690_W<'a> {
    w: &'a mut W,
}
impl<'a> B1690_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1691`"]
pub type B1691_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1691`"]
pub struct B1691_W<'a> {
    w: &'a mut W,
}
impl<'a> B1691_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1692`"]
pub type B1692_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1692`"]
pub struct B1692_W<'a> {
    w: &'a mut W,
}
impl<'a> B1692_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1693`"]
pub type B1693_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1693`"]
pub struct B1693_W<'a> {
    w: &'a mut W,
}
impl<'a> B1693_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1694`"]
pub type B1694_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1694`"]
pub struct B1694_W<'a> {
    w: &'a mut W,
}
impl<'a> B1694_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1695`"]
pub type B1695_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1695`"]
pub struct B1695_W<'a> {
    w: &'a mut W,
}
impl<'a> B1695_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1664"]
    #[inline(always)]
    pub fn b1664(&self) -> B1664_R {
        B1664_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1665"]
    #[inline(always)]
    pub fn b1665(&self) -> B1665_R {
        B1665_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1666"]
    #[inline(always)]
    pub fn b1666(&self) -> B1666_R {
        B1666_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1667"]
    #[inline(always)]
    pub fn b1667(&self) -> B1667_R {
        B1667_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1668"]
    #[inline(always)]
    pub fn b1668(&self) -> B1668_R {
        B1668_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1669"]
    #[inline(always)]
    pub fn b1669(&self) -> B1669_R {
        B1669_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1670"]
    #[inline(always)]
    pub fn b1670(&self) -> B1670_R {
        B1670_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1671"]
    #[inline(always)]
    pub fn b1671(&self) -> B1671_R {
        B1671_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1672"]
    #[inline(always)]
    pub fn b1672(&self) -> B1672_R {
        B1672_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1673"]
    #[inline(always)]
    pub fn b1673(&self) -> B1673_R {
        B1673_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1674"]
    #[inline(always)]
    pub fn b1674(&self) -> B1674_R {
        B1674_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1675"]
    #[inline(always)]
    pub fn b1675(&self) -> B1675_R {
        B1675_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1676"]
    #[inline(always)]
    pub fn b1676(&self) -> B1676_R {
        B1676_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1677"]
    #[inline(always)]
    pub fn b1677(&self) -> B1677_R {
        B1677_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1678"]
    #[inline(always)]
    pub fn b1678(&self) -> B1678_R {
        B1678_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1679"]
    #[inline(always)]
    pub fn b1679(&self) -> B1679_R {
        B1679_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1680"]
    #[inline(always)]
    pub fn b1680(&self) -> B1680_R {
        B1680_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1681"]
    #[inline(always)]
    pub fn b1681(&self) -> B1681_R {
        B1681_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1682"]
    #[inline(always)]
    pub fn b1682(&self) -> B1682_R {
        B1682_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1683"]
    #[inline(always)]
    pub fn b1683(&self) -> B1683_R {
        B1683_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1684"]
    #[inline(always)]
    pub fn b1684(&self) -> B1684_R {
        B1684_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1685"]
    #[inline(always)]
    pub fn b1685(&self) -> B1685_R {
        B1685_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1686"]
    #[inline(always)]
    pub fn b1686(&self) -> B1686_R {
        B1686_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1687"]
    #[inline(always)]
    pub fn b1687(&self) -> B1687_R {
        B1687_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1688"]
    #[inline(always)]
    pub fn b1688(&self) -> B1688_R {
        B1688_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1689"]
    #[inline(always)]
    pub fn b1689(&self) -> B1689_R {
        B1689_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1690"]
    #[inline(always)]
    pub fn b1690(&self) -> B1690_R {
        B1690_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1691"]
    #[inline(always)]
    pub fn b1691(&self) -> B1691_R {
        B1691_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1692"]
    #[inline(always)]
    pub fn b1692(&self) -> B1692_R {
        B1692_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1693"]
    #[inline(always)]
    pub fn b1693(&self) -> B1693_R {
        B1693_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1694"]
    #[inline(always)]
    pub fn b1694(&self) -> B1694_R {
        B1694_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1695"]
    #[inline(always)]
    pub fn b1695(&self) -> B1695_R {
        B1695_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1664"]
    #[inline(always)]
    pub fn b1664(&mut self) -> B1664_W {
        B1664_W { w: self }
    }
    #[doc = "Bit 1 - B1665"]
    #[inline(always)]
    pub fn b1665(&mut self) -> B1665_W {
        B1665_W { w: self }
    }
    #[doc = "Bit 2 - B1666"]
    #[inline(always)]
    pub fn b1666(&mut self) -> B1666_W {
        B1666_W { w: self }
    }
    #[doc = "Bit 3 - B1667"]
    #[inline(always)]
    pub fn b1667(&mut self) -> B1667_W {
        B1667_W { w: self }
    }
    #[doc = "Bit 4 - B1668"]
    #[inline(always)]
    pub fn b1668(&mut self) -> B1668_W {
        B1668_W { w: self }
    }
    #[doc = "Bit 5 - B1669"]
    #[inline(always)]
    pub fn b1669(&mut self) -> B1669_W {
        B1669_W { w: self }
    }
    #[doc = "Bit 6 - B1670"]
    #[inline(always)]
    pub fn b1670(&mut self) -> B1670_W {
        B1670_W { w: self }
    }
    #[doc = "Bit 7 - B1671"]
    #[inline(always)]
    pub fn b1671(&mut self) -> B1671_W {
        B1671_W { w: self }
    }
    #[doc = "Bit 8 - B1672"]
    #[inline(always)]
    pub fn b1672(&mut self) -> B1672_W {
        B1672_W { w: self }
    }
    #[doc = "Bit 9 - B1673"]
    #[inline(always)]
    pub fn b1673(&mut self) -> B1673_W {
        B1673_W { w: self }
    }
    #[doc = "Bit 10 - B1674"]
    #[inline(always)]
    pub fn b1674(&mut self) -> B1674_W {
        B1674_W { w: self }
    }
    #[doc = "Bit 11 - B1675"]
    #[inline(always)]
    pub fn b1675(&mut self) -> B1675_W {
        B1675_W { w: self }
    }
    #[doc = "Bit 12 - B1676"]
    #[inline(always)]
    pub fn b1676(&mut self) -> B1676_W {
        B1676_W { w: self }
    }
    #[doc = "Bit 13 - B1677"]
    #[inline(always)]
    pub fn b1677(&mut self) -> B1677_W {
        B1677_W { w: self }
    }
    #[doc = "Bit 14 - B1678"]
    #[inline(always)]
    pub fn b1678(&mut self) -> B1678_W {
        B1678_W { w: self }
    }
    #[doc = "Bit 15 - B1679"]
    #[inline(always)]
    pub fn b1679(&mut self) -> B1679_W {
        B1679_W { w: self }
    }
    #[doc = "Bit 16 - B1680"]
    #[inline(always)]
    pub fn b1680(&mut self) -> B1680_W {
        B1680_W { w: self }
    }
    #[doc = "Bit 17 - B1681"]
    #[inline(always)]
    pub fn b1681(&mut self) -> B1681_W {
        B1681_W { w: self }
    }
    #[doc = "Bit 18 - B1682"]
    #[inline(always)]
    pub fn b1682(&mut self) -> B1682_W {
        B1682_W { w: self }
    }
    #[doc = "Bit 19 - B1683"]
    #[inline(always)]
    pub fn b1683(&mut self) -> B1683_W {
        B1683_W { w: self }
    }
    #[doc = "Bit 20 - B1684"]
    #[inline(always)]
    pub fn b1684(&mut self) -> B1684_W {
        B1684_W { w: self }
    }
    #[doc = "Bit 21 - B1685"]
    #[inline(always)]
    pub fn b1685(&mut self) -> B1685_W {
        B1685_W { w: self }
    }
    #[doc = "Bit 22 - B1686"]
    #[inline(always)]
    pub fn b1686(&mut self) -> B1686_W {
        B1686_W { w: self }
    }
    #[doc = "Bit 23 - B1687"]
    #[inline(always)]
    pub fn b1687(&mut self) -> B1687_W {
        B1687_W { w: self }
    }
    #[doc = "Bit 24 - B1688"]
    #[inline(always)]
    pub fn b1688(&mut self) -> B1688_W {
        B1688_W { w: self }
    }
    #[doc = "Bit 25 - B1689"]
    #[inline(always)]
    pub fn b1689(&mut self) -> B1689_W {
        B1689_W { w: self }
    }
    #[doc = "Bit 26 - B1690"]
    #[inline(always)]
    pub fn b1690(&mut self) -> B1690_W {
        B1690_W { w: self }
    }
    #[doc = "Bit 27 - B1691"]
    #[inline(always)]
    pub fn b1691(&mut self) -> B1691_W {
        B1691_W { w: self }
    }
    #[doc = "Bit 28 - B1692"]
    #[inline(always)]
    pub fn b1692(&mut self) -> B1692_W {
        B1692_W { w: self }
    }
    #[doc = "Bit 29 - B1693"]
    #[inline(always)]
    pub fn b1693(&mut self) -> B1693_W {
        B1693_W { w: self }
    }
    #[doc = "Bit 30 - B1694"]
    #[inline(always)]
    pub fn b1694(&mut self) -> B1694_W {
        B1694_W { w: self }
    }
    #[doc = "Bit 31 - B1695"]
    #[inline(always)]
    pub fn b1695(&mut self) -> B1695_W {
        B1695_W { w: self }
    }
}
