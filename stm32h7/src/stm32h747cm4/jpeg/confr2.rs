#[doc = "Reader of register CONFR2"]
pub type R = crate::R<u32, super::CONFR2>;
#[doc = "Writer for register CONFR2"]
pub type W = crate::W<u32, super::CONFR2>;
#[doc = "Register CONFR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NMCU`"]
pub type NMCU_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NMCU`"]
pub struct NMCU_W<'a> {
    w: &'a mut W,
}
impl<'a> NMCU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&self) -> NMCU_R {
        NMCU_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&mut self) -> NMCU_W {
        NMCU_W { w: self }
    }
}
