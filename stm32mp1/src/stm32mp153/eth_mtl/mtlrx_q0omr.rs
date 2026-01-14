///Register `MTLRxQ0OMR` reader
pub type R = crate::R<MTLRX_Q0OMRrs>;
///Register `MTLRxQ0OMR` writer
pub type W = crate::W<MTLRX_Q0OMRrs>;
///Field `RTC` reader - RTC
pub type RTC_R = crate::FieldReader;
///Field `RTC` writer - RTC
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FUP` reader - FUP
pub type FUP_R = crate::BitReader;
///Field `FUP` writer - FUP
pub type FUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEP` reader - FEP
pub type FEP_R = crate::BitReader;
///Field `FEP` writer - FEP
pub type FEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSF` reader - RSF
pub type RSF_R = crate::BitReader;
///Field `RSF` writer - RSF
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_TCP_EF` reader - DIS_TCP_EF
pub type DIS_TCP_EF_R = crate::BitReader;
///Field `DIS_TCP_EF` writer - DIS_TCP_EF
pub type DIS_TCP_EF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EHFC` reader - EHFC
pub type EHFC_R = crate::BitReader;
///Field `EHFC` writer - EHFC
pub type EHFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFA` reader - RFA
pub type RFA_R = crate::FieldReader;
///Field `RFA` writer - RFA
pub type RFA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RFD` reader - RFD
pub type RFD_R = crate::FieldReader;
///Field `RFD` writer - RFD
pub type RFD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RQS` reader - RQS
pub type RQS_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - RTC
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - FUP
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FEP
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RSF
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DIS_TCP_EF
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - EHFC
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - RFA
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 14:16 - RFD
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 20:23 - RQS
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRxQ0OMR")
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
    ///Bits 0:1 - RTC
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, MTLRX_Q0OMRrs> {
        RTC_W::new(self, 0)
    }
    ///Bit 3 - FUP
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W<'_, MTLRX_Q0OMRrs> {
        FUP_W::new(self, 3)
    }
    ///Bit 4 - FEP
    #[inline(always)]
    pub fn fep(&mut self) -> FEP_W<'_, MTLRX_Q0OMRrs> {
        FEP_W::new(self, 4)
    }
    ///Bit 5 - RSF
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<'_, MTLRX_Q0OMRrs> {
        RSF_W::new(self, 5)
    }
    ///Bit 6 - DIS_TCP_EF
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<'_, MTLRX_Q0OMRrs> {
        DIS_TCP_EF_W::new(self, 6)
    }
    ///Bit 7 - EHFC
    #[inline(always)]
    pub fn ehfc(&mut self) -> EHFC_W<'_, MTLRX_Q0OMRrs> {
        EHFC_W::new(self, 7)
    }
    ///Bits 8:10 - RFA
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W<'_, MTLRX_Q0OMRrs> {
        RFA_W::new(self, 8)
    }
    ///Bits 14:16 - RFD
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W<'_, MTLRX_Q0OMRrs> {
        RFD_W::new(self, 14)
    }
}
/**Rx queue 0 operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q0omr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_q0omr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ0OMR)*/
pub struct MTLRX_Q0OMRrs;
impl crate::RegisterSpec for MTLRX_Q0OMRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrx_q0omr::R`](R) reader structure
impl crate::Readable for MTLRX_Q0OMRrs {}
///`write(|w| ..)` method takes [`mtlrx_q0omr::W`](W) writer structure
impl crate::Writable for MTLRX_Q0OMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRxQ0OMR to value 0x0070_0000
impl crate::Resettable for MTLRX_Q0OMRrs {
    const RESET_VALUE: u32 = 0x0070_0000;
}
