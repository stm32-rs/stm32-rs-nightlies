#[doc = "Register `ETH_DMAC1SFCSR` reader"]
pub struct R(crate::R<ETH_DMAC1SFCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1SFCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1SFCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1SFCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC1SFCSR` writer"]
pub struct W(crate::W<ETH_DMAC1SFCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC1SFCSR_SPEC>;
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
impl From<crate::W<ETH_DMAC1SFCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC1SFCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC` reader - ESC"]
pub struct ESC_R(crate::FieldReader<bool, bool>);
impl ESC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESC` writer - ESC"]
pub struct ESC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESC_W<'a> {
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
#[doc = "Field `ASC` reader - ASC"]
pub struct ASC_R(crate::FieldReader<bool, bool>);
impl ASC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASC` writer - ASC"]
pub struct ASC_W<'a> {
    w: &'a mut W,
}
impl<'a> ASC_W<'a> {
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
#[doc = "Field `RSN` reader - RSN"]
pub struct RSN_R(crate::FieldReader<u8, u8>);
impl RSN_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSN` writer - RSN"]
pub struct RSN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    pub fn esc(&mut self) -> ESC_W {
        ESC_W { w: self }
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    pub fn asc(&mut self) -> ASC_W {
        ASC_W { w: self }
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    pub fn rsn(&mut self) -> RSN_W {
        RSN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel i slot function control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1sfcsr](index.html) module"]
pub struct ETH_DMAC1SFCSR_SPEC;
impl crate::RegisterSpec for ETH_DMAC1SFCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1sfcsr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1SFCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac1sfcsr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC1SFCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC1SFCSR to value 0"]
impl crate::Resettable for ETH_DMAC1SFCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
