#[doc = "Reader of register RCC_MC_AXIMENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_AXIMENCLRR>;
#[doc = "Writer for register RCC_MC_AXIMENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_AXIMENCLRR>;
#[doc = "Register RCC_MC_AXIMENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MC_AXIMENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSRAMEN`"]
pub type SYSRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRAMEN`"]
pub struct SYSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAMEN_W<'a> {
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
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&self) -> SYSRAMEN_R {
        SYSRAMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&mut self) -> SYSRAMEN_W {
        SYSRAMEN_W { w: self }
    }
}
