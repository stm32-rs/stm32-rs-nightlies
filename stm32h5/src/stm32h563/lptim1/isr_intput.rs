#[doc = "Register `ISR_intput` reader"]
pub type R = crate::R<ISR_INTPUTrs>;
#[doc = "Field `CC1IF` reader - capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
pub type ARRM_R = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
pub type EXTTRIG_R = crate::BitReader;
#[doc = "Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
pub type ARROK_R = crate::BitReader;
#[doc = "Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type UP_R = crate::BitReader;
#[doc = "Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type DOWN_R = crate::BitReader;
#[doc = "Field `UE` reader - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register."]
pub type UE_R = crate::BitReader;
#[doc = "Field `REPOK` reader - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
pub type REPOK_R = crate::BitReader;
#[doc = "Field `CC2IF` reader - Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC1OF` reader - Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC2OF` reader - Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `DIEROK` reader - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
pub type DIEROK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - capture 1 interrupt flag If channel CC1 is configured as input: CC1IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR1 register. The corresponding interrupt or DMA request is generated if enabled. The CC1OF flag is set if the CC1IF flag was already high."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 2 interrupt flag If channel CC2 is configured as input: CC2IF is set by hardware to inform application that the current value of the counter is captured in LPTIM_CCR2 register. The corresponding interrupt or DMA request is generated if enabled. The CC2OF flag is set if the CC2IF flag was already high. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture 2 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC2OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn dierok(&self) -> DIEROK_R {
        DIEROK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "LPTIM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr_intput::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_INTPUTrs;
impl crate::RegisterSpec for ISR_INTPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr_intput::R`](R) reader structure"]
impl crate::Readable for ISR_INTPUTrs {}
#[doc = "`reset()` method sets ISR_intput to value 0"]
impl crate::Resettable for ISR_INTPUTrs {
    const RESET_VALUE: u32 = 0;
}
