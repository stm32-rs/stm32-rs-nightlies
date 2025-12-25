///Register `CREL` reader
pub type R = crate::R<CRELrs>;
///Register `CREL` writer
pub type W = crate::W<CRELrs>;
///Field `DAY` reader - Time Stamp Day
pub type DAY_R = crate::FieldReader;
///Field `DAY` writer - Time Stamp Day
pub type DAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MON` reader - Time Stamp Month
pub type MON_R = crate::FieldReader;
///Field `MON` writer - Time Stamp Month
pub type MON_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `YEAR` reader - Time Stamp Year
pub type YEAR_R = crate::FieldReader;
///Field `YEAR` writer - Time Stamp Year
pub type YEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SUBSTEP` reader - Sub-step of Core Release
pub type SUBSTEP_R = crate::FieldReader;
///Field `SUBSTEP` writer - Sub-step of Core Release
pub type SUBSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `STEP` reader - Step of Core Release
pub type STEP_R = crate::FieldReader;
///Field `STEP` writer - Step of Core Release
pub type STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REL` reader - Core Release
pub type REL_R = crate::FieldReader;
///Field `REL` writer - Core Release
pub type REL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:7 - Time Stamp Day
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Time Stamp Month
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Time Stamp Year
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Sub-step of Core Release
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Step of Core Release
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Core Release
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
impl W {
    ///Bits 0:7 - Time Stamp Day
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W<'_, CRELrs> {
        DAY_W::new(self, 0)
    }
    ///Bits 8:15 - Time Stamp Month
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W<'_, CRELrs> {
        MON_W::new(self, 8)
    }
    ///Bits 16:19 - Time Stamp Year
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W<'_, CRELrs> {
        YEAR_W::new(self, 16)
    }
    ///Bits 20:23 - Sub-step of Core Release
    #[inline(always)]
    pub fn substep(&mut self) -> SUBSTEP_W<'_, CRELrs> {
        SUBSTEP_W::new(self, 20)
    }
    ///Bits 24:27 - Step of Core Release
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W<'_, CRELrs> {
        STEP_W::new(self, 24)
    }
    ///Bits 28:31 - Core Release
    #[inline(always)]
    pub fn rel(&mut self) -> REL_W<'_, CRELrs> {
        REL_W::new(self, 28)
    }
}
/**Clock Calibration Unit Core Release Register

You can [`read`](crate::Reg::read) this register and get [`crel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#CAN_CCU:CREL)*/
pub struct CRELrs;
impl crate::RegisterSpec for CRELrs {
    type Ux = u32;
}
///`read()` method returns [`crel::R`](R) reader structure
impl crate::Readable for CRELrs {}
///`write(|w| ..)` method takes [`crel::W`](W) writer structure
impl crate::Writable for CRELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CREL to value 0
impl crate::Resettable for CRELrs {}
