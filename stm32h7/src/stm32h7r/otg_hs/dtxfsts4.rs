///Register `DTXFSTS4` reader
pub type R = crate::R<DTXFSTS4rs>;
///Field `INEPTFSAV` reader - IN endpoint Tx FIFO space available Indicates the amount of free space available in the endpoint Tx FIFO. Values are in terms of 32-bit words: 0xn: n words available Others: Reserved
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IN endpoint Tx FIFO space available Indicates the amount of free space available in the endpoint Tx FIFO. Values are in terms of 32-bit words: 0xn: n words available Others: Reserved
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS4")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**OTG device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#OTG_HS:DTXFSTS4)*/
pub struct DTXFSTS4rs;
impl crate::RegisterSpec for DTXFSTS4rs {
    type Ux = u32;
}
///`read()` method returns [`dtxfsts4::R`](R) reader structure
impl crate::Readable for DTXFSTS4rs {}
///`reset()` method sets DTXFSTS4 to value 0x0200
impl crate::Resettable for DTXFSTS4rs {
    const RESET_VALUE: u32 = 0x0200;
}
