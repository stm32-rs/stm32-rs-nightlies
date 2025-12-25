///Register `D3PCR2H` reader
pub type R = crate::R<D3PCR2Hrs>;
///Register `D3PCR2H` writer
pub type W = crate::W<D3PCR2Hrs>;
/**Pending request clear input signal selection on Event input x= truncate ((n+96)/2)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS48 {
    ///0: DMA ch6 event selected as D3 domain pendclear source
    DmaCh6 = 0,
    ///1: DMA ch7 event selected as D3 domain pendclear source
    DmaCh7 = 1,
    ///2: LPTIM4 out selected as D3 domain pendclear source
    Lptim4 = 2,
    ///3: LPTIM5 out selected as D3 domain pendclear source
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
impl crate::IsEnum for PCS48 {}
///Field `PCS48` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub type PCS48_R = crate::FieldReader<PCS48>;
impl PCS48_R {
    ///Get enumerated values variant
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
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS48::DmaCh6
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS48::DmaCh7
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS48::Lptim4
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS48::Lptim5
    }
}
///Field `PCS48` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub type PCS48_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PCS48, crate::Safe>;
impl<'a, REG> PCS48_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::DmaCh6)
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::DmaCh7)
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::Lptim4)
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS48::Lptim5)
    }
}
///Field `PCS49` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_R as PCS49_R;
///Field `PCS50` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_R as PCS50_R;
///Field `PCS51` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_R as PCS51_R;
///Field `PCS52` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_R as PCS52_R;
///Field `PCS53` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_R as PCS53_R;
///Field `PCS49` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_W as PCS49_W;
///Field `PCS50` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_W as PCS50_W;
///Field `PCS51` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_W as PCS51_W;
///Field `PCS52` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_W as PCS52_W;
///Field `PCS53` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
pub use PCS48_W as PCS53_W;
impl R {
    ///Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs48(&self) -> PCS48_R {
        PCS48_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs49(&self) -> PCS49_R {
        PCS49_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs50(&self) -> PCS50_R {
        PCS50_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs51(&self) -> PCS51_R {
        PCS51_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs52(&self) -> PCS52_R {
        PCS52_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs53(&self) -> PCS53_R {
        PCS53_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PCR2H")
            .field("pcs48", &self.pcs48())
            .field("pcs49", &self.pcs49())
            .field("pcs50", &self.pcs50())
            .field("pcs51", &self.pcs51())
            .field("pcs52", &self.pcs52())
            .field("pcs53", &self.pcs53())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs48(&mut self) -> PCS48_W<'_, D3PCR2Hrs> {
        PCS48_W::new(self, 0)
    }
    ///Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs49(&mut self) -> PCS49_W<'_, D3PCR2Hrs> {
        PCS49_W::new(self, 2)
    }
    ///Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs50(&mut self) -> PCS50_W<'_, D3PCR2Hrs> {
        PCS50_W::new(self, 4)
    }
    ///Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs51(&mut self) -> PCS51_W<'_, D3PCR2Hrs> {
        PCS51_W::new(self, 6)
    }
    ///Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs52(&mut self) -> PCS52_W<'_, D3PCR2Hrs> {
        PCS52_W::new(self, 8)
    }
    ///Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)
    #[inline(always)]
    pub fn pcs53(&mut self) -> PCS53_W<'_, D3PCR2Hrs> {
        PCS53_W::new(self, 10)
    }
}
/**EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr2h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#EXTI:D3PCR2H)*/
pub struct D3PCR2Hrs;
impl crate::RegisterSpec for D3PCR2Hrs {
    type Ux = u32;
}
///`read()` method returns [`d3pcr2h::R`](R) reader structure
impl crate::Readable for D3PCR2Hrs {}
///`write(|w| ..)` method takes [`d3pcr2h::W`](W) writer structure
impl crate::Writable for D3PCR2Hrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PCR2H to value 0
impl crate::Resettable for D3PCR2Hrs {}
