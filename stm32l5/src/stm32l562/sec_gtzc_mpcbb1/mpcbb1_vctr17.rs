#[doc = "Reader of register MPCBB1_VCTR17"]
pub type R = crate::R<u32, super::MPCBB1_VCTR17>;
#[doc = "Writer for register MPCBB1_VCTR17"]
pub type W = crate::W<u32, super::MPCBB1_VCTR17>;
#[doc = "Register MPCBB1_VCTR17 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB1_VCTR17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B544`"]
pub type B544_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B544`"]
pub struct B544_W<'a> {
    w: &'a mut W,
}
impl<'a> B544_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B545`"]
pub type B545_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B545`"]
pub struct B545_W<'a> {
    w: &'a mut W,
}
impl<'a> B545_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B546`"]
pub type B546_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B546`"]
pub struct B546_W<'a> {
    w: &'a mut W,
}
impl<'a> B546_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B547`"]
pub type B547_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B547`"]
pub struct B547_W<'a> {
    w: &'a mut W,
}
impl<'a> B547_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B548`"]
pub type B548_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B548`"]
pub struct B548_W<'a> {
    w: &'a mut W,
}
impl<'a> B548_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B549`"]
pub type B549_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B549`"]
pub struct B549_W<'a> {
    w: &'a mut W,
}
impl<'a> B549_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B550`"]
pub type B550_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B550`"]
pub struct B550_W<'a> {
    w: &'a mut W,
}
impl<'a> B550_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B551`"]
pub type B551_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B551`"]
pub struct B551_W<'a> {
    w: &'a mut W,
}
impl<'a> B551_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B552`"]
pub type B552_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B552`"]
pub struct B552_W<'a> {
    w: &'a mut W,
}
impl<'a> B552_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B553`"]
pub type B553_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B553`"]
pub struct B553_W<'a> {
    w: &'a mut W,
}
impl<'a> B553_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B554`"]
pub type B554_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B554`"]
pub struct B554_W<'a> {
    w: &'a mut W,
}
impl<'a> B554_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B555`"]
pub type B555_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B555`"]
pub struct B555_W<'a> {
    w: &'a mut W,
}
impl<'a> B555_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B556`"]
pub type B556_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B556`"]
pub struct B556_W<'a> {
    w: &'a mut W,
}
impl<'a> B556_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B557`"]
pub type B557_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B557`"]
pub struct B557_W<'a> {
    w: &'a mut W,
}
impl<'a> B557_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B558`"]
pub type B558_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B558`"]
pub struct B558_W<'a> {
    w: &'a mut W,
}
impl<'a> B558_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B559`"]
pub type B559_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B559`"]
pub struct B559_W<'a> {
    w: &'a mut W,
}
impl<'a> B559_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B560`"]
pub type B560_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B560`"]
pub struct B560_W<'a> {
    w: &'a mut W,
}
impl<'a> B560_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B561`"]
pub type B561_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B561`"]
pub struct B561_W<'a> {
    w: &'a mut W,
}
impl<'a> B561_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B562`"]
pub type B562_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B562`"]
pub struct B562_W<'a> {
    w: &'a mut W,
}
impl<'a> B562_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B563`"]
pub type B563_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B563`"]
pub struct B563_W<'a> {
    w: &'a mut W,
}
impl<'a> B563_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B564`"]
pub type B564_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B564`"]
pub struct B564_W<'a> {
    w: &'a mut W,
}
impl<'a> B564_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B565`"]
pub type B565_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B565`"]
pub struct B565_W<'a> {
    w: &'a mut W,
}
impl<'a> B565_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B566`"]
pub type B566_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B566`"]
pub struct B566_W<'a> {
    w: &'a mut W,
}
impl<'a> B566_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B567`"]
pub type B567_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B567`"]
pub struct B567_W<'a> {
    w: &'a mut W,
}
impl<'a> B567_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B568`"]
pub type B568_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B568`"]
pub struct B568_W<'a> {
    w: &'a mut W,
}
impl<'a> B568_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B569`"]
pub type B569_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B569`"]
pub struct B569_W<'a> {
    w: &'a mut W,
}
impl<'a> B569_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B570`"]
pub type B570_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B570`"]
pub struct B570_W<'a> {
    w: &'a mut W,
}
impl<'a> B570_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B571`"]
pub type B571_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B571`"]
pub struct B571_W<'a> {
    w: &'a mut W,
}
impl<'a> B571_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B572`"]
pub type B572_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B572`"]
pub struct B572_W<'a> {
    w: &'a mut W,
}
impl<'a> B572_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B573`"]
pub type B573_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B573`"]
pub struct B573_W<'a> {
    w: &'a mut W,
}
impl<'a> B573_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B574`"]
pub type B574_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B574`"]
pub struct B574_W<'a> {
    w: &'a mut W,
}
impl<'a> B574_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B575`"]
pub type B575_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B575`"]
pub struct B575_W<'a> {
    w: &'a mut W,
}
impl<'a> B575_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B544"]
    #[inline(always)]
    pub fn b544(&self) -> B544_R {
        B544_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B545"]
    #[inline(always)]
    pub fn b545(&self) -> B545_R {
        B545_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B546"]
    #[inline(always)]
    pub fn b546(&self) -> B546_R {
        B546_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B547"]
    #[inline(always)]
    pub fn b547(&self) -> B547_R {
        B547_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B548"]
    #[inline(always)]
    pub fn b548(&self) -> B548_R {
        B548_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B549"]
    #[inline(always)]
    pub fn b549(&self) -> B549_R {
        B549_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B550"]
    #[inline(always)]
    pub fn b550(&self) -> B550_R {
        B550_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B551"]
    #[inline(always)]
    pub fn b551(&self) -> B551_R {
        B551_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B552"]
    #[inline(always)]
    pub fn b552(&self) -> B552_R {
        B552_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B553"]
    #[inline(always)]
    pub fn b553(&self) -> B553_R {
        B553_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B554"]
    #[inline(always)]
    pub fn b554(&self) -> B554_R {
        B554_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B555"]
    #[inline(always)]
    pub fn b555(&self) -> B555_R {
        B555_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B556"]
    #[inline(always)]
    pub fn b556(&self) -> B556_R {
        B556_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B557"]
    #[inline(always)]
    pub fn b557(&self) -> B557_R {
        B557_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B558"]
    #[inline(always)]
    pub fn b558(&self) -> B558_R {
        B558_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B559"]
    #[inline(always)]
    pub fn b559(&self) -> B559_R {
        B559_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B560"]
    #[inline(always)]
    pub fn b560(&self) -> B560_R {
        B560_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B561"]
    #[inline(always)]
    pub fn b561(&self) -> B561_R {
        B561_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B562"]
    #[inline(always)]
    pub fn b562(&self) -> B562_R {
        B562_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B563"]
    #[inline(always)]
    pub fn b563(&self) -> B563_R {
        B563_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B564"]
    #[inline(always)]
    pub fn b564(&self) -> B564_R {
        B564_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B565"]
    #[inline(always)]
    pub fn b565(&self) -> B565_R {
        B565_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B566"]
    #[inline(always)]
    pub fn b566(&self) -> B566_R {
        B566_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B567"]
    #[inline(always)]
    pub fn b567(&self) -> B567_R {
        B567_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B568"]
    #[inline(always)]
    pub fn b568(&self) -> B568_R {
        B568_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B569"]
    #[inline(always)]
    pub fn b569(&self) -> B569_R {
        B569_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B570"]
    #[inline(always)]
    pub fn b570(&self) -> B570_R {
        B570_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B571"]
    #[inline(always)]
    pub fn b571(&self) -> B571_R {
        B571_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B572"]
    #[inline(always)]
    pub fn b572(&self) -> B572_R {
        B572_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B573"]
    #[inline(always)]
    pub fn b573(&self) -> B573_R {
        B573_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B574"]
    #[inline(always)]
    pub fn b574(&self) -> B574_R {
        B574_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B575"]
    #[inline(always)]
    pub fn b575(&self) -> B575_R {
        B575_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B544"]
    #[inline(always)]
    pub fn b544(&mut self) -> B544_W {
        B544_W { w: self }
    }
    #[doc = "Bit 1 - B545"]
    #[inline(always)]
    pub fn b545(&mut self) -> B545_W {
        B545_W { w: self }
    }
    #[doc = "Bit 2 - B546"]
    #[inline(always)]
    pub fn b546(&mut self) -> B546_W {
        B546_W { w: self }
    }
    #[doc = "Bit 3 - B547"]
    #[inline(always)]
    pub fn b547(&mut self) -> B547_W {
        B547_W { w: self }
    }
    #[doc = "Bit 4 - B548"]
    #[inline(always)]
    pub fn b548(&mut self) -> B548_W {
        B548_W { w: self }
    }
    #[doc = "Bit 5 - B549"]
    #[inline(always)]
    pub fn b549(&mut self) -> B549_W {
        B549_W { w: self }
    }
    #[doc = "Bit 6 - B550"]
    #[inline(always)]
    pub fn b550(&mut self) -> B550_W {
        B550_W { w: self }
    }
    #[doc = "Bit 7 - B551"]
    #[inline(always)]
    pub fn b551(&mut self) -> B551_W {
        B551_W { w: self }
    }
    #[doc = "Bit 8 - B552"]
    #[inline(always)]
    pub fn b552(&mut self) -> B552_W {
        B552_W { w: self }
    }
    #[doc = "Bit 9 - B553"]
    #[inline(always)]
    pub fn b553(&mut self) -> B553_W {
        B553_W { w: self }
    }
    #[doc = "Bit 10 - B554"]
    #[inline(always)]
    pub fn b554(&mut self) -> B554_W {
        B554_W { w: self }
    }
    #[doc = "Bit 11 - B555"]
    #[inline(always)]
    pub fn b555(&mut self) -> B555_W {
        B555_W { w: self }
    }
    #[doc = "Bit 12 - B556"]
    #[inline(always)]
    pub fn b556(&mut self) -> B556_W {
        B556_W { w: self }
    }
    #[doc = "Bit 13 - B557"]
    #[inline(always)]
    pub fn b557(&mut self) -> B557_W {
        B557_W { w: self }
    }
    #[doc = "Bit 14 - B558"]
    #[inline(always)]
    pub fn b558(&mut self) -> B558_W {
        B558_W { w: self }
    }
    #[doc = "Bit 15 - B559"]
    #[inline(always)]
    pub fn b559(&mut self) -> B559_W {
        B559_W { w: self }
    }
    #[doc = "Bit 16 - B560"]
    #[inline(always)]
    pub fn b560(&mut self) -> B560_W {
        B560_W { w: self }
    }
    #[doc = "Bit 17 - B561"]
    #[inline(always)]
    pub fn b561(&mut self) -> B561_W {
        B561_W { w: self }
    }
    #[doc = "Bit 18 - B562"]
    #[inline(always)]
    pub fn b562(&mut self) -> B562_W {
        B562_W { w: self }
    }
    #[doc = "Bit 19 - B563"]
    #[inline(always)]
    pub fn b563(&mut self) -> B563_W {
        B563_W { w: self }
    }
    #[doc = "Bit 20 - B564"]
    #[inline(always)]
    pub fn b564(&mut self) -> B564_W {
        B564_W { w: self }
    }
    #[doc = "Bit 21 - B565"]
    #[inline(always)]
    pub fn b565(&mut self) -> B565_W {
        B565_W { w: self }
    }
    #[doc = "Bit 22 - B566"]
    #[inline(always)]
    pub fn b566(&mut self) -> B566_W {
        B566_W { w: self }
    }
    #[doc = "Bit 23 - B567"]
    #[inline(always)]
    pub fn b567(&mut self) -> B567_W {
        B567_W { w: self }
    }
    #[doc = "Bit 24 - B568"]
    #[inline(always)]
    pub fn b568(&mut self) -> B568_W {
        B568_W { w: self }
    }
    #[doc = "Bit 25 - B569"]
    #[inline(always)]
    pub fn b569(&mut self) -> B569_W {
        B569_W { w: self }
    }
    #[doc = "Bit 26 - B570"]
    #[inline(always)]
    pub fn b570(&mut self) -> B570_W {
        B570_W { w: self }
    }
    #[doc = "Bit 27 - B571"]
    #[inline(always)]
    pub fn b571(&mut self) -> B571_W {
        B571_W { w: self }
    }
    #[doc = "Bit 28 - B572"]
    #[inline(always)]
    pub fn b572(&mut self) -> B572_W {
        B572_W { w: self }
    }
    #[doc = "Bit 29 - B573"]
    #[inline(always)]
    pub fn b573(&mut self) -> B573_W {
        B573_W { w: self }
    }
    #[doc = "Bit 30 - B574"]
    #[inline(always)]
    pub fn b574(&mut self) -> B574_W {
        B574_W { w: self }
    }
    #[doc = "Bit 31 - B575"]
    #[inline(always)]
    pub fn b575(&mut self) -> B575_W {
        B575_W { w: self }
    }
}
