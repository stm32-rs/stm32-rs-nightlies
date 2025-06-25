///Register `TIM_CR2` reader
pub type R = crate::R<TIM_CR2rs>;
///Register `TIM_CR2` writer
pub type W = crate::W<TIM_CR2rs>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS` reader - MMS\[0\]: Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - MMS\[0\]: Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - tim_ti1 selection
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - tim_ti1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS_1` reader - MMS\[3\]
pub type MMS_1_R = crate::BitReader;
///Field `MMS_1` writer - MMS\[3\]
pub type MMS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - MMS\[0\]: Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - tim_ti1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 25 - MMS\[3\]
    #[inline(always)]
    pub fn mms_1(&self) -> MMS_1_R {
        MMS_1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM_CR2")
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .field("mms_1", &self.mms_1())
            .finish()
    }
}
impl W {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<TIM_CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - MMS\[0\]: Master mode selection These bits allow to select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on tim_trgo, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Others: Reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<TIM_CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - tim_ti1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<TIM_CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Bit 25 - MMS\[3\]
    #[inline(always)]
    pub fn mms_1(&mut self) -> MMS_1_W<TIM_CR2rs> {
        MMS_1_W::new(self, 25)
    }
}
/**TIM control register 2

You can [`read`](crate::Reg::read) this register and get [`tim_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TIM2:TIM_CR2)*/
pub struct TIM_CR2rs;
impl crate::RegisterSpec for TIM_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`tim_cr2::R`](R) reader structure
impl crate::Readable for TIM_CR2rs {}
///`write(|w| ..)` method takes [`tim_cr2::W`](W) writer structure
impl crate::Writable for TIM_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM_CR2 to value 0
impl crate::Resettable for TIM_CR2rs {}
