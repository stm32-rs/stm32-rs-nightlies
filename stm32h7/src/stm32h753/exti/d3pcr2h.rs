#[doc = "Register `D3PCR2H` reader"]
pub struct R(crate::R<D3PCR2H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR2H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR2H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR2H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR2H` writer"]
pub struct W(crate::W<D3PCR2H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR2H_SPEC>;
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
impl From<crate::W<D3PCR2H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR2H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS48_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DMA_CH6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DMA_CH7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    LPTIM4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    LPTIM5 = 3,
}
impl From<PCS48_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS48_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS48` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS48_R(crate::FieldReader<u8, PCS48_A>);
impl PCS48_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS48_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS48_A {
        match self.bits {
            0 => PCS48_A::DMA_CH6,
            1 => PCS48_A::DMA_CH7,
            2 => PCS48_A::LPTIM4,
            3 => PCS48_A::LPTIM5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_CH6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        **self == PCS48_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        **self == PCS48_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        **self == PCS48_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        **self == PCS48_A::LPTIM5
    }
}
impl core::ops::Deref for PCS48_R {
    type Target = crate::FieldReader<u8, PCS48_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS48` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS48_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS48_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS48_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS48_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS48_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS48_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS48_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS49_A = PCS48_A;
#[doc = "Field `PCS49` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS49_R = PCS48_R;
#[doc = "Field `PCS49` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS49_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS49_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS49_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS49_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS49_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS49_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS50_A = PCS48_A;
#[doc = "Field `PCS50` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS50_R = PCS48_R;
#[doc = "Field `PCS50` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS50_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS50_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS50_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS50_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS50_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS50_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS50_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS51_A = PCS48_A;
#[doc = "Field `PCS51` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS51_R = PCS48_R;
#[doc = "Field `PCS51` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS51_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS51_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS51_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS51_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS51_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS51_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS52_A = PCS48_A;
#[doc = "Field `PCS52` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS52_R = PCS48_R;
#[doc = "Field `PCS52` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS52_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS52_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS52_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS52_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS52_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS52_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS52_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS53_A = PCS48_A;
#[doc = "Field `PCS53` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS53_R = PCS48_R;
#[doc = "Field `PCS53` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub struct PCS53_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS53_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS53_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS53_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS53_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS53_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS53_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&self) -> PCS48_R {
        PCS48_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&self) -> PCS49_R {
        PCS49_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&self) -> PCS50_R {
        PCS50_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&self) -> PCS51_R {
        PCS51_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&self) -> PCS52_R {
        PCS52_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&self) -> PCS53_R {
        PCS53_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&mut self) -> PCS48_W {
        PCS48_W { w: self }
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&mut self) -> PCS49_W {
        PCS49_W { w: self }
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&mut self) -> PCS50_W {
        PCS50_W { w: self }
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&mut self) -> PCS51_W {
        PCS51_W { w: self }
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&mut self) -> PCS52_W {
        PCS52_W { w: self }
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&mut self) -> PCS53_W {
        PCS53_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr2h](index.html) module"]
pub struct D3PCR2H_SPEC;
impl crate::RegisterSpec for D3PCR2H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr2h::R](R) reader structure"]
impl crate::Readable for D3PCR2H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr2h::W](W) writer structure"]
impl crate::Writable for D3PCR2H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PCR2H to value 0"]
impl crate::Resettable for D3PCR2H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
