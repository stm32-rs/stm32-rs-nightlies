#[doc = "Register `ETH_MACTSAR` reader"]
pub struct R(crate::R<ETH_MACTSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACTSAR` writer"]
pub struct W(crate::W<ETH_MACTSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACTSAR_SPEC>;
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
impl From<crate::W<ETH_MACTSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACTSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSAR` reader - TSAR"]
pub struct TSAR_R(crate::FieldReader<u32, u32>);
impl TSAR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAR` writer - TSAR"]
pub struct TSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSAR"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSAR"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W {
        TSAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsar](index.html) module"]
pub struct ETH_MACTSAR_SPEC;
impl crate::RegisterSpec for ETH_MACTSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactsar::R](R) reader structure"]
impl crate::Readable for ETH_MACTSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mactsar::W](W) writer structure"]
impl crate::Writable for ETH_MACTSAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACTSAR to value 0"]
impl crate::Resettable for ETH_MACTSAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
