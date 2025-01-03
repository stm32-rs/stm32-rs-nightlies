///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
pub type CCPC_R = crate::BitReader;
///Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
pub type CCUS_R = crate::BitReader;
///Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS` reader - MMS\[2:0\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - MMS\[2:0\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - tim_ti1 selection
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - tim_ti1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1` reader - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1_R = crate::BitReader;
///Field `OIS1` writer - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS1N` reader - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1N_R = crate::BitReader;
///Field `OIS1N` writer - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2` reader - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit
pub type OIS2_R = crate::BitReader;
///Field `OIS2` writer - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS2N` reader - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit
pub type OIS2N_R = crate::BitReader;
///Field `OIS2N` writer - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit
pub type OIS3_R = crate::BitReader;
///Field `OIS3` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS3N` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit
pub type OIS3N_R = crate::BitReader;
///Field `OIS3N` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS4` reader - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit
pub type OIS4_R = crate::BitReader;
///Field `OIS4` writer - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS4N` reader - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit
pub type OIS4N_R = crate::BitReader;
///Field `OIS4N` writer - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit
pub type OIS4N_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS5` reader - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit
pub type OIS5_R = crate::BitReader;
///Field `OIS5` writer - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OIS6` reader - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit
pub type OIS6_R = crate::BitReader;
///Field `OIS6` writer - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS2_R = crate::FieldReader;
///Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MMS_1` reader - MMS\[3\]
pub type MMS_1_R = crate::BitReader;
///Field `MMS_1` writer - MMS\[3\]
pub type MMS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - MMS\[2:0\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - tim_ti1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois4n(&self) -> OIS4N_R {
        OIS4N_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 25 - MMS\[3\]
    #[inline(always)]
    pub fn mms_1(&self) -> MMS_1_R {
        MMS_1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ccpc", &self.ccpc())
            .field("ccus", &self.ccus())
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("ois1", &self.ois1())
            .field("ois1n", &self.ois1n())
            .field("ois2", &self.ois2())
            .field("ois2n", &self.ois2n())
            .field("ois3", &self.ois3())
            .field("ois3n", &self.ois3n())
            .field("ois4", &self.ois4())
            .field("ois4n", &self.ois4n())
            .field("ois5", &self.ois5())
            .field("ois6", &self.ois6())
            .field("mms2", &self.mms2())
            .field("mms_1", &self.mms_1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - MMS\[2:0\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - tim_ti1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<CR2rs> {
        OIS1_W::new(self, 8)
    }
    ///Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<CR2rs> {
        OIS1N_W::new(self, 9)
    }
    ///Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS2_W<CR2rs> {
        OIS2_W::new(self, 10)
    }
    ///Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois2n(&mut self) -> OIS2N_W<CR2rs> {
        OIS2N_W::new(self, 11)
    }
    ///Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS3_W<CR2rs> {
        OIS3_W::new(self, 12)
    }
    ///Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois3n(&mut self) -> OIS3N_W<CR2rs> {
        OIS3N_W::new(self, 13)
    }
    ///Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS4_W<CR2rs> {
        OIS4_W::new(self, 14)
    }
    ///Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit
    #[inline(always)]
    pub fn ois4n(&mut self) -> OIS4N_W<CR2rs> {
        OIS4N_W::new(self, 15)
    }
    ///Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS5_W<CR2rs> {
        OIS5_W::new(self, 16)
    }
    ///Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS6_W<CR2rs> {
        OIS6_W::new(self, 18)
    }
    ///Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W<CR2rs> {
        MMS2_W::new(self, 20)
    }
    ///Bit 25 - MMS\[3\]
    #[inline(always)]
    pub fn mms_1(&mut self) -> MMS_1_W<CR2rs> {
        MMS_1_W::new(self, 25)
    }
}
/**TIM1 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#TIM1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
