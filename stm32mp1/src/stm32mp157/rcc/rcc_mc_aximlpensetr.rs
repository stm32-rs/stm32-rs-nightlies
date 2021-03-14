#[doc = "Reader of register RCC_MC_AXIMLPENSETR"]
pub type R = crate::R<u32, super::RCC_MC_AXIMLPENSETR>;
#[doc = "Writer for register RCC_MC_AXIMLPENSETR"]
pub type W = crate::W<u32, super::RCC_MC_AXIMLPENSETR>;
#[doc = "Register RCC_MC_AXIMLPENSETR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_MC_AXIMLPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SYSRAMLPEN`"]
pub type SYSRAMLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRAMLPEN`"]
pub struct SYSRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAMLPEN_W<'a> {
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
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    pub fn sysramlpen(&self) -> SYSRAMLPEN_R {
        SYSRAMLPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W {
        SYSRAMLPEN_W { w: self }
    }
}
