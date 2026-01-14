///Register `ISR_INPUT` reader
pub type R = crate::R<ISR_INPUTrs>;
///Field `CC1IF` reader - capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high.
pub type CC1IF_R = crate::BitReader;
///Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registers value reached the LPTIM_ARR registers value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
pub type ARRM_R = crate::BitReader;
///Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
pub type EXTTRIG_R = crate::BitReader;
///Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
pub type ARROK_R = crate::BitReader;
///Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type UP_R = crate::BitReader;
///Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type DOWN_R = crate::BitReader;
///Field `UE` reader - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register.
pub type UE_R = crate::BitReader;
///Field `REPOK` reader - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register.
pub type REPOK_R = crate::BitReader;
///Field `CC2IF` reader - Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2IF_R = crate::BitReader;
///Field `CC3IF` reader - Capture 3 interrupt flag If channel CC3 is configured as input: CC3IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR3 register. The corresponding interrupt or DMA request is generated if enabled. The CC3OF flag is set if the CC3IF flag was already high. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3IF_R = crate::BitReader;
///Field `CC4IF` reader - Capture 4 interrupt flag If channel CC4 is configured as input: CC4IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR4 register. The corresponding interrupt or DMA request is generated if enabled. The CC4OF flag is set if the CC4IF flag was already high. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4IF_R = crate::BitReader;
///Field `CC1OF` reader - Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
pub type CC1OF_R = crate::BitReader;
///Field `CC2OF` reader - Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2OF_R = crate::BitReader;
///Field `CC3OF` reader - Capture 3 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC3OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3OF_R = crate::BitReader;
///Field `CC4OF` reader - Capture 4 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC4OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4OF_R = crate::BitReader;
///Field `DIEROK` reader - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register.
pub type DIEROK_R = crate::BitReader;
impl R {
    ///Bit 0 - capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high.
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registers value reached the LPTIM_ARR registers value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture 3 interrupt flag If channel CC3 is configured as input: CC3IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR3 register. The corresponding interrupt or DMA request is generated if enabled. The CC3OF flag is set if the CC3IF flag was already high. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture 4 interrupt flag If channel CC4 is configured as input: CC4IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR4 register. The corresponding interrupt or DMA request is generated if enabled. The CC4OF flag is set if the CC4IF flag was already high. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Capture 3 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC3OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Capture 4 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC4OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn dierok(&self) -> DIEROK_R {
        DIEROK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR_INPUT")
            .field("cc1if", &self.cc1if())
            .field("arrm", &self.arrm())
            .field("exttrig", &self.exttrig())
            .field("arrok", &self.arrok())
            .field("up", &self.up())
            .field("down", &self.down())
            .field("ue", &self.ue())
            .field("repok", &self.repok())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .field("dierok", &self.dierok())
            .finish()
    }
}
/**LPTIM1 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_input::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM1:ISR_INPUT)*/
pub struct ISR_INPUTrs;
impl crate::RegisterSpec for ISR_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`isr_input::R`](R) reader structure
impl crate::Readable for ISR_INPUTrs {}
///`reset()` method sets ISR_INPUT to value 0
impl crate::Resettable for ISR_INPUTrs {}
