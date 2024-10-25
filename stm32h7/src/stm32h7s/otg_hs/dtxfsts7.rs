///Register `DTXFSTS7` reader
pub type R = crate::R<DTXFSTS7rs>;
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
        f.debug_struct("DTXFSTS7")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**OTG device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#OTG_HS:DTXFSTS7)*/
pub struct DTXFSTS7rs;
impl crate::RegisterSpec for DTXFSTS7rs {
    type Ux = u32;
}
///`read()` method returns [`dtxfsts7::R`](R) reader structure
impl crate::Readable for DTXFSTS7rs {}
///`reset()` method sets DTXFSTS7 to value 0x0200
impl crate::Resettable for DTXFSTS7rs {
    const RESET_VALUE: u32 = 0x0200;
}
