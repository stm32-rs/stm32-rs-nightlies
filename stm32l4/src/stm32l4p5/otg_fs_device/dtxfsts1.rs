///Register `DTXFSTS1` reader
pub type R = crate::R<DTXFSTS1rs>;
///Field `INEPTFSAV` reader - IN endpoint TxFIFO space available
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IN endpoint TxFIFO space available
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS1")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**OTG_FS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`dtxfsts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DTXFSTS1)*/
pub struct DTXFSTS1rs;
impl crate::RegisterSpec for DTXFSTS1rs {
    type Ux = u32;
}
///`read()` method returns [`dtxfsts1::R`](R) reader structure
impl crate::Readable for DTXFSTS1rs {}
///`reset()` method sets DTXFSTS1 to value 0
impl crate::Resettable for DTXFSTS1rs {}
