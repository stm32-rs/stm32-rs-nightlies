#[doc = "Register `WIFCR` writer"]
pub struct W(crate::W<WIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFCR_SPEC>;
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
impl From<crate::W<WIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEIF` writer - CTEIF"]
pub struct CTEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CERIF` writer - CERIF"]
pub struct CERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CERIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CPLLLIF` writer - CPLLLIF"]
pub struct CPLLLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPLLLIF_W<'a> {
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
#[doc = "Field `CPLLUIF` writer - CPLLUIF"]
pub struct CPLLUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPLLUIF_W<'a> {
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
#[doc = "Field `CRRIF` writer - CRRIF"]
pub struct CRRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRRIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
    }
    #[doc = "Bit 1 - CERIF"]
    #[inline(always)]
    pub fn cerif(&mut self) -> CERIF_W {
        CERIF_W { w: self }
    }
    #[doc = "Bit 9 - CPLLLIF"]
    #[inline(always)]
    pub fn cplllif(&mut self) -> CPLLLIF_W {
        CPLLLIF_W { w: self }
    }
    #[doc = "Bit 10 - CPLLUIF"]
    #[inline(always)]
    pub fn cplluif(&mut self) -> CPLLUIF_W {
        CPLLUIF_W { w: self }
    }
    #[doc = "Bit 13 - CRRIF"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W {
        CRRIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifcr](index.html) module"]
pub struct WIFCR_SPEC;
impl crate::RegisterSpec for WIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wifcr::W](W) writer structure"]
impl crate::Writable for WIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFCR to value 0"]
impl crate::Resettable for WIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
