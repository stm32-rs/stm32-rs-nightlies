#[doc = "Register `EXMIN` reader"]
pub struct R(crate::R<EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXMIN` writer"]
pub struct W(crate::W<EXMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXMIN_SPEC>;
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
impl From<crate::W<EXMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXMINCH` reader - Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\\[23:0\\]. Bits are cleared by reading of this register."]
pub struct EXMINCH_R(crate::FieldReader<u8, u8>);
impl EXMINCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXMINCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMINCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMIN` reader - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
pub struct EXMIN_R(crate::FieldReader<u32, u32>);
impl EXMIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMIN` writer - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
pub struct EXMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\\[23:0\\]. Bits are cleared by reading of this register."]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W {
        EXMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exmin](index.html) module"]
pub struct EXMIN_SPEC;
impl crate::RegisterSpec for EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exmin::R](R) reader structure"]
impl crate::Readable for EXMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exmin::W](W) writer structure"]
impl crate::Writable for EXMIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for EXMIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ff00
    }
}
