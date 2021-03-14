#[doc = "Reader of register MPCBB1_VCTR30"]
pub type R = crate::R<u32, super::MPCBB1_VCTR30>;
#[doc = "Writer for register MPCBB1_VCTR30"]
pub type W = crate::W<u32, super::MPCBB1_VCTR30>;
#[doc = "Register MPCBB1_VCTR30 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B960`"]
pub type B960_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B960`"]
pub struct B960_W<'a> {
    w: &'a mut W,
}
impl<'a> B960_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B961`"]
pub type B961_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B961`"]
pub struct B961_W<'a> {
    w: &'a mut W,
}
impl<'a> B961_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B962`"]
pub type B962_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B962`"]
pub struct B962_W<'a> {
    w: &'a mut W,
}
impl<'a> B962_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B963`"]
pub type B963_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B963`"]
pub struct B963_W<'a> {
    w: &'a mut W,
}
impl<'a> B963_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B964`"]
pub type B964_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B964`"]
pub struct B964_W<'a> {
    w: &'a mut W,
}
impl<'a> B964_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B965`"]
pub type B965_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B965`"]
pub struct B965_W<'a> {
    w: &'a mut W,
}
impl<'a> B965_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B966`"]
pub type B966_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B966`"]
pub struct B966_W<'a> {
    w: &'a mut W,
}
impl<'a> B966_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B967`"]
pub type B967_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B967`"]
pub struct B967_W<'a> {
    w: &'a mut W,
}
impl<'a> B967_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B968`"]
pub type B968_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B968`"]
pub struct B968_W<'a> {
    w: &'a mut W,
}
impl<'a> B968_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B969`"]
pub type B969_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B969`"]
pub struct B969_W<'a> {
    w: &'a mut W,
}
impl<'a> B969_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B970`"]
pub type B970_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B970`"]
pub struct B970_W<'a> {
    w: &'a mut W,
}
impl<'a> B970_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B971`"]
pub type B971_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B971`"]
pub struct B971_W<'a> {
    w: &'a mut W,
}
impl<'a> B971_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B972`"]
pub type B972_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B972`"]
pub struct B972_W<'a> {
    w: &'a mut W,
}
impl<'a> B972_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B973`"]
pub type B973_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B973`"]
pub struct B973_W<'a> {
    w: &'a mut W,
}
impl<'a> B973_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B974`"]
pub type B974_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B974`"]
pub struct B974_W<'a> {
    w: &'a mut W,
}
impl<'a> B974_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B975`"]
pub type B975_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B975`"]
pub struct B975_W<'a> {
    w: &'a mut W,
}
impl<'a> B975_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B976`"]
pub type B976_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B976`"]
pub struct B976_W<'a> {
    w: &'a mut W,
}
impl<'a> B976_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B977`"]
pub type B977_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B977`"]
pub struct B977_W<'a> {
    w: &'a mut W,
}
impl<'a> B977_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B978`"]
pub type B978_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B978`"]
pub struct B978_W<'a> {
    w: &'a mut W,
}
impl<'a> B978_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B979`"]
pub type B979_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B979`"]
pub struct B979_W<'a> {
    w: &'a mut W,
}
impl<'a> B979_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B980`"]
pub type B980_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B980`"]
pub struct B980_W<'a> {
    w: &'a mut W,
}
impl<'a> B980_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B981`"]
pub type B981_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B981`"]
pub struct B981_W<'a> {
    w: &'a mut W,
}
impl<'a> B981_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B982`"]
pub type B982_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B982`"]
pub struct B982_W<'a> {
    w: &'a mut W,
}
impl<'a> B982_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B983`"]
pub type B983_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B983`"]
pub struct B983_W<'a> {
    w: &'a mut W,
}
impl<'a> B983_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B984`"]
pub type B984_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B984`"]
pub struct B984_W<'a> {
    w: &'a mut W,
}
impl<'a> B984_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B985`"]
pub type B985_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B985`"]
pub struct B985_W<'a> {
    w: &'a mut W,
}
impl<'a> B985_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B986`"]
pub type B986_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B986`"]
pub struct B986_W<'a> {
    w: &'a mut W,
}
impl<'a> B986_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B987`"]
pub type B987_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B987`"]
pub struct B987_W<'a> {
    w: &'a mut W,
}
impl<'a> B987_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B988`"]
pub type B988_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B988`"]
pub struct B988_W<'a> {
    w: &'a mut W,
}
impl<'a> B988_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B989`"]
pub type B989_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B989`"]
pub struct B989_W<'a> {
    w: &'a mut W,
}
impl<'a> B989_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B990`"]
pub type B990_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B990`"]
pub struct B990_W<'a> {
    w: &'a mut W,
}
impl<'a> B990_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B991`"]
pub type B991_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B991`"]
pub struct B991_W<'a> {
    w: &'a mut W,
}
impl<'a> B991_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B960"]
    #[inline(always)]
    pub fn b960(&self) -> B960_R {
        B960_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B961"]
    #[inline(always)]
    pub fn b961(&self) -> B961_R {
        B961_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B962"]
    #[inline(always)]
    pub fn b962(&self) -> B962_R {
        B962_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B963"]
    #[inline(always)]
    pub fn b963(&self) -> B963_R {
        B963_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B964"]
    #[inline(always)]
    pub fn b964(&self) -> B964_R {
        B964_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B965"]
    #[inline(always)]
    pub fn b965(&self) -> B965_R {
        B965_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B966"]
    #[inline(always)]
    pub fn b966(&self) -> B966_R {
        B966_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B967"]
    #[inline(always)]
    pub fn b967(&self) -> B967_R {
        B967_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B968"]
    #[inline(always)]
    pub fn b968(&self) -> B968_R {
        B968_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B969"]
    #[inline(always)]
    pub fn b969(&self) -> B969_R {
        B969_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B970"]
    #[inline(always)]
    pub fn b970(&self) -> B970_R {
        B970_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B971"]
    #[inline(always)]
    pub fn b971(&self) -> B971_R {
        B971_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B972"]
    #[inline(always)]
    pub fn b972(&self) -> B972_R {
        B972_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B973"]
    #[inline(always)]
    pub fn b973(&self) -> B973_R {
        B973_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B974"]
    #[inline(always)]
    pub fn b974(&self) -> B974_R {
        B974_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B975"]
    #[inline(always)]
    pub fn b975(&self) -> B975_R {
        B975_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B976"]
    #[inline(always)]
    pub fn b976(&self) -> B976_R {
        B976_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B977"]
    #[inline(always)]
    pub fn b977(&self) -> B977_R {
        B977_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B978"]
    #[inline(always)]
    pub fn b978(&self) -> B978_R {
        B978_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B979"]
    #[inline(always)]
    pub fn b979(&self) -> B979_R {
        B979_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B980"]
    #[inline(always)]
    pub fn b980(&self) -> B980_R {
        B980_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B981"]
    #[inline(always)]
    pub fn b981(&self) -> B981_R {
        B981_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B982"]
    #[inline(always)]
    pub fn b982(&self) -> B982_R {
        B982_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B983"]
    #[inline(always)]
    pub fn b983(&self) -> B983_R {
        B983_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B984"]
    #[inline(always)]
    pub fn b984(&self) -> B984_R {
        B984_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B985"]
    #[inline(always)]
    pub fn b985(&self) -> B985_R {
        B985_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B986"]
    #[inline(always)]
    pub fn b986(&self) -> B986_R {
        B986_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B987"]
    #[inline(always)]
    pub fn b987(&self) -> B987_R {
        B987_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B988"]
    #[inline(always)]
    pub fn b988(&self) -> B988_R {
        B988_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B989"]
    #[inline(always)]
    pub fn b989(&self) -> B989_R {
        B989_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B990"]
    #[inline(always)]
    pub fn b990(&self) -> B990_R {
        B990_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B991"]
    #[inline(always)]
    pub fn b991(&self) -> B991_R {
        B991_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B960"]
    #[inline(always)]
    pub fn b960(&mut self) -> B960_W {
        B960_W { w: self }
    }
    #[doc = "Bit 1 - B961"]
    #[inline(always)]
    pub fn b961(&mut self) -> B961_W {
        B961_W { w: self }
    }
    #[doc = "Bit 2 - B962"]
    #[inline(always)]
    pub fn b962(&mut self) -> B962_W {
        B962_W { w: self }
    }
    #[doc = "Bit 3 - B963"]
    #[inline(always)]
    pub fn b963(&mut self) -> B963_W {
        B963_W { w: self }
    }
    #[doc = "Bit 4 - B964"]
    #[inline(always)]
    pub fn b964(&mut self) -> B964_W {
        B964_W { w: self }
    }
    #[doc = "Bit 5 - B965"]
    #[inline(always)]
    pub fn b965(&mut self) -> B965_W {
        B965_W { w: self }
    }
    #[doc = "Bit 6 - B966"]
    #[inline(always)]
    pub fn b966(&mut self) -> B966_W {
        B966_W { w: self }
    }
    #[doc = "Bit 7 - B967"]
    #[inline(always)]
    pub fn b967(&mut self) -> B967_W {
        B967_W { w: self }
    }
    #[doc = "Bit 8 - B968"]
    #[inline(always)]
    pub fn b968(&mut self) -> B968_W {
        B968_W { w: self }
    }
    #[doc = "Bit 9 - B969"]
    #[inline(always)]
    pub fn b969(&mut self) -> B969_W {
        B969_W { w: self }
    }
    #[doc = "Bit 10 - B970"]
    #[inline(always)]
    pub fn b970(&mut self) -> B970_W {
        B970_W { w: self }
    }
    #[doc = "Bit 11 - B971"]
    #[inline(always)]
    pub fn b971(&mut self) -> B971_W {
        B971_W { w: self }
    }
    #[doc = "Bit 12 - B972"]
    #[inline(always)]
    pub fn b972(&mut self) -> B972_W {
        B972_W { w: self }
    }
    #[doc = "Bit 13 - B973"]
    #[inline(always)]
    pub fn b973(&mut self) -> B973_W {
        B973_W { w: self }
    }
    #[doc = "Bit 14 - B974"]
    #[inline(always)]
    pub fn b974(&mut self) -> B974_W {
        B974_W { w: self }
    }
    #[doc = "Bit 15 - B975"]
    #[inline(always)]
    pub fn b975(&mut self) -> B975_W {
        B975_W { w: self }
    }
    #[doc = "Bit 16 - B976"]
    #[inline(always)]
    pub fn b976(&mut self) -> B976_W {
        B976_W { w: self }
    }
    #[doc = "Bit 17 - B977"]
    #[inline(always)]
    pub fn b977(&mut self) -> B977_W {
        B977_W { w: self }
    }
    #[doc = "Bit 18 - B978"]
    #[inline(always)]
    pub fn b978(&mut self) -> B978_W {
        B978_W { w: self }
    }
    #[doc = "Bit 19 - B979"]
    #[inline(always)]
    pub fn b979(&mut self) -> B979_W {
        B979_W { w: self }
    }
    #[doc = "Bit 20 - B980"]
    #[inline(always)]
    pub fn b980(&mut self) -> B980_W {
        B980_W { w: self }
    }
    #[doc = "Bit 21 - B981"]
    #[inline(always)]
    pub fn b981(&mut self) -> B981_W {
        B981_W { w: self }
    }
    #[doc = "Bit 22 - B982"]
    #[inline(always)]
    pub fn b982(&mut self) -> B982_W {
        B982_W { w: self }
    }
    #[doc = "Bit 23 - B983"]
    #[inline(always)]
    pub fn b983(&mut self) -> B983_W {
        B983_W { w: self }
    }
    #[doc = "Bit 24 - B984"]
    #[inline(always)]
    pub fn b984(&mut self) -> B984_W {
        B984_W { w: self }
    }
    #[doc = "Bit 25 - B985"]
    #[inline(always)]
    pub fn b985(&mut self) -> B985_W {
        B985_W { w: self }
    }
    #[doc = "Bit 26 - B986"]
    #[inline(always)]
    pub fn b986(&mut self) -> B986_W {
        B986_W { w: self }
    }
    #[doc = "Bit 27 - B987"]
    #[inline(always)]
    pub fn b987(&mut self) -> B987_W {
        B987_W { w: self }
    }
    #[doc = "Bit 28 - B988"]
    #[inline(always)]
    pub fn b988(&mut self) -> B988_W {
        B988_W { w: self }
    }
    #[doc = "Bit 29 - B989"]
    #[inline(always)]
    pub fn b989(&mut self) -> B989_W {
        B989_W { w: self }
    }
    #[doc = "Bit 30 - B990"]
    #[inline(always)]
    pub fn b990(&mut self) -> B990_W {
        B990_W { w: self }
    }
    #[doc = "Bit 31 - B991"]
    #[inline(always)]
    pub fn b991(&mut self) -> B991_W {
        B991_W { w: self }
    }
}
