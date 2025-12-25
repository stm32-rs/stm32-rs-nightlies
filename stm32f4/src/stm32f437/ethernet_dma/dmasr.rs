///Register `DMASR` reader
pub type R = crate::R<DMASRrs>;
///Register `DMASR` writer
pub type W = crate::W<DMASRrs>;
///Field `TS` reader - TS
pub type TS_R = crate::BitReader;
///Field `TS` writer - TS
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPSS` reader - TPSS
pub type TPSS_R = crate::BitReader;
///Field `TPSS` writer - TPSS
pub type TPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBUS` reader - TPSS
pub type TBUS_R = crate::BitReader;
///Field `TBUS` writer - TPSS
pub type TBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TJTS` reader - TJTS
pub type TJTS_R = crate::BitReader;
///Field `TJTS` writer - TJTS
pub type TJTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROS` reader - ROS
pub type ROS_R = crate::BitReader;
///Field `ROS` writer - ROS
pub type ROS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TUS` reader - TUS
pub type TUS_R = crate::BitReader;
///Field `TUS` writer - TUS
pub type TUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS` reader - RS
pub type RS_R = crate::BitReader;
///Field `RS` writer - RS
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBUS` reader - RBUS
pub type RBUS_R = crate::BitReader;
///Field `RBUS` writer - RBUS
pub type RBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPSS` reader - RPSS
pub type RPSS_R = crate::BitReader;
///Field `RPSS` writer - RPSS
pub type RPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWTS` reader - PWTS
pub type PWTS_R = crate::BitReader;
///Field `PWTS` writer - PWTS
pub type PWTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETS` reader - ETS
pub type ETS_R = crate::BitReader;
///Field `ETS` writer - ETS
pub type ETS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBES` reader - FBES
pub type FBES_R = crate::BitReader;
///Field `FBES` writer - FBES
pub type FBES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERS` reader - ERS
pub type ERS_R = crate::BitReader;
///Field `ERS` writer - ERS
pub type ERS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AIS` reader - AIS
pub type AIS_R = crate::BitReader;
///Field `AIS` writer - AIS
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NIS` reader - NIS
pub type NIS_R = crate::BitReader;
///Field `NIS` writer - NIS
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPS` reader - RPS
pub type RPS_R = crate::FieldReader;
///Field `TPS` reader - TPS
pub type TPS_R = crate::FieldReader;
///Field `EBS` reader - EBS
pub type EBS_R = crate::FieldReader;
///Field `MMCS` reader - MMCS
pub type MMCS_R = crate::BitReader;
///Field `PMTS` reader - PMTS
pub type PMTS_R = crate::BitReader;
///Field `TSTS` reader - TSTS
pub type TSTS_R = crate::BitReader;
impl R {
    ///Bit 0 - TS
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TPSS
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TPSS
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TJTS
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ROS
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TUS
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RS
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RBUS
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RPSS
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ETS
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - FBES
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ERS
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AIS
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - NIS
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - RPS
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22 - TPS
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - EBS
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 27 - MMCS
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PMTS
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - TSTS
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASR")
            .field("ts", &self.ts())
            .field("tpss", &self.tpss())
            .field("tbus", &self.tbus())
            .field("tjts", &self.tjts())
            .field("ros", &self.ros())
            .field("tus", &self.tus())
            .field("rs", &self.rs())
            .field("rbus", &self.rbus())
            .field("rpss", &self.rpss())
            .field("pwts", &self.pwts())
            .field("ets", &self.ets())
            .field("fbes", &self.fbes())
            .field("ers", &self.ers())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("rps", &self.rps())
            .field("tps", &self.tps())
            .field("ebs", &self.ebs())
            .field("mmcs", &self.mmcs())
            .field("pmts", &self.pmts())
            .field("tsts", &self.tsts())
            .finish()
    }
}
impl W {
    ///Bit 0 - TS
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, DMASRrs> {
        TS_W::new(self, 0)
    }
    ///Bit 1 - TPSS
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W<'_, DMASRrs> {
        TPSS_W::new(self, 1)
    }
    ///Bit 2 - TPSS
    #[inline(always)]
    pub fn tbus(&mut self) -> TBUS_W<'_, DMASRrs> {
        TBUS_W::new(self, 2)
    }
    ///Bit 3 - TJTS
    #[inline(always)]
    pub fn tjts(&mut self) -> TJTS_W<'_, DMASRrs> {
        TJTS_W::new(self, 3)
    }
    ///Bit 4 - ROS
    #[inline(always)]
    pub fn ros(&mut self) -> ROS_W<'_, DMASRrs> {
        ROS_W::new(self, 4)
    }
    ///Bit 5 - TUS
    #[inline(always)]
    pub fn tus(&mut self) -> TUS_W<'_, DMASRrs> {
        TUS_W::new(self, 5)
    }
    ///Bit 6 - RS
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W<'_, DMASRrs> {
        RS_W::new(self, 6)
    }
    ///Bit 7 - RBUS
    #[inline(always)]
    pub fn rbus(&mut self) -> RBUS_W<'_, DMASRrs> {
        RBUS_W::new(self, 7)
    }
    ///Bit 8 - RPSS
    #[inline(always)]
    pub fn rpss(&mut self) -> RPSS_W<'_, DMASRrs> {
        RPSS_W::new(self, 8)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    pub fn pwts(&mut self) -> PWTS_W<'_, DMASRrs> {
        PWTS_W::new(self, 9)
    }
    ///Bit 10 - ETS
    #[inline(always)]
    pub fn ets(&mut self) -> ETS_W<'_, DMASRrs> {
        ETS_W::new(self, 10)
    }
    ///Bit 13 - FBES
    #[inline(always)]
    pub fn fbes(&mut self) -> FBES_W<'_, DMASRrs> {
        FBES_W::new(self, 13)
    }
    ///Bit 14 - ERS
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W<'_, DMASRrs> {
        ERS_W::new(self, 14)
    }
    ///Bit 15 - AIS
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<'_, DMASRrs> {
        AIS_W::new(self, 15)
    }
    ///Bit 16 - NIS
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<'_, DMASRrs> {
        NIS_W::new(self, 16)
    }
}
/**Ethernet DMA status register

You can [`read`](crate::Reg::read) this register and get [`dmasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#Ethernet_DMA:DMASR)*/
pub struct DMASRrs;
impl crate::RegisterSpec for DMASRrs {
    type Ux = u32;
}
///`read()` method returns [`dmasr::R`](R) reader structure
impl crate::Readable for DMASRrs {}
///`write(|w| ..)` method takes [`dmasr::W`](W) writer structure
impl crate::Writable for DMASRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMASR to value 0
impl crate::Resettable for DMASRrs {}
