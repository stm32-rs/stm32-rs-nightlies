///Register `GPIOZ_IPIDR` reader
pub type R = crate::R<GPIOZ_IPIDRrs>;
///Field `IPIDR` reader - IPIDR
pub type IPIDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - IPIDR
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOZ_IPIDR")
            .field("ipidr", &self.ipidr())
            .finish()
    }
}
/**GPIO identification register

You can [`read`](crate::Reg::read) this register and get [`gpioz_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOZ:GPIOZ_IPIDR)*/
pub struct GPIOZ_IPIDRrs;
impl crate::RegisterSpec for GPIOZ_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioz_ipidr::R`](R) reader structure
impl crate::Readable for GPIOZ_IPIDRrs {}
///`reset()` method sets GPIOZ_IPIDR to value 0x000f_0002
impl crate::Resettable for GPIOZ_IPIDRrs {
    const RESET_VALUE: u32 = 0x000f_0002;
}
