#[doc = "Reader of register D3PCR1H"]
pub type R = crate::R<u32, super::D3PCR1H>;
#[doc = "Writer for register D3PCR1H"]
pub type W = crate::W<u32, super::D3PCR1H>;
#[doc = "Register D3PCR1H `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR1H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `PCS19`"]
pub type PCS19_R = crate::R<u8, PCS19_A>;
impl PCS19_R {
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
        *self == PCS19_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS19_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS19_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS19_A::LPTIM5
    }
}
#[doc = "Write proxy for field `PCS19`"]
pub struct PCS19_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS19_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS20_A = PCS19_A;
#[doc = "Reader of field `PCS20`"]
pub type PCS20_R = crate::R<u8, PCS19_A>;
#[doc = "Write proxy for field `PCS20`"]
pub struct PCS20_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS20_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS21_A = PCS19_A;
#[doc = "Reader of field `PCS21`"]
pub type PCS21_R = crate::R<u8, PCS19_A>;
#[doc = "Write proxy for field `PCS21`"]
pub struct PCS21_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS21_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS25_A = PCS19_A;
#[doc = "Reader of field `PCS25`"]
pub type PCS25_R = crate::R<u8, PCS19_A>;
#[doc = "Write proxy for field `PCS25`"]
pub struct PCS25_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS25_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
}
