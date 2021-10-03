#[doc = "Register `DMACTxDLAR` reader"]
pub struct R(crate::R<DMACTXDLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTXDLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTXDLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTXDLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTxDLAR` writer"]
pub struct W(crate::W<DMACTXDLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTXDLAR_SPEC>;
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
impl From<crate::W<DMACTXDLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTXDLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDESLA` reader - Start of Transmit List"]
pub struct TDESLA_R(crate::FieldReader<u32, u32>);
impl TDESLA_R {
    pub(crate) fn new(bits: u32) -> Self {
        TDESLA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDESLA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDESLA` writer - Start of Transmit List"]
pub struct TDESLA_W<'a> {
    w: &'a mut W,
}
impl<'a> TDESLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&self) -> TDESLA_R {
        TDESLA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of Transmit List"]
    #[inline(always)]
    pub fn tdesla(&mut self) -> TDESLA_W {
        TDESLA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Tx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_dlar](index.html) module"]
pub struct DMACTXDLAR_SPEC;
impl crate::RegisterSpec for DMACTXDLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactx_dlar::R](R) reader structure"]
impl crate::Readable for DMACTXDLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactx_dlar::W](W) writer structure"]
impl crate::Writable for DMACTXDLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTxDLAR to value 0"]
impl crate::Resettable for DMACTXDLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
