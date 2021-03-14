#[doc = "Reader of register MPCBB1_VCTR51"]
pub type R = crate::R<u32, super::MPCBB1_VCTR51>;
#[doc = "Writer for register MPCBB1_VCTR51"]
pub type W = crate::W<u32, super::MPCBB1_VCTR51>;
#[doc = "Register MPCBB1_VCTR51 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR51 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1632`"]
pub type B1632_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1632`"]
pub struct B1632_W<'a> {
    w: &'a mut W,
}
impl<'a> B1632_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1633`"]
pub type B1633_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1633`"]
pub struct B1633_W<'a> {
    w: &'a mut W,
}
impl<'a> B1633_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1634`"]
pub type B1634_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1634`"]
pub struct B1634_W<'a> {
    w: &'a mut W,
}
impl<'a> B1634_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1635`"]
pub type B1635_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1635`"]
pub struct B1635_W<'a> {
    w: &'a mut W,
}
impl<'a> B1635_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1636`"]
pub type B1636_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1636`"]
pub struct B1636_W<'a> {
    w: &'a mut W,
}
impl<'a> B1636_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1637`"]
pub type B1637_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1637`"]
pub struct B1637_W<'a> {
    w: &'a mut W,
}
impl<'a> B1637_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1638`"]
pub type B1638_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1638`"]
pub struct B1638_W<'a> {
    w: &'a mut W,
}
impl<'a> B1638_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1639`"]
pub type B1639_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1639`"]
pub struct B1639_W<'a> {
    w: &'a mut W,
}
impl<'a> B1639_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1640`"]
pub type B1640_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1640`"]
pub struct B1640_W<'a> {
    w: &'a mut W,
}
impl<'a> B1640_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1641`"]
pub type B1641_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1641`"]
pub struct B1641_W<'a> {
    w: &'a mut W,
}
impl<'a> B1641_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1642`"]
pub type B1642_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1642`"]
pub struct B1642_W<'a> {
    w: &'a mut W,
}
impl<'a> B1642_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1643`"]
pub type B1643_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1643`"]
pub struct B1643_W<'a> {
    w: &'a mut W,
}
impl<'a> B1643_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1644`"]
pub type B1644_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1644`"]
pub struct B1644_W<'a> {
    w: &'a mut W,
}
impl<'a> B1644_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1645`"]
pub type B1645_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1645`"]
pub struct B1645_W<'a> {
    w: &'a mut W,
}
impl<'a> B1645_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1646`"]
pub type B1646_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1646`"]
pub struct B1646_W<'a> {
    w: &'a mut W,
}
impl<'a> B1646_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1647`"]
pub type B1647_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1647`"]
pub struct B1647_W<'a> {
    w: &'a mut W,
}
impl<'a> B1647_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1648`"]
pub type B1648_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1648`"]
pub struct B1648_W<'a> {
    w: &'a mut W,
}
impl<'a> B1648_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1649`"]
pub type B1649_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1649`"]
pub struct B1649_W<'a> {
    w: &'a mut W,
}
impl<'a> B1649_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1650`"]
pub type B1650_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1650`"]
pub struct B1650_W<'a> {
    w: &'a mut W,
}
impl<'a> B1650_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1651`"]
pub type B1651_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1651`"]
pub struct B1651_W<'a> {
    w: &'a mut W,
}
impl<'a> B1651_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1652`"]
pub type B1652_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1652`"]
pub struct B1652_W<'a> {
    w: &'a mut W,
}
impl<'a> B1652_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1653`"]
pub type B1653_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1653`"]
pub struct B1653_W<'a> {
    w: &'a mut W,
}
impl<'a> B1653_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1654`"]
pub type B1654_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1654`"]
pub struct B1654_W<'a> {
    w: &'a mut W,
}
impl<'a> B1654_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1655`"]
pub type B1655_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1655`"]
pub struct B1655_W<'a> {
    w: &'a mut W,
}
impl<'a> B1655_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1656`"]
pub type B1656_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1656`"]
pub struct B1656_W<'a> {
    w: &'a mut W,
}
impl<'a> B1656_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1657`"]
pub type B1657_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1657`"]
pub struct B1657_W<'a> {
    w: &'a mut W,
}
impl<'a> B1657_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1658`"]
pub type B1658_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1658`"]
pub struct B1658_W<'a> {
    w: &'a mut W,
}
impl<'a> B1658_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1659`"]
pub type B1659_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1659`"]
pub struct B1659_W<'a> {
    w: &'a mut W,
}
impl<'a> B1659_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1660`"]
pub type B1660_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1660`"]
pub struct B1660_W<'a> {
    w: &'a mut W,
}
impl<'a> B1660_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1661`"]
pub type B1661_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1661`"]
pub struct B1661_W<'a> {
    w: &'a mut W,
}
impl<'a> B1661_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1662`"]
pub type B1662_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1662`"]
pub struct B1662_W<'a> {
    w: &'a mut W,
}
impl<'a> B1662_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B1663`"]
pub type B1663_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B1663`"]
pub struct B1663_W<'a> {
    w: &'a mut W,
}
impl<'a> B1663_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1632"]
    #[inline(always)]
    pub fn b1632(&self) -> B1632_R {
        B1632_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1633"]
    #[inline(always)]
    pub fn b1633(&self) -> B1633_R {
        B1633_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1634"]
    #[inline(always)]
    pub fn b1634(&self) -> B1634_R {
        B1634_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1635"]
    #[inline(always)]
    pub fn b1635(&self) -> B1635_R {
        B1635_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1636"]
    #[inline(always)]
    pub fn b1636(&self) -> B1636_R {
        B1636_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1637"]
    #[inline(always)]
    pub fn b1637(&self) -> B1637_R {
        B1637_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1638"]
    #[inline(always)]
    pub fn b1638(&self) -> B1638_R {
        B1638_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1639"]
    #[inline(always)]
    pub fn b1639(&self) -> B1639_R {
        B1639_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1640"]
    #[inline(always)]
    pub fn b1640(&self) -> B1640_R {
        B1640_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1641"]
    #[inline(always)]
    pub fn b1641(&self) -> B1641_R {
        B1641_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1642"]
    #[inline(always)]
    pub fn b1642(&self) -> B1642_R {
        B1642_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1643"]
    #[inline(always)]
    pub fn b1643(&self) -> B1643_R {
        B1643_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1644"]
    #[inline(always)]
    pub fn b1644(&self) -> B1644_R {
        B1644_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1645"]
    #[inline(always)]
    pub fn b1645(&self) -> B1645_R {
        B1645_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1646"]
    #[inline(always)]
    pub fn b1646(&self) -> B1646_R {
        B1646_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1647"]
    #[inline(always)]
    pub fn b1647(&self) -> B1647_R {
        B1647_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1648"]
    #[inline(always)]
    pub fn b1648(&self) -> B1648_R {
        B1648_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1649"]
    #[inline(always)]
    pub fn b1649(&self) -> B1649_R {
        B1649_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1650"]
    #[inline(always)]
    pub fn b1650(&self) -> B1650_R {
        B1650_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1651"]
    #[inline(always)]
    pub fn b1651(&self) -> B1651_R {
        B1651_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1652"]
    #[inline(always)]
    pub fn b1652(&self) -> B1652_R {
        B1652_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1653"]
    #[inline(always)]
    pub fn b1653(&self) -> B1653_R {
        B1653_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1654"]
    #[inline(always)]
    pub fn b1654(&self) -> B1654_R {
        B1654_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1655"]
    #[inline(always)]
    pub fn b1655(&self) -> B1655_R {
        B1655_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1656"]
    #[inline(always)]
    pub fn b1656(&self) -> B1656_R {
        B1656_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1657"]
    #[inline(always)]
    pub fn b1657(&self) -> B1657_R {
        B1657_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1658"]
    #[inline(always)]
    pub fn b1658(&self) -> B1658_R {
        B1658_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1659"]
    #[inline(always)]
    pub fn b1659(&self) -> B1659_R {
        B1659_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1660"]
    #[inline(always)]
    pub fn b1660(&self) -> B1660_R {
        B1660_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1661"]
    #[inline(always)]
    pub fn b1661(&self) -> B1661_R {
        B1661_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1662"]
    #[inline(always)]
    pub fn b1662(&self) -> B1662_R {
        B1662_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1663"]
    #[inline(always)]
    pub fn b1663(&self) -> B1663_R {
        B1663_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1632"]
    #[inline(always)]
    pub fn b1632(&mut self) -> B1632_W {
        B1632_W { w: self }
    }
    #[doc = "Bit 1 - B1633"]
    #[inline(always)]
    pub fn b1633(&mut self) -> B1633_W {
        B1633_W { w: self }
    }
    #[doc = "Bit 2 - B1634"]
    #[inline(always)]
    pub fn b1634(&mut self) -> B1634_W {
        B1634_W { w: self }
    }
    #[doc = "Bit 3 - B1635"]
    #[inline(always)]
    pub fn b1635(&mut self) -> B1635_W {
        B1635_W { w: self }
    }
    #[doc = "Bit 4 - B1636"]
    #[inline(always)]
    pub fn b1636(&mut self) -> B1636_W {
        B1636_W { w: self }
    }
    #[doc = "Bit 5 - B1637"]
    #[inline(always)]
    pub fn b1637(&mut self) -> B1637_W {
        B1637_W { w: self }
    }
    #[doc = "Bit 6 - B1638"]
    #[inline(always)]
    pub fn b1638(&mut self) -> B1638_W {
        B1638_W { w: self }
    }
    #[doc = "Bit 7 - B1639"]
    #[inline(always)]
    pub fn b1639(&mut self) -> B1639_W {
        B1639_W { w: self }
    }
    #[doc = "Bit 8 - B1640"]
    #[inline(always)]
    pub fn b1640(&mut self) -> B1640_W {
        B1640_W { w: self }
    }
    #[doc = "Bit 9 - B1641"]
    #[inline(always)]
    pub fn b1641(&mut self) -> B1641_W {
        B1641_W { w: self }
    }
    #[doc = "Bit 10 - B1642"]
    #[inline(always)]
    pub fn b1642(&mut self) -> B1642_W {
        B1642_W { w: self }
    }
    #[doc = "Bit 11 - B1643"]
    #[inline(always)]
    pub fn b1643(&mut self) -> B1643_W {
        B1643_W { w: self }
    }
    #[doc = "Bit 12 - B1644"]
    #[inline(always)]
    pub fn b1644(&mut self) -> B1644_W {
        B1644_W { w: self }
    }
    #[doc = "Bit 13 - B1645"]
    #[inline(always)]
    pub fn b1645(&mut self) -> B1645_W {
        B1645_W { w: self }
    }
    #[doc = "Bit 14 - B1646"]
    #[inline(always)]
    pub fn b1646(&mut self) -> B1646_W {
        B1646_W { w: self }
    }
    #[doc = "Bit 15 - B1647"]
    #[inline(always)]
    pub fn b1647(&mut self) -> B1647_W {
        B1647_W { w: self }
    }
    #[doc = "Bit 16 - B1648"]
    #[inline(always)]
    pub fn b1648(&mut self) -> B1648_W {
        B1648_W { w: self }
    }
    #[doc = "Bit 17 - B1649"]
    #[inline(always)]
    pub fn b1649(&mut self) -> B1649_W {
        B1649_W { w: self }
    }
    #[doc = "Bit 18 - B1650"]
    #[inline(always)]
    pub fn b1650(&mut self) -> B1650_W {
        B1650_W { w: self }
    }
    #[doc = "Bit 19 - B1651"]
    #[inline(always)]
    pub fn b1651(&mut self) -> B1651_W {
        B1651_W { w: self }
    }
    #[doc = "Bit 20 - B1652"]
    #[inline(always)]
    pub fn b1652(&mut self) -> B1652_W {
        B1652_W { w: self }
    }
    #[doc = "Bit 21 - B1653"]
    #[inline(always)]
    pub fn b1653(&mut self) -> B1653_W {
        B1653_W { w: self }
    }
    #[doc = "Bit 22 - B1654"]
    #[inline(always)]
    pub fn b1654(&mut self) -> B1654_W {
        B1654_W { w: self }
    }
    #[doc = "Bit 23 - B1655"]
    #[inline(always)]
    pub fn b1655(&mut self) -> B1655_W {
        B1655_W { w: self }
    }
    #[doc = "Bit 24 - B1656"]
    #[inline(always)]
    pub fn b1656(&mut self) -> B1656_W {
        B1656_W { w: self }
    }
    #[doc = "Bit 25 - B1657"]
    #[inline(always)]
    pub fn b1657(&mut self) -> B1657_W {
        B1657_W { w: self }
    }
    #[doc = "Bit 26 - B1658"]
    #[inline(always)]
    pub fn b1658(&mut self) -> B1658_W {
        B1658_W { w: self }
    }
    #[doc = "Bit 27 - B1659"]
    #[inline(always)]
    pub fn b1659(&mut self) -> B1659_W {
        B1659_W { w: self }
    }
    #[doc = "Bit 28 - B1660"]
    #[inline(always)]
    pub fn b1660(&mut self) -> B1660_W {
        B1660_W { w: self }
    }
    #[doc = "Bit 29 - B1661"]
    #[inline(always)]
    pub fn b1661(&mut self) -> B1661_W {
        B1661_W { w: self }
    }
    #[doc = "Bit 30 - B1662"]
    #[inline(always)]
    pub fn b1662(&mut self) -> B1662_W {
        B1662_W { w: self }
    }
    #[doc = "Bit 31 - B1663"]
    #[inline(always)]
    pub fn b1663(&mut self) -> B1663_W {
        B1663_W { w: self }
    }
}
