///Register `MTLTXQ0OMR` reader
pub type R = crate::R<MTLTXQ0OMRrs>;
///Register `MTLTXQ0OMR` writer
pub type W = crate::W<MTLTXQ0OMRrs>;
///Field `FTQ` reader - Flush Transmit Queue
pub type FTQ_R = crate::BitReader;
///Field `FTQ` writer - Flush Transmit Queue
pub type FTQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSF` reader - Transmit Store and Forward
pub type TSF_R = crate::BitReader;
///Field `TSF` writer - Transmit Store and Forward
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXQEN` reader - Transmit Queue Enable
pub type TXQEN_R = crate::FieldReader;
///Field `TXQEN` writer - Transmit Queue Enable
pub type TXQEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TTC` reader - Transmit Threshold Control
pub type TTC_R = crate::FieldReader;
///Field `TTC` writer - Transmit Threshold Control
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TQS` reader - Transmit queue size
pub type TQS_R = crate::FieldReader;
///Field `TQS` writer - Transmit queue size
pub type TQS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Flush Transmit Queue
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Store and Forward
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Transmit Queue Enable
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Transmit Threshold Control
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 16:19 - Transmit queue size
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ0OMR")
            .field("ftq", &self.ftq())
            .field("tsf", &self.tsf())
            .field("txqen", &self.txqen())
            .field("ttc", &self.ttc())
            .field("tqs", &self.tqs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flush Transmit Queue
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W<'_, MTLTXQ0OMRrs> {
        FTQ_W::new(self, 0)
    }
    ///Bit 1 - Transmit Store and Forward
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, MTLTXQ0OMRrs> {
        TSF_W::new(self, 1)
    }
    ///Bits 2:3 - Transmit Queue Enable
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W<'_, MTLTXQ0OMRrs> {
        TXQEN_W::new(self, 2)
    }
    ///Bits 4:6 - Transmit Threshold Control
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<'_, MTLTXQ0OMRrs> {
        TTC_W::new(self, 4)
    }
    ///Bits 16:19 - Transmit queue size
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W<'_, MTLTXQ0OMRrs> {
        TQS_W::new(self, 16)
    }
}
/**T0 queue 0 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0omr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq0omr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLTXQ0OMR)*/
pub struct MTLTXQ0OMRrs;
impl crate::RegisterSpec for MTLTXQ0OMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq0omr::R`](R) reader structure
impl crate::Readable for MTLTXQ0OMRrs {}
///`write(|w| ..)` method takes [`mtltxq0omr::W`](W) writer structure
impl crate::Writable for MTLTXQ0OMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ0OMR to value 0
impl crate::Resettable for MTLTXQ0OMRrs {}
