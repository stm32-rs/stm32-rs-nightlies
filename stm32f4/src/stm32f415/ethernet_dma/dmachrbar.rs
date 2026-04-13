///Register `DMACHRBAR` reader
pub type R = crate::R<DMACHRBARrs>;
///Field `HRBAP` reader - HRBAP
pub type HRBAP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HRBAP
    #[inline(always)]
    pub fn hrbap(&self) -> HRBAP_R {
        HRBAP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACHRBAR")
            .field("hrbap", &self.hrbap())
            .finish()
    }
}
/**Ethernet DMA current host receive buffer address register

You can [`read`](crate::Reg::read) this register and get [`dmachrbar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#Ethernet_DMA:DMACHRBAR)*/
pub struct DMACHRBARrs;
impl crate::RegisterSpec for DMACHRBARrs {
    type Ux = u32;
}
///`read()` method returns [`dmachrbar::R`](R) reader structure
impl crate::Readable for DMACHRBARrs {}
///`reset()` method sets DMACHRBAR to value 0
impl crate::Resettable for DMACHRBARrs {}
