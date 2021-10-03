#[doc = "Register `ETH_DMAC0CR` reader"]
pub struct R(crate::R<ETH_DMAC0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0CR` writer"]
pub struct W(crate::W<ETH_DMAC0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0CR_SPEC>;
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
impl From<crate::W<ETH_DMAC0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSS` reader - MSS"]
pub struct MSS_R(crate::FieldReader<u16, u16>);
impl MSS_R {
    pub(crate) fn new(bits: u16) -> Self {
        MSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSS` writer - MSS"]
pub struct MSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `PBLX8` reader - PBLX8"]
pub struct PBLX8_R(crate::FieldReader<bool, bool>);
impl PBLX8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBLX8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBLX8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBLX8` writer - PBLX8"]
pub struct PBLX8_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLX8_W<'a> {
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
#[doc = "Field `DSL` reader - DSL"]
pub struct DSL_R(crate::FieldReader<u8, u8>);
impl DSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSL` writer - DSL"]
pub struct DSL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - MSS"]
    #[inline(always)]
    pub fn mss(&mut self) -> MSS_W {
        MSS_W { w: self }
    }
    #[doc = "Bit 16 - PBLX8"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W {
        PBLX8_W { w: self }
    }
    #[doc = "Bits 18:20 - DSL"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0cr](index.html) module"]
pub struct ETH_DMAC0CR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0cr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0cr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC0CR to value 0"]
impl crate::Resettable for ETH_DMAC0CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
