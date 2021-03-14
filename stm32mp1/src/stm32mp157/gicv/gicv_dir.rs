#[doc = "Writer for register GICV_DIR"]
pub type W = crate::W<u32, super::GICV_DIR>;
#[doc = "Register GICV_DIR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICV_DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTERRUPT_ID`"]
pub struct INTERRUPT_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Write proxy for field `CPUID`"]
pub struct CPUID_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUID_W<'a> {
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
impl W {
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W {
        INTERRUPT_ID_W { w: self }
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&mut self) -> CPUID_W {
        CPUID_W { w: self }
    }
}
