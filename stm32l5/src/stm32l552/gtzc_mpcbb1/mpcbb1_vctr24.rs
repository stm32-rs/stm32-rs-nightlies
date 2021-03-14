#[doc = "Reader of register MPCBB1_VCTR24"]
pub type R = crate::R<u32, super::MPCBB1_VCTR24>;
#[doc = "Writer for register MPCBB1_VCTR24"]
pub type W = crate::W<u32, super::MPCBB1_VCTR24>;
#[doc = "Register MPCBB1_VCTR24 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MPCBB1_VCTR24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `B768`"]
pub type B768_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B768`"]
pub struct B768_W<'a> {
    w: &'a mut W,
}
impl<'a> B768_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B769`"]
pub type B769_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B769`"]
pub struct B769_W<'a> {
    w: &'a mut W,
}
impl<'a> B769_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B770`"]
pub type B770_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B770`"]
pub struct B770_W<'a> {
    w: &'a mut W,
}
impl<'a> B770_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B771`"]
pub type B771_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B771`"]
pub struct B771_W<'a> {
    w: &'a mut W,
}
impl<'a> B771_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B772`"]
pub type B772_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B772`"]
pub struct B772_W<'a> {
    w: &'a mut W,
}
impl<'a> B772_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B773`"]
pub type B773_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B773`"]
pub struct B773_W<'a> {
    w: &'a mut W,
}
impl<'a> B773_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B774`"]
pub type B774_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B774`"]
pub struct B774_W<'a> {
    w: &'a mut W,
}
impl<'a> B774_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B775`"]
pub type B775_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B775`"]
pub struct B775_W<'a> {
    w: &'a mut W,
}
impl<'a> B775_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B776`"]
pub type B776_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B776`"]
pub struct B776_W<'a> {
    w: &'a mut W,
}
impl<'a> B776_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B777`"]
pub type B777_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B777`"]
pub struct B777_W<'a> {
    w: &'a mut W,
}
impl<'a> B777_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B778`"]
pub type B778_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B778`"]
pub struct B778_W<'a> {
    w: &'a mut W,
}
impl<'a> B778_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B779`"]
pub type B779_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B779`"]
pub struct B779_W<'a> {
    w: &'a mut W,
}
impl<'a> B779_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B780`"]
pub type B780_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B780`"]
pub struct B780_W<'a> {
    w: &'a mut W,
}
impl<'a> B780_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B781`"]
pub type B781_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B781`"]
pub struct B781_W<'a> {
    w: &'a mut W,
}
impl<'a> B781_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B782`"]
pub type B782_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B782`"]
pub struct B782_W<'a> {
    w: &'a mut W,
}
impl<'a> B782_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B783`"]
pub type B783_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B783`"]
pub struct B783_W<'a> {
    w: &'a mut W,
}
impl<'a> B783_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B784`"]
pub type B784_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B784`"]
pub struct B784_W<'a> {
    w: &'a mut W,
}
impl<'a> B784_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B785`"]
pub type B785_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B785`"]
pub struct B785_W<'a> {
    w: &'a mut W,
}
impl<'a> B785_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B786`"]
pub type B786_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B786`"]
pub struct B786_W<'a> {
    w: &'a mut W,
}
impl<'a> B786_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B787`"]
pub type B787_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B787`"]
pub struct B787_W<'a> {
    w: &'a mut W,
}
impl<'a> B787_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B788`"]
pub type B788_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B788`"]
pub struct B788_W<'a> {
    w: &'a mut W,
}
impl<'a> B788_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B789`"]
pub type B789_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B789`"]
pub struct B789_W<'a> {
    w: &'a mut W,
}
impl<'a> B789_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B790`"]
pub type B790_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B790`"]
pub struct B790_W<'a> {
    w: &'a mut W,
}
impl<'a> B790_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B791`"]
pub type B791_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B791`"]
pub struct B791_W<'a> {
    w: &'a mut W,
}
impl<'a> B791_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B792`"]
pub type B792_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B792`"]
pub struct B792_W<'a> {
    w: &'a mut W,
}
impl<'a> B792_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B793`"]
pub type B793_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B793`"]
pub struct B793_W<'a> {
    w: &'a mut W,
}
impl<'a> B793_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B794`"]
pub type B794_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B794`"]
pub struct B794_W<'a> {
    w: &'a mut W,
}
impl<'a> B794_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B795`"]
pub type B795_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B795`"]
pub struct B795_W<'a> {
    w: &'a mut W,
}
impl<'a> B795_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B796`"]
pub type B796_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B796`"]
pub struct B796_W<'a> {
    w: &'a mut W,
}
impl<'a> B796_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B797`"]
pub type B797_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B797`"]
pub struct B797_W<'a> {
    w: &'a mut W,
}
impl<'a> B797_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B798`"]
pub type B798_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B798`"]
pub struct B798_W<'a> {
    w: &'a mut W,
}
impl<'a> B798_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `B799`"]
pub type B799_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `B799`"]
pub struct B799_W<'a> {
    w: &'a mut W,
}
impl<'a> B799_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B768"]
    #[inline(always)]
    pub fn b768(&self) -> B768_R {
        B768_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B769"]
    #[inline(always)]
    pub fn b769(&self) -> B769_R {
        B769_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B770"]
    #[inline(always)]
    pub fn b770(&self) -> B770_R {
        B770_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B771"]
    #[inline(always)]
    pub fn b771(&self) -> B771_R {
        B771_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B772"]
    #[inline(always)]
    pub fn b772(&self) -> B772_R {
        B772_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B773"]
    #[inline(always)]
    pub fn b773(&self) -> B773_R {
        B773_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B774"]
    #[inline(always)]
    pub fn b774(&self) -> B774_R {
        B774_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B775"]
    #[inline(always)]
    pub fn b775(&self) -> B775_R {
        B775_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B776"]
    #[inline(always)]
    pub fn b776(&self) -> B776_R {
        B776_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B777"]
    #[inline(always)]
    pub fn b777(&self) -> B777_R {
        B777_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B778"]
    #[inline(always)]
    pub fn b778(&self) -> B778_R {
        B778_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B779"]
    #[inline(always)]
    pub fn b779(&self) -> B779_R {
        B779_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B780"]
    #[inline(always)]
    pub fn b780(&self) -> B780_R {
        B780_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B781"]
    #[inline(always)]
    pub fn b781(&self) -> B781_R {
        B781_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B782"]
    #[inline(always)]
    pub fn b782(&self) -> B782_R {
        B782_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B783"]
    #[inline(always)]
    pub fn b783(&self) -> B783_R {
        B783_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B784"]
    #[inline(always)]
    pub fn b784(&self) -> B784_R {
        B784_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B785"]
    #[inline(always)]
    pub fn b785(&self) -> B785_R {
        B785_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B786"]
    #[inline(always)]
    pub fn b786(&self) -> B786_R {
        B786_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B787"]
    #[inline(always)]
    pub fn b787(&self) -> B787_R {
        B787_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B788"]
    #[inline(always)]
    pub fn b788(&self) -> B788_R {
        B788_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B789"]
    #[inline(always)]
    pub fn b789(&self) -> B789_R {
        B789_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B790"]
    #[inline(always)]
    pub fn b790(&self) -> B790_R {
        B790_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B791"]
    #[inline(always)]
    pub fn b791(&self) -> B791_R {
        B791_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B792"]
    #[inline(always)]
    pub fn b792(&self) -> B792_R {
        B792_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B793"]
    #[inline(always)]
    pub fn b793(&self) -> B793_R {
        B793_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B794"]
    #[inline(always)]
    pub fn b794(&self) -> B794_R {
        B794_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B795"]
    #[inline(always)]
    pub fn b795(&self) -> B795_R {
        B795_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B796"]
    #[inline(always)]
    pub fn b796(&self) -> B796_R {
        B796_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B797"]
    #[inline(always)]
    pub fn b797(&self) -> B797_R {
        B797_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B798"]
    #[inline(always)]
    pub fn b798(&self) -> B798_R {
        B798_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B799"]
    #[inline(always)]
    pub fn b799(&self) -> B799_R {
        B799_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B768"]
    #[inline(always)]
    pub fn b768(&mut self) -> B768_W {
        B768_W { w: self }
    }
    #[doc = "Bit 1 - B769"]
    #[inline(always)]
    pub fn b769(&mut self) -> B769_W {
        B769_W { w: self }
    }
    #[doc = "Bit 2 - B770"]
    #[inline(always)]
    pub fn b770(&mut self) -> B770_W {
        B770_W { w: self }
    }
    #[doc = "Bit 3 - B771"]
    #[inline(always)]
    pub fn b771(&mut self) -> B771_W {
        B771_W { w: self }
    }
    #[doc = "Bit 4 - B772"]
    #[inline(always)]
    pub fn b772(&mut self) -> B772_W {
        B772_W { w: self }
    }
    #[doc = "Bit 5 - B773"]
    #[inline(always)]
    pub fn b773(&mut self) -> B773_W {
        B773_W { w: self }
    }
    #[doc = "Bit 6 - B774"]
    #[inline(always)]
    pub fn b774(&mut self) -> B774_W {
        B774_W { w: self }
    }
    #[doc = "Bit 7 - B775"]
    #[inline(always)]
    pub fn b775(&mut self) -> B775_W {
        B775_W { w: self }
    }
    #[doc = "Bit 8 - B776"]
    #[inline(always)]
    pub fn b776(&mut self) -> B776_W {
        B776_W { w: self }
    }
    #[doc = "Bit 9 - B777"]
    #[inline(always)]
    pub fn b777(&mut self) -> B777_W {
        B777_W { w: self }
    }
    #[doc = "Bit 10 - B778"]
    #[inline(always)]
    pub fn b778(&mut self) -> B778_W {
        B778_W { w: self }
    }
    #[doc = "Bit 11 - B779"]
    #[inline(always)]
    pub fn b779(&mut self) -> B779_W {
        B779_W { w: self }
    }
    #[doc = "Bit 12 - B780"]
    #[inline(always)]
    pub fn b780(&mut self) -> B780_W {
        B780_W { w: self }
    }
    #[doc = "Bit 13 - B781"]
    #[inline(always)]
    pub fn b781(&mut self) -> B781_W {
        B781_W { w: self }
    }
    #[doc = "Bit 14 - B782"]
    #[inline(always)]
    pub fn b782(&mut self) -> B782_W {
        B782_W { w: self }
    }
    #[doc = "Bit 15 - B783"]
    #[inline(always)]
    pub fn b783(&mut self) -> B783_W {
        B783_W { w: self }
    }
    #[doc = "Bit 16 - B784"]
    #[inline(always)]
    pub fn b784(&mut self) -> B784_W {
        B784_W { w: self }
    }
    #[doc = "Bit 17 - B785"]
    #[inline(always)]
    pub fn b785(&mut self) -> B785_W {
        B785_W { w: self }
    }
    #[doc = "Bit 18 - B786"]
    #[inline(always)]
    pub fn b786(&mut self) -> B786_W {
        B786_W { w: self }
    }
    #[doc = "Bit 19 - B787"]
    #[inline(always)]
    pub fn b787(&mut self) -> B787_W {
        B787_W { w: self }
    }
    #[doc = "Bit 20 - B788"]
    #[inline(always)]
    pub fn b788(&mut self) -> B788_W {
        B788_W { w: self }
    }
    #[doc = "Bit 21 - B789"]
    #[inline(always)]
    pub fn b789(&mut self) -> B789_W {
        B789_W { w: self }
    }
    #[doc = "Bit 22 - B790"]
    #[inline(always)]
    pub fn b790(&mut self) -> B790_W {
        B790_W { w: self }
    }
    #[doc = "Bit 23 - B791"]
    #[inline(always)]
    pub fn b791(&mut self) -> B791_W {
        B791_W { w: self }
    }
    #[doc = "Bit 24 - B792"]
    #[inline(always)]
    pub fn b792(&mut self) -> B792_W {
        B792_W { w: self }
    }
    #[doc = "Bit 25 - B793"]
    #[inline(always)]
    pub fn b793(&mut self) -> B793_W {
        B793_W { w: self }
    }
    #[doc = "Bit 26 - B794"]
    #[inline(always)]
    pub fn b794(&mut self) -> B794_W {
        B794_W { w: self }
    }
    #[doc = "Bit 27 - B795"]
    #[inline(always)]
    pub fn b795(&mut self) -> B795_W {
        B795_W { w: self }
    }
    #[doc = "Bit 28 - B796"]
    #[inline(always)]
    pub fn b796(&mut self) -> B796_W {
        B796_W { w: self }
    }
    #[doc = "Bit 29 - B797"]
    #[inline(always)]
    pub fn b797(&mut self) -> B797_W {
        B797_W { w: self }
    }
    #[doc = "Bit 30 - B798"]
    #[inline(always)]
    pub fn b798(&mut self) -> B798_W {
        B798_W { w: self }
    }
    #[doc = "Bit 31 - B799"]
    #[inline(always)]
    pub fn b799(&mut self) -> B799_W {
        B799_W { w: self }
    }
}
