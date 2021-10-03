#[doc = "Register `SPI2S_IFCR` writer"]
pub struct W(crate::W<SPI2S_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2S_IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI2S_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2S_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOTC` writer - EOTC"]
pub struct EOTC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TXTFC` writer - TXTFC"]
pub struct TXTFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `UDRC` writer - UDRC"]
pub struct UDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OVRC` writer - OVRC"]
pub struct OVRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CRCEC` writer - CRCEC"]
pub struct CRCEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `TIFREC` writer - TIFREC"]
pub struct TIFREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFREC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `MODFC` writer - MODFC"]
pub struct MODFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TSERFC` writer - TSERFC"]
pub struct TSERFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSERFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SUSPC` writer - SUSPC"]
pub struct SUSPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 3 - EOTC"]
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W {
        EOTC_W { w: self }
    }
    #[doc = "Bit 4 - TXTFC"]
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W {
        TXTFC_W { w: self }
    }
    #[doc = "Bit 5 - UDRC"]
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W {
        UDRC_W { w: self }
    }
    #[doc = "Bit 6 - OVRC"]
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W {
        OVRC_W { w: self }
    }
    #[doc = "Bit 7 - CRCEC"]
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W {
        CRCEC_W { w: self }
    }
    #[doc = "Bit 8 - TIFREC"]
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W {
        TIFREC_W { w: self }
    }
    #[doc = "Bit 9 - MODFC"]
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W {
        MODFC_W { w: self }
    }
    #[doc = "Bit 10 - TSERFC"]
    #[inline(always)]
    pub fn tserfc(&mut self) -> TSERFC_W {
        TSERFC_W { w: self }
    }
    #[doc = "Bit 11 - SUSPC"]
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W {
        SUSPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI/I2S interrupt/status flags clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_ifcr](index.html) module"]
pub struct SPI2S_IFCR_SPEC;
impl crate::RegisterSpec for SPI2S_IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi2s_ifcr::W](W) writer structure"]
impl crate::Writable for SPI2S_IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI2S_IFCR to value 0"]
impl crate::Resettable for SPI2S_IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
