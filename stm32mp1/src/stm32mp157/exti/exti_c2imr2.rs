#[doc = "Reader of register EXTI_C2IMR2"]
pub type R = crate::R<u32, super::EXTI_C2IMR2>;
#[doc = "Writer for register EXTI_C2IMR2"]
pub type W = crate::W<u32, super::EXTI_C2IMR2>;
#[doc = "Register EXTI_C2IMR2 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::EXTI_C2IMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `IM32`"]
pub type IM32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM32`"]
pub struct IM32_W<'a> {
    w: &'a mut W,
}
impl<'a> IM32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM33`"]
pub type IM33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM33`"]
pub struct IM33_W<'a> {
    w: &'a mut W,
}
impl<'a> IM33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM34`"]
pub type IM34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM34`"]
pub struct IM34_W<'a> {
    w: &'a mut W,
}
impl<'a> IM34_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM35`"]
pub type IM35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM35`"]
pub struct IM35_W<'a> {
    w: &'a mut W,
}
impl<'a> IM35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM36`"]
pub type IM36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM36`"]
pub struct IM36_W<'a> {
    w: &'a mut W,
}
impl<'a> IM36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM37`"]
pub type IM37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM37`"]
pub struct IM37_W<'a> {
    w: &'a mut W,
}
impl<'a> IM37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM38`"]
pub type IM38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM38`"]
pub struct IM38_W<'a> {
    w: &'a mut W,
}
impl<'a> IM38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM39`"]
pub type IM39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM39`"]
pub struct IM39_W<'a> {
    w: &'a mut W,
}
impl<'a> IM39_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM40`"]
pub type IM40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM40`"]
pub struct IM40_W<'a> {
    w: &'a mut W,
}
impl<'a> IM40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM41`"]
pub type IM41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM41`"]
pub struct IM41_W<'a> {
    w: &'a mut W,
}
impl<'a> IM41_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM42`"]
pub type IM42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM42`"]
pub struct IM42_W<'a> {
    w: &'a mut W,
}
impl<'a> IM42_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM43`"]
pub type IM43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM43`"]
pub struct IM43_W<'a> {
    w: &'a mut W,
}
impl<'a> IM43_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM44`"]
pub type IM44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM44`"]
pub struct IM44_W<'a> {
    w: &'a mut W,
}
impl<'a> IM44_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM45`"]
pub type IM45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM45`"]
pub struct IM45_W<'a> {
    w: &'a mut W,
}
impl<'a> IM45_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM46`"]
pub type IM46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM46`"]
pub struct IM46_W<'a> {
    w: &'a mut W,
}
impl<'a> IM46_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM47`"]
pub type IM47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM47`"]
pub struct IM47_W<'a> {
    w: &'a mut W,
}
impl<'a> IM47_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM48`"]
pub type IM48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM48`"]
pub struct IM48_W<'a> {
    w: &'a mut W,
}
impl<'a> IM48_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM49`"]
pub type IM49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM49`"]
pub struct IM49_W<'a> {
    w: &'a mut W,
}
impl<'a> IM49_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM50`"]
pub type IM50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM50`"]
pub struct IM50_W<'a> {
    w: &'a mut W,
}
impl<'a> IM50_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM51`"]
pub type IM51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM51`"]
pub struct IM51_W<'a> {
    w: &'a mut W,
}
impl<'a> IM51_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM52`"]
pub type IM52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM52`"]
pub struct IM52_W<'a> {
    w: &'a mut W,
}
impl<'a> IM52_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM53`"]
pub type IM53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM53`"]
pub struct IM53_W<'a> {
    w: &'a mut W,
}
impl<'a> IM53_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM54`"]
pub type IM54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM54`"]
pub struct IM54_W<'a> {
    w: &'a mut W,
}
impl<'a> IM54_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM55`"]
pub type IM55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM55`"]
pub struct IM55_W<'a> {
    w: &'a mut W,
}
impl<'a> IM55_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM56`"]
pub type IM56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM56`"]
pub struct IM56_W<'a> {
    w: &'a mut W,
}
impl<'a> IM56_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM57`"]
pub type IM57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM57`"]
pub struct IM57_W<'a> {
    w: &'a mut W,
}
impl<'a> IM57_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM58`"]
pub type IM58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM58`"]
pub struct IM58_W<'a> {
    w: &'a mut W,
}
impl<'a> IM58_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM59`"]
pub type IM59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM59`"]
pub struct IM59_W<'a> {
    w: &'a mut W,
}
impl<'a> IM59_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM60`"]
pub type IM60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM60`"]
pub struct IM60_W<'a> {
    w: &'a mut W,
}
impl<'a> IM60_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM61`"]
pub type IM61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM61`"]
pub struct IM61_W<'a> {
    w: &'a mut W,
}
impl<'a> IM61_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM62`"]
pub type IM62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM62`"]
pub struct IM62_W<'a> {
    w: &'a mut W,
}
impl<'a> IM62_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `IM63`"]
pub type IM63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IM63`"]
pub struct IM63_W<'a> {
    w: &'a mut W,
}
impl<'a> IM63_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - IM32"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IM33"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IM34"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IM35"]
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IM36"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IM37"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IM38"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IM39"]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IM40"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IM41"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IM42"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - IM43"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IM44"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IM45"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IM46"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IM47"]
    #[inline(always)]
    pub fn im47(&self) -> IM47_R {
        IM47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IM48"]
    #[inline(always)]
    pub fn im48(&self) -> IM48_R {
        IM48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IM49"]
    #[inline(always)]
    pub fn im49(&self) -> IM49_R {
        IM49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IM50"]
    #[inline(always)]
    pub fn im50(&self) -> IM50_R {
        IM50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IM51"]
    #[inline(always)]
    pub fn im51(&self) -> IM51_R {
        IM51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IM52"]
    #[inline(always)]
    pub fn im52(&self) -> IM52_R {
        IM52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IM53"]
    #[inline(always)]
    pub fn im53(&self) -> IM53_R {
        IM53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IM54"]
    #[inline(always)]
    pub fn im54(&self) -> IM54_R {
        IM54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - IM55"]
    #[inline(always)]
    pub fn im55(&self) -> IM55_R {
        IM55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - IM56"]
    #[inline(always)]
    pub fn im56(&self) -> IM56_R {
        IM56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - IM57"]
    #[inline(always)]
    pub fn im57(&self) -> IM57_R {
        IM57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - IM58"]
    #[inline(always)]
    pub fn im58(&self) -> IM58_R {
        IM58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IM59"]
    #[inline(always)]
    pub fn im59(&self) -> IM59_R {
        IM59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IM60"]
    #[inline(always)]
    pub fn im60(&self) -> IM60_R {
        IM60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IM61"]
    #[inline(always)]
    pub fn im61(&self) -> IM61_R {
        IM61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - IM62"]
    #[inline(always)]
    pub fn im62(&self) -> IM62_R {
        IM62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IM63"]
    #[inline(always)]
    pub fn im63(&self) -> IM63_R {
        IM63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM32"]
    #[inline(always)]
    pub fn im32(&mut self) -> IM32_W {
        IM32_W { w: self }
    }
    #[doc = "Bit 1 - IM33"]
    #[inline(always)]
    pub fn im33(&mut self) -> IM33_W {
        IM33_W { w: self }
    }
    #[doc = "Bit 2 - IM34"]
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W {
        IM34_W { w: self }
    }
    #[doc = "Bit 3 - IM35"]
    #[inline(always)]
    pub fn im35(&mut self) -> IM35_W {
        IM35_W { w: self }
    }
    #[doc = "Bit 4 - IM36"]
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W {
        IM36_W { w: self }
    }
    #[doc = "Bit 5 - IM37"]
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W {
        IM37_W { w: self }
    }
    #[doc = "Bit 6 - IM38"]
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W {
        IM38_W { w: self }
    }
    #[doc = "Bit 7 - IM39"]
    #[inline(always)]
    pub fn im39(&mut self) -> IM39_W {
        IM39_W { w: self }
    }
    #[doc = "Bit 8 - IM40"]
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W {
        IM40_W { w: self }
    }
    #[doc = "Bit 9 - IM41"]
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W {
        IM41_W { w: self }
    }
    #[doc = "Bit 10 - IM42"]
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W {
        IM42_W { w: self }
    }
    #[doc = "Bit 11 - IM43"]
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W {
        IM43_W { w: self }
    }
    #[doc = "Bit 12 - IM44"]
    #[inline(always)]
    pub fn im44(&mut self) -> IM44_W {
        IM44_W { w: self }
    }
    #[doc = "Bit 13 - IM45"]
    #[inline(always)]
    pub fn im45(&mut self) -> IM45_W {
        IM45_W { w: self }
    }
    #[doc = "Bit 14 - IM46"]
    #[inline(always)]
    pub fn im46(&mut self) -> IM46_W {
        IM46_W { w: self }
    }
    #[doc = "Bit 15 - IM47"]
    #[inline(always)]
    pub fn im47(&mut self) -> IM47_W {
        IM47_W { w: self }
    }
    #[doc = "Bit 16 - IM48"]
    #[inline(always)]
    pub fn im48(&mut self) -> IM48_W {
        IM48_W { w: self }
    }
    #[doc = "Bit 17 - IM49"]
    #[inline(always)]
    pub fn im49(&mut self) -> IM49_W {
        IM49_W { w: self }
    }
    #[doc = "Bit 18 - IM50"]
    #[inline(always)]
    pub fn im50(&mut self) -> IM50_W {
        IM50_W { w: self }
    }
    #[doc = "Bit 19 - IM51"]
    #[inline(always)]
    pub fn im51(&mut self) -> IM51_W {
        IM51_W { w: self }
    }
    #[doc = "Bit 20 - IM52"]
    #[inline(always)]
    pub fn im52(&mut self) -> IM52_W {
        IM52_W { w: self }
    }
    #[doc = "Bit 21 - IM53"]
    #[inline(always)]
    pub fn im53(&mut self) -> IM53_W {
        IM53_W { w: self }
    }
    #[doc = "Bit 22 - IM54"]
    #[inline(always)]
    pub fn im54(&mut self) -> IM54_W {
        IM54_W { w: self }
    }
    #[doc = "Bit 23 - IM55"]
    #[inline(always)]
    pub fn im55(&mut self) -> IM55_W {
        IM55_W { w: self }
    }
    #[doc = "Bit 24 - IM56"]
    #[inline(always)]
    pub fn im56(&mut self) -> IM56_W {
        IM56_W { w: self }
    }
    #[doc = "Bit 25 - IM57"]
    #[inline(always)]
    pub fn im57(&mut self) -> IM57_W {
        IM57_W { w: self }
    }
    #[doc = "Bit 26 - IM58"]
    #[inline(always)]
    pub fn im58(&mut self) -> IM58_W {
        IM58_W { w: self }
    }
    #[doc = "Bit 27 - IM59"]
    #[inline(always)]
    pub fn im59(&mut self) -> IM59_W {
        IM59_W { w: self }
    }
    #[doc = "Bit 28 - IM60"]
    #[inline(always)]
    pub fn im60(&mut self) -> IM60_W {
        IM60_W { w: self }
    }
    #[doc = "Bit 29 - IM61"]
    #[inline(always)]
    pub fn im61(&mut self) -> IM61_W {
        IM61_W { w: self }
    }
    #[doc = "Bit 30 - IM62"]
    #[inline(always)]
    pub fn im62(&mut self) -> IM62_W {
        IM62_W { w: self }
    }
    #[doc = "Bit 31 - IM63"]
    #[inline(always)]
    pub fn im63(&mut self) -> IM63_W {
        IM63_W { w: self }
    }
}
