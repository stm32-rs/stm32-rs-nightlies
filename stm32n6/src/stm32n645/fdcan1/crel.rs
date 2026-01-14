///Register `CREL` reader
pub type R = crate::R<CRELrs>;
///Field `DAY` reader - Timestamp day =18
pub type DAY_R = crate::FieldReader;
///Field `MON` reader - Timestamp month = 12
pub type MON_R = crate::FieldReader;
///Field `YEAR` reader - Timestamp year = 4
pub type YEAR_R = crate::FieldReader;
///Field `SUBSTEP` reader - Sub-step of core release = 1
pub type SUBSTEP_R = crate::FieldReader;
///Field `STEP` reader - Step of core release = 2
pub type STEP_R = crate::FieldReader;
///Field `REL` reader - Core release = 3
pub type REL_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Timestamp day =18
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Timestamp month = 12
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Timestamp year = 4
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Sub-step of core release = 1
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Step of core release = 2
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Core release = 3
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CREL")
            .field("day", &self.day())
            .field("mon", &self.mon())
            .field("year", &self.year())
            .field("substep", &self.substep())
            .field("step", &self.step())
            .field("rel", &self.rel())
            .finish()
    }
}
/**FDCAN core release register

You can [`read`](crate::Reg::read) this register and get [`crel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FDCAN1:CREL)*/
pub struct CRELrs;
impl crate::RegisterSpec for CRELrs {
    type Ux = u32;
}
///`read()` method returns [`crel::R`](R) reader structure
impl crate::Readable for CRELrs {}
///`reset()` method sets CREL to value 0x3214_1218
impl crate::Resettable for CRELrs {
    const RESET_VALUE: u32 = 0x3214_1218;
}
