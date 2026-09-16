///Register `ID` reader
pub type R = crate::R<IDrs>;
///Field `IPID` reader - IPID
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - IPID
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID").field("ipid", &self.ipid()).finish()
    }
}
/**PWR IP identification register

You can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:ID)*/
pub struct IDrs;
impl crate::RegisterSpec for IDrs {
    type Ux = u32;
}
///`read()` method returns [`id::R`](R) reader structure
impl crate::Readable for IDrs {}
///`reset()` method sets ID to value 0x0001_0001
impl crate::Resettable for IDrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
