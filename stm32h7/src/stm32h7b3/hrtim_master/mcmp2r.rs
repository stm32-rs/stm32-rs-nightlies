///Register `MCMP2R` reader
pub type R = crate::R<MCMP2Rrs>;
///Register `MCMP2R` writer
pub type W = crate::W<MCMP2Rrs>;
///Field `MCMP2` reader - Master Timer Compare 2 value
pub type MCMP2_R = crate::FieldReader<u16>;
///Field `MCMP2` writer - Master Timer Compare 2 value
pub type MCMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Master Timer Compare 2 value
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMP2R")
            .field("mcmp2", &self.mcmp2())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 2 value
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> MCMP2_W<MCMP2Rrs> {
        MCMP2_W::new(self, 0)
    }
}
/**Master Timer Compare 2 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#HRTIM_Master:MCMP2R)*/
pub struct MCMP2Rrs;
impl crate::RegisterSpec for MCMP2Rrs {
    type Ux = u32;
}
///`read()` method returns [`mcmp2r::R`](R) reader structure
impl crate::Readable for MCMP2Rrs {}
///`write(|w| ..)` method takes [`mcmp2r::W`](W) writer structure
impl crate::Writable for MCMP2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MCMP2R to value 0
impl crate::Resettable for MCMP2Rrs {
    const RESET_VALUE: u32 = 0;
}
