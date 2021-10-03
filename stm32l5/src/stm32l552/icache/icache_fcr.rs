#[doc = "Register `ICACHE_FCR` writer"]
pub struct W(crate::W<ICACHE_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_FCR_SPEC>;
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
impl From<crate::W<ICACHE_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBSYENDF` writer - CBSYENDF"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CERRF` writer - CERRF"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_fcr](index.html) module"]
pub struct ICACHE_FCR_SPEC;
impl crate::RegisterSpec for ICACHE_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icache_fcr::W](W) writer structure"]
impl crate::Writable for ICACHE_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_FCR to value 0"]
impl crate::Resettable for ICACHE_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
