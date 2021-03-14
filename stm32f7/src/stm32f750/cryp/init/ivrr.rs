#[doc = "Reader of register IVRR"]
pub type R = crate::R<u32, super::IVRR>;
#[doc = "Writer for register IVRR"]
pub type W = crate::W<u32, super::IVRR>;
#[doc = "Register IVRR `reset()`'s with value 0"]
impl crate::ResetValue for super::IVRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV63`"]
pub type IV63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV63`"]
pub struct IV63_W<'a> {
    w: &'a mut W,
}
impl<'a> IV63_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV62`"]
pub type IV62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV62`"]
pub struct IV62_W<'a> {
    w: &'a mut W,
}
impl<'a> IV62_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV61`"]
pub type IV61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV61`"]
pub struct IV61_W<'a> {
    w: &'a mut W,
}
impl<'a> IV61_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV60`"]
pub type IV60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV60`"]
pub struct IV60_W<'a> {
    w: &'a mut W,
}
impl<'a> IV60_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV59`"]
pub type IV59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV59`"]
pub struct IV59_W<'a> {
    w: &'a mut W,
}
impl<'a> IV59_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV58`"]
pub type IV58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV58`"]
pub struct IV58_W<'a> {
    w: &'a mut W,
}
impl<'a> IV58_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV57`"]
pub type IV57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV57`"]
pub struct IV57_W<'a> {
    w: &'a mut W,
}
impl<'a> IV57_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV56`"]
pub type IV56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV56`"]
pub struct IV56_W<'a> {
    w: &'a mut W,
}
impl<'a> IV56_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV55`"]
pub type IV55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV55`"]
pub struct IV55_W<'a> {
    w: &'a mut W,
}
impl<'a> IV55_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV54`"]
pub type IV54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV54`"]
pub struct IV54_W<'a> {
    w: &'a mut W,
}
impl<'a> IV54_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV53`"]
pub type IV53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV53`"]
pub struct IV53_W<'a> {
    w: &'a mut W,
}
impl<'a> IV53_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV52`"]
pub type IV52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV52`"]
pub struct IV52_W<'a> {
    w: &'a mut W,
}
impl<'a> IV52_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV51`"]
pub type IV51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV51`"]
pub struct IV51_W<'a> {
    w: &'a mut W,
}
impl<'a> IV51_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV50`"]
pub type IV50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV50`"]
pub struct IV50_W<'a> {
    w: &'a mut W,
}
impl<'a> IV50_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV49`"]
pub type IV49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV49`"]
pub struct IV49_W<'a> {
    w: &'a mut W,
}
impl<'a> IV49_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV48`"]
pub type IV48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV48`"]
pub struct IV48_W<'a> {
    w: &'a mut W,
}
impl<'a> IV48_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV47`"]
pub type IV47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV47`"]
pub struct IV47_W<'a> {
    w: &'a mut W,
}
impl<'a> IV47_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV46`"]
pub type IV46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV46`"]
pub struct IV46_W<'a> {
    w: &'a mut W,
}
impl<'a> IV46_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV45`"]
pub type IV45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV45`"]
pub struct IV45_W<'a> {
    w: &'a mut W,
}
impl<'a> IV45_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV44`"]
pub type IV44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV44`"]
pub struct IV44_W<'a> {
    w: &'a mut W,
}
impl<'a> IV44_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV43`"]
pub type IV43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV43`"]
pub struct IV43_W<'a> {
    w: &'a mut W,
}
impl<'a> IV43_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV42`"]
pub type IV42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV42`"]
pub struct IV42_W<'a> {
    w: &'a mut W,
}
impl<'a> IV42_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV41`"]
pub type IV41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV41`"]
pub struct IV41_W<'a> {
    w: &'a mut W,
}
impl<'a> IV41_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV40`"]
pub type IV40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV40`"]
pub struct IV40_W<'a> {
    w: &'a mut W,
}
impl<'a> IV40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV39`"]
pub type IV39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV39`"]
pub struct IV39_W<'a> {
    w: &'a mut W,
}
impl<'a> IV39_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV38`"]
pub type IV38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV38`"]
pub struct IV38_W<'a> {
    w: &'a mut W,
}
impl<'a> IV38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV37`"]
pub type IV37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV37`"]
pub struct IV37_W<'a> {
    w: &'a mut W,
}
impl<'a> IV37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV36`"]
pub type IV36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV36`"]
pub struct IV36_W<'a> {
    w: &'a mut W,
}
impl<'a> IV36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV35`"]
pub type IV35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV35`"]
pub struct IV35_W<'a> {
    w: &'a mut W,
}
impl<'a> IV35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV34`"]
pub type IV34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV34`"]
pub struct IV34_W<'a> {
    w: &'a mut W,
}
impl<'a> IV34_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV33`"]
pub type IV33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV33`"]
pub struct IV33_W<'a> {
    w: &'a mut W,
}
impl<'a> IV33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IV32`"]
pub type IV32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IV32`"]
pub struct IV32_W<'a> {
    w: &'a mut W,
}
impl<'a> IV32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IV63"]
    #[inline(always)]
    pub fn iv63(&self) -> IV63_R {
        IV63_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IV62"]
    #[inline(always)]
    pub fn iv62(&self) -> IV62_R {
        IV62_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IV61"]
    #[inline(always)]
    pub fn iv61(&self) -> IV61_R {
        IV61_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IV60"]
    #[inline(always)]
    pub fn iv60(&self) -> IV60_R {
        IV60_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IV59"]
    #[inline(always)]
    pub fn iv59(&self) -> IV59_R {
        IV59_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IV58"]
    #[inline(always)]
    pub fn iv58(&self) -> IV58_R {
        IV58_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IV57"]
    #[inline(always)]
    pub fn iv57(&self) -> IV57_R {
        IV57_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IV56"]
    #[inline(always)]
    pub fn iv56(&self) -> IV56_R {
        IV56_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IV55"]
    #[inline(always)]
    pub fn iv55(&self) -> IV55_R {
        IV55_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IV54"]
    #[inline(always)]
    pub fn iv54(&self) -> IV54_R {
        IV54_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IV53"]
    #[inline(always)]
    pub fn iv53(&self) -> IV53_R {
        IV53_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IV52"]
    #[inline(always)]
    pub fn iv52(&self) -> IV52_R {
        IV52_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IV51"]
    #[inline(always)]
    pub fn iv51(&self) -> IV51_R {
        IV51_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IV50"]
    #[inline(always)]
    pub fn iv50(&self) -> IV50_R {
        IV50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IV49"]
    #[inline(always)]
    pub fn iv49(&self) -> IV49_R {
        IV49_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IV48"]
    #[inline(always)]
    pub fn iv48(&self) -> IV48_R {
        IV48_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IV47"]
    #[inline(always)]
    pub fn iv47(&self) -> IV47_R {
        IV47_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IV46"]
    #[inline(always)]
    pub fn iv46(&self) -> IV46_R {
        IV46_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IV45"]
    #[inline(always)]
    pub fn iv45(&self) -> IV45_R {
        IV45_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IV44"]
    #[inline(always)]
    pub fn iv44(&self) -> IV44_R {
        IV44_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IV43"]
    #[inline(always)]
    pub fn iv43(&self) -> IV43_R {
        IV43_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IV42"]
    #[inline(always)]
    pub fn iv42(&self) -> IV42_R {
        IV42_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IV41"]
    #[inline(always)]
    pub fn iv41(&self) -> IV41_R {
        IV41_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IV40"]
    #[inline(always)]
    pub fn iv40(&self) -> IV40_R {
        IV40_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IV39"]
    #[inline(always)]
    pub fn iv39(&self) -> IV39_R {
        IV39_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IV38"]
    #[inline(always)]
    pub fn iv38(&self) -> IV38_R {
        IV38_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IV37"]
    #[inline(always)]
    pub fn iv37(&self) -> IV37_R {
        IV37_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IV36"]
    #[inline(always)]
    pub fn iv36(&self) -> IV36_R {
        IV36_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IV35"]
    #[inline(always)]
    pub fn iv35(&self) -> IV35_R {
        IV35_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IV34"]
    #[inline(always)]
    pub fn iv34(&self) -> IV34_R {
        IV34_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IV33"]
    #[inline(always)]
    pub fn iv33(&self) -> IV33_R {
        IV33_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IV32"]
    #[inline(always)]
    pub fn iv32(&self) -> IV32_R {
        IV32_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IV63"]
    #[inline(always)]
    pub fn iv63(&mut self) -> IV63_W {
        IV63_W { w: self }
    }
    #[doc = "Bit 1 - IV62"]
    #[inline(always)]
    pub fn iv62(&mut self) -> IV62_W {
        IV62_W { w: self }
    }
    #[doc = "Bit 2 - IV61"]
    #[inline(always)]
    pub fn iv61(&mut self) -> IV61_W {
        IV61_W { w: self }
    }
    #[doc = "Bit 3 - IV60"]
    #[inline(always)]
    pub fn iv60(&mut self) -> IV60_W {
        IV60_W { w: self }
    }
    #[doc = "Bit 4 - IV59"]
    #[inline(always)]
    pub fn iv59(&mut self) -> IV59_W {
        IV59_W { w: self }
    }
    #[doc = "Bit 5 - IV58"]
    #[inline(always)]
    pub fn iv58(&mut self) -> IV58_W {
        IV58_W { w: self }
    }
    #[doc = "Bit 6 - IV57"]
    #[inline(always)]
    pub fn iv57(&mut self) -> IV57_W {
        IV57_W { w: self }
    }
    #[doc = "Bit 7 - IV56"]
    #[inline(always)]
    pub fn iv56(&mut self) -> IV56_W {
        IV56_W { w: self }
    }
    #[doc = "Bit 8 - IV55"]
    #[inline(always)]
    pub fn iv55(&mut self) -> IV55_W {
        IV55_W { w: self }
    }
    #[doc = "Bit 9 - IV54"]
    #[inline(always)]
    pub fn iv54(&mut self) -> IV54_W {
        IV54_W { w: self }
    }
    #[doc = "Bit 10 - IV53"]
    #[inline(always)]
    pub fn iv53(&mut self) -> IV53_W {
        IV53_W { w: self }
    }
    #[doc = "Bit 11 - IV52"]
    #[inline(always)]
    pub fn iv52(&mut self) -> IV52_W {
        IV52_W { w: self }
    }
    #[doc = "Bit 12 - IV51"]
    #[inline(always)]
    pub fn iv51(&mut self) -> IV51_W {
        IV51_W { w: self }
    }
    #[doc = "Bit 13 - IV50"]
    #[inline(always)]
    pub fn iv50(&mut self) -> IV50_W {
        IV50_W { w: self }
    }
    #[doc = "Bit 14 - IV49"]
    #[inline(always)]
    pub fn iv49(&mut self) -> IV49_W {
        IV49_W { w: self }
    }
    #[doc = "Bit 15 - IV48"]
    #[inline(always)]
    pub fn iv48(&mut self) -> IV48_W {
        IV48_W { w: self }
    }
    #[doc = "Bit 16 - IV47"]
    #[inline(always)]
    pub fn iv47(&mut self) -> IV47_W {
        IV47_W { w: self }
    }
    #[doc = "Bit 17 - IV46"]
    #[inline(always)]
    pub fn iv46(&mut self) -> IV46_W {
        IV46_W { w: self }
    }
    #[doc = "Bit 18 - IV45"]
    #[inline(always)]
    pub fn iv45(&mut self) -> IV45_W {
        IV45_W { w: self }
    }
    #[doc = "Bit 19 - IV44"]
    #[inline(always)]
    pub fn iv44(&mut self) -> IV44_W {
        IV44_W { w: self }
    }
    #[doc = "Bit 20 - IV43"]
    #[inline(always)]
    pub fn iv43(&mut self) -> IV43_W {
        IV43_W { w: self }
    }
    #[doc = "Bit 21 - IV42"]
    #[inline(always)]
    pub fn iv42(&mut self) -> IV42_W {
        IV42_W { w: self }
    }
    #[doc = "Bit 22 - IV41"]
    #[inline(always)]
    pub fn iv41(&mut self) -> IV41_W {
        IV41_W { w: self }
    }
    #[doc = "Bit 23 - IV40"]
    #[inline(always)]
    pub fn iv40(&mut self) -> IV40_W {
        IV40_W { w: self }
    }
    #[doc = "Bit 24 - IV39"]
    #[inline(always)]
    pub fn iv39(&mut self) -> IV39_W {
        IV39_W { w: self }
    }
    #[doc = "Bit 25 - IV38"]
    #[inline(always)]
    pub fn iv38(&mut self) -> IV38_W {
        IV38_W { w: self }
    }
    #[doc = "Bit 26 - IV37"]
    #[inline(always)]
    pub fn iv37(&mut self) -> IV37_W {
        IV37_W { w: self }
    }
    #[doc = "Bit 27 - IV36"]
    #[inline(always)]
    pub fn iv36(&mut self) -> IV36_W {
        IV36_W { w: self }
    }
    #[doc = "Bit 28 - IV35"]
    #[inline(always)]
    pub fn iv35(&mut self) -> IV35_W {
        IV35_W { w: self }
    }
    #[doc = "Bit 29 - IV34"]
    #[inline(always)]
    pub fn iv34(&mut self) -> IV34_W {
        IV34_W { w: self }
    }
    #[doc = "Bit 30 - IV33"]
    #[inline(always)]
    pub fn iv33(&mut self) -> IV33_W {
        IV33_W { w: self }
    }
    #[doc = "Bit 31 - IV32"]
    #[inline(always)]
    pub fn iv32(&mut self) -> IV32_W {
        IV32_W { w: self }
    }
}
