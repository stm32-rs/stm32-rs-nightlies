///Register `ISR_OUTPUT` reader
pub type R = crate::R<ISR_OUTPUTrs>;
///Field `CC1IF` reader - Compare 1 interrupt flag If channel CC1 is configured as output: The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register.
pub type CC1IF_R = crate::BitReader;
///Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registers value reached the LPTIM_ARR registers value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
pub type ARRM_R = crate::BitReader;
///Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
pub type EXTTRIG_R = crate::BitReader;
///Field `CMP1OK` reader - Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register.
pub type CMP1OK_R = crate::BitReader;
///Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
pub type ARROK_R = crate::BitReader;
///Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type UP_R = crate::BitReader;
///Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Refer to Section125.3.
pub type DOWN_R = crate::BitReader;
///Field `UE` reader - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. The corresponding interrupt or DMA request is generated if enabled. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register. The UE flag is automatically cleared by hardware once the LPTIM_ARR register is written by any bus master like CPU or DMA.
pub type UE_R = crate::BitReader;
///Field `REPOK` reader - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register.
pub type REPOK_R = crate::BitReader;
///Field `CC2IF` reader - Compare 2 interrupt flag If channel CC2 is configured as output: The CC2IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CC2IF_R = crate::BitReader;
///Field `CC3IF` reader - Compare 3 interrupt flag If channel CC3 is configured as output: The CC3IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CC3IF_R = crate::BitReader;
///Field `CC4IF` reader - Compare 4 interrupt flag If channel CC4 is configured as output: The CC4IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CC4IF_R = crate::BitReader;
///Field `CMP2OK` reader - Compare register 2 update OK CMP2OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR2 register has been successfully completed. CMP2OK flag can be cleared by writing 1 to the CMP2OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
pub type CMP2OK_R = crate::BitReader;
///Field `CMP3OK` reader - Compare register 3 update OK CMP3OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR3 register has been successfully completed. CMP3OK flag can be cleared by writing 1 to the CMP3OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
pub type CMP3OK_R = crate::BitReader;
///Field `CMP4OK` reader - Compare register 4 update OK CMP4OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR4 register has been successfully completed. CMP4OK flag can be cleared by writing 1 to the CMP4OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
pub type CMP4OK_R = crate::BitReader;
///Field `DIEROK` reader - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register.
pub type DIEROK_R = crate::BitReader;
impl R {
    ///Bit 0 - Compare 1 interrupt flag If channel CC1 is configured as output: The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register.
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
    ///Bit 3 - Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn cmp1ok(&self) -> CMP1OK_R {
        CMP1OK_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bit 7 - LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. The corresponding interrupt or DMA request is generated if enabled. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register. The UE flag is automatically cleared by hardware once the LPTIM_ARR register is written by any bus master like CPU or DMA.
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Compare 2 interrupt flag If channel CC2 is configured as output: The CC2IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC2IF flag can be cleared by writing 1 to the CC2CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Compare 3 interrupt flag If channel CC3 is configured as output: The CC3IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC3IF flag can be cleared by writing 1 to the CC3CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Compare 4 interrupt flag If channel CC4 is configured as output: The CC4IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. CC4IF flag can be cleared by writing 1 to the CC4CF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 19 - Compare register 2 update OK CMP2OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR2 register has been successfully completed. CMP2OK flag can be cleared by writing 1 to the CMP2OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 2 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp2ok(&self) -> CMP2OK_R {
        CMP2OK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Compare register 3 update OK CMP3OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR3 register has been successfully completed. CMP3OK flag can be cleared by writing 1 to the CMP3OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 3 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp3ok(&self) -> CMP3OK_R {
        CMP3OK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Compare register 4 update OK CMP4OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR4 register has been successfully completed. CMP4OK flag can be cleared by writing 1 to the CMP4OKCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 4 channels this bit is reserved. Refer to Section125.3.
    #[inline(always)]
    pub fn cmp4ok(&self) -> CMP4OK_R {
        CMP4OK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn dierok(&self) -> DIEROK_R {
        DIEROK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR_OUTPUT")
            .field("cc1if", &self.cc1if())
            .field("arrm", &self.arrm())
            .field("exttrig", &self.exttrig())
            .field("cmp1ok", &self.cmp1ok())
            .field("arrok", &self.arrok())
            .field("up", &self.up())
            .field("down", &self.down())
            .field("ue", &self.ue())
            .field("repok", &self.repok())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("cmp2ok", &self.cmp2ok())
            .field("cmp3ok", &self.cmp3ok())
            .field("cmp4ok", &self.cmp4ok())
            .field("dierok", &self.dierok())
            .finish()
    }
}
/**LPTIM2 interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr_output::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM2:ISR_OUTPUT)*/
pub struct ISR_OUTPUTrs;
impl crate::RegisterSpec for ISR_OUTPUTrs {
    type Ux = u32;
}
///`read()` method returns [`isr_output::R`](R) reader structure
impl crate::Readable for ISR_OUTPUTrs {}
///`reset()` method sets ISR_OUTPUT to value 0
impl crate::Resettable for ISR_OUTPUTrs {}
