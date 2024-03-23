#[doc = "Register `LPTIM_ISR` reader"]
pub type R = crate::R<LPTIM_ISRrs>;
#[doc = "Field `CMPM` reader - CMPM"]
pub type CMPM_R = crate::BitReader;
#[doc = "Field `ARRM` reader - ARRM"]
pub type ARRM_R = crate::BitReader;
#[doc = "Field `EXTTRIG` reader - EXTTRIG"]
pub type EXTTRIG_R = crate::BitReader;
#[doc = "Field `CMPOK` reader - CMPOK"]
pub type CMPOK_R = crate::BitReader;
#[doc = "Field `ARROK` reader - ARROK"]
pub type ARROK_R = crate::BitReader;
#[doc = "Field `UP` reader - UP"]
pub type UP_R = crate::BitReader;
#[doc = "Field `DOWN` reader - DOWN"]
pub type DOWN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CMPM"]
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARRM"]
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EXTTRIG"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMPOK"]
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARROK"]
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UP"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOWN"]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "LPTIM interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_ISRrs;
impl crate::RegisterSpec for LPTIM_ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_isr::R`](R) reader structure"]
impl crate::Readable for LPTIM_ISRrs {}
#[doc = "`reset()` method sets LPTIM_ISR to value 0"]
impl crate::Resettable for LPTIM_ISRrs {
    const RESET_VALUE: u32 = 0;
}
