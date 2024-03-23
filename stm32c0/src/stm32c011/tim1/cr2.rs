#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_R = crate::BitReader;
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_R = crate::BitReader;
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS` reader - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader;
#[doc = "Field `MMS` writer - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type TI1S_R = crate::BitReader;
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_R = crate::BitReader;
#[doc = "Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_R = crate::BitReader;
#[doc = "Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub type OIS2_R = crate::BitReader;
#[doc = "Field `OIS2` writer - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub type OIS2N_R = crate::BitReader;
#[doc = "Field `OIS2N` writer - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub type OIS3_R = crate::BitReader;
#[doc = "Field `OIS3` writer - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub type OIS3N_R = crate::BitReader;
#[doc = "Field `OIS3N` writer - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub type OIS4_R = crate::BitReader;
#[doc = "Field `OIS4` writer - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub type OIS5_R = crate::BitReader;
#[doc = "Field `OIS5` writer - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub type OIS6_R = crate::BitReader;
#[doc = "Field `OIS6` writer - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_R = crate::FieldReader;
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<CR2rs> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<CR2rs> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<CR2rs> {
        OIS1N_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<CR2rs> {
        OIS2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<CR2rs> {
        OIS2N_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<CR2rs> {
        OIS3_W::new(self, 12)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<CR2rs> {
        OIS3N_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<CR2rs> {
        OIS4_W::new(self, 14)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<CR2rs> {
        OIS5_W::new(self, 16)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<CR2rs> {
        OIS6_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<CR2rs> {
        MMS2_W::new(self, 20)
    }
}
#[doc = "TIM1 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
