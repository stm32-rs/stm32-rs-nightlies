#[doc = "Register `D3PCR3H` reader"]
pub struct R(crate::R<D3PCR3H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR3H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR3H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR3H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR3H` writer"]
pub struct W(crate::W<D3PCR3H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR3H_SPEC>;
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
impl From<crate::W<D3PCR3H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR3H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x= truncate N+160/2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS88_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DMA_CH6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DMA_CH7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    LPTIM4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    LPTIM5 = 3,
}
impl From<PCS88_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS88_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS88` reader - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
pub struct PCS88_R(crate::FieldReader<u8, PCS88_A>);
impl PCS88_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS88_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS88_A {
        match self.bits {
            0 => PCS88_A::DMA_CH6,
            1 => PCS88_A::DMA_CH7,
            2 => PCS88_A::LPTIM4,
            3 => PCS88_A::LPTIM5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_CH6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        **self == PCS88_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        **self == PCS88_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        **self == PCS88_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        **self == PCS88_A::LPTIM5
    }
}
impl core::ops::Deref for PCS88_R {
    type Target = crate::FieldReader<u8, PCS88_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS88` writer - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
pub struct PCS88_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS88_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS88_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS88_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS88_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS88_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&self) -> PCS88_R {
        PCS88_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&mut self) -> PCS88_W {
        PCS88_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr3h](index.html) module"]
pub struct D3PCR3H_SPEC;
impl crate::RegisterSpec for D3PCR3H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr3h::R](R) reader structure"]
impl crate::Readable for D3PCR3H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr3h::W](W) writer structure"]
impl crate::Writable for D3PCR3H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PCR3H to value 0"]
impl crate::Resettable for D3PCR3H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
