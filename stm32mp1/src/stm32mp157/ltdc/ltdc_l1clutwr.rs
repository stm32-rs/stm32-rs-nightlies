#[doc = "Register `LTDC_L1CLUTWR` writer"]
pub struct W(crate::W<LTDC_L1CLUTWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CLUTWR_SPEC>;
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
impl From<crate::W<LTDC_L1CLUTWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CLUTWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE` writer - BLUE"]
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `GREEN` writer - GREEN"]
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RED` writer - RED"]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CLUTADD` writer - CLUTADD"]
pub struct CLUTADD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUTADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Bits 24:31 - CLUTADD"]
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W {
        CLUTADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the CLUT address and the RGB value.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1clutwr](index.html) module"]
pub struct LTDC_L1CLUTWR_SPEC;
impl crate::RegisterSpec for LTDC_L1CLUTWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1clutwr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CLUTWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L1CLUTWR to value 0"]
impl crate::Resettable for LTDC_L1CLUTWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
