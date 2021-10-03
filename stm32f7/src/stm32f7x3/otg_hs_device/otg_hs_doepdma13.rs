#[doc = "Register `OTG_HS_DOEPDMA13` reader"]
pub struct R(crate::R<OTG_HS_DOEPDMA13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_DOEPDMA13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_DOEPDMA13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_DOEPDMA13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_DOEPDMA13` writer"]
pub struct W(crate::W<OTG_HS_DOEPDMA13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_DOEPDMA13_SPEC>;
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
impl From<crate::W<OTG_HS_DOEPDMA13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_DOEPDMA13_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Device channel-x DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepdma13](index.html) module"]
pub struct OTG_HS_DOEPDMA13_SPEC;
impl crate::RegisterSpec for OTG_HS_DOEPDMA13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_doepdma13::R](R) reader structure"]
impl crate::Readable for OTG_HS_DOEPDMA13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepdma13::W](W) writer structure"]
impl crate::Writable for OTG_HS_DOEPDMA13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HS_DOEPDMA13 to value 0"]
impl crate::Resettable for OTG_HS_DOEPDMA13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
