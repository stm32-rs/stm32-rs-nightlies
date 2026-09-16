///Register `IPID` reader
pub type R = crate::R<IPIDrs>;
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
        f.debug_struct("IPID").field("ipid", &self.ipid()).finish()
    }
}
/**UCPD IP ID register

You can [`read`](crate::Reg::read) this register and get [`ipid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:IPID)*/
pub struct IPIDrs;
impl crate::RegisterSpec for IPIDrs {
    type Ux = u32;
}
///`read()` method returns [`ipid::R`](R) reader structure
impl crate::Readable for IPIDrs {}
///`reset()` method sets IPID to value 0x0015_0021
impl crate::Resettable for IPIDrs {
    const RESET_VALUE: u32 = 0x0015_0021;
}
