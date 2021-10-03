#[doc = "Register `DDRCTRL_DERATEEN` reader"]
pub struct R(crate::R<DDRCTRL_DERATEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DERATEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DERATEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DERATEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DERATEEN` writer"]
pub struct W(crate::W<DDRCTRL_DERATEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DERATEEN_SPEC>;
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
impl From<crate::W<DDRCTRL_DERATEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DERATEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DERATE_ENABLE` reader - DERATE_ENABLE"]
pub struct DERATE_ENABLE_R(crate::FieldReader<bool, bool>);
impl DERATE_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DERATE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERATE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERATE_ENABLE` writer - DERATE_ENABLE"]
pub struct DERATE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERATE_ENABLE_W<'a> {
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
#[doc = "Field `DERATE_VALUE` reader - DERATE_VALUE"]
pub struct DERATE_VALUE_R(crate::FieldReader<u8, u8>);
impl DERATE_VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DERATE_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERATE_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERATE_VALUE` writer - DERATE_VALUE"]
pub struct DERATE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERATE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `DERATE_BYTE` reader - DERATE_BYTE"]
pub struct DERATE_BYTE_R(crate::FieldReader<u8, u8>);
impl DERATE_BYTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DERATE_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERATE_BYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERATE_BYTE` writer - DERATE_BYTE"]
pub struct DERATE_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DERATE_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DERATE_ENABLE"]
    #[inline(always)]
    pub fn derate_enable(&self) -> DERATE_ENABLE_R {
        DERATE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - DERATE_VALUE"]
    #[inline(always)]
    pub fn derate_value(&self) -> DERATE_VALUE_R {
        DERATE_VALUE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - DERATE_BYTE"]
    #[inline(always)]
    pub fn derate_byte(&self) -> DERATE_BYTE_R {
        DERATE_BYTE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DERATE_ENABLE"]
    #[inline(always)]
    pub fn derate_enable(&mut self) -> DERATE_ENABLE_W {
        DERATE_ENABLE_W { w: self }
    }
    #[doc = "Bits 1:2 - DERATE_VALUE"]
    #[inline(always)]
    pub fn derate_value(&mut self) -> DERATE_VALUE_W {
        DERATE_VALUE_W { w: self }
    }
    #[doc = "Bits 4:7 - DERATE_BYTE"]
    #[inline(always)]
    pub fn derate_byte(&mut self) -> DERATE_BYTE_W {
        DERATE_BYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL temperature derate enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_derateen](index.html) module"]
pub struct DDRCTRL_DERATEEN_SPEC;
impl crate::RegisterSpec for DDRCTRL_DERATEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_derateen::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DERATEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_derateen::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DERATEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DERATEEN to value 0"]
impl crate::Resettable for DDRCTRL_DERATEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
