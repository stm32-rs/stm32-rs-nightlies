///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader;
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MMS` reader - Master mode selection These bits permit to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_R = crate::FieldReader;
///Field `MMS` writer - Master mode selection These bits permit to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TI1S` reader - TI1 selection
pub type TI1S_R = crate::BitReader;
///Field `TI1S` writer - TI1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection These bits permit to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ccds", &self.ccds())
            .field("mms", &self.mms())
            .field("ti1s", &self.ti1s())
            .finish()
    }
}
impl W {
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - Master mode selection These bits permit to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
}
/**TIM3 control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#TIM3:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u16 = 0;
}
