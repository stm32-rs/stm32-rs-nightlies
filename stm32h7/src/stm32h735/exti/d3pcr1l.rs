#[doc = "Register `D3PCR1L` reader"]
pub struct R(crate::R<D3PCR1L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR1L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR1L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR1L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PCR1L` writer"]
pub struct W(crate::W<D3PCR1L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR1L_SPEC>;
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
impl From<crate::W<D3PCR1L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR1L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS0_A {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DMA_CH6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DMA_CH7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    LPTIM4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    LPTIM5 = 3,
}
impl From<PCS0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCS0` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS0_R(crate::FieldReader<u8, PCS0_A>);
impl PCS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS0_A {
        match self.bits {
            0 => PCS0_A::DMA_CH6,
            1 => PCS0_A::DMA_CH7,
            2 => PCS0_A::LPTIM4,
            3 => PCS0_A::LPTIM5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_CH6`"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        **self == PCS0_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        **self == PCS0_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        **self == PCS0_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        **self == PCS0_A::LPTIM5
    }
}
impl core::ops::Deref for PCS0_R {
    type Target = crate::FieldReader<u8, PCS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS0` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS0_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS0_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS0_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS0_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS1_A = PCS0_A;
#[doc = "Field `PCS1` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS1_R = PCS0_R;
#[doc = "Field `PCS1` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS1_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS1_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS1_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS1_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS2_A = PCS0_A;
#[doc = "Field `PCS2` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS2_R = PCS0_R;
#[doc = "Field `PCS2` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS2_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS2_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS2_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS2_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS3_A = PCS0_A;
#[doc = "Field `PCS3` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS3_R = PCS0_R;
#[doc = "Field `PCS3` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS3_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS3_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS3_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS3_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS4_A = PCS0_A;
#[doc = "Field `PCS4` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS4_R = PCS0_R;
#[doc = "Field `PCS4` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS4_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS4_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS4_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS4_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS5_A = PCS0_A;
#[doc = "Field `PCS5` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS5_R = PCS0_R;
#[doc = "Field `PCS5` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS5_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS5_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS5_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS5_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS6_A = PCS0_A;
#[doc = "Field `PCS6` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS6_R = PCS0_R;
#[doc = "Field `PCS6` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS6_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS6_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS6_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS6_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS7_A = PCS0_A;
#[doc = "Field `PCS7` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS7_R = PCS0_R;
#[doc = "Field `PCS7` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS7_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS7_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS7_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS7_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS8_A = PCS0_A;
#[doc = "Field `PCS8` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS8_R = PCS0_R;
#[doc = "Field `PCS8` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS8_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS8_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS8_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS8_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS8_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS9_A = PCS0_A;
#[doc = "Field `PCS9` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS9_R = PCS0_R;
#[doc = "Field `PCS9` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS9_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS9_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS9_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS9_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS9_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS10_A = PCS0_A;
#[doc = "Field `PCS10` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS10_R = PCS0_R;
#[doc = "Field `PCS10` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS10_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS10_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS10_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS10_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS10_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS11_A = PCS0_A;
#[doc = "Field `PCS11` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS11_R = PCS0_R;
#[doc = "Field `PCS11` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS11_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS11_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS11_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS11_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS11_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS12_A = PCS0_A;
#[doc = "Field `PCS12` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS12_R = PCS0_R;
#[doc = "Field `PCS12` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS12_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS12_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS12_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS12_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS12_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS13_A = PCS0_A;
#[doc = "Field `PCS13` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS13_R = PCS0_R;
#[doc = "Field `PCS13` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS13_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS13_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS13_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS13_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS13_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS14_A = PCS0_A;
#[doc = "Field `PCS14` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS14_R = PCS0_R;
#[doc = "Field `PCS14` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS14_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS14_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS14_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS14_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS14_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS15_A = PCS0_A;
#[doc = "Field `PCS15` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS15_R = PCS0_R;
#[doc = "Field `PCS15` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub struct PCS15_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS15_A::DMA_CH6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS15_A::DMA_CH7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS15_A::LPTIM4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS15_A::LPTIM5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs0(&self) -> PCS0_R {
        PCS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs1(&self) -> PCS1_R {
        PCS1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs2(&self) -> PCS2_R {
        PCS2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs3(&self) -> PCS3_R {
        PCS3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs4(&self) -> PCS4_R {
        PCS4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs5(&self) -> PCS5_R {
        PCS5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs6(&self) -> PCS6_R {
        PCS6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs7(&self) -> PCS7_R {
        PCS7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs8(&self) -> PCS8_R {
        PCS8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs9(&self) -> PCS9_R {
        PCS9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs10(&self) -> PCS10_R {
        PCS10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs11(&self) -> PCS11_R {
        PCS11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs12(&self) -> PCS12_R {
        PCS12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs13(&self) -> PCS13_R {
        PCS13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs14(&self) -> PCS14_R {
        PCS14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs15(&self) -> PCS15_R {
        PCS15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs0(&mut self) -> PCS0_W {
        PCS0_W { w: self }
    }
    #[doc = "Bits 2:3 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs1(&mut self) -> PCS1_W {
        PCS1_W { w: self }
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs2(&mut self) -> PCS2_W {
        PCS2_W { w: self }
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs3(&mut self) -> PCS3_W {
        PCS3_W { w: self }
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs4(&mut self) -> PCS4_W {
        PCS4_W { w: self }
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs5(&mut self) -> PCS5_W {
        PCS5_W { w: self }
    }
    #[doc = "Bits 12:13 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs6(&mut self) -> PCS6_W {
        PCS6_W { w: self }
    }
    #[doc = "Bits 14:15 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs7(&mut self) -> PCS7_W {
        PCS7_W { w: self }
    }
    #[doc = "Bits 16:17 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs8(&mut self) -> PCS8_W {
        PCS8_W { w: self }
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs9(&mut self) -> PCS9_W {
        PCS9_W { w: self }
    }
    #[doc = "Bits 20:21 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs10(&mut self) -> PCS10_W {
        PCS10_W { w: self }
    }
    #[doc = "Bits 22:23 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs11(&mut self) -> PCS11_W {
        PCS11_W { w: self }
    }
    #[doc = "Bits 24:25 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs12(&mut self) -> PCS12_W {
        PCS12_W { w: self }
    }
    #[doc = "Bits 26:27 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs13(&mut self) -> PCS13_W {
        PCS13_W { w: self }
    }
    #[doc = "Bits 28:29 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs14(&mut self) -> PCS14_W {
        PCS14_W { w: self }
    }
    #[doc = "Bits 30:31 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs15(&mut self) -> PCS15_W {
        PCS15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pcr1l](index.html) module"]
pub struct D3PCR1L_SPEC;
impl crate::RegisterSpec for D3PCR1L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pcr1l::R](R) reader structure"]
impl crate::Readable for D3PCR1L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pcr1l::W](W) writer structure"]
impl crate::Writable for D3PCR1L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PCR1L to value 0"]
impl crate::Resettable for D3PCR1L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
