///Register `DMAMR` reader
pub type R = crate::R<DMAMRrs>;
///Register `DMAMR` writer
pub type W = crate::W<DMAMRrs>;
///Field `SWR` reader - Software Reset
pub type SWR_R = crate::BitReader;
///Field `SWR` writer - Software Reset
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - DMA Tx or Rx Arbitration Scheme
pub type DA_R = crate::BitReader;
///Field `DA` writer - DMA Tx or Rx Arbitration Scheme
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXPR` reader - Transmit priority
pub type TXPR_R = crate::BitReader;
///Field `TXPR` writer - Transmit priority
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PR` reader - Priority ratio
pub type PR_R = crate::FieldReader;
///Field `PR` writer - Priority ratio
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    ///Bit 1 - DMA Tx or Rx Arbitration Scheme
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Priority ratio
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
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
            .field("intm", &self.intm())
            .field("pr", &self.pr())
            .field("txpr", &self.txpr())
            .field("da", &self.da())
            .field("swr", &self.swr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<DMAMRrs> {
        SWR_W::new(self, 0)
    }
    ///Bit 1 - DMA Tx or Rx Arbitration Scheme
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<DMAMRrs> {
        DA_W::new(self, 1)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<DMAMRrs> {
        TXPR_W::new(self, 11)
    }
    ///Bits 12:14 - Priority ratio
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<DMAMRrs> {
        PR_W::new(self, 12)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    pub fn intm(&mut self) -> INTM_W<DMAMRrs> {
        INTM_W::new(self, 16)
    }
}
/**DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_DMA:DMAMR)*/
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
