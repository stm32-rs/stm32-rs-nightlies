#[doc = "Reader of register D3PCR3H"]
pub type R = crate::R<u32, super::D3PCR3H>;
#[doc = "Writer for register D3PCR3H"]
pub type W = crate::W<u32, super::D3PCR3H>;
#[doc = "Register D3PCR3H `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR3H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `PCS88`"]
pub type PCS88_R = crate::R<u8, PCS88_A>;
impl PCS88_R {
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
        *self == PCS88_A::DMA_CH6
    }
    #[doc = "Checks if the value of the field is `DMA_CH7`"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS88_A::DMA_CH7
    }
    #[doc = "Checks if the value of the field is `LPTIM4`"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS88_A::LPTIM4
    }
    #[doc = "Checks if the value of the field is `LPTIM5`"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS88_A::LPTIM5
    }
}
#[doc = "Write proxy for field `PCS88`"]
pub struct PCS88_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS88_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
}
