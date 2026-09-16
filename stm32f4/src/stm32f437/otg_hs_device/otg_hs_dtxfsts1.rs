///Register `OTG_HS_DTXFSTS1` reader
pub type R = crate::R<OTG_HS_DTXFSTS1rs>;
///Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - IN endpoint TxFIFO space avail
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DTXFSTS1")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**OTG_HS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_DEVICE:OTG_HS_DTXFSTS1)*/
pub struct OTG_HS_DTXFSTS1rs;
impl crate::RegisterSpec for OTG_HS_DTXFSTS1rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_dtxfsts1::R`](R) reader structure
impl crate::Readable for OTG_HS_DTXFSTS1rs {}
///`reset()` method sets OTG_HS_DTXFSTS1 to value 0
impl crate::Resettable for OTG_HS_DTXFSTS1rs {}
