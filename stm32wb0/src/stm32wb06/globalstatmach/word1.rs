///Register `WORD1` reader
pub type R = crate::R<WORD1rs>;
///Register `WORD1` writer
pub type W = crate::W<WORD1rs>;
///Field `CurStMachNum` reader - current connection machine number.
pub type CUR_ST_MACH_NUM_R = crate::FieldReader;
///Field `CurStMachNum` writer - current connection machine number.
pub type CUR_ST_MACH_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `Active` reader - Must be at '1' when the trig event (Wakeup Timer, Timer1 or Timer2) occurs to starts a Bluetooth LE link layer sequence.
pub type ACTIVE_R = crate::BitReader;
///Field `Active` writer - Must be at '1' when the trig event (Wakeup Timer, Timer1 or Timer2) occurs to starts a Bluetooth LE link layer sequence.
pub type ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WakeupInitDelay` reader - Delay between wakeup timer trig event on Sequencer and RX/TX request sending to the Radio FSM.
pub type WAKEUP_INIT_DELAY_R = crate::FieldReader;
///Field `WakeupInitDelay` writer - Delay between wakeup timer trig event on Sequencer and RX/TX request sending to the Radio FSM.
pub type WAKEUP_INIT_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `Timer12InitDelayCal` reader - Delay between Timer1 or Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
pub type TIMER12INIT_DELAY_CAL_R = crate::FieldReader;
///Field `Timer12InitDelayCal` writer - Delay between Timer1 or Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
pub type TIMER12INIT_DELAY_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `Timer2InitDelayNoCal` reader - Delay between Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
pub type TIMER2INIT_DELAY_NO_CAL_R = crate::FieldReader;
///Field `Timer2InitDelayNoCal` writer - Delay between Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
pub type TIMER2INIT_DELAY_NO_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:6 - current connection machine number.
    #[inline(always)]
    pub fn cur_st_mach_num(&self) -> CUR_ST_MACH_NUM_R {
        CUR_ST_MACH_NUM_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Must be at '1' when the trig event (Wakeup Timer, Timer1 or Timer2) occurs to starts a Bluetooth LE link layer sequence.
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Delay between wakeup timer trig event on Sequencer and RX/TX request sending to the Radio FSM.
    #[inline(always)]
    pub fn wakeup_init_delay(&self) -> WAKEUP_INIT_DELAY_R {
        WAKEUP_INIT_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Delay between Timer1 or Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
    #[inline(always)]
    pub fn timer12init_delay_cal(&self) -> TIMER12INIT_DELAY_CAL_R {
        TIMER12INIT_DELAY_CAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Delay between Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
    #[inline(always)]
    pub fn timer2init_delay_no_cal(&self) -> TIMER2INIT_DELAY_NO_CAL_R {
        TIMER2INIT_DELAY_NO_CAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD1")
            .field("cur_st_mach_num", &self.cur_st_mach_num())
            .field("active", &self.active())
            .field("wakeup_init_delay", &self.wakeup_init_delay())
            .field("timer12init_delay_cal", &self.timer12init_delay_cal())
            .field("timer2init_delay_no_cal", &self.timer2init_delay_no_cal())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - current connection machine number.
    #[inline(always)]
    pub fn cur_st_mach_num(&mut self) -> CUR_ST_MACH_NUM_W<'_, WORD1rs> {
        CUR_ST_MACH_NUM_W::new(self, 0)
    }
    ///Bit 7 - Must be at '1' when the trig event (Wakeup Timer, Timer1 or Timer2) occurs to starts a Bluetooth LE link layer sequence.
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W<'_, WORD1rs> {
        ACTIVE_W::new(self, 7)
    }
    ///Bits 8:15 - Delay between wakeup timer trig event on Sequencer and RX/TX request sending to the Radio FSM.
    #[inline(always)]
    pub fn wakeup_init_delay(&mut self) -> WAKEUP_INIT_DELAY_W<'_, WORD1rs> {
        WAKEUP_INIT_DELAY_W::new(self, 8)
    }
    ///Bits 16:23 - Delay between Timer1 or Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
    #[inline(always)]
    pub fn timer12init_delay_cal(&mut self) -> TIMER12INIT_DELAY_CAL_W<'_, WORD1rs> {
        TIMER12INIT_DELAY_CAL_W::new(self, 16)
    }
    ///Bits 24:31 - Delay between Timer2 trig event on Sequencer and RX/TX request sending to the Radio FSM.
    #[inline(always)]
    pub fn timer2init_delay_no_cal(&mut self) -> TIMER2INIT_DELAY_NO_CAL_W<'_, WORD1rs> {
        TIMER2INIT_DELAY_NO_CAL_W::new(self, 24)
    }
}
/**WORD1 register

You can [`read`](crate::Reg::read) this register and get [`word1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#GLOBALSTATMACH:WORD1)*/
pub struct WORD1rs;
impl crate::RegisterSpec for WORD1rs {
    type Ux = u32;
}
///`read()` method returns [`word1::R`](R) reader structure
impl crate::Readable for WORD1rs {}
///`write(|w| ..)` method takes [`word1::W`](W) writer structure
impl crate::Writable for WORD1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD1 to value 0
impl crate::Resettable for WORD1rs {}
