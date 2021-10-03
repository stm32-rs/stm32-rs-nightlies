#[doc = "Register `D3PCR1H` reader"]
pub struct R(crate::R<D3PCR1H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR1H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR1H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR1H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR1H` writer"]
pub struct W(crate::W<D3PCR1H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR1H_SPEC>;
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
impl From<crate::W<D3PCR1H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR1H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS19_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DMA_CH6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DMA_CH7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    LPTIM4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    LPTIM5 = 3,
}
impl From<PCS19_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS19_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS19` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub struct PCS19_R(crate::FieldReader<u8, PCS19_A>);
impl PCS19_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS19_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS19_A {
        match self.bits {
            0 => PCS19_A::DMA_CH6,
            1 => PCS19_A::DMA_CH7,
            2 => PCS19_A::LPTIM4,
            3 => PCS19_A::LPTIM5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_CH6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        **self == PCS19_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        **self == PCS19_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        **self == PCS19_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        **self == PCS19_A::LPTIM5
    }
}
impl core::ops::Deref for PCS19_R {
    type Target = crate::FieldReader<u8, PCS19_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS19` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub struct PCS19_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS19_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS19_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS19_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS19_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS19_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS20_A = PCS19_A;
#[doc = "Field `PCS20` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS20_R = PCS19_R;
#[doc = "Field `PCS20` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub struct PCS20_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS20_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS20_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS20_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS20_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS20_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS21_A = PCS19_A;
#[doc = "Field `PCS21` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS21_R = PCS19_R;
#[doc = "Field `PCS21` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub struct PCS21_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS21_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS21_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS21_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS21_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS21_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS25_A = PCS19_A;
#[doc = "Field `PCS25` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS25_R = PCS19_R;
#[doc = "Field `PCS25` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub struct PCS25_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS25_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS25_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS25_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS25_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS25_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&self) -> PCS19_R {
        PCS19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&self) -> PCS20_R {
        PCS20_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&self) -> PCS21_R {
        PCS21_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&self) -> PCS25_R {
        PCS25_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&mut self) -> PCS19_W {
        PCS19_W { w: self }
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&mut self) -> PCS20_W {
        PCS20_W { w: self }
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&mut self) -> PCS21_W {
        PCS21_W { w: self }
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&mut self) -> PCS25_W {
        PCS25_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr1h](index.html) module"]
pub struct D3PCR1H_SPEC;
impl crate::RegisterSpec for D3PCR1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr1h::R](R) reader structure"]
impl crate::Readable for D3PCR1H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr1h::W](W) writer structure"]
impl crate::Writable for D3PCR1H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PCR1H to value 0"]
impl crate::Resettable for D3PCR1H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
