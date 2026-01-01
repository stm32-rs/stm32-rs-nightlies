///Register `D3PCR1H` reader
pub type R = crate::R<D3PCR1Hrs>;
///Register `D3PCR1H` writer
pub type W = crate::W<D3PCR1Hrs>;
/**D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS19 {
    ///0: DMA ch6 event selected as D3 domain pendclear source
    DmaCh6 = 0,
    ///1: DMA ch7 event selected as D3 domain pendclear source
    DmaCh7 = 1,
    ///2: LPTIM4 out selected as D3 domain pendclear source
    Lptim4 = 2,
    ///3: LPTIM5 out selected as D3 domain pendclear source
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
impl crate::IsEnum for PCS19 {}
///Field `PCS19` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub type PCS19_R = crate::FieldReader<PCS19>;
impl PCS19_R {
    ///Get enumerated values variant
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
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS19::DmaCh6
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS19::DmaCh7
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS19::Lptim4
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS19::Lptim5
    }
}
///Field `PCS19` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub type PCS19_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PCS19, crate::Safe>;
impl<'a, REG> PCS19_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::DmaCh6)
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::DmaCh7)
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::Lptim4)
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS19::Lptim5)
    }
}
///Field `PCS20` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub use PCS19_R as PCS20_R;
///Field `PCS21` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub use PCS19_R as PCS21_R;
///Field `PCS25` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub use PCS19_R as PCS25_R;
///Field `PCS20` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub use PCS19_W as PCS20_W;
///Field `PCS21` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub use PCS19_W as PCS21_W;
///Field `PCS25` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
pub use PCS19_W as PCS25_W;
impl R {
    ///Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs19(&self) -> PCS19_R {
        PCS19_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs20(&self) -> PCS20_R {
        PCS20_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs21(&self) -> PCS21_R {
        PCS21_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs25(&self) -> PCS25_R {
        PCS25_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PCR1H")
            .field("pcs19", &self.pcs19())
            .field("pcs20", &self.pcs20())
            .field("pcs21", &self.pcs21())
            .field("pcs25", &self.pcs25())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs19(&mut self) -> PCS19_W<'_, D3PCR1Hrs> {
        PCS19_W::new(self, 6)
    }
    ///Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs20(&mut self) -> PCS20_W<'_, D3PCR1Hrs> {
        PCS20_W::new(self, 8)
    }
    ///Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs21(&mut self) -> PCS21_W<'_, D3PCR1Hrs> {
        PCS21_W::new(self, 10)
    }
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)
    #[inline(always)]
    pub fn pcs25(&mut self) -> PCS25_W<'_, D3PCR1Hrs> {
        PCS25_W::new(self, 18)
    }
}
/**EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr1h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#EXTI:D3PCR1H)*/
pub struct D3PCR1Hrs;
impl crate::RegisterSpec for D3PCR1Hrs {
    type Ux = u32;
}
///`read()` method returns [`d3pcr1h::R`](R) reader structure
impl crate::Readable for D3PCR1Hrs {}
///`write(|w| ..)` method takes [`d3pcr1h::W`](W) writer structure
impl crate::Writable for D3PCR1Hrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PCR1H to value 0
impl crate::Resettable for D3PCR1Hrs {}
