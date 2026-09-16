///Register `VERSION_ID` reader
pub type R = crate::R<VERSION_IDrs>;
///Field `VERSION_ID` reader - version of the embedded IP.
pub type VERSION_ID_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - version of the embedded IP.
    #[inline(always)]
    pub fn version_id(&self) -> VERSION_ID_R {
        VERSION_ID_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION_ID")
            .field("version_id", &self.version_id())
            .finish()
    }
}
/**VERSION_ID register

You can [`read`](crate::Reg::read) this register and get [`version_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#ADC:VERSION_ID)*/
pub struct VERSION_IDrs;
impl crate::RegisterSpec for VERSION_IDrs {
    type Ux = u32;
}
///`read()` method returns [`version_id::R`](R) reader structure
impl crate::Readable for VERSION_IDrs {}
///`reset()` method sets VERSION_ID to value 0x20
impl crate::Resettable for VERSION_IDrs {
    const RESET_VALUE: u32 = 0x20;
}
