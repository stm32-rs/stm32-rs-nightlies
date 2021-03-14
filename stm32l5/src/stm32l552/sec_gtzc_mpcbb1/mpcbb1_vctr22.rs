#[doc = "Reader of register MPCBB1_VCTR22"]
pub type R = crate::R<u32, super::MPCBB1_VCTR22>;
#[doc = "Writer for register MPCBB1_VCTR22"]
pub type W = crate::W<u32, super::MPCBB1_VCTR22>;
#[doc = "Register MPCBB1_VCTR22 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B704`"]
pub type B704_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B704`"]
pub struct B704_W<'a> {
    w: &'a mut W,
}
impl<'a> B704_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B705`"]
pub type B705_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B705`"]
pub struct B705_W<'a> {
    w: &'a mut W,
}
impl<'a> B705_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B706`"]
pub type B706_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B706`"]
pub struct B706_W<'a> {
    w: &'a mut W,
}
impl<'a> B706_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B707`"]
pub type B707_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B707`"]
pub struct B707_W<'a> {
    w: &'a mut W,
}
impl<'a> B707_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B708`"]
pub type B708_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B708`"]
pub struct B708_W<'a> {
    w: &'a mut W,
}
impl<'a> B708_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B709`"]
pub type B709_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B709`"]
pub struct B709_W<'a> {
    w: &'a mut W,
}
impl<'a> B709_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B710`"]
pub type B710_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B710`"]
pub struct B710_W<'a> {
    w: &'a mut W,
}
impl<'a> B710_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B711`"]
pub type B711_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B711`"]
pub struct B711_W<'a> {
    w: &'a mut W,
}
impl<'a> B711_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B712`"]
pub type B712_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B712`"]
pub struct B712_W<'a> {
    w: &'a mut W,
}
impl<'a> B712_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B713`"]
pub type B713_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B713`"]
pub struct B713_W<'a> {
    w: &'a mut W,
}
impl<'a> B713_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B714`"]
pub type B714_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B714`"]
pub struct B714_W<'a> {
    w: &'a mut W,
}
impl<'a> B714_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B715`"]
pub type B715_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B715`"]
pub struct B715_W<'a> {
    w: &'a mut W,
}
impl<'a> B715_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B716`"]
pub type B716_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B716`"]
pub struct B716_W<'a> {
    w: &'a mut W,
}
impl<'a> B716_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B717`"]
pub type B717_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B717`"]
pub struct B717_W<'a> {
    w: &'a mut W,
}
impl<'a> B717_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B718`"]
pub type B718_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B718`"]
pub struct B718_W<'a> {
    w: &'a mut W,
}
impl<'a> B718_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B719`"]
pub type B719_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B719`"]
pub struct B719_W<'a> {
    w: &'a mut W,
}
impl<'a> B719_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B720`"]
pub type B720_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B720`"]
pub struct B720_W<'a> {
    w: &'a mut W,
}
impl<'a> B720_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B721`"]
pub type B721_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B721`"]
pub struct B721_W<'a> {
    w: &'a mut W,
}
impl<'a> B721_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B722`"]
pub type B722_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B722`"]
pub struct B722_W<'a> {
    w: &'a mut W,
}
impl<'a> B722_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B723`"]
pub type B723_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B723`"]
pub struct B723_W<'a> {
    w: &'a mut W,
}
impl<'a> B723_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B724`"]
pub type B724_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B724`"]
pub struct B724_W<'a> {
    w: &'a mut W,
}
impl<'a> B724_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B725`"]
pub type B725_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B725`"]
pub struct B725_W<'a> {
    w: &'a mut W,
}
impl<'a> B725_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B726`"]
pub type B726_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B726`"]
pub struct B726_W<'a> {
    w: &'a mut W,
}
impl<'a> B726_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B727`"]
pub type B727_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B727`"]
pub struct B727_W<'a> {
    w: &'a mut W,
}
impl<'a> B727_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B728`"]
pub type B728_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B728`"]
pub struct B728_W<'a> {
    w: &'a mut W,
}
impl<'a> B728_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B729`"]
pub type B729_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B729`"]
pub struct B729_W<'a> {
    w: &'a mut W,
}
impl<'a> B729_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B730`"]
pub type B730_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B730`"]
pub struct B730_W<'a> {
    w: &'a mut W,
}
impl<'a> B730_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B731`"]
pub type B731_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B731`"]
pub struct B731_W<'a> {
    w: &'a mut W,
}
impl<'a> B731_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B732`"]
pub type B732_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B732`"]
pub struct B732_W<'a> {
    w: &'a mut W,
}
impl<'a> B732_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B733`"]
pub type B733_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B733`"]
pub struct B733_W<'a> {
    w: &'a mut W,
}
impl<'a> B733_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B734`"]
pub type B734_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B734`"]
pub struct B734_W<'a> {
    w: &'a mut W,
}
impl<'a> B734_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B735`"]
pub type B735_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B735`"]
pub struct B735_W<'a> {
    w: &'a mut W,
}
impl<'a> B735_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B704"]
    #[inline(always)]
    pub fn b704(&self) -> B704_R {
        B704_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B705"]
    #[inline(always)]
    pub fn b705(&self) -> B705_R {
        B705_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B706"]
    #[inline(always)]
    pub fn b706(&self) -> B706_R {
        B706_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B707"]
    #[inline(always)]
    pub fn b707(&self) -> B707_R {
        B707_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B708"]
    #[inline(always)]
    pub fn b708(&self) -> B708_R {
        B708_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B709"]
    #[inline(always)]
    pub fn b709(&self) -> B709_R {
        B709_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B710"]
    #[inline(always)]
    pub fn b710(&self) -> B710_R {
        B710_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B711"]
    #[inline(always)]
    pub fn b711(&self) -> B711_R {
        B711_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B712"]
    #[inline(always)]
    pub fn b712(&self) -> B712_R {
        B712_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B713"]
    #[inline(always)]
    pub fn b713(&self) -> B713_R {
        B713_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B714"]
    #[inline(always)]
    pub fn b714(&self) -> B714_R {
        B714_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B715"]
    #[inline(always)]
    pub fn b715(&self) -> B715_R {
        B715_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B716"]
    #[inline(always)]
    pub fn b716(&self) -> B716_R {
        B716_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B717"]
    #[inline(always)]
    pub fn b717(&self) -> B717_R {
        B717_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B718"]
    #[inline(always)]
    pub fn b718(&self) -> B718_R {
        B718_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B719"]
    #[inline(always)]
    pub fn b719(&self) -> B719_R {
        B719_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B720"]
    #[inline(always)]
    pub fn b720(&self) -> B720_R {
        B720_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B721"]
    #[inline(always)]
    pub fn b721(&self) -> B721_R {
        B721_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B722"]
    #[inline(always)]
    pub fn b722(&self) -> B722_R {
        B722_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B723"]
    #[inline(always)]
    pub fn b723(&self) -> B723_R {
        B723_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B724"]
    #[inline(always)]
    pub fn b724(&self) -> B724_R {
        B724_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B725"]
    #[inline(always)]
    pub fn b725(&self) -> B725_R {
        B725_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B726"]
    #[inline(always)]
    pub fn b726(&self) -> B726_R {
        B726_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B727"]
    #[inline(always)]
    pub fn b727(&self) -> B727_R {
        B727_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B728"]
    #[inline(always)]
    pub fn b728(&self) -> B728_R {
        B728_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B729"]
    #[inline(always)]
    pub fn b729(&self) -> B729_R {
        B729_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B730"]
    #[inline(always)]
    pub fn b730(&self) -> B730_R {
        B730_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B731"]
    #[inline(always)]
    pub fn b731(&self) -> B731_R {
        B731_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B732"]
    #[inline(always)]
    pub fn b732(&self) -> B732_R {
        B732_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B733"]
    #[inline(always)]
    pub fn b733(&self) -> B733_R {
        B733_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B734"]
    #[inline(always)]
    pub fn b734(&self) -> B734_R {
        B734_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B735"]
    #[inline(always)]
    pub fn b735(&self) -> B735_R {
        B735_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B704"]
    #[inline(always)]
    pub fn b704(&mut self) -> B704_W {
        B704_W { w: self }
    }
    #[doc = "Bit 1 - B705"]
    #[inline(always)]
    pub fn b705(&mut self) -> B705_W {
        B705_W { w: self }
    }
    #[doc = "Bit 2 - B706"]
    #[inline(always)]
    pub fn b706(&mut self) -> B706_W {
        B706_W { w: self }
    }
    #[doc = "Bit 3 - B707"]
    #[inline(always)]
    pub fn b707(&mut self) -> B707_W {
        B707_W { w: self }
    }
    #[doc = "Bit 4 - B708"]
    #[inline(always)]
    pub fn b708(&mut self) -> B708_W {
        B708_W { w: self }
    }
    #[doc = "Bit 5 - B709"]
    #[inline(always)]
    pub fn b709(&mut self) -> B709_W {
        B709_W { w: self }
    }
    #[doc = "Bit 6 - B710"]
    #[inline(always)]
    pub fn b710(&mut self) -> B710_W {
        B710_W { w: self }
    }
    #[doc = "Bit 7 - B711"]
    #[inline(always)]
    pub fn b711(&mut self) -> B711_W {
        B711_W { w: self }
    }
    #[doc = "Bit 8 - B712"]
    #[inline(always)]
    pub fn b712(&mut self) -> B712_W {
        B712_W { w: self }
    }
    #[doc = "Bit 9 - B713"]
    #[inline(always)]
    pub fn b713(&mut self) -> B713_W {
        B713_W { w: self }
    }
    #[doc = "Bit 10 - B714"]
    #[inline(always)]
    pub fn b714(&mut self) -> B714_W {
        B714_W { w: self }
    }
    #[doc = "Bit 11 - B715"]
    #[inline(always)]
    pub fn b715(&mut self) -> B715_W {
        B715_W { w: self }
    }
    #[doc = "Bit 12 - B716"]
    #[inline(always)]
    pub fn b716(&mut self) -> B716_W {
        B716_W { w: self }
    }
    #[doc = "Bit 13 - B717"]
    #[inline(always)]
    pub fn b717(&mut self) -> B717_W {
        B717_W { w: self }
    }
    #[doc = "Bit 14 - B718"]
    #[inline(always)]
    pub fn b718(&mut self) -> B718_W {
        B718_W { w: self }
    }
    #[doc = "Bit 15 - B719"]
    #[inline(always)]
    pub fn b719(&mut self) -> B719_W {
        B719_W { w: self }
    }
    #[doc = "Bit 16 - B720"]
    #[inline(always)]
    pub fn b720(&mut self) -> B720_W {
        B720_W { w: self }
    }
    #[doc = "Bit 17 - B721"]
    #[inline(always)]
    pub fn b721(&mut self) -> B721_W {
        B721_W { w: self }
    }
    #[doc = "Bit 18 - B722"]
    #[inline(always)]
    pub fn b722(&mut self) -> B722_W {
        B722_W { w: self }
    }
    #[doc = "Bit 19 - B723"]
    #[inline(always)]
    pub fn b723(&mut self) -> B723_W {
        B723_W { w: self }
    }
    #[doc = "Bit 20 - B724"]
    #[inline(always)]
    pub fn b724(&mut self) -> B724_W {
        B724_W { w: self }
    }
    #[doc = "Bit 21 - B725"]
    #[inline(always)]
    pub fn b725(&mut self) -> B725_W {
        B725_W { w: self }
    }
    #[doc = "Bit 22 - B726"]
    #[inline(always)]
    pub fn b726(&mut self) -> B726_W {
        B726_W { w: self }
    }
    #[doc = "Bit 23 - B727"]
    #[inline(always)]
    pub fn b727(&mut self) -> B727_W {
        B727_W { w: self }
    }
    #[doc = "Bit 24 - B728"]
    #[inline(always)]
    pub fn b728(&mut self) -> B728_W {
        B728_W { w: self }
    }
    #[doc = "Bit 25 - B729"]
    #[inline(always)]
    pub fn b729(&mut self) -> B729_W {
        B729_W { w: self }
    }
    #[doc = "Bit 26 - B730"]
    #[inline(always)]
    pub fn b730(&mut self) -> B730_W {
        B730_W { w: self }
    }
    #[doc = "Bit 27 - B731"]
    #[inline(always)]
    pub fn b731(&mut self) -> B731_W {
        B731_W { w: self }
    }
    #[doc = "Bit 28 - B732"]
    #[inline(always)]
    pub fn b732(&mut self) -> B732_W {
        B732_W { w: self }
    }
    #[doc = "Bit 29 - B733"]
    #[inline(always)]
    pub fn b733(&mut self) -> B733_W {
        B733_W { w: self }
    }
    #[doc = "Bit 30 - B734"]
    #[inline(always)]
    pub fn b734(&mut self) -> B734_W {
        B734_W { w: self }
    }
    #[doc = "Bit 31 - B735"]
    #[inline(always)]
    pub fn b735(&mut self) -> B735_W {
        B735_W { w: self }
    }
}
