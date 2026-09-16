///Register `MACTXTSSSR` reader
pub type R = crate::R<MACTXTSSSRrs>;
///Field `TXTSSHI` reader - Transmit Timestamp Status High
pub type TXTSSHI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Transmit Timestamp Status High
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTXTSSSR")
            .field("txtsshi", &self.txtsshi())
            .finish()
    }
}
/**Tx timestamp status seconds register

You can [`read`](crate::Reg::read) this register and get [`mactxtsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACTXTSSSR)*/
pub struct MACTXTSSSRrs;
impl crate::RegisterSpec for MACTXTSSSRrs {
    type Ux = u32;
}
///`read()` method returns [`mactxtsssr::R`](R) reader structure
impl crate::Readable for MACTXTSSSRrs {}
///`reset()` method sets MACTXTSSSR to value 0
impl crate::Resettable for MACTXTSSSRrs {}
