///Register `MCMP3R` reader
pub type R = crate::R<MCMP3Rrs>;
///Register `MCMP3R` writer
pub type W = crate::W<MCMP3Rrs>;
///Field `MCMP3` reader - Master Timer Compare 3 value
pub type MCMP3_R = crate::FieldReader<u16>;
///Field `MCMP3` writer - Master Timer Compare 3 value
pub type MCMP3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Master Timer Compare 3 value
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMP3R")
            .field("mcmp3", &self.mcmp3())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 3 value
    #[inline(always)]
    #[must_use]
    pub fn mcmp3(&mut self) -> MCMP3_W<MCMP3Rrs> {
        MCMP3_W::new(self, 0)
    }
}
/**Master Timer Compare 3 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Master:MCMP3R)*/
pub struct MCMP3Rrs;
impl crate::RegisterSpec for MCMP3Rrs {
    type Ux = u32;
}
///`read()` method returns [`mcmp3r::R`](R) reader structure
impl crate::Readable for MCMP3Rrs {}
///`write(|w| ..)` method takes [`mcmp3r::W`](W) writer structure
impl crate::Writable for MCMP3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MCMP3R to value 0
impl crate::Resettable for MCMP3Rrs {
    const RESET_VALUE: u32 = 0;
}
