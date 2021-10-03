#[doc = "Register `PMC` reader"]
pub struct R(crate::R<PMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC` writer"]
pub struct W(crate::W<PMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SPEC>;
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
impl From<crate::W<PMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_PU` reader - USB pull-up"]
pub struct USB_PU_R(crate::FieldReader<bool, bool>);
impl USB_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_PU` writer - USB pull-up"]
pub struct USB_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_PU_W<'a> {
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
#[doc = "Field `LCD_CAPA` reader - decoupling capacitance connection"]
pub struct LCD_CAPA_R(crate::FieldReader<u8, u8>);
impl LCD_CAPA_R {
    pub(crate) fn new(bits: u8) -> Self {
        LCD_CAPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCD_CAPA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCD_CAPA` writer - decoupling capacitance connection"]
pub struct LCD_CAPA_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_CAPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    pub fn usb_pu(&self) -> USB_PU_R {
        USB_PU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    pub fn lcd_capa(&self) -> LCD_CAPA_R {
        LCD_CAPA_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB pull-up"]
    #[inline(always)]
    pub fn usb_pu(&mut self) -> USB_PU_W {
        USB_PU_W { w: self }
    }
    #[doc = "Bits 1:5 - decoupling capacitance connection"]
    #[inline(always)]
    pub fn lcd_capa(&mut self) -> LCD_CAPA_W {
        LCD_CAPA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc](index.html) module"]
pub struct PMC_SPEC;
impl crate::RegisterSpec for PMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc::R](R) reader structure"]
impl crate::Readable for PMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc::W](W) writer structure"]
impl crate::Writable for PMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC to value 0"]
impl crate::Resettable for PMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
