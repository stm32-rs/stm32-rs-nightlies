#[doc = "Register `OPTCR2` reader"]
pub struct R(crate::R<OPTCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR2` writer"]
pub struct W(crate::W<OPTCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR2_SPEC>;
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
impl From<crate::W<OPTCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP_RDP` reader - PCROP zone preserved when RDP level decreased"]
pub struct PCROP_RDP_R(crate::FieldReader<bool, bool>);
impl PCROP_RDP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCROP_RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP_RDP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP_RDP` writer - PCROP zone preserved when RDP level decreased"]
pub struct PCROP_RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP_RDP_W<'a> {
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
#[doc = "Field `PCROPi` reader - PCROP option byte"]
pub struct PCROPI_R(crate::FieldReader<u8, u8>);
impl PCROPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROPi` writer - PCROP option byte"]
pub struct PCROPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    pub fn pcropi(&self) -> PCROPI_R {
        PCROPI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W {
        PCROP_RDP_W { w: self }
    }
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    pub fn pcropi(&mut self) -> PCROPI_W {
        PCROPI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr2](index.html) module"]
pub struct OPTCR2_SPEC;
impl crate::RegisterSpec for OPTCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr2::R](R) reader structure"]
impl crate::Readable for OPTCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr2::W](W) writer structure"]
impl crate::Writable for OPTCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCR2 to value 0x8000_00ff"]
impl crate::Resettable for OPTCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_00ff
    }
}
