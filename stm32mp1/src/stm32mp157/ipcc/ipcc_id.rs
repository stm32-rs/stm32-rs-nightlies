///Register `IPCC_ID` reader
pub type R = crate::R<IPCC_IDrs>;
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
        f.debug_struct("IPCC_ID")
            .field("ipid", &self.ipid())
            .finish()
    }
}
/**IPCC IP Identification register

You can [`read`](crate::Reg::read) this register and get [`ipcc_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_ID)*/
pub struct IPCC_IDrs;
impl crate::RegisterSpec for IPCC_IDrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_id::R`](R) reader structure
impl crate::Readable for IPCC_IDrs {}
///`reset()` method sets IPCC_ID to value 0x0010_0071
impl crate::Resettable for IPCC_IDrs {
    const RESET_VALUE: u32 = 0x0010_0071;
}
