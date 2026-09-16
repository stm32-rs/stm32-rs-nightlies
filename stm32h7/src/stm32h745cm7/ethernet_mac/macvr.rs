///Register `MACVR` reader
pub type R = crate::R<MACVRrs>;
///Field `SNPSVER` reader - IP version
pub type SNPSVER_R = crate::FieldReader;
///Field `USERVER` reader - ST-defined version
pub type USERVER_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - IP version
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ST-defined version
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVR")
            .field("snpsver", &self.snpsver())
            .field("userver", &self.userver())
            .finish()
    }
}
/**Version register

You can [`read`](crate::Reg::read) this register and get [`macvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#Ethernet_MAC:MACVR)*/
pub struct MACVRrs;
impl crate::RegisterSpec for MACVRrs {
    type Ux = u32;
}
///`read()` method returns [`macvr::R`](R) reader structure
impl crate::Readable for MACVRrs {}
///`reset()` method sets MACVR to value 0x3041
impl crate::Resettable for MACVRrs {
    const RESET_VALUE: u32 = 0x3041;
}
