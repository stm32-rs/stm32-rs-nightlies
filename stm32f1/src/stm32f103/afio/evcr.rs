#[doc = "Register `EVCR` reader"]
pub struct R(crate::R<EVCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCR` writer"]
pub struct W(crate::W<EVCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCR_SPEC>;
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
impl From<crate::W<EVCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - Pin selection"]
pub struct PIN_R(crate::FieldReader<u8, u8>);
impl PIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN` writer - Pin selection"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PORT` reader - Port selection"]
pub struct PORT_R(crate::FieldReader<u8, u8>);
impl PORT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT` writer - Port selection"]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `EVOE` reader - Event Output Enable"]
pub struct EVOE_R(crate::FieldReader<bool, bool>);
impl EVOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVOE` writer - Event Output Enable"]
pub struct EVOE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVOE_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&self) -> EVOE_R {
        EVOE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin selection"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
    #[doc = "Bits 4:6 - Port selection"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
    #[doc = "Bit 7 - Event Output Enable"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EVOE_W {
        EVOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control Register (AFIO_EVCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evcr](index.html) module"]
pub struct EVCR_SPEC;
impl crate::RegisterSpec for EVCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evcr::R](R) reader structure"]
impl crate::Readable for EVCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evcr::W](W) writer structure"]
impl crate::Writable for EVCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCR to value 0"]
impl crate::Resettable for EVCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
