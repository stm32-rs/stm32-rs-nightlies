///Register `SHHR` reader
pub type R = crate::R<SHHRrs>;
///Register `SHHR` writer
pub type W = crate::W<SHHRrs>;
///Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
pub type THOLD1_R = crate::FieldReader<u16>;
///Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
pub type THOLD1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHHR")
            .field("thold1", &self.thold1())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)
    #[inline(always)]
    pub fn thold1(&mut self) -> THOLD1_W<SHHRrs> {
        THOLD1_W::new(self, 0)
    }
}
/**Sample and Hold hold time register

You can [`read`](crate::Reg::read) this register and get [`shhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DAC:SHHR)*/
pub struct SHHRrs;
impl crate::RegisterSpec for SHHRrs {
    type Ux = u32;
}
///`read()` method returns [`shhr::R`](R) reader structure
impl crate::Readable for SHHRrs {}
///`write(|w| ..)` method takes [`shhr::W`](W) writer structure
impl crate::Writable for SHHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SHHR to value 0x0001_0001
impl crate::Resettable for SHHRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
