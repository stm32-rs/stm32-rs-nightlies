///Register `DMACHTBAR` reader
pub type R = crate::R<DMACHTBARrs>;
///Field `HTBAP` reader - Host transmit buffer address pointer
pub type HTBAP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Host transmit buffer address pointer
    #[inline(always)]
    pub fn htbap(&self) -> HTBAP_R {
        HTBAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACHTBAR")
            .field("htbap", &self.htbap())
            .finish()
    }
}
/**Ethernet DMA current host transmit buffer address register

You can [`read`](crate::Reg::read) this register and get [`dmachtbar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#Ethernet_DMA:DMACHTBAR)*/
pub struct DMACHTBARrs;
impl crate::RegisterSpec for DMACHTBARrs {
    type Ux = u32;
}
///`read()` method returns [`dmachtbar::R`](R) reader structure
impl crate::Readable for DMACHTBARrs {}
///`reset()` method sets DMACHTBAR to value 0
impl crate::Resettable for DMACHTBARrs {}
