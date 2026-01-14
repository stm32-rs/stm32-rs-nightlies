///Register `D3PCR3H` reader
pub type R = crate::R<D3PCR3Hrs>;
///Register `D3PCR3H` writer
pub type W = crate::W<D3PCR3Hrs>;
/**D3 Pending request clear input signal selection on Event input x= truncate N+160/2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS88 {
    ///0: DMA ch6 event selected as D3 domain pendclear source
    DmaCh6 = 0,
    ///1: DMA ch7 event selected as D3 domain pendclear source
    DmaCh7 = 1,
    ///2: LPTIM4 out selected as D3 domain pendclear source
    Lptim4 = 2,
    ///3: LPTIM5 out selected as D3 domain pendclear source
    Lptim5 = 3,
}
impl From<PCS88> for u8 {
    #[inline(always)]
    fn from(variant: PCS88) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCS88 {
    type Ux = u8;
}
impl crate::IsEnum for PCS88 {}
///Field `PCS88` reader - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
pub type PCS88_R = crate::FieldReader<PCS88>;
impl PCS88_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCS88 {
        match self.bits {
            0 => PCS88::DmaCh6,
            1 => PCS88::DmaCh7,
            2 => PCS88::Lptim4,
            3 => PCS88::Lptim5,
            _ => unreachable!(),
        }
    }
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS88::DmaCh6
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS88::DmaCh7
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS88::Lptim4
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS88::Lptim5
    }
}
///Field `PCS88` writer - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
pub type PCS88_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PCS88, crate::Safe>;
impl<'a, REG> PCS88_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut crate::W<REG> {
        self.variant(PCS88::DmaCh6)
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut crate::W<REG> {
        self.variant(PCS88::DmaCh7)
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut crate::W<REG> {
        self.variant(PCS88::Lptim4)
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut crate::W<REG> {
        self.variant(PCS88::Lptim5)
    }
}
impl R {
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
    #[inline(always)]
    pub fn pcs88(&self) -> PCS88_R {
        PCS88_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D3PCR3H")
            .field("pcs88", &self.pcs88())
            .finish()
    }
}
impl W {
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
    #[inline(always)]
    pub fn pcs88(&mut self) -> PCS88_W<'_, D3PCR3Hrs> {
        PCS88_W::new(self, 18)
    }
}
/**EXTI D3 pending clear selection register high

You can [`read`](crate::Reg::read) this register and get [`d3pcr3h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr3h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#EXTI:D3PCR3H)*/
pub struct D3PCR3Hrs;
impl crate::RegisterSpec for D3PCR3Hrs {
    type Ux = u32;
}
///`read()` method returns [`d3pcr3h::R`](R) reader structure
impl crate::Readable for D3PCR3Hrs {}
///`write(|w| ..)` method takes [`d3pcr3h::W`](W) writer structure
impl crate::Writable for D3PCR3Hrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets D3PCR3H to value 0
impl crate::Resettable for D3PCR3Hrs {}
