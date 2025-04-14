///Register `DFISTAT` reader
pub type R = crate::R<DFISTATrs>;
///Field `DFI_INIT_COMPLETE` reader - DFI_INIT_COMPLETE
pub type DFI_INIT_COMPLETE_R = crate::BitReader;
///Field `DFI_LP_ACK` reader - DFI_LP_ACK
pub type DFI_LP_ACK_R = crate::BitReader;
impl R {
    ///Bit 0 - DFI_INIT_COMPLETE
    #[inline(always)]
    pub fn dfi_init_complete(&self) -> DFI_INIT_COMPLETE_R {
        DFI_INIT_COMPLETE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DFI_LP_ACK
    #[inline(always)]
    pub fn dfi_lp_ack(&self) -> DFI_LP_ACK_R {
        DFI_LP_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFISTAT")
            .field("dfi_init_complete", &self.dfi_init_complete())
            .field("dfi_lp_ack", &self.dfi_lp_ack())
            .finish()
    }
}
/**DDRCTRL DFI status register

You can [`read`](crate::Reg::read) this register and get [`dfistat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DFISTAT)*/
pub struct DFISTATrs;
impl crate::RegisterSpec for DFISTATrs {
    type Ux = u32;
}
///`read()` method returns [`dfistat::R`](R) reader structure
impl crate::Readable for DFISTATrs {}
///`reset()` method sets DFISTAT to value 0
impl crate::Resettable for DFISTATrs {}
