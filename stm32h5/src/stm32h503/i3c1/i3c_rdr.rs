///Register `I3C_RDR` reader
pub type R = crate::R<I3C_RDRrs>;
///Field `RDB0` reader - 8-bit received data on I3C bus.
pub type RDB0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - 8-bit received data on I3C bus.
    #[inline(always)]
    pub fn rdb0(&self) -> RDB0_R {
        RDB0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_RDR")
            .field("rdb0", &self.rdb0())
            .finish()
    }
}
/**I3C receive data byte register

You can [`read`](crate::Reg::read) this register and get [`i3c_rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_RDR)*/
pub struct I3C_RDRrs;
impl crate::RegisterSpec for I3C_RDRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_rdr::R`](R) reader structure
impl crate::Readable for I3C_RDRrs {}
///`reset()` method sets I3C_RDR to value 0
impl crate::Resettable for I3C_RDRrs {
    const RESET_VALUE: u32 = 0;
}
