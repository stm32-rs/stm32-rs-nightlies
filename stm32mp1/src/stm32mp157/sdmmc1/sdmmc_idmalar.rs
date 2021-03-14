#[doc = "Reader of register SDMMC_IDMALAR"]
pub type R = crate::R<u32, super::SDMMC_IDMALAR>;
#[doc = "Writer for register SDMMC_IDMALAR"]
pub type W = crate::W<u32, super::SDMMC_IDMALAR>;
#[doc = "Register SDMMC_IDMALAR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_IDMALAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMALA`"]
pub type IDMALA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDMALA`"]
pub struct IDMALA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMALA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `ABR`"]
pub type ABR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABR`"]
pub struct ABR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ULS`"]
pub type ULS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULS`"]
pub struct ULS_W<'a> {
    w: &'a mut W,
}
impl<'a> ULS_W<'a> {
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
#[doc = "Reader of field `ULA`"]
pub type ULA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULA`"]
pub struct ULA_W<'a> {
    w: &'a mut W,
}
impl<'a> ULA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - IDMALA"]
    #[inline(always)]
    pub fn idmala(&self) -> IDMALA_R {
        IDMALA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 29 - ABR"]
    #[inline(always)]
    pub fn abr(&self) -> ABR_R {
        ABR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ULS"]
    #[inline(always)]
    pub fn uls(&self) -> ULS_R {
        ULS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ULA"]
    #[inline(always)]
    pub fn ula(&self) -> ULA_R {
        ULA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - IDMALA"]
    #[inline(always)]
    pub fn idmala(&mut self) -> IDMALA_W {
        IDMALA_W { w: self }
    }
    #[doc = "Bit 29 - ABR"]
    #[inline(always)]
    pub fn abr(&mut self) -> ABR_W {
        ABR_W { w: self }
    }
    #[doc = "Bit 30 - ULS"]
    #[inline(always)]
    pub fn uls(&mut self) -> ULS_W {
        ULS_W { w: self }
    }
    #[doc = "Bit 31 - ULA"]
    #[inline(always)]
    pub fn ula(&mut self) -> ULA_W {
        ULA_W { w: self }
    }
}
