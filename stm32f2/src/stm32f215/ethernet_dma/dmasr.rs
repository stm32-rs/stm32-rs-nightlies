#[doc = "Register `DMASR` reader"]
pub type R = crate::R<DMASRrs>;
#[doc = "Register `DMASR` writer"]
pub type W = crate::W<DMASRrs>;
#[doc = "Field `TS` reader - Transmit status"]
pub type TS_R = crate::BitReader;
#[doc = "Field `TS` writer - Transmit status"]
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPSS` reader - Transmit process stopped status"]
pub type TPSS_R = crate::BitReader;
#[doc = "Field `TPSS` writer - Transmit process stopped status"]
pub type TPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUS` reader - Transmit buffer unavailable status"]
pub type TBUS_R = crate::BitReader;
#[doc = "Field `TBUS` writer - Transmit buffer unavailable status"]
pub type TBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJTS` reader - Transmit jabber timeout status"]
pub type TJTS_R = crate::BitReader;
#[doc = "Field `TJTS` writer - Transmit jabber timeout status"]
pub type TJTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROS` reader - Receive overflow status"]
pub type ROS_R = crate::BitReader;
#[doc = "Field `ROS` writer - Receive overflow status"]
pub type ROS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUS` reader - Transmit underflow status"]
pub type TUS_R = crate::BitReader;
#[doc = "Field `TUS` writer - Transmit underflow status"]
pub type TUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Receive status"]
pub type RS_R = crate::BitReader;
#[doc = "Field `RS` writer - Receive status"]
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUS` reader - Receive buffer unavailable status"]
pub type RBUS_R = crate::BitReader;
#[doc = "Field `RBUS` writer - Receive buffer unavailable status"]
pub type RBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPSS` reader - Receive process stopped status"]
pub type RPSS_R = crate::BitReader;
#[doc = "Field `RPSS` writer - Receive process stopped status"]
pub type RPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWTS` reader - Receive watchdog timeout status"]
pub type PWTS_R = crate::BitReader;
#[doc = "Field `PWTS` writer - Receive watchdog timeout status"]
pub type PWTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETS` reader - Early transmit status"]
pub type ETS_R = crate::BitReader;
#[doc = "Field `ETS` writer - Early transmit status"]
pub type ETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBES` reader - Fatal bus error status"]
pub type FBES_R = crate::BitReader;
#[doc = "Field `FBES` writer - Fatal bus error status"]
pub type FBES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERS` reader - Early receive status"]
pub type ERS_R = crate::BitReader;
#[doc = "Field `ERS` writer - Early receive status"]
pub type ERS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive process state"]
pub type RPS_R = crate::FieldReader;
#[doc = "Field `TPS` reader - Transmit process state"]
pub type TPS_R = crate::FieldReader;
#[doc = "Field `EBS` reader - Error bits status"]
pub type EBS_R = crate::FieldReader;
#[doc = "Field `MMCS` reader - MMC status"]
pub type MMCS_R = crate::BitReader;
#[doc = "Field `PMTS` reader - PMT status"]
pub type PMTS_R = crate::BitReader;
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub type TSTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits status"]
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<DMASRrs> {
        TS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    #[must_use]
    pub fn tpss(&mut self) -> TPSS_W<DMASRrs> {
        TPSS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    #[must_use]
    pub fn tbus(&mut self) -> TBUS_W<DMASRrs> {
        TBUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    #[must_use]
    pub fn tjts(&mut self) -> TJTS_W<DMASRrs> {
        TJTS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> ROS_W<DMASRrs> {
        ROS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    #[must_use]
    pub fn tus(&mut self) -> TUS_W<DMASRrs> {
        TUS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<DMASRrs> {
        RS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    #[must_use]
    pub fn rbus(&mut self) -> RBUS_W<DMASRrs> {
        RBUS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    #[must_use]
    pub fn rpss(&mut self) -> RPSS_W<DMASRrs> {
        RPSS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    #[must_use]
    pub fn pwts(&mut self) -> PWTS_W<DMASRrs> {
        PWTS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    #[must_use]
    pub fn ets(&mut self) -> ETS_W<DMASRrs> {
        ETS_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    #[must_use]
    pub fn fbes(&mut self) -> FBES_W<DMASRrs> {
        FBES_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    #[must_use]
    pub fn ers(&mut self) -> ERS_W<DMASRrs> {
        ERS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<DMASRrs> {
        AIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<DMASRrs> {
        NIS_W::new(self, 16)
    }
}
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASRrs;
impl crate::RegisterSpec for DMASRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasr::R`](R) reader structure"]
impl crate::Readable for DMASRrs {}
#[doc = "`write(|w| ..)` method takes [`dmasr::W`](W) writer structure"]
impl crate::Writable for DMASRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DMASRrs {
    const RESET_VALUE: u32 = 0;
}
