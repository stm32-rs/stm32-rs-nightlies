///Register `MTLTxQ0OMR` reader
pub type R = crate::R<MTLTX_Q0OMRrs>;
///Register `MTLTxQ0OMR` writer
pub type W = crate::W<MTLTX_Q0OMRrs>;
///Field `FTQ` reader - FTQ
pub type FTQ_R = crate::BitReader;
///Field `FTQ` writer - FTQ
pub type FTQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSF` reader - TSF
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - TSF
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXQEN` reader - TXQEN
pub type TXQEN_R = crate::FieldReader;
///Field `TXQEN` writer - TXQEN
pub type TXQEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TTC` reader - TTC
pub type TTC_R = crate::FieldReader;
///Field `TTC` writer - TTC
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TQS` reader - TQS
pub type TQS_R = crate::FieldReader<u16>;
///Field `TQS` writer - TQS
pub type TQS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bit 0 - FTQ
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSF
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - TXQEN
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - TTC
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:24 - TQS
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ0OMR")
            .field("ftq", &self.ftq())
            .field("tsf", &self.tsf())
            .field("txqen", &self.txqen())
            .field("ttc", &self.ttc())
            .field("tqs", &self.tqs())
            .finish()
    }
}
impl W {
    ///Bit 0 - FTQ
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W<'_, MTLTX_Q0OMRrs> {
        FTQ_W::new(self, 0)
    }
    ///Bit 1 - TSF
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, MTLTX_Q0OMRrs> {
        TSF_W::new(self, 1)
    }
    ///Bits 2:3 - TXQEN
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W<'_, MTLTX_Q0OMRrs> {
        TXQEN_W::new(self, 2)
    }
    ///Bits 4:5 - TTC
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<'_, MTLTX_Q0OMRrs> {
        TTC_W::new(self, 4)
    }
    ///Bits 16:24 - TQS
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W<'_, MTLTX_Q0OMRrs> {
        TQS_W::new(self, 16)
    }
}
/**Tx queue 0 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q0omr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q0omr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ0OMR)*/
pub struct MTLTX_Q0OMRrs;
impl crate::RegisterSpec for MTLTX_Q0OMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q0omr::R`](R) reader structure
impl crate::Readable for MTLTX_Q0OMRrs {}
///`write(|w| ..)` method takes [`mtltx_q0omr::W`](W) writer structure
impl crate::Writable for MTLTX_Q0OMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQ0OMR to value 0
impl crate::Resettable for MTLTX_Q0OMRrs {}
