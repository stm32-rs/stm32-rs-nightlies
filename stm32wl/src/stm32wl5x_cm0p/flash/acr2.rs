#[doc = "Register `ACR2` reader"]
pub struct R(crate::R<ACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR2` writer"]
pub struct W(crate::W<ACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR2_SPEC>;
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
impl From<crate::W<ACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVMODE` reader - CFI privileged mode enable"]
pub struct PRIVMODE_R(crate::FieldReader<bool, bool>);
impl PRIVMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIVMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIVMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIVMODE` writer - CFI privileged mode enable"]
pub struct PRIVMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVMODE_W<'a> {
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
#[doc = "Field `HDPADIS` reader - Flash user hide protection area access disable"]
pub struct HDPADIS_R(crate::FieldReader<bool, bool>);
impl HDPADIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDPADIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDPADIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDPADIS` writer - Flash user hide protection area access disable"]
pub struct HDPADIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPADIS_W<'a> {
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
#[doc = "Field `C2SWDBGEN` reader - CPU2 Software debug enable"]
pub struct C2SWDBGEN_R(crate::FieldReader<bool, bool>);
impl C2SWDBGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2SWDBGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2SWDBGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2SWDBGEN` writer - CPU2 Software debug enable"]
pub struct C2SWDBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C2SWDBGEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    pub fn privmode(&self) -> PRIVMODE_R {
        PRIVMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    pub fn hdpadis(&self) -> HDPADIS_R {
        HDPADIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    pub fn c2swdbgen(&self) -> C2SWDBGEN_R {
        C2SWDBGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CFI privileged mode enable"]
    #[inline(always)]
    pub fn privmode(&mut self) -> PRIVMODE_W {
        PRIVMODE_W { w: self }
    }
    #[doc = "Bit 1 - Flash user hide protection area access disable"]
    #[inline(always)]
    pub fn hdpadis(&mut self) -> HDPADIS_W {
        HDPADIS_W { w: self }
    }
    #[doc = "Bit 2 - CPU2 Software debug enable"]
    #[inline(always)]
    pub fn c2swdbgen(&mut self) -> C2SWDBGEN_W {
        C2SWDBGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr2](index.html) module"]
pub struct ACR2_SPEC;
impl crate::RegisterSpec for ACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr2::R](R) reader structure"]
impl crate::Readable for ACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr2::W](W) writer structure"]
impl crate::Writable for ACR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR2 to value 0"]
impl crate::Resettable for ACR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
