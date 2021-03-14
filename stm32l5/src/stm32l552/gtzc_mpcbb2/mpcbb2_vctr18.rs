#[doc = "Reader of register MPCBB2_VCTR18"]
pub type R = crate::R<u32, super::MPCBB2_VCTR18>;
#[doc = "Writer for register MPCBB2_VCTR18"]
pub type W = crate::W<u32, super::MPCBB2_VCTR18>;
#[doc = "Register MPCBB2_VCTR18 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB2_VCTR18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B576`"]
pub type B576_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B576`"]
pub struct B576_W<'a> {
    w: &'a mut W,
}
impl<'a> B576_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B577`"]
pub type B577_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B577`"]
pub struct B577_W<'a> {
    w: &'a mut W,
}
impl<'a> B577_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B578`"]
pub type B578_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B578`"]
pub struct B578_W<'a> {
    w: &'a mut W,
}
impl<'a> B578_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B579`"]
pub type B579_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B579`"]
pub struct B579_W<'a> {
    w: &'a mut W,
}
impl<'a> B579_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B580`"]
pub type B580_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B580`"]
pub struct B580_W<'a> {
    w: &'a mut W,
}
impl<'a> B580_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B581`"]
pub type B581_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B581`"]
pub struct B581_W<'a> {
    w: &'a mut W,
}
impl<'a> B581_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B582`"]
pub type B582_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B582`"]
pub struct B582_W<'a> {
    w: &'a mut W,
}
impl<'a> B582_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B583`"]
pub type B583_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B583`"]
pub struct B583_W<'a> {
    w: &'a mut W,
}
impl<'a> B583_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B584`"]
pub type B584_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B584`"]
pub struct B584_W<'a> {
    w: &'a mut W,
}
impl<'a> B584_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B585`"]
pub type B585_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B585`"]
pub struct B585_W<'a> {
    w: &'a mut W,
}
impl<'a> B585_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B586`"]
pub type B586_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B586`"]
pub struct B586_W<'a> {
    w: &'a mut W,
}
impl<'a> B586_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B587`"]
pub type B587_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B587`"]
pub struct B587_W<'a> {
    w: &'a mut W,
}
impl<'a> B587_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B588`"]
pub type B588_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B588`"]
pub struct B588_W<'a> {
    w: &'a mut W,
}
impl<'a> B588_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B589`"]
pub type B589_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B589`"]
pub struct B589_W<'a> {
    w: &'a mut W,
}
impl<'a> B589_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B590`"]
pub type B590_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B590`"]
pub struct B590_W<'a> {
    w: &'a mut W,
}
impl<'a> B590_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B591`"]
pub type B591_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B591`"]
pub struct B591_W<'a> {
    w: &'a mut W,
}
impl<'a> B591_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B592`"]
pub type B592_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B592`"]
pub struct B592_W<'a> {
    w: &'a mut W,
}
impl<'a> B592_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B593`"]
pub type B593_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B593`"]
pub struct B593_W<'a> {
    w: &'a mut W,
}
impl<'a> B593_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B594`"]
pub type B594_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B594`"]
pub struct B594_W<'a> {
    w: &'a mut W,
}
impl<'a> B594_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B595`"]
pub type B595_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B595`"]
pub struct B595_W<'a> {
    w: &'a mut W,
}
impl<'a> B595_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B596`"]
pub type B596_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B596`"]
pub struct B596_W<'a> {
    w: &'a mut W,
}
impl<'a> B596_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B597`"]
pub type B597_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B597`"]
pub struct B597_W<'a> {
    w: &'a mut W,
}
impl<'a> B597_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B598`"]
pub type B598_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B598`"]
pub struct B598_W<'a> {
    w: &'a mut W,
}
impl<'a> B598_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B599`"]
pub type B599_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B599`"]
pub struct B599_W<'a> {
    w: &'a mut W,
}
impl<'a> B599_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B600`"]
pub type B600_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B600`"]
pub struct B600_W<'a> {
    w: &'a mut W,
}
impl<'a> B600_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B601`"]
pub type B601_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B601`"]
pub struct B601_W<'a> {
    w: &'a mut W,
}
impl<'a> B601_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B602`"]
pub type B602_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B602`"]
pub struct B602_W<'a> {
    w: &'a mut W,
}
impl<'a> B602_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B603`"]
pub type B603_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B603`"]
pub struct B603_W<'a> {
    w: &'a mut W,
}
impl<'a> B603_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B604`"]
pub type B604_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B604`"]
pub struct B604_W<'a> {
    w: &'a mut W,
}
impl<'a> B604_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B605`"]
pub type B605_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B605`"]
pub struct B605_W<'a> {
    w: &'a mut W,
}
impl<'a> B605_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B606`"]
pub type B606_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B606`"]
pub struct B606_W<'a> {
    w: &'a mut W,
}
impl<'a> B606_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B607`"]
pub type B607_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B607`"]
pub struct B607_W<'a> {
    w: &'a mut W,
}
impl<'a> B607_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B576"]
    #[inline(always)]
    pub fn b576(&self) -> B576_R {
        B576_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B577"]
    #[inline(always)]
    pub fn b577(&self) -> B577_R {
        B577_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B578"]
    #[inline(always)]
    pub fn b578(&self) -> B578_R {
        B578_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B579"]
    #[inline(always)]
    pub fn b579(&self) -> B579_R {
        B579_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B580"]
    #[inline(always)]
    pub fn b580(&self) -> B580_R {
        B580_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B581"]
    #[inline(always)]
    pub fn b581(&self) -> B581_R {
        B581_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B582"]
    #[inline(always)]
    pub fn b582(&self) -> B582_R {
        B582_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B583"]
    #[inline(always)]
    pub fn b583(&self) -> B583_R {
        B583_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B584"]
    #[inline(always)]
    pub fn b584(&self) -> B584_R {
        B584_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B585"]
    #[inline(always)]
    pub fn b585(&self) -> B585_R {
        B585_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B586"]
    #[inline(always)]
    pub fn b586(&self) -> B586_R {
        B586_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B587"]
    #[inline(always)]
    pub fn b587(&self) -> B587_R {
        B587_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B588"]
    #[inline(always)]
    pub fn b588(&self) -> B588_R {
        B588_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B589"]
    #[inline(always)]
    pub fn b589(&self) -> B589_R {
        B589_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B590"]
    #[inline(always)]
    pub fn b590(&self) -> B590_R {
        B590_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B591"]
    #[inline(always)]
    pub fn b591(&self) -> B591_R {
        B591_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B592"]
    #[inline(always)]
    pub fn b592(&self) -> B592_R {
        B592_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B593"]
    #[inline(always)]
    pub fn b593(&self) -> B593_R {
        B593_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B594"]
    #[inline(always)]
    pub fn b594(&self) -> B594_R {
        B594_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B595"]
    #[inline(always)]
    pub fn b595(&self) -> B595_R {
        B595_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B596"]
    #[inline(always)]
    pub fn b596(&self) -> B596_R {
        B596_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B597"]
    #[inline(always)]
    pub fn b597(&self) -> B597_R {
        B597_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B598"]
    #[inline(always)]
    pub fn b598(&self) -> B598_R {
        B598_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B599"]
    #[inline(always)]
    pub fn b599(&self) -> B599_R {
        B599_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B600"]
    #[inline(always)]
    pub fn b600(&self) -> B600_R {
        B600_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B601"]
    #[inline(always)]
    pub fn b601(&self) -> B601_R {
        B601_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B602"]
    #[inline(always)]
    pub fn b602(&self) -> B602_R {
        B602_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B603"]
    #[inline(always)]
    pub fn b603(&self) -> B603_R {
        B603_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B604"]
    #[inline(always)]
    pub fn b604(&self) -> B604_R {
        B604_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B605"]
    #[inline(always)]
    pub fn b605(&self) -> B605_R {
        B605_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B606"]
    #[inline(always)]
    pub fn b606(&self) -> B606_R {
        B606_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B607"]
    #[inline(always)]
    pub fn b607(&self) -> B607_R {
        B607_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B576"]
    #[inline(always)]
    pub fn b576(&mut self) -> B576_W {
        B576_W { w: self }
    }
    #[doc = "Bit 1 - B577"]
    #[inline(always)]
    pub fn b577(&mut self) -> B577_W {
        B577_W { w: self }
    }
    #[doc = "Bit 2 - B578"]
    #[inline(always)]
    pub fn b578(&mut self) -> B578_W {
        B578_W { w: self }
    }
    #[doc = "Bit 3 - B579"]
    #[inline(always)]
    pub fn b579(&mut self) -> B579_W {
        B579_W { w: self }
    }
    #[doc = "Bit 4 - B580"]
    #[inline(always)]
    pub fn b580(&mut self) -> B580_W {
        B580_W { w: self }
    }
    #[doc = "Bit 5 - B581"]
    #[inline(always)]
    pub fn b581(&mut self) -> B581_W {
        B581_W { w: self }
    }
    #[doc = "Bit 6 - B582"]
    #[inline(always)]
    pub fn b582(&mut self) -> B582_W {
        B582_W { w: self }
    }
    #[doc = "Bit 7 - B583"]
    #[inline(always)]
    pub fn b583(&mut self) -> B583_W {
        B583_W { w: self }
    }
    #[doc = "Bit 8 - B584"]
    #[inline(always)]
    pub fn b584(&mut self) -> B584_W {
        B584_W { w: self }
    }
    #[doc = "Bit 9 - B585"]
    #[inline(always)]
    pub fn b585(&mut self) -> B585_W {
        B585_W { w: self }
    }
    #[doc = "Bit 10 - B586"]
    #[inline(always)]
    pub fn b586(&mut self) -> B586_W {
        B586_W { w: self }
    }
    #[doc = "Bit 11 - B587"]
    #[inline(always)]
    pub fn b587(&mut self) -> B587_W {
        B587_W { w: self }
    }
    #[doc = "Bit 12 - B588"]
    #[inline(always)]
    pub fn b588(&mut self) -> B588_W {
        B588_W { w: self }
    }
    #[doc = "Bit 13 - B589"]
    #[inline(always)]
    pub fn b589(&mut self) -> B589_W {
        B589_W { w: self }
    }
    #[doc = "Bit 14 - B590"]
    #[inline(always)]
    pub fn b590(&mut self) -> B590_W {
        B590_W { w: self }
    }
    #[doc = "Bit 15 - B591"]
    #[inline(always)]
    pub fn b591(&mut self) -> B591_W {
        B591_W { w: self }
    }
    #[doc = "Bit 16 - B592"]
    #[inline(always)]
    pub fn b592(&mut self) -> B592_W {
        B592_W { w: self }
    }
    #[doc = "Bit 17 - B593"]
    #[inline(always)]
    pub fn b593(&mut self) -> B593_W {
        B593_W { w: self }
    }
    #[doc = "Bit 18 - B594"]
    #[inline(always)]
    pub fn b594(&mut self) -> B594_W {
        B594_W { w: self }
    }
    #[doc = "Bit 19 - B595"]
    #[inline(always)]
    pub fn b595(&mut self) -> B595_W {
        B595_W { w: self }
    }
    #[doc = "Bit 20 - B596"]
    #[inline(always)]
    pub fn b596(&mut self) -> B596_W {
        B596_W { w: self }
    }
    #[doc = "Bit 21 - B597"]
    #[inline(always)]
    pub fn b597(&mut self) -> B597_W {
        B597_W { w: self }
    }
    #[doc = "Bit 22 - B598"]
    #[inline(always)]
    pub fn b598(&mut self) -> B598_W {
        B598_W { w: self }
    }
    #[doc = "Bit 23 - B599"]
    #[inline(always)]
    pub fn b599(&mut self) -> B599_W {
        B599_W { w: self }
    }
    #[doc = "Bit 24 - B600"]
    #[inline(always)]
    pub fn b600(&mut self) -> B600_W {
        B600_W { w: self }
    }
    #[doc = "Bit 25 - B601"]
    #[inline(always)]
    pub fn b601(&mut self) -> B601_W {
        B601_W { w: self }
    }
    #[doc = "Bit 26 - B602"]
    #[inline(always)]
    pub fn b602(&mut self) -> B602_W {
        B602_W { w: self }
    }
    #[doc = "Bit 27 - B603"]
    #[inline(always)]
    pub fn b603(&mut self) -> B603_W {
        B603_W { w: self }
    }
    #[doc = "Bit 28 - B604"]
    #[inline(always)]
    pub fn b604(&mut self) -> B604_W {
        B604_W { w: self }
    }
    #[doc = "Bit 29 - B605"]
    #[inline(always)]
    pub fn b605(&mut self) -> B605_W {
        B605_W { w: self }
    }
    #[doc = "Bit 30 - B606"]
    #[inline(always)]
    pub fn b606(&mut self) -> B606_W {
        B606_W { w: self }
    }
    #[doc = "Bit 31 - B607"]
    #[inline(always)]
    pub fn b607(&mut self) -> B607_W {
        B607_W { w: self }
    }
}
