///Register `SR1` reader
pub type R = crate::R<SR1rs>;
///Field `CWUF1` reader - Wakeup flag 1
pub type CWUF1_R = crate::BitReader;
///Field `CWUF2` reader - Wakeup flag 2
pub type CWUF2_R = crate::BitReader;
///Field `CWUF3` reader - Wakeup flag 3
pub type CWUF3_R = crate::BitReader;
///Field `CWUF4` reader - Wakeup flag 4
pub type CWUF4_R = crate::BitReader;
///Field `CWUF5` reader - Wakeup flag 5
pub type CWUF5_R = crate::BitReader;
///Field `CSBF` reader - Standby flag
pub type CSBF_R = crate::BitReader;
///Field `WUFI` reader - Wakeup flag internal
pub type WUFI_R = crate::BitReader;
impl R {
    ///Bit 0 - Wakeup flag 1
    #[inline(always)]
    pub fn cwuf1(&self) -> CWUF1_R {
        CWUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2
    #[inline(always)]
    pub fn cwuf2(&self) -> CWUF2_R {
        CWUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3
    #[inline(always)]
    pub fn cwuf3(&self) -> CWUF3_R {
        CWUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup flag 4
    #[inline(always)]
    pub fn cwuf4(&self) -> CWUF4_R {
        CWUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup flag 5
    #[inline(always)]
    pub fn cwuf5(&self) -> CWUF5_R {
        CWUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - Wakeup flag internal
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("wufi", &self.wufi())
            .field("csbf", &self.csbf())
            .field("cwuf5", &self.cwuf5())
            .field("cwuf4", &self.cwuf4())
            .field("cwuf3", &self.cwuf3())
            .field("cwuf2", &self.cwuf2())
            .field("cwuf1", &self.cwuf1())
            .finish()
    }
}
/**Power status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#PWR:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
