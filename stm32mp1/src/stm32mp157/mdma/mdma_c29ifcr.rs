#[doc = "Register `MDMA_C29IFCR` writer"]
pub struct W(crate::W<MDMA_C29IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C29IFCR_SPEC>;
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
impl From<crate::W<MDMA_C29IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C29IFCR_SPEC>) -> Self {
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
#[doc = "Field `CCTCIF` writer - CCTCIF"]
pub struct CCTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF_W<'a> {
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
#[doc = "Field `CBRTIF` writer - CBRTIF"]
pub struct CBRTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CBTIF` writer - CBTIF"]
pub struct CBTIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF_W<'a> {
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
#[doc = "Field `CLTCIF` writer - CLTCIF"]
pub struct CLTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W {
        CCTCIF_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF"]
    #[inline(always)]
    pub fn cbrtif(&mut self) -> CBRTIF_W {
        CBRTIF_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF"]
    #[inline(always)]
    pub fn cbtif(&mut self) -> CBTIF_W {
        CBTIF_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF"]
    #[inline(always)]
    pub fn cltcif(&mut self) -> CLTCIF_W {
        CLTCIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel 29 interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c29ifcr](index.html) module"]
pub struct MDMA_C29IFCR_SPEC;
impl crate::RegisterSpec for MDMA_C29IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mdma_c29ifcr::W](W) writer structure"]
impl crate::Writable for MDMA_C29IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C29IFCR to value 0"]
impl crate::Resettable for MDMA_C29IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
