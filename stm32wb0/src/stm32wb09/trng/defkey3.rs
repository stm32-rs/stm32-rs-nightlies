///Register `DEFKEY3` reader
pub type R = crate::R<DEFKEY3rs>;
///Register `DEFKEY3` writer
pub type W = crate::W<DEFKEY3rs>;
///Field `RNG_DEFKEY3` reader - Bits 127 to 96 of AES 128-bit Default Key.
pub type RNG_DEFKEY3_R = crate::FieldReader<u32>;
///Field `RNG_DEFKEY3` writer - Bits 127 to 96 of AES 128-bit Default Key.
pub type RNG_DEFKEY3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bits 127 to 96 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey3(&self) -> RNG_DEFKEY3_R {
        RNG_DEFKEY3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEFKEY3")
            .field("rng_defkey3", &self.rng_defkey3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bits 127 to 96 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey3(&mut self) -> RNG_DEFKEY3_W<'_, DEFKEY3rs> {
        RNG_DEFKEY3_W::new(self, 0)
    }
}
/**TRNG_DEFKEY3 register

You can [`read`](crate::Reg::read) this register and get [`defkey3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY3)*/
pub struct DEFKEY3rs;
impl crate::RegisterSpec for DEFKEY3rs {
    type Ux = u32;
}
///`read()` method returns [`defkey3::R`](R) reader structure
impl crate::Readable for DEFKEY3rs {}
///`write(|w| ..)` method takes [`defkey3::W`](W) writer structure
impl crate::Writable for DEFKEY3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEFKEY3 to value 0xffff_ffff
impl crate::Resettable for DEFKEY3rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
