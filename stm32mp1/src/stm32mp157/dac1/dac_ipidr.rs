///Register `DAC_IPIDR` reader
pub type R = crate::R<DAC_IPIDRrs>;
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
        f.debug_struct("DAC_IPIDR").field("id", &self.id()).finish()
    }
}
/**No

You can [`read`](crate::Reg::read) this register and get [`dac_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DAC1:DAC_IPIDR)*/
pub struct DAC_IPIDRrs;
impl crate::RegisterSpec for DAC_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_ipidr::R`](R) reader structure
impl crate::Readable for DAC_IPIDRrs {}
///`reset()` method sets DAC_IPIDR to value 0x0011_0011
impl crate::Resettable for DAC_IPIDRrs {
    const RESET_VALUE: u32 = 0x0011_0011;
}
