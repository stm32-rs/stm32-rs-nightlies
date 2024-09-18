///Register `PWR_ID` reader
pub type R = crate::R<PWR_IDrs>;
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
        f.debug_struct("PWR_ID")
            .field("ipid", &self.ipid())
            .finish()
    }
}
/**PWR IP identification register

You can [`read`](crate::Reg::read) this register and get [`pwr_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#PWR:PWR_ID)*/
pub struct PWR_IDrs;
impl crate::RegisterSpec for PWR_IDrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_id::R`](R) reader structure
impl crate::Readable for PWR_IDrs {}
///`reset()` method sets PWR_ID to value 0x0001_0001
impl crate::Resettable for PWR_IDrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
