///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `CMPM` reader - Compare match
pub type CMPM_R = crate::BitReader;
///Field `ARRM` reader - Autoreload match
pub type ARRM_R = crate::BitReader;
///Field `EXTTRIG` reader - External trigger edge event
pub type EXTTRIG_R = crate::BitReader;
///Field `CMPOK` reader - Compare register update OK
pub type CMPOK_R = crate::BitReader;
///Field `ARROK` reader - Autoreload register update OK
pub type ARROK_R = crate::BitReader;
///Field `UP` reader - Counter direction change down to up
pub type UP_R = crate::BitReader;
///Field `DOWN` reader - Counter direction change up to down
pub type DOWN_R = crate::BitReader;
impl R {
    ///Bit 0 - Compare match
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger edge event
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register update OK
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Counter direction change down to up
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter direction change up to down
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("down", &self.down())
            .field("up", &self.up())
            .field("arrok", &self.arrok())
            .field("cmpok", &self.cmpok())
            .field("exttrig", &self.exttrig())
            .field("arrm", &self.arrm())
            .field("cmpm", &self.cmpm())
            .finish()
    }
}
/**Interrupt and Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#LPTIM:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
