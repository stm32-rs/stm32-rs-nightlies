///Register `SYSCFG_IPIDR` reader
pub type R = crate::R<SYSCFG_IPIDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**SYSCFG identification register

You can [`read`](crate::Reg::read) this register and get [`syscfg_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:SYSCFG_IPIDR)*/
pub struct SYSCFG_IPIDRrs;
impl crate::RegisterSpec for SYSCFG_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_ipidr::R`](R) reader structure
impl crate::Readable for SYSCFG_IPIDRrs {}
///`reset()` method sets SYSCFG_IPIDR to value 0x0003_0001
impl crate::Resettable for SYSCFG_IPIDRrs {
    const RESET_VALUE: u32 = 0x0003_0001;
}
