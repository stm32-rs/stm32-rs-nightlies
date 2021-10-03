#[doc = "Register `FDCAN_TTTS` reader"]
pub struct R(crate::R<FDCAN_TTTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTTS` writer"]
pub struct W(crate::W<FDCAN_TTTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTTS_SPEC>;
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
impl From<crate::W<FDCAN_TTTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTDEL` reader - Stop watch trigger input selection"]
pub struct SWTDEL_R(crate::FieldReader<u8, u8>);
impl SWTDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWTDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWTDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWTDEL` writer - Stop watch trigger input selection"]
pub struct SWTDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `EVTSEL` reader - Event trigger input selection"]
pub struct EVTSEL_R(crate::FieldReader<u8, u8>);
impl EVTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVTSEL` writer - Event trigger input selection"]
pub struct EVTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EVTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    pub fn swtdel(&mut self) -> SWTDEL_W {
        SWTDEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    pub fn evtsel(&mut self) -> EVTSEL_W {
        EVTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Trigger Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttts](index.html) module"]
pub struct FDCAN_TTTS_SPEC;
impl crate::RegisterSpec for FDCAN_TTTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttts::R](R) reader structure"]
impl crate::Readable for FDCAN_TTTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttts::W](W) writer structure"]
impl crate::Writable for FDCAN_TTTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_TTTS to value 0"]
impl crate::Resettable for FDCAN_TTTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
