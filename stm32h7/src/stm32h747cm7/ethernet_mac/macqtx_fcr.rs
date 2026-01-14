///Register `MACQTxFCR` reader
pub type R = crate::R<MACQTX_FCRrs>;
///Register `MACQTxFCR` writer
pub type W = crate::W<MACQTX_FCRrs>;
///Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate
pub type FCB_BPA_R = crate::BitReader;
///Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TFE` reader - Transmit Flow Control Enable
pub type TFE_R = crate::BitReader;
///Field `TFE` writer - Transmit Flow Control Enable
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLT` reader - Pause Low Threshold
pub type PLT_R = crate::FieldReader;
///Field `PLT` writer - Pause Low Threshold
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DZPQ` reader - Disable Zero-Quanta Pause
pub type DZPQ_R = crate::BitReader;
///Field `DZPQ` writer - Disable Zero-Quanta Pause
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PT` reader - Pause Time
pub type PT_R = crate::FieldReader<u16>;
///Field `PT` writer - Pause Time
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Flow Control Busy or Backpressure Activate
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Flow Control Enable
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - Pause Low Threshold
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Disable Zero-Quanta Pause
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:31 - Pause Time
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACQTxFCR")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("tfe", &self.tfe())
            .field("plt", &self.plt())
            .field("dzpq", &self.dzpq())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flow Control Busy or Backpressure Activate
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<'_, MACQTX_FCRrs> {
        FCB_BPA_W::new(self, 0)
    }
    ///Bit 1 - Transmit Flow Control Enable
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<'_, MACQTX_FCRrs> {
        TFE_W::new(self, 1)
    }
    ///Bits 4:6 - Pause Low Threshold
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, MACQTX_FCRrs> {
        PLT_W::new(self, 4)
    }
    ///Bit 7 - Disable Zero-Quanta Pause
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W<'_, MACQTX_FCRrs> {
        DZPQ_W::new(self, 7)
    }
    ///Bits 16:31 - Pause Time
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, MACQTX_FCRrs> {
        PT_W::new(self, 16)
    }
}
/**Tx Queue flow control register

You can [`read`](crate::Reg::read) this register and get [`macqtx_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macqtx_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#Ethernet_MAC:MACQTxFCR)*/
pub struct MACQTX_FCRrs;
impl crate::RegisterSpec for MACQTX_FCRrs {
    type Ux = u32;
}
///`read()` method returns [`macqtx_fcr::R`](R) reader structure
impl crate::Readable for MACQTX_FCRrs {}
///`write(|w| ..)` method takes [`macqtx_fcr::W`](W) writer structure
impl crate::Writable for MACQTX_FCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACQTxFCR to value 0
impl crate::Resettable for MACQTX_FCRrs {}
