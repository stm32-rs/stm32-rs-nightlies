///Register `MTLRxQOMR` reader
pub type R = crate::R<MTLRX_QOMRrs>;
///Register `MTLRxQOMR` writer
pub type W = crate::W<MTLRX_QOMRrs>;
///Field `RTC` reader - Receive Queue Threshold Control
pub type RTC_R = crate::FieldReader;
///Field `RTC` writer - Receive Queue Threshold Control
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FUP` reader - Forward Undersized Good Packets
pub type FUP_R = crate::BitReader;
///Field `FUP` writer - Forward Undersized Good Packets
pub type FUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEP` reader - Forward Error Packets
pub type FEP_R = crate::BitReader;
///Field `FEP` writer - Forward Error Packets
pub type FEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSF` reader - Receive Queue Store and Forward
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - Receive Queue Store and Forward
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_TCP_EF` reader - Disable Dropping of TCP/IP Checksum Error Packets
pub type DIS_TCP_EF_R = crate::BitReader;
///Field `DIS_TCP_EF` writer - Disable Dropping of TCP/IP Checksum Error Packets
pub type DIS_TCP_EF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EHFC` reader - Enable Hardware Flow Control
pub type EHFC_R = crate::BitReader;
///Field `EHFC` writer - Enable Hardware Flow Control
pub type EHFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFA` reader - Threshold for Activating Flow Control (in half-duplex and full-duplex modes)
pub type RFA_R = crate::FieldReader;
///Field `RFA` writer - Threshold for Activating Flow Control (in half-duplex and full-duplex modes)
pub type RFA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RFD` reader - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)
pub type RFD_R = crate::FieldReader;
///Field `RFD` writer - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)
pub type RFD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RQS` reader - Receive Queue Size
pub type RQS_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Receive Queue Threshold Control
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Forward Undersized Good Packets
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Forward Error Packets
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Queue Store and Forward
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Enable Hardware Flow Control
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Threshold for Activating Flow Control (in half-duplex and full-duplex modes)
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 14:16 - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 20:22 - Receive Queue Size
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRxQOMR")
            .field("rtc", &self.rtc())
            .field("fup", &self.fup())
            .field("fep", &self.fep())
            .field("rsf", &self.rsf())
            .field("dis_tcp_ef", &self.dis_tcp_ef())
            .field("ehfc", &self.ehfc())
            .field("rfa", &self.rfa())
            .field("rfd", &self.rfd())
            .field("rqs", &self.rqs())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Receive Queue Threshold Control
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, MTLRX_QOMRrs> {
        RTC_W::new(self, 0)
    }
    ///Bit 3 - Forward Undersized Good Packets
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W<'_, MTLRX_QOMRrs> {
        FUP_W::new(self, 3)
    }
    ///Bit 4 - Forward Error Packets
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W<'_, MTLRX_QOMRrs> {
        FEP_W::new(self, 4)
    }
    ///Bit 5 - Receive Queue Store and Forward
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, MTLRX_QOMRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<'_, MTLRX_QOMRrs> {
        DIS_TCP_EF_W::new(self, 6)
    }
    ///Bit 7 - Enable Hardware Flow Control
    #[inline(always)]
    pub fn ehfc(&mut self) -> EHFC_W<'_, MTLRX_QOMRrs> {
        EHFC_W::new(self, 7)
    }
    ///Bits 8:10 - Threshold for Activating Flow Control (in half-duplex and full-duplex modes)
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W<'_, MTLRX_QOMRrs> {
        RFA_W::new(self, 8)
    }
    ///Bits 14:16 - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes)
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W<'_, MTLRX_QOMRrs> {
        RFD_W::new(self, 14)
    }
}
/**Rx queue operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_qomr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_qomr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#Ethernet_MTL:MTLRxQOMR)*/
pub struct MTLRX_QOMRrs;
impl crate::RegisterSpec for MTLRX_QOMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrx_qomr::R`](R) reader structure
impl crate::Readable for MTLRX_QOMRrs {}
///`write(|w| ..)` method takes [`mtlrx_qomr::W`](W) writer structure
impl crate::Writable for MTLRX_QOMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRxQOMR to value 0x0070_0000
impl crate::Resettable for MTLRX_QOMRrs {
    const RESET_VALUE: u32 = 0x0070_0000;
}
