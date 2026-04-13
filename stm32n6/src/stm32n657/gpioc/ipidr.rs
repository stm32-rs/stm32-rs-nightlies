///Register `IPIDR` reader
pub type R = crate::R<IPIDRrs>;
///Field `IPID` reader - GPIO identifier
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - GPIO identifier
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPIDR").field("ipid", &self.ipid()).finish()
    }
}
/**GPIO port C identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GPIOC:IPIDR)*/
pub struct IPIDRrs;
impl crate::RegisterSpec for IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`ipidr::R`](R) reader structure
impl crate::Readable for IPIDRrs {}
///`reset()` method sets IPIDR to value 0x000f_0004
impl crate::Resettable for IPIDRrs {
    const RESET_VALUE: u32 = 0x000f_0004;
}
