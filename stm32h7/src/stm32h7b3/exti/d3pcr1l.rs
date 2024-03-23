#[doc = "Register `D3PCR1L` reader"]
pub type R = crate::R<D3PCR1Lrs>;
#[doc = "Register `D3PCR1L` writer"]
pub type W = crate::W<D3PCR1Lrs>;
#[doc = "D3 Pending request clear input signal selection on Event input x = truncate (n/2)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS0 {
    #[doc = "0: DMA ch6 event selected as D3 domain pendclear source"]
    DmaCh6 = 0,
    #[doc = "1: DMA ch7 event selected as D3 domain pendclear source"]
    DmaCh7 = 1,
    #[doc = "2: LPTIM4 out selected as D3 domain pendclear source"]
    Lptim4 = 2,
    #[doc = "3: LPTIM5 out selected as D3 domain pendclear source"]
    Lptim5 = 3,
}
impl From<PCS0> for u8 {
    #[inline(always)]
    fn from(variant: PCS0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCS0 {
    type Ux = u8;
}
#[doc = "Field `PCS0` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS0_R = crate::FieldReader<PCS0>;
impl PCS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCS0 {
        match self.bits {
            0 => PCS0::DmaCh6,
            1 => PCS0::DmaCh7,
            2 => PCS0::Lptim4,
            3 => PCS0::Lptim5,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS0::DmaCh6
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS0::DmaCh7
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS0::Lptim4
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS0::Lptim5
    }
}
#[doc = "Field `PCS0` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub type PCS0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PCS0>;
impl<'a, REG> PCS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA ch6 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS0::DmaCh6)
    }
    #[doc = "DMA ch7 event selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS0::DmaCh7)
    }
    #[doc = "LPTIM4 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS0::Lptim4)
    }
    #[doc = "LPTIM5 out selected as D3 domain pendclear source"]
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS0::Lptim5)
    }
}
#[doc = "Field `PCS1` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS1_R;
#[doc = "Field `PCS2` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS2_R;
#[doc = "Field `PCS3` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS3_R;
#[doc = "Field `PCS4` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS4_R;
#[doc = "Field `PCS5` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS5_R;
#[doc = "Field `PCS6` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS6_R;
#[doc = "Field `PCS7` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS7_R;
#[doc = "Field `PCS8` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS8_R;
#[doc = "Field `PCS9` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS9_R;
#[doc = "Field `PCS10` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS10_R;
#[doc = "Field `PCS11` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS11_R;
#[doc = "Field `PCS12` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS12_R;
#[doc = "Field `PCS13` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS13_R;
#[doc = "Field `PCS14` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS14_R;
#[doc = "Field `PCS15` reader - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_R as PCS15_R;
#[doc = "Field `PCS1` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS1_W;
#[doc = "Field `PCS2` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS2_W;
#[doc = "Field `PCS3` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS3_W;
#[doc = "Field `PCS4` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS4_W;
#[doc = "Field `PCS5` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS5_W;
#[doc = "Field `PCS6` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS6_W;
#[doc = "Field `PCS7` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS7_W;
#[doc = "Field `PCS8` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS8_W;
#[doc = "Field `PCS9` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS9_W;
#[doc = "Field `PCS10` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS10_W;
#[doc = "Field `PCS11` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS11_W;
#[doc = "Field `PCS12` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS12_W;
#[doc = "Field `PCS13` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS13_W;
#[doc = "Field `PCS14` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS14_W;
#[doc = "Field `PCS15` writer - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
pub use PCS0_W as PCS15_W;
impl R {
    #[doc = "Bits 0:1 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs0(&self) -> PCS0_R {
        PCS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs1(&self) -> PCS1_R {
        PCS1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs2(&self) -> PCS2_R {
        PCS2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs3(&self) -> PCS3_R {
        PCS3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs4(&self) -> PCS4_R {
        PCS4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs5(&self) -> PCS5_R {
        PCS5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs6(&self) -> PCS6_R {
        PCS6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs7(&self) -> PCS7_R {
        PCS7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs8(&self) -> PCS8_R {
        PCS8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs9(&self) -> PCS9_R {
        PCS9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs10(&self) -> PCS10_R {
        PCS10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs11(&self) -> PCS11_R {
        PCS11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs12(&self) -> PCS12_R {
        PCS12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs13(&self) -> PCS13_R {
        PCS13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs14(&self) -> PCS14_R {
        PCS14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    pub fn pcs15(&self) -> PCS15_R {
        PCS15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs0(&mut self) -> PCS0_W<D3PCR1Lrs> {
        PCS0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs1(&mut self) -> PCS1_W<D3PCR1Lrs> {
        PCS1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs2(&mut self) -> PCS2_W<D3PCR1Lrs> {
        PCS2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs3(&mut self) -> PCS3_W<D3PCR1Lrs> {
        PCS3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs4(&mut self) -> PCS4_W<D3PCR1Lrs> {
        PCS4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs5(&mut self) -> PCS5_W<D3PCR1Lrs> {
        PCS5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs6(&mut self) -> PCS6_W<D3PCR1Lrs> {
        PCS6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs7(&mut self) -> PCS7_W<D3PCR1Lrs> {
        PCS7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs8(&mut self) -> PCS8_W<D3PCR1Lrs> {
        PCS8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs9(&mut self) -> PCS9_W<D3PCR1Lrs> {
        PCS9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs10(&mut self) -> PCS10_W<D3PCR1Lrs> {
        PCS10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs11(&mut self) -> PCS11_W<D3PCR1Lrs> {
        PCS11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs12(&mut self) -> PCS12_W<D3PCR1Lrs> {
        PCS12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs13(&mut self) -> PCS13_W<D3PCR1Lrs> {
        PCS13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs14(&mut self) -> PCS14_W<D3PCR1Lrs> {
        PCS14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - D3 Pending request clear input signal selection on Event input x = truncate (n/2)"]
    #[inline(always)]
    #[must_use]
    pub fn pcs15(&mut self) -> PCS15_W<D3PCR1Lrs> {
        PCS15_W::new(self, 30)
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pcr1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pcr1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PCR1Lrs;
impl crate::RegisterSpec for D3PCR1Lrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr1l::R`](R) reader structure"]
impl crate::Readable for D3PCR1Lrs {}
#[doc = "`write(|w| ..)` method takes [`d3pcr1l::W`](W) writer structure"]
impl crate::Writable for D3PCR1Lrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets D3PCR1L to value 0"]
impl crate::Resettable for D3PCR1Lrs {
    const RESET_VALUE: u32 = 0;
}
