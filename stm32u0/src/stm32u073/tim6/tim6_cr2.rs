///Register `TIM6_CR2` reader
pub type R = crate::R<TIM6_CR2rs>;
///Register `TIM6_CR2` writer
pub type W = crate::W<TIM6_CR2rs>;
///Field `MMS` reader - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in the TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in the TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in the TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM6_CR2")
            .field("mms", &self.mms())
            .finish()
    }
}
impl W {
    ///Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in the TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<TIM6_CR2rs> {
        MMS_W::new(self, 4)
    }
}
/**TIM6 control register 2

You can [`read`](crate::Reg::read) this register and get [`tim6_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_CR2)*/
pub struct TIM6_CR2rs;
impl crate::RegisterSpec for TIM6_CR2rs {
    type Ux = u16;
}
///`read()` method returns [`tim6_cr2::R`](R) reader structure
impl crate::Readable for TIM6_CR2rs {}
///`write(|w| ..)` method takes [`tim6_cr2::W`](W) writer structure
impl crate::Writable for TIM6_CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM6_CR2 to value 0
impl crate::Resettable for TIM6_CR2rs {
    const RESET_VALUE: u16 = 0;
}
