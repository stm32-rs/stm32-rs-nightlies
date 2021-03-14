#[doc = "Reader of register MPCBB2_VCTR28"]
pub type R = crate::R<u32, super::MPCBB2_VCTR28>;
#[doc = "Writer for register MPCBB2_VCTR28"]
pub type W = crate::W<u32, super::MPCBB2_VCTR28>;
#[doc = "Register MPCBB2_VCTR28 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B896`"]
pub type B896_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B896`"]
pub struct B896_W<'a> {
    w: &'a mut W,
}
impl<'a> B896_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B897`"]
pub type B897_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B897`"]
pub struct B897_W<'a> {
    w: &'a mut W,
}
impl<'a> B897_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B898`"]
pub type B898_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B898`"]
pub struct B898_W<'a> {
    w: &'a mut W,
}
impl<'a> B898_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B899`"]
pub type B899_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B899`"]
pub struct B899_W<'a> {
    w: &'a mut W,
}
impl<'a> B899_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B900`"]
pub type B900_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B900`"]
pub struct B900_W<'a> {
    w: &'a mut W,
}
impl<'a> B900_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B901`"]
pub type B901_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B901`"]
pub struct B901_W<'a> {
    w: &'a mut W,
}
impl<'a> B901_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B902`"]
pub type B902_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B902`"]
pub struct B902_W<'a> {
    w: &'a mut W,
}
impl<'a> B902_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B903`"]
pub type B903_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B903`"]
pub struct B903_W<'a> {
    w: &'a mut W,
}
impl<'a> B903_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B904`"]
pub type B904_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B904`"]
pub struct B904_W<'a> {
    w: &'a mut W,
}
impl<'a> B904_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B905`"]
pub type B905_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B905`"]
pub struct B905_W<'a> {
    w: &'a mut W,
}
impl<'a> B905_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B906`"]
pub type B906_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B906`"]
pub struct B906_W<'a> {
    w: &'a mut W,
}
impl<'a> B906_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B907`"]
pub type B907_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B907`"]
pub struct B907_W<'a> {
    w: &'a mut W,
}
impl<'a> B907_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B908`"]
pub type B908_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B908`"]
pub struct B908_W<'a> {
    w: &'a mut W,
}
impl<'a> B908_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B909`"]
pub type B909_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B909`"]
pub struct B909_W<'a> {
    w: &'a mut W,
}
impl<'a> B909_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B910`"]
pub type B910_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B910`"]
pub struct B910_W<'a> {
    w: &'a mut W,
}
impl<'a> B910_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B911`"]
pub type B911_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B911`"]
pub struct B911_W<'a> {
    w: &'a mut W,
}
impl<'a> B911_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B912`"]
pub type B912_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B912`"]
pub struct B912_W<'a> {
    w: &'a mut W,
}
impl<'a> B912_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B913`"]
pub type B913_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B913`"]
pub struct B913_W<'a> {
    w: &'a mut W,
}
impl<'a> B913_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B914`"]
pub type B914_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B914`"]
pub struct B914_W<'a> {
    w: &'a mut W,
}
impl<'a> B914_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B915`"]
pub type B915_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B915`"]
pub struct B915_W<'a> {
    w: &'a mut W,
}
impl<'a> B915_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B916`"]
pub type B916_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B916`"]
pub struct B916_W<'a> {
    w: &'a mut W,
}
impl<'a> B916_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B917`"]
pub type B917_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B917`"]
pub struct B917_W<'a> {
    w: &'a mut W,
}
impl<'a> B917_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B918`"]
pub type B918_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B918`"]
pub struct B918_W<'a> {
    w: &'a mut W,
}
impl<'a> B918_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B919`"]
pub type B919_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B919`"]
pub struct B919_W<'a> {
    w: &'a mut W,
}
impl<'a> B919_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B920`"]
pub type B920_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B920`"]
pub struct B920_W<'a> {
    w: &'a mut W,
}
impl<'a> B920_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B921`"]
pub type B921_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B921`"]
pub struct B921_W<'a> {
    w: &'a mut W,
}
impl<'a> B921_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B922`"]
pub type B922_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B922`"]
pub struct B922_W<'a> {
    w: &'a mut W,
}
impl<'a> B922_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B923`"]
pub type B923_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B923`"]
pub struct B923_W<'a> {
    w: &'a mut W,
}
impl<'a> B923_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B924`"]
pub type B924_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B924`"]
pub struct B924_W<'a> {
    w: &'a mut W,
}
impl<'a> B924_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B925`"]
pub type B925_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B925`"]
pub struct B925_W<'a> {
    w: &'a mut W,
}
impl<'a> B925_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B926`"]
pub type B926_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B926`"]
pub struct B926_W<'a> {
    w: &'a mut W,
}
impl<'a> B926_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B927`"]
pub type B927_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B927`"]
pub struct B927_W<'a> {
    w: &'a mut W,
}
impl<'a> B927_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B896"]
    #[inline(always)]
    pub fn b896(&self) -> B896_R {
        B896_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B897"]
    #[inline(always)]
    pub fn b897(&self) -> B897_R {
        B897_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B898"]
    #[inline(always)]
    pub fn b898(&self) -> B898_R {
        B898_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B899"]
    #[inline(always)]
    pub fn b899(&self) -> B899_R {
        B899_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B900"]
    #[inline(always)]
    pub fn b900(&self) -> B900_R {
        B900_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B901"]
    #[inline(always)]
    pub fn b901(&self) -> B901_R {
        B901_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B902"]
    #[inline(always)]
    pub fn b902(&self) -> B902_R {
        B902_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B903"]
    #[inline(always)]
    pub fn b903(&self) -> B903_R {
        B903_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B904"]
    #[inline(always)]
    pub fn b904(&self) -> B904_R {
        B904_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B905"]
    #[inline(always)]
    pub fn b905(&self) -> B905_R {
        B905_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B906"]
    #[inline(always)]
    pub fn b906(&self) -> B906_R {
        B906_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B907"]
    #[inline(always)]
    pub fn b907(&self) -> B907_R {
        B907_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B908"]
    #[inline(always)]
    pub fn b908(&self) -> B908_R {
        B908_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B909"]
    #[inline(always)]
    pub fn b909(&self) -> B909_R {
        B909_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B910"]
    #[inline(always)]
    pub fn b910(&self) -> B910_R {
        B910_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B911"]
    #[inline(always)]
    pub fn b911(&self) -> B911_R {
        B911_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B912"]
    #[inline(always)]
    pub fn b912(&self) -> B912_R {
        B912_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B913"]
    #[inline(always)]
    pub fn b913(&self) -> B913_R {
        B913_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B914"]
    #[inline(always)]
    pub fn b914(&self) -> B914_R {
        B914_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B915"]
    #[inline(always)]
    pub fn b915(&self) -> B915_R {
        B915_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B916"]
    #[inline(always)]
    pub fn b916(&self) -> B916_R {
        B916_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B917"]
    #[inline(always)]
    pub fn b917(&self) -> B917_R {
        B917_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B918"]
    #[inline(always)]
    pub fn b918(&self) -> B918_R {
        B918_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B919"]
    #[inline(always)]
    pub fn b919(&self) -> B919_R {
        B919_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B920"]
    #[inline(always)]
    pub fn b920(&self) -> B920_R {
        B920_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B921"]
    #[inline(always)]
    pub fn b921(&self) -> B921_R {
        B921_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B922"]
    #[inline(always)]
    pub fn b922(&self) -> B922_R {
        B922_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B923"]
    #[inline(always)]
    pub fn b923(&self) -> B923_R {
        B923_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B924"]
    #[inline(always)]
    pub fn b924(&self) -> B924_R {
        B924_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B925"]
    #[inline(always)]
    pub fn b925(&self) -> B925_R {
        B925_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B926"]
    #[inline(always)]
    pub fn b926(&self) -> B926_R {
        B926_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B927"]
    #[inline(always)]
    pub fn b927(&self) -> B927_R {
        B927_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B896"]
    #[inline(always)]
    pub fn b896(&mut self) -> B896_W {
        B896_W { w: self }
    }
    #[doc = "Bit 1 - B897"]
    #[inline(always)]
    pub fn b897(&mut self) -> B897_W {
        B897_W { w: self }
    }
    #[doc = "Bit 2 - B898"]
    #[inline(always)]
    pub fn b898(&mut self) -> B898_W {
        B898_W { w: self }
    }
    #[doc = "Bit 3 - B899"]
    #[inline(always)]
    pub fn b899(&mut self) -> B899_W {
        B899_W { w: self }
    }
    #[doc = "Bit 4 - B900"]
    #[inline(always)]
    pub fn b900(&mut self) -> B900_W {
        B900_W { w: self }
    }
    #[doc = "Bit 5 - B901"]
    #[inline(always)]
    pub fn b901(&mut self) -> B901_W {
        B901_W { w: self }
    }
    #[doc = "Bit 6 - B902"]
    #[inline(always)]
    pub fn b902(&mut self) -> B902_W {
        B902_W { w: self }
    }
    #[doc = "Bit 7 - B903"]
    #[inline(always)]
    pub fn b903(&mut self) -> B903_W {
        B903_W { w: self }
    }
    #[doc = "Bit 8 - B904"]
    #[inline(always)]
    pub fn b904(&mut self) -> B904_W {
        B904_W { w: self }
    }
    #[doc = "Bit 9 - B905"]
    #[inline(always)]
    pub fn b905(&mut self) -> B905_W {
        B905_W { w: self }
    }
    #[doc = "Bit 10 - B906"]
    #[inline(always)]
    pub fn b906(&mut self) -> B906_W {
        B906_W { w: self }
    }
    #[doc = "Bit 11 - B907"]
    #[inline(always)]
    pub fn b907(&mut self) -> B907_W {
        B907_W { w: self }
    }
    #[doc = "Bit 12 - B908"]
    #[inline(always)]
    pub fn b908(&mut self) -> B908_W {
        B908_W { w: self }
    }
    #[doc = "Bit 13 - B909"]
    #[inline(always)]
    pub fn b909(&mut self) -> B909_W {
        B909_W { w: self }
    }
    #[doc = "Bit 14 - B910"]
    #[inline(always)]
    pub fn b910(&mut self) -> B910_W {
        B910_W { w: self }
    }
    #[doc = "Bit 15 - B911"]
    #[inline(always)]
    pub fn b911(&mut self) -> B911_W {
        B911_W { w: self }
    }
    #[doc = "Bit 16 - B912"]
    #[inline(always)]
    pub fn b912(&mut self) -> B912_W {
        B912_W { w: self }
    }
    #[doc = "Bit 17 - B913"]
    #[inline(always)]
    pub fn b913(&mut self) -> B913_W {
        B913_W { w: self }
    }
    #[doc = "Bit 18 - B914"]
    #[inline(always)]
    pub fn b914(&mut self) -> B914_W {
        B914_W { w: self }
    }
    #[doc = "Bit 19 - B915"]
    #[inline(always)]
    pub fn b915(&mut self) -> B915_W {
        B915_W { w: self }
    }
    #[doc = "Bit 20 - B916"]
    #[inline(always)]
    pub fn b916(&mut self) -> B916_W {
        B916_W { w: self }
    }
    #[doc = "Bit 21 - B917"]
    #[inline(always)]
    pub fn b917(&mut self) -> B917_W {
        B917_W { w: self }
    }
    #[doc = "Bit 22 - B918"]
    #[inline(always)]
    pub fn b918(&mut self) -> B918_W {
        B918_W { w: self }
    }
    #[doc = "Bit 23 - B919"]
    #[inline(always)]
    pub fn b919(&mut self) -> B919_W {
        B919_W { w: self }
    }
    #[doc = "Bit 24 - B920"]
    #[inline(always)]
    pub fn b920(&mut self) -> B920_W {
        B920_W { w: self }
    }
    #[doc = "Bit 25 - B921"]
    #[inline(always)]
    pub fn b921(&mut self) -> B921_W {
        B921_W { w: self }
    }
    #[doc = "Bit 26 - B922"]
    #[inline(always)]
    pub fn b922(&mut self) -> B922_W {
        B922_W { w: self }
    }
    #[doc = "Bit 27 - B923"]
    #[inline(always)]
    pub fn b923(&mut self) -> B923_W {
        B923_W { w: self }
    }
    #[doc = "Bit 28 - B924"]
    #[inline(always)]
    pub fn b924(&mut self) -> B924_W {
        B924_W { w: self }
    }
    #[doc = "Bit 29 - B925"]
    #[inline(always)]
    pub fn b925(&mut self) -> B925_W {
        B925_W { w: self }
    }
    #[doc = "Bit 30 - B926"]
    #[inline(always)]
    pub fn b926(&mut self) -> B926_W {
        B926_W { w: self }
    }
    #[doc = "Bit 31 - B927"]
    #[inline(always)]
    pub fn b927(&mut self) -> B927_W {
        B927_W { w: self }
    }
}
