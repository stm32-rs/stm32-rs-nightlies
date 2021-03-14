#[doc = "Reader of register MPCBB1_VCTR16"]
pub type R = crate::R<u32, super::MPCBB1_VCTR16>;
#[doc = "Writer for register MPCBB1_VCTR16"]
pub type W = crate::W<u32, super::MPCBB1_VCTR16>;
#[doc = "Register MPCBB1_VCTR16 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B512`"]
pub type B512_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B512`"]
pub struct B512_W<'a> {
    w: &'a mut W,
}
impl<'a> B512_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B513`"]
pub type B513_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B513`"]
pub struct B513_W<'a> {
    w: &'a mut W,
}
impl<'a> B513_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B514`"]
pub type B514_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B514`"]
pub struct B514_W<'a> {
    w: &'a mut W,
}
impl<'a> B514_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B515`"]
pub type B515_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B515`"]
pub struct B515_W<'a> {
    w: &'a mut W,
}
impl<'a> B515_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B516`"]
pub type B516_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B516`"]
pub struct B516_W<'a> {
    w: &'a mut W,
}
impl<'a> B516_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B517`"]
pub type B517_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B517`"]
pub struct B517_W<'a> {
    w: &'a mut W,
}
impl<'a> B517_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B518`"]
pub type B518_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B518`"]
pub struct B518_W<'a> {
    w: &'a mut W,
}
impl<'a> B518_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B519`"]
pub type B519_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B519`"]
pub struct B519_W<'a> {
    w: &'a mut W,
}
impl<'a> B519_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B520`"]
pub type B520_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B520`"]
pub struct B520_W<'a> {
    w: &'a mut W,
}
impl<'a> B520_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B521`"]
pub type B521_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B521`"]
pub struct B521_W<'a> {
    w: &'a mut W,
}
impl<'a> B521_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B522`"]
pub type B522_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B522`"]
pub struct B522_W<'a> {
    w: &'a mut W,
}
impl<'a> B522_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B523`"]
pub type B523_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B523`"]
pub struct B523_W<'a> {
    w: &'a mut W,
}
impl<'a> B523_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B524`"]
pub type B524_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B524`"]
pub struct B524_W<'a> {
    w: &'a mut W,
}
impl<'a> B524_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B525`"]
pub type B525_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B525`"]
pub struct B525_W<'a> {
    w: &'a mut W,
}
impl<'a> B525_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B526`"]
pub type B526_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B526`"]
pub struct B526_W<'a> {
    w: &'a mut W,
}
impl<'a> B526_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B527`"]
pub type B527_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B527`"]
pub struct B527_W<'a> {
    w: &'a mut W,
}
impl<'a> B527_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B528`"]
pub type B528_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B528`"]
pub struct B528_W<'a> {
    w: &'a mut W,
}
impl<'a> B528_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B529`"]
pub type B529_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B529`"]
pub struct B529_W<'a> {
    w: &'a mut W,
}
impl<'a> B529_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B530`"]
pub type B530_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B530`"]
pub struct B530_W<'a> {
    w: &'a mut W,
}
impl<'a> B530_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B531`"]
pub type B531_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B531`"]
pub struct B531_W<'a> {
    w: &'a mut W,
}
impl<'a> B531_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B532`"]
pub type B532_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B532`"]
pub struct B532_W<'a> {
    w: &'a mut W,
}
impl<'a> B532_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B533`"]
pub type B533_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B533`"]
pub struct B533_W<'a> {
    w: &'a mut W,
}
impl<'a> B533_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B534`"]
pub type B534_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B534`"]
pub struct B534_W<'a> {
    w: &'a mut W,
}
impl<'a> B534_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B535`"]
pub type B535_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B535`"]
pub struct B535_W<'a> {
    w: &'a mut W,
}
impl<'a> B535_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B536`"]
pub type B536_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B536`"]
pub struct B536_W<'a> {
    w: &'a mut W,
}
impl<'a> B536_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B537`"]
pub type B537_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B537`"]
pub struct B537_W<'a> {
    w: &'a mut W,
}
impl<'a> B537_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B538`"]
pub type B538_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B538`"]
pub struct B538_W<'a> {
    w: &'a mut W,
}
impl<'a> B538_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B539`"]
pub type B539_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B539`"]
pub struct B539_W<'a> {
    w: &'a mut W,
}
impl<'a> B539_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B540`"]
pub type B540_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B540`"]
pub struct B540_W<'a> {
    w: &'a mut W,
}
impl<'a> B540_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B541`"]
pub type B541_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B541`"]
pub struct B541_W<'a> {
    w: &'a mut W,
}
impl<'a> B541_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B542`"]
pub type B542_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B542`"]
pub struct B542_W<'a> {
    w: &'a mut W,
}
impl<'a> B542_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B543`"]
pub type B543_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B543`"]
pub struct B543_W<'a> {
    w: &'a mut W,
}
impl<'a> B543_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B512"]
    #[inline(always)]
    pub fn b512(&self) -> B512_R {
        B512_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B513"]
    #[inline(always)]
    pub fn b513(&self) -> B513_R {
        B513_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B514"]
    #[inline(always)]
    pub fn b514(&self) -> B514_R {
        B514_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B515"]
    #[inline(always)]
    pub fn b515(&self) -> B515_R {
        B515_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B516"]
    #[inline(always)]
    pub fn b516(&self) -> B516_R {
        B516_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B517"]
    #[inline(always)]
    pub fn b517(&self) -> B517_R {
        B517_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B518"]
    #[inline(always)]
    pub fn b518(&self) -> B518_R {
        B518_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B519"]
    #[inline(always)]
    pub fn b519(&self) -> B519_R {
        B519_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B520"]
    #[inline(always)]
    pub fn b520(&self) -> B520_R {
        B520_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B521"]
    #[inline(always)]
    pub fn b521(&self) -> B521_R {
        B521_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B522"]
    #[inline(always)]
    pub fn b522(&self) -> B522_R {
        B522_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B523"]
    #[inline(always)]
    pub fn b523(&self) -> B523_R {
        B523_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B524"]
    #[inline(always)]
    pub fn b524(&self) -> B524_R {
        B524_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B525"]
    #[inline(always)]
    pub fn b525(&self) -> B525_R {
        B525_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B526"]
    #[inline(always)]
    pub fn b526(&self) -> B526_R {
        B526_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B527"]
    #[inline(always)]
    pub fn b527(&self) -> B527_R {
        B527_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B528"]
    #[inline(always)]
    pub fn b528(&self) -> B528_R {
        B528_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B529"]
    #[inline(always)]
    pub fn b529(&self) -> B529_R {
        B529_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B530"]
    #[inline(always)]
    pub fn b530(&self) -> B530_R {
        B530_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B531"]
    #[inline(always)]
    pub fn b531(&self) -> B531_R {
        B531_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B532"]
    #[inline(always)]
    pub fn b532(&self) -> B532_R {
        B532_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B533"]
    #[inline(always)]
    pub fn b533(&self) -> B533_R {
        B533_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B534"]
    #[inline(always)]
    pub fn b534(&self) -> B534_R {
        B534_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B535"]
    #[inline(always)]
    pub fn b535(&self) -> B535_R {
        B535_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B536"]
    #[inline(always)]
    pub fn b536(&self) -> B536_R {
        B536_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B537"]
    #[inline(always)]
    pub fn b537(&self) -> B537_R {
        B537_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B538"]
    #[inline(always)]
    pub fn b538(&self) -> B538_R {
        B538_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B539"]
    #[inline(always)]
    pub fn b539(&self) -> B539_R {
        B539_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B540"]
    #[inline(always)]
    pub fn b540(&self) -> B540_R {
        B540_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B541"]
    #[inline(always)]
    pub fn b541(&self) -> B541_R {
        B541_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B542"]
    #[inline(always)]
    pub fn b542(&self) -> B542_R {
        B542_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B543"]
    #[inline(always)]
    pub fn b543(&self) -> B543_R {
        B543_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B512"]
    #[inline(always)]
    pub fn b512(&mut self) -> B512_W {
        B512_W { w: self }
    }
    #[doc = "Bit 1 - B513"]
    #[inline(always)]
    pub fn b513(&mut self) -> B513_W {
        B513_W { w: self }
    }
    #[doc = "Bit 2 - B514"]
    #[inline(always)]
    pub fn b514(&mut self) -> B514_W {
        B514_W { w: self }
    }
    #[doc = "Bit 3 - B515"]
    #[inline(always)]
    pub fn b515(&mut self) -> B515_W {
        B515_W { w: self }
    }
    #[doc = "Bit 4 - B516"]
    #[inline(always)]
    pub fn b516(&mut self) -> B516_W {
        B516_W { w: self }
    }
    #[doc = "Bit 5 - B517"]
    #[inline(always)]
    pub fn b517(&mut self) -> B517_W {
        B517_W { w: self }
    }
    #[doc = "Bit 6 - B518"]
    #[inline(always)]
    pub fn b518(&mut self) -> B518_W {
        B518_W { w: self }
    }
    #[doc = "Bit 7 - B519"]
    #[inline(always)]
    pub fn b519(&mut self) -> B519_W {
        B519_W { w: self }
    }
    #[doc = "Bit 8 - B520"]
    #[inline(always)]
    pub fn b520(&mut self) -> B520_W {
        B520_W { w: self }
    }
    #[doc = "Bit 9 - B521"]
    #[inline(always)]
    pub fn b521(&mut self) -> B521_W {
        B521_W { w: self }
    }
    #[doc = "Bit 10 - B522"]
    #[inline(always)]
    pub fn b522(&mut self) -> B522_W {
        B522_W { w: self }
    }
    #[doc = "Bit 11 - B523"]
    #[inline(always)]
    pub fn b523(&mut self) -> B523_W {
        B523_W { w: self }
    }
    #[doc = "Bit 12 - B524"]
    #[inline(always)]
    pub fn b524(&mut self) -> B524_W {
        B524_W { w: self }
    }
    #[doc = "Bit 13 - B525"]
    #[inline(always)]
    pub fn b525(&mut self) -> B525_W {
        B525_W { w: self }
    }
    #[doc = "Bit 14 - B526"]
    #[inline(always)]
    pub fn b526(&mut self) -> B526_W {
        B526_W { w: self }
    }
    #[doc = "Bit 15 - B527"]
    #[inline(always)]
    pub fn b527(&mut self) -> B527_W {
        B527_W { w: self }
    }
    #[doc = "Bit 16 - B528"]
    #[inline(always)]
    pub fn b528(&mut self) -> B528_W {
        B528_W { w: self }
    }
    #[doc = "Bit 17 - B529"]
    #[inline(always)]
    pub fn b529(&mut self) -> B529_W {
        B529_W { w: self }
    }
    #[doc = "Bit 18 - B530"]
    #[inline(always)]
    pub fn b530(&mut self) -> B530_W {
        B530_W { w: self }
    }
    #[doc = "Bit 19 - B531"]
    #[inline(always)]
    pub fn b531(&mut self) -> B531_W {
        B531_W { w: self }
    }
    #[doc = "Bit 20 - B532"]
    #[inline(always)]
    pub fn b532(&mut self) -> B532_W {
        B532_W { w: self }
    }
    #[doc = "Bit 21 - B533"]
    #[inline(always)]
    pub fn b533(&mut self) -> B533_W {
        B533_W { w: self }
    }
    #[doc = "Bit 22 - B534"]
    #[inline(always)]
    pub fn b534(&mut self) -> B534_W {
        B534_W { w: self }
    }
    #[doc = "Bit 23 - B535"]
    #[inline(always)]
    pub fn b535(&mut self) -> B535_W {
        B535_W { w: self }
    }
    #[doc = "Bit 24 - B536"]
    #[inline(always)]
    pub fn b536(&mut self) -> B536_W {
        B536_W { w: self }
    }
    #[doc = "Bit 25 - B537"]
    #[inline(always)]
    pub fn b537(&mut self) -> B537_W {
        B537_W { w: self }
    }
    #[doc = "Bit 26 - B538"]
    #[inline(always)]
    pub fn b538(&mut self) -> B538_W {
        B538_W { w: self }
    }
    #[doc = "Bit 27 - B539"]
    #[inline(always)]
    pub fn b539(&mut self) -> B539_W {
        B539_W { w: self }
    }
    #[doc = "Bit 28 - B540"]
    #[inline(always)]
    pub fn b540(&mut self) -> B540_W {
        B540_W { w: self }
    }
    #[doc = "Bit 29 - B541"]
    #[inline(always)]
    pub fn b541(&mut self) -> B541_W {
        B541_W { w: self }
    }
    #[doc = "Bit 30 - B542"]
    #[inline(always)]
    pub fn b542(&mut self) -> B542_W {
        B542_W { w: self }
    }
    #[doc = "Bit 31 - B543"]
    #[inline(always)]
    pub fn b543(&mut self) -> B543_W {
        B543_W { w: self }
    }
}
