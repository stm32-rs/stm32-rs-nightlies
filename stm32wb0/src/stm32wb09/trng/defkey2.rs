///Register `DEFKEY2` reader
pub type R = crate::R<DEFKEY2rs>;
///Register `DEFKEY2` writer
pub type W = crate::W<DEFKEY2rs>;
///Field `RNG_DEFKEY2` reader - Bits 95 to 64 of AES 128-bit Default Key.
pub type RNG_DEFKEY2_R = crate::FieldReader<u32>;
///Field `RNG_DEFKEY2` writer - Bits 95 to 64 of AES 128-bit Default Key.
pub type RNG_DEFKEY2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bits 95 to 64 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey2(&self) -> RNG_DEFKEY2_R {
        RNG_DEFKEY2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEFKEY2")
            .field("rng_defkey2", &self.rng_defkey2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bits 95 to 64 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey2(&mut self) -> RNG_DEFKEY2_W<'_, DEFKEY2rs> {
        RNG_DEFKEY2_W::new(self, 0)
    }
}
/**TRNG_DEFKEY2 register

You can [`read`](crate::Reg::read) this register and get [`defkey2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY2)*/
pub struct DEFKEY2rs;
impl crate::RegisterSpec for DEFKEY2rs {
    type Ux = u32;
}
///`read()` method returns [`defkey2::R`](R) reader structure
impl crate::Readable for DEFKEY2rs {}
///`write(|w| ..)` method takes [`defkey2::W`](W) writer structure
impl crate::Writable for DEFKEY2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEFKEY2 to value 0xffff_ffff
impl crate::Resettable for DEFKEY2rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
