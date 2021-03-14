#[doc = "Reader of register AHB3SMENR"]
pub type R = crate::R<u32, super::AHB3SMENR>;
#[doc = "Writer for register AHB3SMENR"]
pub type W = crate::W<u32, super::AHB3SMENR>;
#[doc = "Register AHB3SMENR `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::AHB3SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Reader of field `QSPISMEN`"]
pub type QSPISMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPISMEN`"]
pub struct QSPISMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 8 - QSPISMEN"]
    #[inline(always)]
    pub fn qspismen(&self) -> QSPISMEN_R {
        QSPISMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - QSPISMEN"]
    #[inline(always)]
    pub fn qspismen(&mut self) -> QSPISMEN_W {
        QSPISMEN_W { w: self }
    }
}
