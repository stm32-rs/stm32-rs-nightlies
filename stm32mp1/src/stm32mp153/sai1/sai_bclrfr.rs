#[doc = "Register `SAI_BCLRFR` writer"]
pub struct W(crate::W<SAI_BCLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_BCLRFR_SPEC>;
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
impl From<crate::W<SAI_BCLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_BCLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COVRUDR` writer - COVRUDR"]
pub struct COVRUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> COVRUDR_W<'a> {
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
#[doc = "Field `CMUTEDET` writer - CMUTEDET"]
pub struct CMUTEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUTEDET_W<'a> {
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
#[doc = "Field `CWCKCFG` writer - CWCKCFG"]
pub struct CWCKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CWCKCFG_W<'a> {
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
#[doc = "Field `CCNRDY` writer - CCNRDY"]
pub struct CCNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCNRDY_W<'a> {
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
#[doc = "Field `CAFSDET` writer - CAFSDET"]
pub struct CAFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAFSDET_W<'a> {
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
#[doc = "Field `CLFSDET` writer - CLFSDET"]
pub struct CLFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLFSDET_W<'a> {
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
impl W {
    #[doc = "Bit 0 - COVRUDR"]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W {
        COVRUDR_W { w: self }
    }
    #[doc = "Bit 1 - CMUTEDET"]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W {
        CMUTEDET_W { w: self }
    }
    #[doc = "Bit 2 - CWCKCFG"]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W {
        CWCKCFG_W { w: self }
    }
    #[doc = "Bit 4 - CCNRDY"]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W {
        CCNRDY_W { w: self }
    }
    #[doc = "Bit 5 - CAFSDET"]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W {
        CAFSDET_W { w: self }
    }
    #[doc = "Bit 6 - CLFSDET"]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W {
        CLFSDET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bclrfr](index.html) module"]
pub struct SAI_BCLRFR_SPEC;
impl crate::RegisterSpec for SAI_BCLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sai_bclrfr::W](W) writer structure"]
impl crate::Writable for SAI_BCLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_BCLRFR to value 0"]
impl crate::Resettable for SAI_BCLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}