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
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> CLR_OPTCHANGEERR_W {
        CLR_OPTCHANGEERR_W { w: self }
    }
}
