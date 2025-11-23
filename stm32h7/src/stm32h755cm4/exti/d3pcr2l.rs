///Register `D3PCR2L` reader
pub type R = crate::R<D3PCR2Lrs>;
///Register `D3PCR2L` writer
pub type W = crate::W<D3PCR2Lrs>;
/**D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS34 {
    ///0: DMA ch6 event selected as D3 domain pendclear source
    DmaCh6 = 0,
    ///1: DMA ch7 event selected as D3 domain pendclear source
    DmaCh7 = 1,
    ///2: LPTIM4 out selected as D3 domain pendclear source
    Lptim4 = 2,
    ///3: LPTIM5 out selected as D3 domain pendclear source
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
impl crate::IsEnum for PCS34 {}
///Field `PCS34` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub type PCS34_R = crate::FieldReader<PCS34>;
impl PCS34_R {
    ///Get enumerated values variant
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
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS34::DmaCh6
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS34::DmaCh7
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS34::Lptim4
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS34::Lptim5
    }
}
///Field `PCS34` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub type PCS34_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PCS34, crate::Safe>;
impl<'a, REG> PCS34_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::DmaCh6)
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::DmaCh7)
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::Lptim4)
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS34::Lptim5)
    }
}
///Field `PCS35` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_R as PCS35_R;
///Field `PCS41` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_R as PCS41_R;
///Field `PCS35` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_W as PCS35_W;
///Field `PCS41` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
pub use PCS34_W as PCS41_W;
impl R {
    ///Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs34(&self) -> PCS34_R {
        PCS34_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs35(&self) -> PCS35_R {
        PCS35_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs41(&self) -> PCS41_R {
        PCS41_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PCR2L")
            .field("pcs34", &self.pcs34())
            .field("pcs35", &self.pcs35())
            .field("pcs41", &self.pcs41())
            .finish()
    }
}
impl W {
    ///Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs34(&mut self) -> PCS34_W<'_, D3PCR2Lrs> {
        PCS34_W::new(self, 4)
    }
    ///Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs35(&mut self) -> PCS35_W<'_, D3PCR2Lrs> {
        PCS35_W::new(self, 6)
    }
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)
    #[inline(always)]
    pub fn pcs41(&mut self) -> PCS41_W<'_, D3PCR2Lrs> {
        PCS41_W::new(self, 18)
    }
}
/**EXTI D3 pending clear selection register low

You can [`read`](crate::Reg::read) this register and get [`d3pcr2l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#EXTI:D3PCR2L)*/
pub struct D3PCR2Lrs;
impl crate::RegisterSpec for D3PCR2Lrs {
    type Ux = u32;
}
///`read()` method returns [`d3pcr2l::R`](R) reader structure
impl crate::Readable for D3PCR2Lrs {}
///`write(|w| ..)` method takes [`d3pcr2l::W`](W) writer structure
impl crate::Writable for D3PCR2Lrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PCR2L to value 0
impl crate::Resettable for D3PCR2Lrs {}
