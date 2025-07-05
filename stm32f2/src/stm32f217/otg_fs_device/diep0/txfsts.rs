///Register `TXFSTS` reader
pub type R = crate::R<TXFSTSrs>;
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
        f.debug_struct("TXFSTS")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**OTG_FS device IN endpoint transmit FIFO status register

You can [`read`](crate::Reg::read) this register and get [`txfsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXFSTSrs;
impl crate::RegisterSpec for TXFSTSrs {
    type Ux = u32;
}
///`read()` method returns [`txfsts::R`](R) reader structure
impl crate::Readable for TXFSTSrs {}
///`reset()` method sets TXFSTS to value 0
impl crate::Resettable for TXFSTSrs {}
