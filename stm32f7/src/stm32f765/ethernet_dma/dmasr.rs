///Register `DMASR` reader
pub type R = crate::R<DMASRrs>;
///Register `DMASR` writer
pub type W = crate::W<DMASRrs>;
///Field `TS` reader - Transmit status
pub type TS_R = crate::BitReader;
///Field `TS` writer - Transmit status
pub type TS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPSS` reader - Transmit process stopped status
pub type TPSS_R = crate::BitReader;
///Field `TPSS` writer - Transmit process stopped status
pub type TPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TBUS` reader - Transmit buffer unavailable status
pub type TBUS_R = crate::BitReader;
///Field `TBUS` writer - Transmit buffer unavailable status
pub type TBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TJTS` reader - Transmit jabber timeout status
pub type TJTS_R = crate::BitReader;
///Field `TJTS` writer - Transmit jabber timeout status
pub type TJTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROS` reader - Receive overflow status
pub type ROS_R = crate::BitReader;
///Field `ROS` writer - Receive overflow status
pub type ROS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TUS` reader - Transmit underflow status
pub type TUS_R = crate::BitReader;
///Field `TUS` writer - Transmit underflow status
pub type TUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS` reader - Receive status
pub type RS_R = crate::BitReader;
///Field `RS` writer - Receive status
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBUS` reader - Receive buffer unavailable status
pub type RBUS_R = crate::BitReader;
///Field `RBUS` writer - Receive buffer unavailable status
pub type RBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPSS` reader - Receive process stopped status
pub type RPSS_R = crate::BitReader;
///Field `RPSS` writer - Receive process stopped status
pub type RPSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWTS` reader - PWTS
pub type PWTS_R = crate::BitReader;
///Field `PWTS` writer - PWTS
pub type PWTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETS` reader - Early transmit status
pub type ETS_R = crate::BitReader;
///Field `ETS` writer - Early transmit status
pub type ETS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FBES` reader - Fatal bus error status
pub type FBES_R = crate::BitReader;
///Field `FBES` writer - Fatal bus error status
pub type FBES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERS` reader - Early receive status
pub type ERS_R = crate::BitReader;
///Field `ERS` writer - Early receive status
pub type ERS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AIS` reader - Abnormal interrupt summary
pub type AIS_R = crate::BitReader;
///Field `AIS` writer - Abnormal interrupt summary
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NIS` reader - Normal interrupt summary
pub type NIS_R = crate::BitReader;
///Field `NIS` writer - Normal interrupt summary
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Receive process state

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPS {
    ///0: Stopped, reset or Stop Receive command issued
    Stopped = 0,
    ///1: Running, fetching receive transfer descriptor
    RunningFetching = 1,
    ///3: Running, waiting for receive packet
    RunningWaiting = 3,
    ///4: Suspended, receive descriptor unavailable
    Suspended = 4,
    ///7: Running, writing data to host memory buffer
    RunningWriting = 7,
}
impl From<RPS> for u8 {
    #[inline(always)]
    fn from(variant: RPS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPS {
    type Ux = u8;
}
impl crate::IsEnum for RPS {}
///Field `RPS` reader - Receive process state
pub type RPS_R = crate::FieldReader<RPS>;
impl RPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RPS> {
        match self.bits {
            0 => Some(RPS::Stopped),
            1 => Some(RPS::RunningFetching),
            3 => Some(RPS::RunningWaiting),
            4 => Some(RPS::Suspended),
            7 => Some(RPS::RunningWriting),
            _ => None,
        }
    }
    ///Stopped, reset or Stop Receive command issued
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == RPS::Stopped
    }
    ///Running, fetching receive transfer descriptor
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == RPS::RunningFetching
    }
    ///Running, waiting for receive packet
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == RPS::RunningWaiting
    }
    ///Suspended, receive descriptor unavailable
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == RPS::Suspended
    }
    ///Running, writing data to host memory buffer
    #[inline(always)]
    pub fn is_running_writing(&self) -> bool {
        *self == RPS::RunningWriting
    }
}
/**Transmit process state

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPS {
    ///0: Stopped, Reset or Stop Transmit command issued
    Stopped = 0,
    ///1: Running, fetching transmit transfer descriptor
    RunningFetching = 1,
    ///2: Running, waiting for status
    RunningWaiting = 2,
    ///3: Running, reading data from host memory buffer
    RunningReading = 3,
    ///6: Suspended, transmit descriptor unavailable or transmit buffer underflow
    Suspended = 6,
    ///7: Running, closing transmit descriptor
    Running = 7,
}
impl From<TPS> for u8 {
    #[inline(always)]
    fn from(variant: TPS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPS {
    type Ux = u8;
}
impl crate::IsEnum for TPS {}
///Field `TPS` reader - Transmit process state
pub type TPS_R = crate::FieldReader<TPS>;
impl TPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TPS> {
        match self.bits {
            0 => Some(TPS::Stopped),
            1 => Some(TPS::RunningFetching),
            2 => Some(TPS::RunningWaiting),
            3 => Some(TPS::RunningReading),
            6 => Some(TPS::Suspended),
            7 => Some(TPS::Running),
            _ => None,
        }
    }
    ///Stopped, Reset or Stop Transmit command issued
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == TPS::Stopped
    }
    ///Running, fetching transmit transfer descriptor
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == TPS::RunningFetching
    }
    ///Running, waiting for status
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == TPS::RunningWaiting
    }
    ///Running, reading data from host memory buffer
    #[inline(always)]
    pub fn is_running_reading(&self) -> bool {
        *self == TPS::RunningReading
    }
    ///Suspended, transmit descriptor unavailable or transmit buffer underflow
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == TPS::Suspended
    }
    ///Running, closing transmit descriptor
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == TPS::Running
    }
}
///Field `EBS` reader - Error bits status
pub type EBS_R = crate::FieldReader;
///Field `MMCS` reader - MMC status
pub type MMCS_R = crate::BitReader;
///Field `PMTS` reader - PMT status
pub type PMTS_R = crate::BitReader;
///Field `TSTS` reader - Time stamp trigger status
pub type TSTS_R = crate::BitReader;
impl R {
    ///Bit 0 - Transmit status
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit process stopped status
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit buffer unavailable status
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit jabber timeout status
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Receive overflow status
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Transmit underflow status
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive status
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive buffer unavailable status
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive process stopped status
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early transmit status
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Fatal bus error status
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Early receive status
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Abnormal interrupt summary
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Normal interrupt summary
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Receive process state
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22 - Transmit process state
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - Error bits status
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 27 - MMC status
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - PMT status
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Time stamp trigger status
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
    ///Bit 0 - Transmit status
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<'_, DMASRrs> {
        TS_W::new(self, 0)
    }
    ///Bit 1 - Transmit process stopped status
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W<'_, DMASRrs> {
        TPSS_W::new(self, 1)
    }
    ///Bit 2 - Transmit buffer unavailable status
    #[inline(always)]
    pub fn tbus(&mut self) -> TBUS_W<'_, DMASRrs> {
        TBUS_W::new(self, 2)
    }
    ///Bit 3 - Transmit jabber timeout status
    #[inline(always)]
    pub fn tjts(&mut self) -> TJTS_W<'_, DMASRrs> {
        TJTS_W::new(self, 3)
    }
    ///Bit 4 - Receive overflow status
    #[inline(always)]
    pub fn ros(&mut self) -> ROS_W<'_, DMASRrs> {
        ROS_W::new(self, 4)
    }
    ///Bit 5 - Transmit underflow status
    #[inline(always)]
    pub fn tus(&mut self) -> TUS_W<'_, DMASRrs> {
        TUS_W::new(self, 5)
    }
    ///Bit 6 - Receive status
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W<'_, DMASRrs> {
        RS_W::new(self, 6)
    }
    ///Bit 7 - Receive buffer unavailable status
    #[inline(always)]
    pub fn rbus(&mut self) -> RBUS_W<'_, DMASRrs> {
        RBUS_W::new(self, 7)
    }
    ///Bit 8 - Receive process stopped status
    #[inline(always)]
    pub fn rpss(&mut self) -> RPSS_W<'_, DMASRrs> {
        RPSS_W::new(self, 8)
    }
    ///Bit 9 - PWTS
    #[inline(always)]
    pub fn pwts(&mut self) -> PWTS_W<'_, DMASRrs> {
        PWTS_W::new(self, 9)
    }
    ///Bit 10 - Early transmit status
    #[inline(always)]
    pub fn ets(&mut self) -> ETS_W<'_, DMASRrs> {
        ETS_W::new(self, 10)
    }
    ///Bit 13 - Fatal bus error status
    #[inline(always)]
    pub fn fbes(&mut self) -> FBES_W<'_, DMASRrs> {
        FBES_W::new(self, 13)
    }
    ///Bit 14 - Early receive status
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W<'_, DMASRrs> {
        ERS_W::new(self, 14)
    }
    ///Bit 15 - Abnormal interrupt summary
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<'_, DMASRrs> {
        AIS_W::new(self, 15)
    }
    ///Bit 16 - Normal interrupt summary
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<'_, DMASRrs> {
        NIS_W::new(self, 16)
    }
}
/**Ethernet DMA status register

You can [`read`](crate::Reg::read) this register and get [`dmasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#Ethernet_DMA:DMASR)*/
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
