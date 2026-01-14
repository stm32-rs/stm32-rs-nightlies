///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `CMPM` reader - CMPM
pub type CMPM_R = crate::BitReader;
///Field `ARRM` reader - ARRM
pub type ARRM_R = crate::BitReader;
///Field `EXTTRIG` reader - EXTTRIG
pub type EXTTRIG_R = crate::BitReader;
///Field `CMPOK` reader - CMPOK
pub type CMPOK_R = crate::BitReader;
///Field `ARROK` reader - ARROK
pub type ARROK_R = crate::BitReader;
///Field `UP` reader - UP
pub type UP_R = crate::BitReader;
///Field `DOWN` reader - DOWN
pub type DOWN_R = crate::BitReader;
impl R {
    ///Bit 0 - CMPM
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ARRM
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EXTTRIG
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMPOK
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ARROK
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - UP
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DOWN
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("cmpm", &self.cmpm())
            .field("arrm", &self.arrm())
            .field("exttrig", &self.exttrig())
            .field("cmpok", &self.cmpok())
            .field("arrok", &self.arrok())
            .field("up", &self.up())
            .field("down", &self.down())
            .finish()
    }
}
/**LPTIM interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LPTIM1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
