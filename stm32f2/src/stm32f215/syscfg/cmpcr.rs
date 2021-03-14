#[doc = "Reader of register CMPCR"]
pub type R = crate::R<u32, super::CMPCR>;
#[doc = "Writer for register CMPCR"]
pub type W = crate::W<u32, super::CMPCR>;
#[doc = "Register CMPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP_PD`"]
pub type CMP_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP_PD`"]
pub struct CMP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_PD_W<'a> {
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
    #[doc = "Bit 7 - READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&self) -> CMP_PD_R {
        CMP_PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compensation cell power-down"]
    #[inline(always)]
    pub fn cmp_pd(&mut self) -> CMP_PD_W {
        CMP_PD_W { w: self }
    }
}
