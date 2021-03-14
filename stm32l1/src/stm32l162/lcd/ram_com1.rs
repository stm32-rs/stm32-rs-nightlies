#[doc = "Reader of register RAM_COM1"]
pub type R = crate::R<u32, super::RAM_COM1>;
#[doc = "Writer for register RAM_COM1"]
pub type W = crate::W<u32, super::RAM_COM1>;
#[doc = "Register RAM_COM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RAM_COM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S31`"]
pub type S31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S31`"]
pub struct S31_W<'a> {
    w: &'a mut W,
}
impl<'a> S31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S30`"]
pub type S30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S30`"]
pub struct S30_W<'a> {
    w: &'a mut W,
}
impl<'a> S30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S29`"]
pub type S29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S29`"]
pub struct S29_W<'a> {
    w: &'a mut W,
}
impl<'a> S29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S28`"]
pub type S28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S28`"]
pub struct S28_W<'a> {
    w: &'a mut W,
}
impl<'a> S28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S27`"]
pub type S27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S27`"]
pub struct S27_W<'a> {
    w: &'a mut W,
}
impl<'a> S27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S26`"]
pub type S26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S26`"]
pub struct S26_W<'a> {
    w: &'a mut W,
}
impl<'a> S26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S25`"]
pub type S25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S25`"]
pub struct S25_W<'a> {
    w: &'a mut W,
}
impl<'a> S25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S24`"]
pub type S24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S24`"]
pub struct S24_W<'a> {
    w: &'a mut W,
}
impl<'a> S24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S23`"]
pub type S23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S23`"]
pub struct S23_W<'a> {
    w: &'a mut W,
}
impl<'a> S23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S22`"]
pub type S22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S22`"]
pub struct S22_W<'a> {
    w: &'a mut W,
}
impl<'a> S22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S21`"]
pub type S21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S21`"]
pub struct S21_W<'a> {
    w: &'a mut W,
}
impl<'a> S21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S20`"]
pub type S20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S20`"]
pub struct S20_W<'a> {
    w: &'a mut W,
}
impl<'a> S20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S19`"]
pub type S19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S19`"]
pub struct S19_W<'a> {
    w: &'a mut W,
}
impl<'a> S19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S18`"]
pub type S18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S18`"]
pub struct S18_W<'a> {
    w: &'a mut W,
}
impl<'a> S18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S17`"]
pub type S17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S17`"]
pub struct S17_W<'a> {
    w: &'a mut W,
}
impl<'a> S17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S16`"]
pub type S16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S16`"]
pub struct S16_W<'a> {
    w: &'a mut W,
}
impl<'a> S16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S15`"]
pub type S15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S15`"]
pub struct S15_W<'a> {
    w: &'a mut W,
}
impl<'a> S15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S14`"]
pub type S14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S14`"]
pub struct S14_W<'a> {
    w: &'a mut W,
}
impl<'a> S14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S13`"]
pub type S13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S13`"]
pub struct S13_W<'a> {
    w: &'a mut W,
}
impl<'a> S13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S12`"]
pub type S12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S12`"]
pub struct S12_W<'a> {
    w: &'a mut W,
}
impl<'a> S12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S11`"]
pub type S11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S11`"]
pub struct S11_W<'a> {
    w: &'a mut W,
}
impl<'a> S11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S10`"]
pub type S10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S10`"]
pub struct S10_W<'a> {
    w: &'a mut W,
}
impl<'a> S10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S09`"]
pub type S09_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S09`"]
pub struct S09_W<'a> {
    w: &'a mut W,
}
impl<'a> S09_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S08`"]
pub type S08_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S08`"]
pub struct S08_W<'a> {
    w: &'a mut W,
}
impl<'a> S08_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S07`"]
pub type S07_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S07`"]
pub struct S07_W<'a> {
    w: &'a mut W,
}
impl<'a> S07_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S06`"]
pub type S06_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S06`"]
pub struct S06_W<'a> {
    w: &'a mut W,
}
impl<'a> S06_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S05`"]
pub type S05_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S05`"]
pub struct S05_W<'a> {
    w: &'a mut W,
}
impl<'a> S05_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S04`"]
pub type S04_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S04`"]
pub struct S04_W<'a> {
    w: &'a mut W,
}
impl<'a> S04_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S03`"]
pub type S03_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S03`"]
pub struct S03_W<'a> {
    w: &'a mut W,
}
impl<'a> S03_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S02`"]
pub type S02_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S02`"]
pub struct S02_W<'a> {
    w: &'a mut W,
}
impl<'a> S02_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S01`"]
pub type S01_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S01`"]
pub struct S01_W<'a> {
    w: &'a mut W,
}
impl<'a> S01_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Reader of field `S00`"]
pub type S00_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S00`"]
pub struct S00_W<'a> {
    w: &'a mut W,
}
impl<'a> S00_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 31 - S31"]
    #[inline(always)]
    pub fn s31(&self) -> S31_R {
        S31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - S30"]
    #[inline(always)]
    pub fn s30(&self) -> S30_R {
        S30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - S29"]
    #[inline(always)]
    pub fn s29(&self) -> S29_R {
        S29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - S28"]
    #[inline(always)]
    pub fn s28(&self) -> S28_R {
        S28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - S27"]
    #[inline(always)]
    pub fn s27(&self) -> S27_R {
        S27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - S26"]
    #[inline(always)]
    pub fn s26(&self) -> S26_R {
        S26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - S25"]
    #[inline(always)]
    pub fn s25(&self) -> S25_R {
        S25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - S24"]
    #[inline(always)]
    pub fn s24(&self) -> S24_R {
        S24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - S23"]
    #[inline(always)]
    pub fn s23(&self) -> S23_R {
        S23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - S22"]
    #[inline(always)]
    pub fn s22(&self) -> S22_R {
        S22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - S21"]
    #[inline(always)]
    pub fn s21(&self) -> S21_R {
        S21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - S20"]
    #[inline(always)]
    pub fn s20(&self) -> S20_R {
        S20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - S19"]
    #[inline(always)]
    pub fn s19(&self) -> S19_R {
        S19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - S18"]
    #[inline(always)]
    pub fn s18(&self) -> S18_R {
        S18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - S17"]
    #[inline(always)]
    pub fn s17(&self) -> S17_R {
        S17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - S16"]
    #[inline(always)]
    pub fn s16(&self) -> S16_R {
        S16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - S15"]
    #[inline(always)]
    pub fn s15(&self) -> S15_R {
        S15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - S14"]
    #[inline(always)]
    pub fn s14(&self) -> S14_R {
        S14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&self) -> S13_R {
        S13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&self) -> S12_R {
        S12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&self) -> S11_R {
        S11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&self) -> S10_R {
        S10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - S09"]
    #[inline(always)]
    pub fn s09(&self) -> S09_R {
        S09_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - S08"]
    #[inline(always)]
    pub fn s08(&self) -> S08_R {
        S08_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - S07"]
    #[inline(always)]
    pub fn s07(&self) -> S07_R {
        S07_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - S06"]
    #[inline(always)]
    pub fn s06(&self) -> S06_R {
        S06_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - S05"]
    #[inline(always)]
    pub fn s05(&self) -> S05_R {
        S05_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - S04"]
    #[inline(always)]
    pub fn s04(&self) -> S04_R {
        S04_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - S03"]
    #[inline(always)]
    pub fn s03(&self) -> S03_R {
        S03_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - S02"]
    #[inline(always)]
    pub fn s02(&self) -> S02_R {
        S02_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - S01"]
    #[inline(always)]
    pub fn s01(&self) -> S01_R {
        S01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - S00"]
    #[inline(always)]
    pub fn s00(&self) -> S00_R {
        S00_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - S31"]
    #[inline(always)]
    pub fn s31(&mut self) -> S31_W {
        S31_W { w: self }
    }
    #[doc = "Bit 30 - S30"]
    #[inline(always)]
    pub fn s30(&mut self) -> S30_W {
        S30_W { w: self }
    }
    #[doc = "Bit 29 - S29"]
    #[inline(always)]
    pub fn s29(&mut self) -> S29_W {
        S29_W { w: self }
    }
    #[doc = "Bit 28 - S28"]
    #[inline(always)]
    pub fn s28(&mut self) -> S28_W {
        S28_W { w: self }
    }
    #[doc = "Bit 27 - S27"]
    #[inline(always)]
    pub fn s27(&mut self) -> S27_W {
        S27_W { w: self }
    }
    #[doc = "Bit 26 - S26"]
    #[inline(always)]
    pub fn s26(&mut self) -> S26_W {
        S26_W { w: self }
    }
    #[doc = "Bit 25 - S25"]
    #[inline(always)]
    pub fn s25(&mut self) -> S25_W {
        S25_W { w: self }
    }
    #[doc = "Bit 24 - S24"]
    #[inline(always)]
    pub fn s24(&mut self) -> S24_W {
        S24_W { w: self }
    }
    #[doc = "Bit 23 - S23"]
    #[inline(always)]
    pub fn s23(&mut self) -> S23_W {
        S23_W { w: self }
    }
    #[doc = "Bit 22 - S22"]
    #[inline(always)]
    pub fn s22(&mut self) -> S22_W {
        S22_W { w: self }
    }
    #[doc = "Bit 21 - S21"]
    #[inline(always)]
    pub fn s21(&mut self) -> S21_W {
        S21_W { w: self }
    }
    #[doc = "Bit 20 - S20"]
    #[inline(always)]
    pub fn s20(&mut self) -> S20_W {
        S20_W { w: self }
    }
    #[doc = "Bit 19 - S19"]
    #[inline(always)]
    pub fn s19(&mut self) -> S19_W {
        S19_W { w: self }
    }
    #[doc = "Bit 18 - S18"]
    #[inline(always)]
    pub fn s18(&mut self) -> S18_W {
        S18_W { w: self }
    }
    #[doc = "Bit 17 - S17"]
    #[inline(always)]
    pub fn s17(&mut self) -> S17_W {
        S17_W { w: self }
    }
    #[doc = "Bit 16 - S16"]
    #[inline(always)]
    pub fn s16(&mut self) -> S16_W {
        S16_W { w: self }
    }
    #[doc = "Bit 15 - S15"]
    #[inline(always)]
    pub fn s15(&mut self) -> S15_W {
        S15_W { w: self }
    }
    #[doc = "Bit 14 - S14"]
    #[inline(always)]
    pub fn s14(&mut self) -> S14_W {
        S14_W { w: self }
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&mut self) -> S13_W {
        S13_W { w: self }
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&mut self) -> S12_W {
        S12_W { w: self }
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&mut self) -> S11_W {
        S11_W { w: self }
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&mut self) -> S10_W {
        S10_W { w: self }
    }
    #[doc = "Bit 9 - S09"]
    #[inline(always)]
    pub fn s09(&mut self) -> S09_W {
        S09_W { w: self }
    }
    #[doc = "Bit 8 - S08"]
    #[inline(always)]
    pub fn s08(&mut self) -> S08_W {
        S08_W { w: self }
    }
    #[doc = "Bit 7 - S07"]
    #[inline(always)]
    pub fn s07(&mut self) -> S07_W {
        S07_W { w: self }
    }
    #[doc = "Bit 6 - S06"]
    #[inline(always)]
    pub fn s06(&mut self) -> S06_W {
        S06_W { w: self }
    }
    #[doc = "Bit 5 - S05"]
    #[inline(always)]
    pub fn s05(&mut self) -> S05_W {
        S05_W { w: self }
    }
    #[doc = "Bit 4 - S04"]
    #[inline(always)]
    pub fn s04(&mut self) -> S04_W {
        S04_W { w: self }
    }
    #[doc = "Bit 3 - S03"]
    #[inline(always)]
    pub fn s03(&mut self) -> S03_W {
        S03_W { w: self }
    }
    #[doc = "Bit 2 - S02"]
    #[inline(always)]
    pub fn s02(&mut self) -> S02_W {
        S02_W { w: self }
    }
    #[doc = "Bit 1 - S01"]
    #[inline(always)]
    pub fn s01(&mut self) -> S01_W {
        S01_W { w: self }
    }
    #[doc = "Bit 0 - S00"]
    #[inline(always)]
    pub fn s00(&mut self) -> S00_W {
        S00_W { w: self }
    }
}
