#[doc = "Reader of register OPTCCR"]
pub type R = crate::R<u32, super::OPTCCR>;
#[doc = "Writer for register OPTCCR"]
pub type W = crate::W<u32, super::OPTCCR>;
#[doc = "Register OPTCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::OPTCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLR_OPTCHANGEERR`"]
pub type CLR_OPTCHANGEERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_OPTCHANGEERR`"]
pub struct CLR_OPTCHANGEERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_OPTCHANGEERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&self) -> CLR_OPTCHANGEERR_R {
        CLR_OPTCHANGEERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W {
        CLR_OPTCHANGEERR_W { w: self }
    }
}
