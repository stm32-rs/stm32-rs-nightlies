#[doc = "Reader of register MPCBB2_VCTR13"]
pub type R = crate::R<u32, super::MPCBB2_VCTR13>;
#[doc = "Writer for register MPCBB2_VCTR13"]
pub type W = crate::W<u32, super::MPCBB2_VCTR13>;
#[doc = "Register MPCBB2_VCTR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_VCTR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B416`"]
pub type B416_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B416`"]
pub struct B416_W<'a> {
    w: &'a mut W,
}
impl<'a> B416_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B417`"]
pub type B417_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B417`"]
pub struct B417_W<'a> {
    w: &'a mut W,
}
impl<'a> B417_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B418`"]
pub type B418_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B418`"]
pub struct B418_W<'a> {
    w: &'a mut W,
}
impl<'a> B418_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B419`"]
pub type B419_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B419`"]
pub struct B419_W<'a> {
    w: &'a mut W,
}
impl<'a> B419_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B420`"]
pub type B420_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B420`"]
pub struct B420_W<'a> {
    w: &'a mut W,
}
impl<'a> B420_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B421`"]
pub type B421_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B421`"]
pub struct B421_W<'a> {
    w: &'a mut W,
}
impl<'a> B421_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B422`"]
pub type B422_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B422`"]
pub struct B422_W<'a> {
    w: &'a mut W,
}
impl<'a> B422_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B423`"]
pub type B423_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B423`"]
pub struct B423_W<'a> {
    w: &'a mut W,
}
impl<'a> B423_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B424`"]
pub type B424_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B424`"]
pub struct B424_W<'a> {
    w: &'a mut W,
}
impl<'a> B424_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B425`"]
pub type B425_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B425`"]
pub struct B425_W<'a> {
    w: &'a mut W,
}
impl<'a> B425_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B426`"]
pub type B426_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B426`"]
pub struct B426_W<'a> {
    w: &'a mut W,
}
impl<'a> B426_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B427`"]
pub type B427_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B427`"]
pub struct B427_W<'a> {
    w: &'a mut W,
}
impl<'a> B427_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B428`"]
pub type B428_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B428`"]
pub struct B428_W<'a> {
    w: &'a mut W,
}
impl<'a> B428_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B429`"]
pub type B429_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B429`"]
pub struct B429_W<'a> {
    w: &'a mut W,
}
impl<'a> B429_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B430`"]
pub type B430_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B430`"]
pub struct B430_W<'a> {
    w: &'a mut W,
}
impl<'a> B430_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B431`"]
pub type B431_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B431`"]
pub struct B431_W<'a> {
    w: &'a mut W,
}
impl<'a> B431_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B432`"]
pub type B432_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B432`"]
pub struct B432_W<'a> {
    w: &'a mut W,
}
impl<'a> B432_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B433`"]
pub type B433_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B433`"]
pub struct B433_W<'a> {
    w: &'a mut W,
}
impl<'a> B433_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B434`"]
pub type B434_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B434`"]
pub struct B434_W<'a> {
    w: &'a mut W,
}
impl<'a> B434_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B435`"]
pub type B435_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B435`"]
pub struct B435_W<'a> {
    w: &'a mut W,
}
impl<'a> B435_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B436`"]
pub type B436_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B436`"]
pub struct B436_W<'a> {
    w: &'a mut W,
}
impl<'a> B436_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B437`"]
pub type B437_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B437`"]
pub struct B437_W<'a> {
    w: &'a mut W,
}
impl<'a> B437_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B438`"]
pub type B438_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B438`"]
pub struct B438_W<'a> {
    w: &'a mut W,
}
impl<'a> B438_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B439`"]
pub type B439_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B439`"]
pub struct B439_W<'a> {
    w: &'a mut W,
}
impl<'a> B439_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B440`"]
pub type B440_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B440`"]
pub struct B440_W<'a> {
    w: &'a mut W,
}
impl<'a> B440_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B441`"]
pub type B441_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B441`"]
pub struct B441_W<'a> {
    w: &'a mut W,
}
impl<'a> B441_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B442`"]
pub type B442_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B442`"]
pub struct B442_W<'a> {
    w: &'a mut W,
}
impl<'a> B442_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B443`"]
pub type B443_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B443`"]
pub struct B443_W<'a> {
    w: &'a mut W,
}
impl<'a> B443_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B444`"]
pub type B444_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B444`"]
pub struct B444_W<'a> {
    w: &'a mut W,
}
impl<'a> B444_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B445`"]
pub type B445_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B445`"]
pub struct B445_W<'a> {
    w: &'a mut W,
}
impl<'a> B445_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B446`"]
pub type B446_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B446`"]
pub struct B446_W<'a> {
    w: &'a mut W,
}
impl<'a> B446_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B447`"]
pub type B447_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B447`"]
pub struct B447_W<'a> {
    w: &'a mut W,
}
impl<'a> B447_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B416"]
    #[inline(always)]
    pub fn b416(&self) -> B416_R {
        B416_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B417"]
    #[inline(always)]
    pub fn b417(&self) -> B417_R {
        B417_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B418"]
    #[inline(always)]
    pub fn b418(&self) -> B418_R {
        B418_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B419"]
    #[inline(always)]
    pub fn b419(&self) -> B419_R {
        B419_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B420"]
    #[inline(always)]
    pub fn b420(&self) -> B420_R {
        B420_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B421"]
    #[inline(always)]
    pub fn b421(&self) -> B421_R {
        B421_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B422"]
    #[inline(always)]
    pub fn b422(&self) -> B422_R {
        B422_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B423"]
    #[inline(always)]
    pub fn b423(&self) -> B423_R {
        B423_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B424"]
    #[inline(always)]
    pub fn b424(&self) -> B424_R {
        B424_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B425"]
    #[inline(always)]
    pub fn b425(&self) -> B425_R {
        B425_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B426"]
    #[inline(always)]
    pub fn b426(&self) -> B426_R {
        B426_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B427"]
    #[inline(always)]
    pub fn b427(&self) -> B427_R {
        B427_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B428"]
    #[inline(always)]
    pub fn b428(&self) -> B428_R {
        B428_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B429"]
    #[inline(always)]
    pub fn b429(&self) -> B429_R {
        B429_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B430"]
    #[inline(always)]
    pub fn b430(&self) -> B430_R {
        B430_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B431"]
    #[inline(always)]
    pub fn b431(&self) -> B431_R {
        B431_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B432"]
    #[inline(always)]
    pub fn b432(&self) -> B432_R {
        B432_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B433"]
    #[inline(always)]
    pub fn b433(&self) -> B433_R {
        B433_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B434"]
    #[inline(always)]
    pub fn b434(&self) -> B434_R {
        B434_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B435"]
    #[inline(always)]
    pub fn b435(&self) -> B435_R {
        B435_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B436"]
    #[inline(always)]
    pub fn b436(&self) -> B436_R {
        B436_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B437"]
    #[inline(always)]
    pub fn b437(&self) -> B437_R {
        B437_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B438"]
    #[inline(always)]
    pub fn b438(&self) -> B438_R {
        B438_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B439"]
    #[inline(always)]
    pub fn b439(&self) -> B439_R {
        B439_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B440"]
    #[inline(always)]
    pub fn b440(&self) -> B440_R {
        B440_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B441"]
    #[inline(always)]
    pub fn b441(&self) -> B441_R {
        B441_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B442"]
    #[inline(always)]
    pub fn b442(&self) -> B442_R {
        B442_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B443"]
    #[inline(always)]
    pub fn b443(&self) -> B443_R {
        B443_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B444"]
    #[inline(always)]
    pub fn b444(&self) -> B444_R {
        B444_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B445"]
    #[inline(always)]
    pub fn b445(&self) -> B445_R {
        B445_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B446"]
    #[inline(always)]
    pub fn b446(&self) -> B446_R {
        B446_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B447"]
    #[inline(always)]
    pub fn b447(&self) -> B447_R {
        B447_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B416"]
    #[inline(always)]
    pub fn b416(&mut self) -> B416_W {
        B416_W { w: self }
    }
    #[doc = "Bit 1 - B417"]
    #[inline(always)]
    pub fn b417(&mut self) -> B417_W {
        B417_W { w: self }
    }
    #[doc = "Bit 2 - B418"]
    #[inline(always)]
    pub fn b418(&mut self) -> B418_W {
        B418_W { w: self }
    }
    #[doc = "Bit 3 - B419"]
    #[inline(always)]
    pub fn b419(&mut self) -> B419_W {
        B419_W { w: self }
    }
    #[doc = "Bit 4 - B420"]
    #[inline(always)]
    pub fn b420(&mut self) -> B420_W {
        B420_W { w: self }
    }
    #[doc = "Bit 5 - B421"]
    #[inline(always)]
    pub fn b421(&mut self) -> B421_W {
        B421_W { w: self }
    }
    #[doc = "Bit 6 - B422"]
    #[inline(always)]
    pub fn b422(&mut self) -> B422_W {
        B422_W { w: self }
    }
    #[doc = "Bit 7 - B423"]
    #[inline(always)]
    pub fn b423(&mut self) -> B423_W {
        B423_W { w: self }
    }
    #[doc = "Bit 8 - B424"]
    #[inline(always)]
    pub fn b424(&mut self) -> B424_W {
        B424_W { w: self }
    }
    #[doc = "Bit 9 - B425"]
    #[inline(always)]
    pub fn b425(&mut self) -> B425_W {
        B425_W { w: self }
    }
    #[doc = "Bit 10 - B426"]
    #[inline(always)]
    pub fn b426(&mut self) -> B426_W {
        B426_W { w: self }
    }
    #[doc = "Bit 11 - B427"]
    #[inline(always)]
    pub fn b427(&mut self) -> B427_W {
        B427_W { w: self }
    }
    #[doc = "Bit 12 - B428"]
    #[inline(always)]
    pub fn b428(&mut self) -> B428_W {
        B428_W { w: self }
    }
    #[doc = "Bit 13 - B429"]
    #[inline(always)]
    pub fn b429(&mut self) -> B429_W {
        B429_W { w: self }
    }
    #[doc = "Bit 14 - B430"]
    #[inline(always)]
    pub fn b430(&mut self) -> B430_W {
        B430_W { w: self }
    }
    #[doc = "Bit 15 - B431"]
    #[inline(always)]
    pub fn b431(&mut self) -> B431_W {
        B431_W { w: self }
    }
    #[doc = "Bit 16 - B432"]
    #[inline(always)]
    pub fn b432(&mut self) -> B432_W {
        B432_W { w: self }
    }
    #[doc = "Bit 17 - B433"]
    #[inline(always)]
    pub fn b433(&mut self) -> B433_W {
        B433_W { w: self }
    }
    #[doc = "Bit 18 - B434"]
    #[inline(always)]
    pub fn b434(&mut self) -> B434_W {
        B434_W { w: self }
    }
    #[doc = "Bit 19 - B435"]
    #[inline(always)]
    pub fn b435(&mut self) -> B435_W {
        B435_W { w: self }
    }
    #[doc = "Bit 20 - B436"]
    #[inline(always)]
    pub fn b436(&mut self) -> B436_W {
        B436_W { w: self }
    }
    #[doc = "Bit 21 - B437"]
    #[inline(always)]
    pub fn b437(&mut self) -> B437_W {
        B437_W { w: self }
    }
    #[doc = "Bit 22 - B438"]
    #[inline(always)]
    pub fn b438(&mut self) -> B438_W {
        B438_W { w: self }
    }
    #[doc = "Bit 23 - B439"]
    #[inline(always)]
    pub fn b439(&mut self) -> B439_W {
        B439_W { w: self }
    }
    #[doc = "Bit 24 - B440"]
    #[inline(always)]
    pub fn b440(&mut self) -> B440_W {
        B440_W { w: self }
    }
    #[doc = "Bit 25 - B441"]
    #[inline(always)]
    pub fn b441(&mut self) -> B441_W {
        B441_W { w: self }
    }
    #[doc = "Bit 26 - B442"]
    #[inline(always)]
    pub fn b442(&mut self) -> B442_W {
        B442_W { w: self }
    }
    #[doc = "Bit 27 - B443"]
    #[inline(always)]
    pub fn b443(&mut self) -> B443_W {
        B443_W { w: self }
    }
    #[doc = "Bit 28 - B444"]
    #[inline(always)]
    pub fn b444(&mut self) -> B444_W {
        B444_W { w: self }
    }
    #[doc = "Bit 29 - B445"]
    #[inline(always)]
    pub fn b445(&mut self) -> B445_W {
        B445_W { w: self }
    }
    #[doc = "Bit 30 - B446"]
    #[inline(always)]
    pub fn b446(&mut self) -> B446_W {
        B446_W { w: self }
    }
    #[doc = "Bit 31 - B447"]
    #[inline(always)]
    pub fn b447(&mut self) -> B447_W {
        B447_W { w: self }
    }
}
