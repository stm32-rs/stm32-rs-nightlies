///Register `TAMPCR` reader
pub type R = crate::R<TAMPCRrs>;
///Register `TAMPCR` writer
pub type W = crate::W<TAMPCRrs>;
///Field `TAMP1E` reader - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled.
pub type TAMP1E_R = crate::BitReader;
///Field `TAMP1E` writer - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled.
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event.
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event.
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPIE` reader - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled.
pub type TAMPIE_R = crate::BitReader;
///Field `TAMPIE` writer - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled.
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPTS` reader - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register.
pub type TAMPTS_R = crate::BitReader;
///Field `TAMPTS` writer - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register.
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
pub type TAMPFREQ_R = crate::FieldReader;
///Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TAMPFLT` reader - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level.
pub type TAMPFLT_R = crate::FieldReader;
///Field `TAMPFLT` writer - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level.
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPRCH` reader - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles
pub type TAMPPRCH_R = crate::FieldReader;
///Field `TAMPPRCH` writer - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TAMPPUDIS` reader - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins.
pub type TAMPPUDIS_R = crate::BitReader;
///Field `TAMPPUDIS` writer - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins.
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1IE` reader - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled.
pub type TAMP1IE_R = crate::BitReader;
///Field `TAMP1IE` writer - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled.
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1NOERASE` reader - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers.
pub type TAMP1NOERASE_R = crate::BitReader;
///Field `TAMP1NOERASE` writer - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers.
pub type TAMP1NOERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1MF` reader - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased.
pub type TAMP1MF_R = crate::BitReader;
///Field `TAMP1MF` writer - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased.
pub type TAMP1MF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled.
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled.
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register.
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level.
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins.
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled.
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers.
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased.
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMPCR")
            .field("tamp1e", &self.tamp1e())
            .field("tamp1trg", &self.tamp1trg())
            .field("tampie", &self.tampie())
            .field("tampts", &self.tampts())
            .field("tampfreq", &self.tampfreq())
            .field("tampflt", &self.tampflt())
            .field("tampprch", &self.tampprch())
            .field("tamppudis", &self.tamppudis())
            .field("tamp1ie", &self.tamp1ie())
            .field("tamp1noerase", &self.tamp1noerase())
            .field("tamp1mf", &self.tamp1mf())
            .finish()
    }
}
impl W {
    ///Bit 0 - RTC_TAMP1 input detection enable 0: RTC_TAMP1 detection disabled 1: RTC_TAMP1 detection enabled.
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<'_, TAMPCRrs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Active level for RTC_TAMP1 input If TAMPFLT != 00 0: RTC_TAMP1 input staying low triggers a tamper detection event. 1: RTC_TAMP1 input staying high triggers a tamper detection event. if TAMPFLT = 00: 0: RTC_TAMP1 input rising edge triggers a tamper detection event. 1: RTC_TAMP1 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, TAMPCRrs> {
        TAMP1TRG_W::new(self, 1)
    }
    ///Bit 2 - Tamper interrupt enable 0: Tamper interrupt disabled 1: Tamper interrupt enabled.
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<'_, TAMPCRrs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 7 - Activate timestamp on tamper detection event 0: Tamper detection event does not cause a timestamp to be saved 1: Save timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register.
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<'_, TAMPCRrs> {
        TAMPTS_W::new(self, 7)
    }
    ///Bits 8:10 - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled. 0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz) 0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz) 0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz) 0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz) 0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz) 0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz) 0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz) 0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<'_, TAMPCRrs> {
        TAMPFREQ_W::new(self, 8)
    }
    ///Bits 11:12 - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs. 0x0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input). 0x1: Tamper event is activated after 2 consecutive samples at the active level. 0x2: Tamper event is activated after 4 consecutive samples at the active level. 0x3: Tamper event is activated after 8 consecutive samples at the active level.
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<'_, TAMPCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    ///Bits 13:14 - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs. 0x0: 1 RTCCLK cycle 0x1: 2 RTCCLK cycles 0x2: 4 RTCCLK cycles 0x3: 8 RTCCLK cycles
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<'_, TAMPCRrs> {
        TAMPPRCH_W::new(self, 13)
    }
    ///Bit 15 - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample. 0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up) 1: Disable precharge of RTC_TAMPx pins.
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<'_, TAMPCRrs> {
        TAMPPUDIS_W::new(self, 15)
    }
    ///Bit 16 - Tamper 1 interrupt enable 0: Tamper 1 interrupt is disabled if TAMPIE = 0. 1: Tamper 1 interrupt enabled.
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<'_, TAMPCRrs> {
        TAMP1IE_W::new(self, 16)
    }
    ///Bit 17 - Tamper 1 no erase 0: Tamper 1 event erases the backup registers. 1: Tamper 1 event does not erase the backup registers.
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W<'_, TAMPCRrs> {
        TAMP1NOERASE_W::new(self, 17)
    }
    ///Bit 18 - Tamper 1 mask flag 0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection. 1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware.The backup registers are not erased.
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W<'_, TAMPCRrs> {
        TAMP1MF_W::new(self, 18)
    }
}
/**RTC_TAMPCR register

You can [`read`](crate::Reg::read) this register and get [`tampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RTC:TAMPCR)*/
pub struct TAMPCRrs;
impl crate::RegisterSpec for TAMPCRrs {
    type Ux = u32;
}
///`read()` method returns [`tampcr::R`](R) reader structure
impl crate::Readable for TAMPCRrs {}
///`write(|w| ..)` method takes [`tampcr::W`](W) writer structure
impl crate::Writable for TAMPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TAMPCR to value 0
impl crate::Resettable for TAMPCRrs {}
