#[doc = "Register `D3PCR1H` reader"]
pub type R = crate::R<D3PCR1Hrs>;
#[doc = "Register `D3PCR1H` writer"]
pub type W = crate::W<D3PCR1Hrs>;
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS19 {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS19> for u8 {
    #[inline(always)]
    fn from(variant: PCS19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCS19 {
    type Ux = u8;
}
#[doc = "Field `PCS19` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS19_R = crate::FieldReader<PCS19>;
impl PCS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCS19 {
        match self.bits {
            0 => PCS19::DmaCh6,
            1 => PCS19::DmaCh7,
            2 => PCS19::Lptim4,
            3 => PCS19::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS19::DmaCh6
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS19::DmaCh7
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS19::Lptim4
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS19::Lptim5
    }
}
#[doc = "Field `PCS19` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type PCS19_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PCS19>;
impl<'a, REG> PCS19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::Lptim5)
    }
}
#[doc = "Field `PCS20` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_R as PCS20_R;
#[doc = "Field `PCS21` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_R as PCS21_R;
#[doc = "Field `PCS25` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_R as PCS25_R;
#[doc = "Field `PCS20` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_W as PCS20_W;
#[doc = "Field `PCS21` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_W as PCS21_W;
#[doc = "Field `PCS25` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub use PCS19_W as PCS25_W;
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&self) -> PCS19_R {
        PCS19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&self) -> PCS20_R {
        PCS20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&self) -> PCS21_R {
        PCS21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&self) -> PCS25_R {
        PCS25_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs19(&mut self) -> PCS19_W<D3PCR1Hrs> {
        PCS19_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs20(&mut self) -> PCS20_W<D3PCR1Hrs> {
        PCS20_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs21(&mut self) -> PCS21_W<D3PCR1Hrs> {
        PCS21_W::new(self, 10)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs25(&mut self) -> PCS25_W<D3PCR1Hrs> {
        PCS25_W::new(self, 18)
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR1Hrs;
impl crate::RegisterSpec for D3PCR1Hrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr1h::R`](R) reader structure"]
impl crate::Readable for D3PCR1Hrs {}
#[doc = "`write(|w| ..)` method takes [`d3pcr1h::W`](W) writer structure"]
impl crate::Writable for D3PCR1Hrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PCR1H to value 0"]
impl crate::Resettable for D3PCR1Hrs {
    const RESET_VALUE: u32 = 0;
}
