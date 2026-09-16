///Register `INIT2` reader
pub type R = crate::R<INIT2rs>;
///Register `INIT2` writer
pub type W = crate::W<INIT2rs>;
///Field `MIN_STABLE_CLOCK_X1` reader - MIN_STABLE_CLOCK_X1
pub type MIN_STABLE_CLOCK_X1_R = crate::FieldReader;
///Field `MIN_STABLE_CLOCK_X1` writer - MIN_STABLE_CLOCK_X1
pub type MIN_STABLE_CLOCK_X1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IDLE_AFTER_RESET_X32` reader - IDLE_AFTER_RESET_X32
pub type IDLE_AFTER_RESET_X32_R = crate::FieldReader;
///Field `IDLE_AFTER_RESET_X32` writer - IDLE_AFTER_RESET_X32
pub type IDLE_AFTER_RESET_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:3 - MIN_STABLE_CLOCK_X1
    #[inline(always)]
    pub fn min_stable_clock_x1(&self) -> MIN_STABLE_CLOCK_X1_R {
        MIN_STABLE_CLOCK_X1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:15 - IDLE_AFTER_RESET_X32
    #[inline(always)]
    pub fn idle_after_reset_x32(&self) -> IDLE_AFTER_RESET_X32_R {
        IDLE_AFTER_RESET_X32_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT2")
            .field("min_stable_clock_x1", &self.min_stable_clock_x1())
            .field("idle_after_reset_x32", &self.idle_after_reset_x32())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - MIN_STABLE_CLOCK_X1
    #[inline(always)]
    pub fn min_stable_clock_x1(&mut self) -> MIN_STABLE_CLOCK_X1_W<'_, INIT2rs> {
        MIN_STABLE_CLOCK_X1_W::new(self, 0)
    }
    ///Bits 8:15 - IDLE_AFTER_RESET_X32
    #[inline(always)]
    pub fn idle_after_reset_x32(&mut self) -> IDLE_AFTER_RESET_X32_W<'_, INIT2rs> {
        IDLE_AFTER_RESET_X32_W::new(self, 8)
    }
}
/**DDRCTRL SDRAM initialization register 2

You can [`read`](crate::Reg::read) this register and get [`init2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:INIT2)*/
pub struct INIT2rs;
impl crate::RegisterSpec for INIT2rs {
    type Ux = u32;
}
///`read()` method returns [`init2::R`](R) reader structure
impl crate::Readable for INIT2rs {}
///`write(|w| ..)` method takes [`init2::W`](W) writer structure
impl crate::Writable for INIT2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INIT2 to value 0x0d05
impl crate::Resettable for INIT2rs {
    const RESET_VALUE: u32 = 0x0d05;
}
