///Register `DMAC1SR` reader
pub type R = crate::R<DMAC1SRrs>;
///Register `DMAC1SR` writer
pub type W = crate::W<DMAC1SRrs>;
///Field `TI` reader - Transmit Interrupt
pub type TI_R = crate::BitReader;
///Field `TI` writer - Transmit Interrupt
pub type TI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPS` reader - Transmit Process Stopped
pub type TPS_R = crate::BitReader;
///Field `TPS` writer - Transmit Process Stopped
pub type TPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBU` reader - Transmit Buffer Unavailable
pub type TBU_R = crate::BitReader;
///Field `TBU` writer - Transmit Buffer Unavailable
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RI` reader - Receive Interrupt
pub type RI_R = crate::BitReader;
///Field `RI` writer - Receive Interrupt
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBU` reader - Receive Buffer Unavailable
pub type RBU_R = crate::BitReader;
///Field `RBU` writer - Receive Buffer Unavailable
pub type RBU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPS` reader - Receive Process Stopped
pub type RPS_R = crate::BitReader;
///Field `RPS` writer - Receive Process Stopped
pub type RPS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RWT` reader - Receive Watchdog Timeout
pub type RWT_R = crate::BitReader;
///Field `RWT` writer - Receive Watchdog Timeout
pub type RWT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETI` reader - Early Transmit Interrupt
pub type ETI_R = crate::BitReader;
///Field `ETI` writer - Early Transmit Interrupt
pub type ETI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERI` reader - Early Receive Interrupt
pub type ERI_R = crate::BitReader;
///Field `ERI` writer - Early Receive Interrupt
pub type ERI_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBE` reader - Fatal Bus Error
pub type FBE_R = crate::BitReader;
///Field `FBE` writer - Fatal Bus Error
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CDE` reader - Context Descriptor Error
pub type CDE_R = crate::BitReader;
///Field `CDE` writer - Context Descriptor Error
pub type CDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AIS` reader - Abnormal Interrupt Summary
pub type AIS_R = crate::BitReader;
///Field `AIS` writer - Abnormal Interrupt Summary
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NIS` reader - Normal Interrupt Summary
pub type NIS_R = crate::BitReader;
///Field `NIS` writer - Normal Interrupt Summary
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEB` reader - Tx DMA Error Bits
pub type TEB_R = crate::FieldReader;
///Field `TEB` writer - Tx DMA Error Bits
pub type TEB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REB` reader - Rx DMA Error Bits
pub type REB_R = crate::FieldReader;
///Field `REB` writer - Rx DMA Error Bits
pub type REB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Transmit Interrupt
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Process Stopped
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit Buffer Unavailable
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Receive Interrupt
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive Buffer Unavailable
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive Process Stopped
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive Watchdog Timeout
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early Transmit Interrupt
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Early Receive Interrupt
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Fatal Bus Error
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Context Descriptor Error
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Abnormal Interrupt Summary
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Normal Interrupt Summary
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - Tx DMA Error Bits
    #[inline(always)]
    pub fn teb(&self) -> TEB_R {
        TEB_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - Rx DMA Error Bits
    #[inline(always)]
    pub fn reb(&self) -> REB_R {
        REB_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1SR")
            .field("ti", &self.ti())
            .field("tps", &self.tps())
            .field("tbu", &self.tbu())
            .field("ri", &self.ri())
            .field("rbu", &self.rbu())
            .field("rps", &self.rps())
            .field("rwt", &self.rwt())
            .field("eti", &self.eti())
            .field("eri", &self.eri())
            .field("fbe", &self.fbe())
            .field("cde", &self.cde())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("teb", &self.teb())
            .field("reb", &self.reb())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Interrupt
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W<'_, DMAC1SRrs> {
        TI_W::new(self, 0)
    }
    ///Bit 1 - Transmit Process Stopped
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W<'_, DMAC1SRrs> {
        TPS_W::new(self, 1)
    }
    ///Bit 2 - Transmit Buffer Unavailable
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W<'_, DMAC1SRrs> {
        TBU_W::new(self, 2)
    }
    ///Bit 6 - Receive Interrupt
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W<'_, DMAC1SRrs> {
        RI_W::new(self, 6)
    }
    ///Bit 7 - Receive Buffer Unavailable
    #[inline(always)]
    pub fn rbu(&mut self) -> RBU_W<'_, DMAC1SRrs> {
        RBU_W::new(self, 7)
    }
    ///Bit 8 - Receive Process Stopped
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W<'_, DMAC1SRrs> {
        RPS_W::new(self, 8)
    }
    ///Bit 9 - Receive Watchdog Timeout
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<'_, DMAC1SRrs> {
        RWT_W::new(self, 9)
    }
    ///Bit 10 - Early Transmit Interrupt
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W<'_, DMAC1SRrs> {
        ETI_W::new(self, 10)
    }
    ///Bit 11 - Early Receive Interrupt
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W<'_, DMAC1SRrs> {
        ERI_W::new(self, 11)
    }
    ///Bit 12 - Fatal Bus Error
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W<'_, DMAC1SRrs> {
        FBE_W::new(self, 12)
    }
    ///Bit 13 - Context Descriptor Error
    #[inline(always)]
    pub fn cde(&mut self) -> CDE_W<'_, DMAC1SRrs> {
        CDE_W::new(self, 13)
    }
    ///Bit 14 - Abnormal Interrupt Summary
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<'_, DMAC1SRrs> {
        AIS_W::new(self, 14)
    }
    ///Bit 15 - Normal Interrupt Summary
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<'_, DMAC1SRrs> {
        NIS_W::new(self, 15)
    }
    ///Bits 16:18 - Tx DMA Error Bits
    #[inline(always)]
    pub fn teb(&mut self) -> TEB_W<'_, DMAC1SRrs> {
        TEB_W::new(self, 16)
    }
    ///Bits 19:21 - Rx DMA Error Bits
    #[inline(always)]
    pub fn reb(&mut self) -> REB_W<'_, DMAC1SRrs> {
        REB_W::new(self, 19)
    }
}
/**Channel status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1SR)*/
pub struct DMAC1SRrs;
impl crate::RegisterSpec for DMAC1SRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1sr::R`](R) reader structure
impl crate::Readable for DMAC1SRrs {}
///`write(|w| ..)` method takes [`dmac1sr::W`](W) writer structure
impl crate::Writable for DMAC1SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1SR to value 0
impl crate::Resettable for DMAC1SRrs {}
