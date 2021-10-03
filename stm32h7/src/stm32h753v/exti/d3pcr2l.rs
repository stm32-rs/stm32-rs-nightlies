#[doc = "Register `D3PCR2L` reader"]
pub struct R(crate::R<D3PCR2L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR2L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR2L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR2L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR2L` writer"]
pub struct W(crate::W<D3PCR2L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR2L_SPEC>;
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
impl From<crate::W<D3PCR2L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR2L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS35_A = PCS34_A;
#[doc = "Field `PCS35` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS35_R = PCS34_R;
#[doc = "Field `PCS35` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub struct PCS35_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS35_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS35_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS35_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS35_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS35_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS34_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DMA_CH6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DMA_CH7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    LPTIM4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    LPTIM5 = 3,
}
impl From<PCS34_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS34_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS34` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub struct PCS34_R(crate::FieldReader<u8, PCS34_A>);
impl PCS34_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS34_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS34_A {
        match self.bits {
            0 => PCS34_A::DMA_CH6,
            1 => PCS34_A::DMA_CH7,
            2 => PCS34_A::LPTIM4,
            3 => PCS34_A::LPTIM5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_CH6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        **self == PCS34_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        **self == PCS34_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        **self == PCS34_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        **self == PCS34_A::LPTIM5
    }
}
impl core::ops::Deref for PCS34_R {
    type Target = crate::FieldReader<u8, PCS34_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS34` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub struct PCS34_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS34_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS34_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS34_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS34_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS34_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS41_A = PCS34_A;
#[doc = "Field `PCS41` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS41_R = PCS34_R;
#[doc = "Field `PCS41` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub struct PCS41_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS41_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS41_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS41_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS41_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS41_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&self) -> PCS35_R {
        PCS35_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&self) -> PCS34_R {
        PCS34_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&self) -> PCS41_R {
        PCS41_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&mut self) -> PCS35_W {
        PCS35_W { w: self }
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&mut self) -> PCS34_W {
        PCS34_W { w: self }
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&mut self) -> PCS41_W {
        PCS41_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr2l](index.html) module"]
pub struct D3PCR2L_SPEC;
impl crate::RegisterSpec for D3PCR2L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr2l::R](R) reader structure"]
impl crate::Readable for D3PCR2L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr2l::W](W) writer structure"]
impl crate::Writable for D3PCR2L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PCR2L to value 0"]
impl crate::Resettable for D3PCR2L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
