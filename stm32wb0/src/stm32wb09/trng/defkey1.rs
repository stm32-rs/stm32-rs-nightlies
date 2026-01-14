///Register `DEFKEY1` reader
pub type R = crate::R<DEFKEY1rs>;
///Register `DEFKEY1` writer
pub type W = crate::W<DEFKEY1rs>;
///Field `RNG_DEFKEY1` reader - Bits 63 to 31 of AES 128-bit Default Key.
pub type RNG_DEFKEY1_R = crate::FieldReader<u32>;
///Field `RNG_DEFKEY1` writer - Bits 63 to 31 of AES 128-bit Default Key.
pub type RNG_DEFKEY1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Bits 63 to 31 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey1(&self) -> RNG_DEFKEY1_R {
        RNG_DEFKEY1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEFKEY1")
            .field("rng_defkey1", &self.rng_defkey1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Bits 63 to 31 of AES 128-bit Default Key.
    #[inline(always)]
    pub fn rng_defkey1(&mut self) -> RNG_DEFKEY1_W<'_, DEFKEY1rs> {
        RNG_DEFKEY1_W::new(self, 0)
    }
}
/**TRNG_DEFKEY1 register

You can [`read`](crate::Reg::read) this register and get [`defkey1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`defkey1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:DEFKEY1)*/
pub struct DEFKEY1rs;
impl crate::RegisterSpec for DEFKEY1rs {
    type Ux = u32;
}
///`read()` method returns [`defkey1::R`](R) reader structure
impl crate::Readable for DEFKEY1rs {}
///`write(|w| ..)` method takes [`defkey1::W`](W) writer structure
impl crate::Writable for DEFKEY1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEFKEY1 to value 0xffff_ffff
impl crate::Resettable for DEFKEY1rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
