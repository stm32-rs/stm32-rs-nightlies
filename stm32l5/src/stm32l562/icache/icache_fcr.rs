#[doc = "Writer for register ICACHE_FCR"]
pub type W = crate::W<u32, super::ICACHE_FCR>;
#[doc = "Register ICACHE_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICACHE_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CBSYENDF`"]
pub struct CBSYENDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBSYENDF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CERRF`"]
pub struct CERRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CERRF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 1 - CBSYENDF"]
    #[inline(always)]
    pub fn cbsyendf(&mut self) -> CBSYENDF_W {
        CBSYENDF_W { w: self }
    }
    #[doc = "Bit 2 - CERRF"]
    #[inline(always)]
    pub fn cerrf(&mut self) -> CERRF_W {
        CERRF_W { w: self }
    }
}
