#[doc = "Register `TAMP_ATSEEDR` writer"]
pub type W = crate::W<TAMP_ATSEEDRrs>;
#[doc = "Field `SEED` writer - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
pub type SEED_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Pseudo-random generator seed value This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
    #[inline(always)]
    #[must_use]
    pub fn seed(&mut self) -> SEED_W<TAMP_ATSEEDRrs> {
        SEED_W::new(self, 0)
    }
}
#[doc = "TAMP active tamper seed register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_atseedr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_ATSEEDRrs;
impl crate::RegisterSpec for TAMP_ATSEEDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tamp_atseedr::W`](W) writer structure"]
impl crate::Writable for TAMP_ATSEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP_ATSEEDR to value 0"]
impl crate::Resettable for TAMP_ATSEEDRrs {
    const RESET_VALUE: u32 = 0;
}
