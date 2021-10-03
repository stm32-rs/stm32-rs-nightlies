#[doc = "Register `LPTIM_ISR` reader"]
pub struct R(crate::R<LPTIM_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMPM` reader - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP registerâ\u{80}\u{99}s value."]
pub struct CMPM_R(crate::FieldReader<bool, bool>);
impl CMPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registerâ\u{80}\u{99}s value reached the LPTIM_ARR registerâ\u{80}\u{99}s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
pub struct ARRM_R(crate::FieldReader<bool, bool>);
impl ARRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
pub struct EXTTRIG_R(crate::FieldReader<bool, bool>);
impl EXTTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPOK` reader - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed."]
pub struct CMPOK_R(crate::FieldReader<bool, bool>);
impl CMPOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
pub struct ARROK_R(crate::FieldReader<bool, bool>);
impl ARROK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARROK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARROK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub struct UP_R(crate::FieldReader<bool, bool>);
impl UP_R {
    pub(crate) fn new(bits: bool) -> Self {
        UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub struct DOWN_R(crate::FieldReader<bool, bool>);
impl DOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP registerâ\u{80}\u{99}s value."]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registerâ\u{80}\u{99}s value reached the LPTIM_ARR registerâ\u{80}\u{99}s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed."]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Interrupt and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_isr](index.html) module"]
pub struct LPTIM_ISR_SPEC;
impl crate::RegisterSpec for LPTIM_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_isr::R](R) reader structure"]
impl crate::Readable for LPTIM_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPTIM_ISR to value 0"]
impl crate::Resettable for LPTIM_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
