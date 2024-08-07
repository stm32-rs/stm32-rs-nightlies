///Register `MCMP1R` reader
pub type R = crate::R<MCMP1Rrs>;
///Register `MCMP1R` writer
pub type W = crate::W<MCMP1Rrs>;
///Field `MCMP1` reader - Master Timer Compare 1 value
pub type MCMP1_R = crate::FieldReader<u16>;
///Field `MCMP1` writer - Master Timer Compare 1 value
pub type MCMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Master Timer Compare 1 value
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCMP1R")
            .field("mcmp1", &self.mcmp1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 1 value
    #[inline(always)]
    #[must_use]
    pub fn mcmp1(&mut self) -> MCMP1_W<MCMP1Rrs> {
        MCMP1_W::new(self, 0)
    }
}
/**Master Timer Compare 1 Register

You can [`read`](crate::Reg::read) this register and get [`mcmp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_Master:MCMP1R)*/
pub struct MCMP1Rrs;
impl crate::RegisterSpec for MCMP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`mcmp1r::R`](R) reader structure
impl crate::Readable for MCMP1Rrs {}
///`write(|w| ..)` method takes [`mcmp1r::W`](W) writer structure
impl crate::Writable for MCMP1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MCMP1R to value 0
impl crate::Resettable for MCMP1Rrs {
    const RESET_VALUE: u32 = 0;
}
