#[doc = "Reader of register SECCFGR"]
pub type R = crate::R<u32, super::SECCFGR>;
#[doc = "Writer for register SECCFGR"]
pub type W = crate::W<u32, super::SECCFGR>;
#[doc = "Register SECCFGR `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::SECCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `C2EWILA`"]
pub type C2EWILA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C2EWILA`"]
pub struct C2EWILA_W<'a> {
    w: &'a mut W,
}
impl<'a> C2EWILA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 15 - wakeup on CPU2 illegal access interrupt enable"]
    #[inline(always)]
    pub fn c2ewila(&self) -> C2EWILA_R {
        C2EWILA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - wakeup on CPU2 illegal access interrupt enable"]
    #[inline(always)]
    pub fn c2ewila(&mut self) -> C2EWILA_W {
        C2EWILA_W { w: self }
    }
}
