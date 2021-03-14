#[doc = "Reader of register D3PCR1L"]
pub type R = crate::R<u32, super::D3PCR1L>;
#[doc = "Writer for register D3PCR1L"]
pub type W = crate::W<u32, super::D3PCR1L>;
#[doc = "Register D3PCR1L `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR1L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `PCS0`"]
pub type PCS0_R = crate::R<u8, PCS0_A>;
impl PCS0_R {
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
        *self == PCS0_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS0_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS0_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS0_A::LPTIM5
    }
}
#[doc = "Write proxy for field `PCS0`"]
pub struct PCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS1_A = PCS0_A;
#[doc = "Reader of field `PCS1`"]
pub type PCS1_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS1`"]
pub struct PCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS2_A = PCS0_A;
#[doc = "Reader of field `PCS2`"]
pub type PCS2_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS2`"]
pub struct PCS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS3_A = PCS0_A;
#[doc = "Reader of field `PCS3`"]
pub type PCS3_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS3`"]
pub struct PCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS4_A = PCS0_A;
#[doc = "Reader of field `PCS4`"]
pub type PCS4_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS4`"]
pub struct PCS4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS5_A = PCS0_A;
#[doc = "Reader of field `PCS5`"]
pub type PCS5_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS5`"]
pub struct PCS5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS6_A = PCS0_A;
#[doc = "Reader of field `PCS6`"]
pub type PCS6_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS6`"]
pub struct PCS6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS7_A = PCS0_A;
#[doc = "Reader of field `PCS7`"]
pub type PCS7_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS7`"]
pub struct PCS7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS8_A = PCS0_A;
#[doc = "Reader of field `PCS8`"]
pub type PCS8_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS8`"]
pub struct PCS8_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS9_A = PCS0_A;
#[doc = "Reader of field `PCS9`"]
pub type PCS9_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS9`"]
pub struct PCS9_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS10_A = PCS0_A;
#[doc = "Reader of field `PCS10`"]
pub type PCS10_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS10`"]
pub struct PCS10_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS11_A = PCS0_A;
#[doc = "Reader of field `PCS11`"]
pub type PCS11_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS11`"]
pub struct PCS11_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS12_A = PCS0_A;
#[doc = "Reader of field `PCS12`"]
pub type PCS12_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS12`"]
pub struct PCS12_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS13_A = PCS0_A;
#[doc = "Reader of field `PCS13`"]
pub type PCS13_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS13`"]
pub struct PCS13_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS14_A = PCS0_A;
#[doc = "Reader of field `PCS14`"]
pub type PCS14_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS14`"]
pub struct PCS14_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS15_A = PCS0_A;
#[doc = "Reader of field `PCS15`"]
pub type PCS15_R = crate::R<u8, PCS0_A>;
#[doc = "Write proxy for field `PCS15`"]
pub struct PCS15_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
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
}
