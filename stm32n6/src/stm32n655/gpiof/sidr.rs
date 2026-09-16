///Register `SIDR` reader
pub type R = crate::R<SIDRrs>;
///Field `SID` reader - Size of the memory region allocated to GPIO registers
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Size of the memory region allocated to GPIO registers
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIDR").field("sid", &self.sid()).finish()
    }
}
/**GPIO port F size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#GPIOF:SIDR)*/
pub struct SIDRrs;
impl crate::RegisterSpec for SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`sidr::R`](R) reader structure
impl crate::Readable for SIDRrs {}
///`reset()` method sets SIDR to value 0xa3c5_dd01
impl crate::Resettable for SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
