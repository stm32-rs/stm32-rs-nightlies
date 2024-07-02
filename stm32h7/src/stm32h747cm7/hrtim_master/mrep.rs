///Register `MREP` reader
pub type R = crate::R<MREPrs>;
///Register `MREP` writer
pub type W = crate::W<MREPrs>;
///Field `MREP` reader - Master Timer Repetition counter value
pub type MREP_R = crate::FieldReader;
///Field `MREP` writer - Master Timer Repetition counter value
pub type MREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MREP").field("mrep", &self.mrep()).finish()
    }
}
impl W {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    #[must_use]
    pub fn mrep(&mut self) -> MREP_W<MREPrs> {
        MREP_W::new(self, 0)
    }
}
/**Master Timer Repetition Register

You can [`read`](crate::Reg::read) this register and get [`mrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_Master:MREP)*/
pub struct MREPrs;
impl crate::RegisterSpec for MREPrs {
    type Ux = u32;
}
///`read()` method returns [`mrep::R`](R) reader structure
impl crate::Readable for MREPrs {}
///`write(|w| ..)` method takes [`mrep::W`](W) writer structure
impl crate::Writable for MREPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MREP to value 0
impl crate::Resettable for MREPrs {
    const RESET_VALUE: u32 = 0;
}
