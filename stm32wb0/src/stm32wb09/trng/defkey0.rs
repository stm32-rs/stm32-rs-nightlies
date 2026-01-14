///Register `DEFKEY0` reader
pub type R = crate::R<DEFKEY0rs>;
///Register `DEFKEY0` writer
pub type W = crate::W<DEFKEY0rs>;
///Field `RNG_DEFKEY0` reader - Bits 31 to 0 of AES 128-bit Default Key.
pub type RNG_DEFKEY0_R = crate::FieldReader<u32>;
///Field `RNG_DEFKEY0` writer - Bits 31 to 0 of AES 128-bit Default Key.
pub type RNG_DEFKEY0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bits 31 to 0 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey0(&self) -> RNG_DEFKEY0_R {
        RNG_DEFKEY0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEFKEY0")
            .field("rng_defkey0", &self.rng_defkey0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bits 31 to 0 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey0(&mut self) -> RNG_DEFKEY0_W<'_, DEFKEY0rs> {
        RNG_DEFKEY0_W::new(self, 0)
    }
}
/**TRNG_DEFKEY0 register

You can [`read`](crate::Reg::read) this register and get [`defkey0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY0)*/
pub struct DEFKEY0rs;
impl crate::RegisterSpec for DEFKEY0rs {
    type Ux = u32;
}
///`read()` method returns [`defkey0::R`](R) reader structure
impl crate::Readable for DEFKEY0rs {}
///`write(|w| ..)` method takes [`defkey0::W`](W) writer structure
impl crate::Writable for DEFKEY0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEFKEY0 to value 0xffff_ffff
impl crate::Resettable for DEFKEY0rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
