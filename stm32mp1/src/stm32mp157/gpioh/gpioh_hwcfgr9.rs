///Register `GPIOH_HWCFGR9` reader
pub type R = crate::R<GPIOH_HWCFGR9rs>;
///Field `EN_IO` reader - EN_IO
pub type EN_IO_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - EN_IO
    #[inline(always)]
    pub fn en_io(&self) -> EN_IO_R {
        EN_IO_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOH_HWCFGR9")
            .field("en_io", &self.en_io())
            .finish()
    }
}
/**For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_HWCFGR9)*/
pub struct GPIOH_HWCFGR9rs;
impl crate::RegisterSpec for GPIOH_HWCFGR9rs {
    type Ux = u32;
}
///`read()` method returns [`gpioh_hwcfgr9::R`](R) reader structure
impl crate::Readable for GPIOH_HWCFGR9rs {}
///`reset()` method sets GPIOH_HWCFGR9 to value 0xff
impl crate::Resettable for GPIOH_HWCFGR9rs {
    const RESET_VALUE: u32 = 0xff;
}
