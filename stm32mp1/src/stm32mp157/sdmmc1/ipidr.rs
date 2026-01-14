///Register `IPIDR` reader
pub type R = crate::R<IPIDRrs>;
///Field `IP_ID` reader - IP_ID
pub type IP_ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - IP_ID
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPIDR")
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
/**SDMMC identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:IPIDR)*/
pub struct IPIDRrs;
impl crate::RegisterSpec for IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`ipidr::R`](R) reader structure
impl crate::Readable for IPIDRrs {}
///`reset()` method sets IPIDR to value 0x0014_0022
impl crate::Resettable for IPIDRrs {
    const RESET_VALUE: u32 = 0x0014_0022;
}
