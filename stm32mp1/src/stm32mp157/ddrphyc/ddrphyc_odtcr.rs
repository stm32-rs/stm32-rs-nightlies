#[doc = "Register `DDRPHYC_ODTCR` reader"]
pub struct R(crate::R<DDRPHYC_ODTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ODTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ODTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ODTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ODTCR` writer"]
pub struct W(crate::W<DDRPHYC_ODTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ODTCR_SPEC>;
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
impl From<crate::W<DDRPHYC_ODTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ODTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDODT` reader - RDODT"]
pub struct RDODT_R(crate::FieldReader<bool, bool>);
impl RDODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDODT` writer - RDODT"]
pub struct RDODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RDODT_W<'a> {
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
#[doc = "Field `WRODT` reader - WRODT"]
pub struct WRODT_R(crate::FieldReader<bool, bool>);
impl WRODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRODT` writer - WRODT"]
pub struct WRODT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRODT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    pub fn rdodt(&self) -> RDODT_R {
        RDODT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    pub fn wrodt(&self) -> WRODT_R {
        WRODT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RDODT"]
    #[inline(always)]
    pub fn rdodt(&mut self) -> RDODT_W {
        RDODT_W { w: self }
    }
    #[doc = "Bit 16 - WRODT"]
    #[inline(always)]
    pub fn wrodt(&mut self) -> WRODT_W {
        WRODT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC ODTC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_odtcr](index.html) module"]
pub struct DDRPHYC_ODTCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_ODTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_odtcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ODTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_odtcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ODTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_ODTCR to value 0x8421_0000"]
impl crate::Resettable for DDRPHYC_ODTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8421_0000
    }
}
