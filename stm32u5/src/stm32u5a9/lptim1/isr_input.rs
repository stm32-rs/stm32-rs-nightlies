#[doc = "Register `ISR_input` reader"]
pub type R = crate::R<ISR_INPUTrs>;
#[doc = "Field `CC1IF` reader - Compare 1 interrupt flag"]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `ARRM` reader - Autoreload match"]
pub type ARRM_R = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - External trigger edge event"]
pub type EXTTRIG_R = crate::BitReader;
#[doc = "Field `ARROK` reader - Autoreload register update OK"]
pub type ARROK_R = crate::BitReader;
#[doc = "Field `UP` reader - Counter direction change down to up"]
pub type UP_R = crate::BitReader;
#[doc = "Field `DOWN` reader - Counter direction change up to down"]
pub type DOWN_R = crate::BitReader;
#[doc = "Field `UE` reader - LPTIM update event occurred"]
pub type UE_R = crate::BitReader;
#[doc = "Field `REPOK` reader - Repetition register update Ok"]
pub type REPOK_R = crate::BitReader;
#[doc = "Field `CC2IF` reader - Capture 2 interrupt flag"]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC1OF` reader - Capture 1 over-capture flag"]
pub type CC1OF_R = crate::BitReader;
#[doc = "Field `CC2OF` reader - Capture 2 over-capture flag"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `DIEROK` reader - Interrupt enable register update OK"]
pub type DIEROK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger edge event"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update OK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter direction change down to up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter direction change up to down"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPTIM update event occurred"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Repetition register update Ok"]
    #[inline(always)]
    pub fn repok(&self) -> REPOK_R {
        REPOK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture 1 over-capture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture 2 over-capture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable register update OK"]
    #[inline(always)]
    pub fn dierok(&self) -> DIEROK_R {
        DIEROK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt and Status Register (intput mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr_input::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_INPUTrs;
impl crate::RegisterSpec for ISR_INPUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr_input::R`](R) reader structure"]
impl crate::Readable for ISR_INPUTrs {}
#[doc = "`reset()` method sets ISR_input to value 0"]
impl crate::Resettable for ISR_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
