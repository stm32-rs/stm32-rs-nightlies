///Register `DMAMR` reader
pub type R = crate::R<DMAMRrs>;
///Register `DMAMR` writer
pub type W = crate::W<DMAMRrs>;
///Field `SWR` reader - Software Reset
pub type SWR_R = crate::BitReader;
///Field `SWR` writer - Software Reset
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAA` reader - Transmit Arbitration Algorithm
pub type TAA_R = crate::FieldReader;
///Field `DSPW` reader - Descriptor Posted Write
pub type DSPW_R = crate::BitReader;
///Field `DSPW` writer - Descriptor Posted Write
pub type DSPW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPR` reader - Transmit priority
pub type TXPR_R = crate::BitReader;
///Field `TXPR` writer - Transmit priority
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTM` reader - Interrupt Mode
pub type INTM_R = crate::FieldReader;
///Field `INTM` writer - Interrupt Mode
pub type INTM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - Transmit Arbitration Algorithm
    #[inline(always)]
    pub fn taa(&self) -> TAA_R {
        TAA_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bit 8 - Descriptor Posted Write
    #[inline(always)]
    pub fn dspw(&self) -> DSPW_R {
        DSPW_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMR")
            .field("swr", &self.swr())
            .field("taa", &self.taa())
            .field("dspw", &self.dspw())
            .field("txpr", &self.txpr())
            .field("intm", &self.intm())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<'_, DMAMRrs> {
        SWR_W::new(self, 0)
    }
    ///Bit 8 - Descriptor Posted Write
    #[inline(always)]
    pub fn dspw(&mut self) -> DSPW_W<'_, DMAMRrs> {
        DSPW_W::new(self, 8)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<'_, DMAMRrs> {
        TXPR_W::new(self, 11)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    pub fn intm(&mut self) -> INTM_W<'_, DMAMRrs> {
        INTM_W::new(self, 16)
    }
}
/**DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAMR)*/
pub struct DMAMRrs;
impl crate::RegisterSpec for DMAMRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamr::R`](R) reader structure
impl crate::Readable for DMAMRrs {}
///`write(|w| ..)` method takes [`dmamr::W`](W) writer structure
impl crate::Writable for DMAMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAMR to value 0
impl crate::Resettable for DMAMRrs {}
