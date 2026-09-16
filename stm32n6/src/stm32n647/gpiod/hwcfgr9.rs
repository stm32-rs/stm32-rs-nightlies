///Register `HWCFGR9` reader
pub type R = crate::R<HWCFGR9rs>;
///Field `EN_IO` reader - Presence granularity, each bit indicate the I/O presence
pub type EN_IO_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Presence granularity, each bit indicate the I/O presence
    #[inline(always)]
    pub fn en_io(&self) -> EN_IO_R {
        EN_IO_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR9")
            .field("en_io", &self.en_io())
            .finish()
    }
}
/**GPIO port D hardware configuration register 9

You can [`read`](crate::Reg::read) this register and get [`hwcfgr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPIOD:HWCFGR9)*/
pub struct HWCFGR9rs;
impl crate::RegisterSpec for HWCFGR9rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr9::R`](R) reader structure
impl crate::Readable for HWCFGR9rs {}
///`reset()` method sets HWCFGR9 to value 0xffff
impl crate::Resettable for HWCFGR9rs {
    const RESET_VALUE: u32 = 0xffff;
}
