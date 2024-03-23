#[doc = "Register `D3PCR2H` reader"]
pub type R = crate::R<D3PCR2Hrs>;
#[doc = "Register `D3PCR2H` writer"]
pub type W = crate::W<D3PCR2Hrs>;
#[doc = "Pending request clear input signal selection on Event input x= truncate ((n+96)/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS48 {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS48> for u8 {
    #[inline(always)]
    fn from(variant: PCS48) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCS48 {
    type Ux = u8;
}
#[doc = "Field `PCS48` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS48_R = crate::FieldReader<PCS48>;
impl PCS48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCS48 {
        match self.bits {
            0 => PCS48::DmaCh6,
            1 => PCS48::DmaCh7,
            2 => PCS48::Lptim4,
            3 => PCS48::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS48::DmaCh6
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS48::DmaCh7
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS48::Lptim4
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS48::Lptim5
    }
}
#[doc = "Field `PCS48` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type PCS48_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PCS48>;
impl<'a, REG> PCS48_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::Lptim5)
    }
}
#[doc = "Field `PCS49` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS49_R;
#[doc = "Field `PCS50` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS50_R;
#[doc = "Field `PCS51` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS51_R;
#[doc = "Field `PCS52` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS52_R;
#[doc = "Field `PCS53` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_R as PCS53_R;
#[doc = "Field `PCS49` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS49_W;
#[doc = "Field `PCS50` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS50_W;
#[doc = "Field `PCS51` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS51_W;
#[doc = "Field `PCS52` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS52_W;
#[doc = "Field `PCS53` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub use PCS48_W as PCS53_W;
impl R {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&self) -> PCS48_R {
        PCS48_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&self) -> PCS49_R {
        PCS49_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&self) -> PCS50_R {
        PCS50_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&self) -> PCS51_R {
        PCS51_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&self) -> PCS52_R {
        PCS52_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&self) -> PCS53_R {
        PCS53_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs48(&mut self) -> PCS48_W<D3PCR2Hrs> {
        PCS48_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs49(&mut self) -> PCS49_W<D3PCR2Hrs> {
        PCS49_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs50(&mut self) -> PCS50_W<D3PCR2Hrs> {
        PCS50_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs51(&mut self) -> PCS51_W<D3PCR2Hrs> {
        PCS51_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs52(&mut self) -> PCS52_W<D3PCR2Hrs> {
        PCS52_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs53(&mut self) -> PCS53_W<D3PCR2Hrs> {
        PCS53_W::new(self, 10)
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr2h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr2h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR2Hrs;
impl crate::RegisterSpec for D3PCR2Hrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr2h::R`](R) reader structure"]
impl crate::Readable for D3PCR2Hrs {}
#[doc = "`write(|w| ..)` method takes [`d3pcr2h::W`](W) writer structure"]
impl crate::Writable for D3PCR2Hrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PCR2H to value 0"]
impl crate::Resettable for D3PCR2Hrs {
    const RESET_VALUE: u32 = 0;
}
