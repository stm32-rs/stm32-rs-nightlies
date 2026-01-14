///Register `WORD2` reader
pub type R = crate::R<WORD2rs>;
///Register `WORD2` writer
pub type W = crate::W<WORD2rs>;
///Field `TransmitCalDelayChk` reader - Delay between TX request sent to the Radio FSM and the start pulse sent to the transmit block.
pub type TRANSMIT_CAL_DELAY_CHK_R = crate::FieldReader;
///Field `TransmitCalDelayChk` writer - Delay between TX request sent to the Radio FSM and the start pulse sent to the transmit block.
pub type TRANSMIT_CAL_DELAY_CHK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TransmitNoCalDelayChk` reader - Delay between TX request sent to the Radio FSM and the start pulse to the transmit block.
pub type TRANSMIT_NO_CAL_DELAY_CHK_R = crate::FieldReader;
///Field `TransmitNoCalDelayChk` writer - Delay between TX request sent to the Radio FSM and the start pulse to the transmit block.
pub type TRANSMIT_NO_CAL_DELAY_CHK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ReceiveCalDelayChk` reader - Delay between RX request sent to the Radio FSM and the start pulse sent to the receive block.
pub type RECEIVE_CAL_DELAY_CHK_R = crate::FieldReader;
///Field `ReceiveCalDelayChk` writer - Delay between RX request sent to the Radio FSM and the start pulse sent to the receive block.
pub type RECEIVE_CAL_DELAY_CHK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ReceiveNoCalDelayChk` reader - Delay between RX request sent to the Radio FSM and the start pulse to the receive block.
pub type RECEIVE_NO_CAL_DELAY_CHK_R = crate::FieldReader;
///Field `ReceiveNoCalDelayChk` writer - Delay between RX request sent to the Radio FSM and the start pulse to the receive block.
pub type RECEIVE_NO_CAL_DELAY_CHK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Delay between TX request sent to the Radio FSM and the start pulse sent to the transmit block.
    #[inline(always)]
    pub fn transmit_cal_delay_chk(&self) -> TRANSMIT_CAL_DELAY_CHK_R {
        TRANSMIT_CAL_DELAY_CHK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Delay between TX request sent to the Radio FSM and the start pulse to the transmit block.
    #[inline(always)]
    pub fn transmit_no_cal_delay_chk(&self) -> TRANSMIT_NO_CAL_DELAY_CHK_R {
        TRANSMIT_NO_CAL_DELAY_CHK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Delay between RX request sent to the Radio FSM and the start pulse sent to the receive block.
    #[inline(always)]
    pub fn receive_cal_delay_chk(&self) -> RECEIVE_CAL_DELAY_CHK_R {
        RECEIVE_CAL_DELAY_CHK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Delay between RX request sent to the Radio FSM and the start pulse to the receive block.
    #[inline(always)]
    pub fn receive_no_cal_delay_chk(&self) -> RECEIVE_NO_CAL_DELAY_CHK_R {
        RECEIVE_NO_CAL_DELAY_CHK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD2")
            .field("transmit_cal_delay_chk", &self.transmit_cal_delay_chk())
            .field(
                "transmit_no_cal_delay_chk",
                &self.transmit_no_cal_delay_chk(),
            )
            .field("receive_cal_delay_chk", &self.receive_cal_delay_chk())
            .field("receive_no_cal_delay_chk", &self.receive_no_cal_delay_chk())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Delay between TX request sent to the Radio FSM and the start pulse sent to the transmit block.
    #[inline(always)]
    pub fn transmit_cal_delay_chk(&mut self) -> TRANSMIT_CAL_DELAY_CHK_W<'_, WORD2rs> {
        TRANSMIT_CAL_DELAY_CHK_W::new(self, 0)
    }
    ///Bits 8:15 - Delay between TX request sent to the Radio FSM and the start pulse to the transmit block.
    #[inline(always)]
    pub fn transmit_no_cal_delay_chk(&mut self) -> TRANSMIT_NO_CAL_DELAY_CHK_W<'_, WORD2rs> {
        TRANSMIT_NO_CAL_DELAY_CHK_W::new(self, 8)
    }
    ///Bits 16:23 - Delay between RX request sent to the Radio FSM and the start pulse sent to the receive block.
    #[inline(always)]
    pub fn receive_cal_delay_chk(&mut self) -> RECEIVE_CAL_DELAY_CHK_W<'_, WORD2rs> {
        RECEIVE_CAL_DELAY_CHK_W::new(self, 16)
    }
    ///Bits 24:31 - Delay between RX request sent to the Radio FSM and the start pulse to the receive block.
    #[inline(always)]
    pub fn receive_no_cal_delay_chk(&mut self) -> RECEIVE_NO_CAL_DELAY_CHK_W<'_, WORD2rs> {
        RECEIVE_NO_CAL_DELAY_CHK_W::new(self, 24)
    }
}
/**WORD2 register

You can [`read`](crate::Reg::read) this register and get [`word2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#GLOBALSTATMACH:WORD2)*/
pub struct WORD2rs;
impl crate::RegisterSpec for WORD2rs {
    type Ux = u32;
}
///`read()` method returns [`word2::R`](R) reader structure
impl crate::Readable for WORD2rs {}
///`write(|w| ..)` method takes [`word2::W`](W) writer structure
impl crate::Writable for WORD2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD2 to value 0
impl crate::Resettable for WORD2rs {}
