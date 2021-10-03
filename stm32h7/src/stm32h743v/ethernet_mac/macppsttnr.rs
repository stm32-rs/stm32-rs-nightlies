#[doc = "Register `MACPPSTTNR` reader"]
pub struct R(crate::R<MACPPSTTNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPPSTTNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPPSTTNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPPSTTNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPPSTTNR` writer"]
pub struct W(crate::W<MACPPSTTNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPPSTTNR_SPEC>;
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
impl From<crate::W<MACPPSTTNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPPSTTNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSL0` reader - Target Time Low for PPS Register"]
pub struct TTSL0_R(crate::FieldReader<u32, u32>);
impl TTSL0_R {
    pub(crate) fn new(bits: u32) -> Self {
        TTSL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTSL0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTSL0` writer - Target Time Low for PPS Register"]
pub struct TTSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `TRGTBUSY0` reader - PPS Target Time Register Busy"]
pub struct TRGTBUSY0_R(crate::FieldReader<bool, bool>);
impl TRGTBUSY0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGTBUSY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTBUSY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTBUSY0` writer - PPS Target Time Register Busy"]
pub struct TRGTBUSY0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGTBUSY0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Time Low for PPS Register"]
    #[inline(always)]
    pub fn ttsl0(&mut self) -> TTSL0_W {
        TTSL0_W { w: self }
    }
    #[doc = "Bit 31 - PPS Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W {
        TRGTBUSY0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS target time nanoseconds register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macppsttnr](index.html) module"]
pub struct MACPPSTTNR_SPEC;
impl crate::RegisterSpec for MACPPSTTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macppsttnr::R](R) reader structure"]
impl crate::Readable for MACPPSTTNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macppsttnr::W](W) writer structure"]
impl crate::Writable for MACPPSTTNR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPPSTTNR to value 0"]
impl crate::Resettable for MACPPSTTNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
