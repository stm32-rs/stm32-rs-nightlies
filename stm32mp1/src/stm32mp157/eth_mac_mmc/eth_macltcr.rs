#[doc = "Register `ETH_MACLTCR` reader"]
pub struct R(crate::R<ETH_MACLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLTCR` writer"]
pub struct W(crate::W<ETH_MACLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLTCR_SPEC>;
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
impl From<crate::W<ETH_MACLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWT` reader - TWT"]
pub struct TWT_R(crate::FieldReader<u16, u16>);
impl TWT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWT` writer - TWT"]
pub struct TWT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `LST` reader - LST"]
pub struct LST_R(crate::FieldReader<u16, u16>);
impl LST_R {
    pub(crate) fn new(bits: u16) -> Self {
        LST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LST` writer - LST"]
pub struct LST_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&mut self) -> TWT_W {
        TWT_W { w: self }
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W {
        LST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macltcr](index.html) module"]
pub struct ETH_MACLTCR_SPEC;
impl crate::RegisterSpec for ETH_MACLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macltcr::R](R) reader structure"]
impl crate::Readable for ETH_MACLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macltcr::W](W) writer structure"]
impl crate::Writable for ETH_MACLTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for ETH_MACLTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03e8_0000
    }
}
