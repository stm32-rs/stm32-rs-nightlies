///Register `TAMP_ATSEEDR` writer
pub type W = crate::W<TAMP_ATSEEDRrs>;
///Field `SEED` writer - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG.
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<TAMP_ATSEEDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG.
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W<TAMP_ATSEEDRrs> {
        SEED_W::new(self, 0)
    }
}
/**TAMP active tamper seed register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_atseedr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#TAMP:TAMP_ATSEEDR)*/
pub struct TAMP_ATSEEDRrs;
impl crate::RegisterSpec for TAMP_ATSEEDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tamp_atseedr::W`](W) writer structure
impl crate::Writable for TAMP_ATSEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAMP_ATSEEDR to value 0
impl crate::Resettable for TAMP_ATSEEDRrs {
    const RESET_VALUE: u32 = 0;
}
