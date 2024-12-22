///Register `RNG_HTCR` reader
pub type R = crate::R<RNG_HTCRrs>;
///Register `RNG_HTCR` writer
pub type W = crate::W<RNG_HTCRrs>;
///Field `HTCFG` reader - health test configuration This configuration is used by RNG to configure the health tests. See Section120.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written.
pub type HTCFG_R = crate::FieldReader<u32>;
///Field `HTCFG` writer - health test configuration This configuration is used by RNG to configure the health tests. See Section120.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written.
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section120.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written.
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_HTCR")
            .field("htcfg", &self.htcfg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - health test configuration This configuration is used by RNG to configure the health tests. See Section120.6: RNG entropy source validation for the recommended value. Note: The RNG behavior, including the read to this register, is not guaranteed if a different value from the recommended value is written.
    #[inline(always)]
    pub fn htcfg(&mut self) -> HTCFG_W<RNG_HTCRrs> {
        HTCFG_W::new(self, 0)
    }
}
/**RNG health test control register

You can [`read`](crate::Reg::read) this register and get [`rng_htcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_htcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RNG:RNG_HTCR)*/
pub struct RNG_HTCRrs;
impl crate::RegisterSpec for RNG_HTCRrs {
    type Ux = u32;
}
///`read()` method returns [`rng_htcr::R`](R) reader structure
impl crate::Readable for RNG_HTCRrs {}
///`write(|w| ..)` method takes [`rng_htcr::W`](W) writer structure
impl crate::Writable for RNG_HTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RNG_HTCR to value 0x72ac
impl crate::Resettable for RNG_HTCRrs {
    const RESET_VALUE: u32 = 0x72ac;
}
