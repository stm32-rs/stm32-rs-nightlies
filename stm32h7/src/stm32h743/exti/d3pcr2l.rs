#[doc = "Register `D3PCR2L` reader"]
pub type R = crate::R<D3PCR2Lrs>;
#[doc = "Register `D3PCR2L` writer"]
pub type W = crate::W<D3PCR2Lrs>;
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS34 {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS34> for u8 {
    #[inline(always)]
    fn from(variant: PCS34) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCS34 {
    type Ux = u8;
}
#[doc = "Field `PCS34` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS34_R = crate::FieldReader<PCS34>;
impl PCS34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCS34 {
        match self.bits {
            0 => PCS34::DmaCh6,
            1 => PCS34::DmaCh7,
            2 => PCS34::Lptim4,
            3 => PCS34::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS34::DmaCh6
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS34::DmaCh7
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS34::Lptim4
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS34::Lptim5
    }
}
#[doc = "Field `PCS34` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type PCS34_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PCS34>;
impl<'a, REG> PCS34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::Lptim5)
    }
}
#[doc = "Field `PCS35` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub use PCS34_R as PCS35_R;
#[doc = "Field `PCS41` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub use PCS34_R as PCS41_R;
#[doc = "Field `PCS35` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub use PCS34_W as PCS35_W;
#[doc = "Field `PCS41` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub use PCS34_W as PCS41_W;
impl R {
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&self) -> PCS34_R {
        PCS34_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&self) -> PCS35_R {
        PCS35_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&self) -> PCS41_R {
        PCS41_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs34(&mut self) -> PCS34_W<D3PCR2Lrs> {
        PCS34_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs35(&mut self) -> PCS35_W<D3PCR2Lrs> {
        PCS35_W::new(self, 6)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs41(&mut self) -> PCS41_W<D3PCR2Lrs> {
        PCS41_W::new(self, 18)
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr2l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr2l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR2Lrs;
impl crate::RegisterSpec for D3PCR2Lrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr2l::R`](R) reader structure"]
impl crate::Readable for D3PCR2Lrs {}
#[doc = "`write(|w| ..)` method takes [`d3pcr2l::W`](W) writer structure"]
impl crate::Writable for D3PCR2Lrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PCR2L to value 0"]
impl crate::Resettable for D3PCR2Lrs {
    const RESET_VALUE: u32 = 0;
}
