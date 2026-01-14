///Register `MTLTxQOMR` reader
pub type R = crate::R<MTLTX_QOMRrs>;
///Register `MTLTxQOMR` writer
pub type W = crate::W<MTLTX_QOMRrs>;
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
///Field `TQS` reader - Transmit Queue Size
pub type TQS_R = crate::FieldReader<u16>;
///Field `TQS` writer - Transmit Queue Size
pub type TQS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
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
    ///Bits 16:24 - Transmit Queue Size
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQOMR")
            .field("tqs", &self.tqs())
            .field("ttc", &self.ttc())
            .field("txqen", &self.txqen())
            .field("tsf", &self.tsf())
            .field("ftq", &self.ftq())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flush Transmit Queue
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W<'_, MTLTX_QOMRrs> {
        FTQ_W::new(self, 0)
    }
    ///Bit 1 - Transmit Store and Forward
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, MTLTX_QOMRrs> {
        TSF_W::new(self, 1)
    }
    ///Bits 2:3 - Transmit Queue Enable
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W<'_, MTLTX_QOMRrs> {
        TXQEN_W::new(self, 2)
    }
    ///Bits 4:6 - Transmit Threshold Control
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<'_, MTLTX_QOMRrs> {
        TTC_W::new(self, 4)
    }
    ///Bits 16:24 - Transmit Queue Size
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W<'_, MTLTX_QOMRrs> {
        TQS_W::new(self, 16)
    }
}
/**Tx queue operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_qomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#Ethernet_MTL:MTLTxQOMR)*/
pub struct MTLTX_QOMRrs;
impl crate::RegisterSpec for MTLTX_QOMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_qomr::R`](R) reader structure
impl crate::Readable for MTLTX_QOMRrs {}
///`write(|w| ..)` method takes [`mtltx_qomr::W`](W) writer structure
impl crate::Writable for MTLTX_QOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQOMR to value 0x0007_0008
impl crate::Resettable for MTLTX_QOMRrs {
    const RESET_VALUE: u32 = 0x0007_0008;
}
