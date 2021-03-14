#[doc = "Reader of register TZC_REGION_BASE_LOW6"]
pub type R = crate::R<u32, super::TZC_REGION_BASE_LOW6>;
#[doc = "Writer for register TZC_REGION_BASE_LOW6"]
pub type W = crate::W<u32, super::TZC_REGION_BASE_LOW6>;
#[doc = "Register TZC_REGION_BASE_LOW6 `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_REGION_BASE_LOW6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASE_ADDRESS_LOW`"]
pub type BASE_ADDRESS_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASE_ADDRESS_LOW`"]
pub struct BASE_ADDRESS_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_ADDRESS_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - BASE_ADDRESS_LOW"]
    #[inline(always)]
    pub fn base_address_low(&self) -> BASE_ADDRESS_LOW_R {
        BASE_ADDRESS_LOW_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - BASE_ADDRESS_LOW"]
    #[inline(always)]
    pub fn base_address_low(&mut self) -> BASE_ADDRESS_LOW_W {
        BASE_ADDRESS_LOW_W { w: self }
    }
}
