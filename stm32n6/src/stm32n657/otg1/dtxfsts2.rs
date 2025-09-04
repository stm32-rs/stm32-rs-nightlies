///Register `DTXFSTS2` reader
pub type R = crate::R<DTXFSTS2rs>;
///Field `INEPTFSAV` reader - IN endpoint Tx FIFO space available
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IN endpoint Tx FIFO space available
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS2")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**OTG device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#OTG1:DTXFSTS2)*/
pub struct DTXFSTS2rs;
impl crate::RegisterSpec for DTXFSTS2rs {
    type Ux = u32;
}
///`read()` method returns [`dtxfsts2::R`](R) reader structure
impl crate::Readable for DTXFSTS2rs {}
///`reset()` method sets DTXFSTS2 to value 0x0200
impl crate::Resettable for DTXFSTS2rs {
    const RESET_VALUE: u32 = 0x0200;
}
