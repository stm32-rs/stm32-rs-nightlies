///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `WUF1` reader - Wakeup flag 1
pub type WUF1_R = crate::BitReader;
///Field `WUF2` reader - Wakeup flag 2
pub type WUF2_R = crate::BitReader;
///Field `WUF3` reader - Wakeup flag 3
pub type WUF3_R = crate::BitReader;
///Field `WUF4` reader - Wakeup flag 4
pub type WUF4_R = crate::BitReader;
///Field `WUF5` reader - Wakeup flag 5
pub type WUF5_R = crate::BitReader;
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader;
///Field `SMPSBYPRDY` reader - SMPSBYPRDY
pub type SMPSBYPRDY_R = crate::BitReader;
///Field `EXTSMPSRDY` reader - EXTSMPSRDY
pub type EXTSMPSRDY_R = crate::BitReader;
///Field `SMPSHPRDY` reader - SMPSHPRDY
pub type SMPSHPRDY_R = crate::BitReader;
impl R {
    ///Bit 0 - Wakeup flag 1
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup flag 4
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup flag 5
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - SMPSBYPRDY
    #[inline(always)]
    pub fn smpsbyprdy(&self) -> SMPSBYPRDY_R {
        SMPSBYPRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EXTSMPSRDY
    #[inline(always)]
    pub fn extsmpsrdy(&self) -> EXTSMPSRDY_R {
        EXTSMPSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - SMPSHPRDY
    #[inline(always)]
    pub fn smpshprdy(&self) -> SMPSHPRDY_R {
        SMPSHPRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("smpshprdy", &self.smpshprdy())
            .field("extsmpsrdy", &self.extsmpsrdy())
            .field("smpsbyprdy", &self.smpsbyprdy())
            .field("sbf", &self.sbf())
            .field("wuf5", &self.wuf5())
            .field("wuf4", &self.wuf4())
            .field("wuf3", &self.wuf3())
            .field("wuf2", &self.wuf2())
            .field("wuf1", &self.wuf1())
            .finish()
    }
}
/**Power status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#PWR:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
