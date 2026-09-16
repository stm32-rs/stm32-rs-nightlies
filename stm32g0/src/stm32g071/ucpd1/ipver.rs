///Register `IPVER` reader
pub type R = crate::R<IPVERrs>;
///Field `IPVER` reader - IPVER
pub type IPVER_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - IPVER
    #[inline(always)]
    pub fn ipver(&self) -> IPVER_R {
        IPVER_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPVER")
            .field("ipver", &self.ipver())
            .finish()
    }
}
/**UCPD IP ID register

You can [`read`](crate::Reg::read) this register and get [`ipver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#UCPD1:IPVER)*/
pub struct IPVERrs;
impl crate::RegisterSpec for IPVERrs {
    type Ux = u32;
}
///`read()` method returns [`ipver::R`](R) reader structure
impl crate::Readable for IPVERrs {}
///`reset()` method sets IPVER to value 0x10
impl crate::Resettable for IPVERrs {
    const RESET_VALUE: u32 = 0x10;
}
