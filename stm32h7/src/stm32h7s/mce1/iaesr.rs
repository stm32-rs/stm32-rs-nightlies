///Register `IAESR` reader
pub type R = crate::R<IAESRrs>;
///Field `IAPRIV` reader - Illegal access privilege When IAEF bit is set in MCE_IASR register IAPRIV bit captures the privileged state of the master that issued the illegal access detected on the AXI system bus.
pub type IAPRIV_R = crate::BitReader;
///Field `IANRW` reader - Illegal access read/write When IAEF bit is set in MCE_IASR register IANRW bit captures the access type of the illegal access detected.
pub type IANRW_R = crate::BitReader;
impl R {
    ///Bit 4 - Illegal access privilege When IAEF bit is set in MCE_IASR register IAPRIV bit captures the privileged state of the master that issued the illegal access detected on the AXI system bus.
    #[inline(always)]
    pub fn iapriv(&self) -> IAPRIV_R {
        IAPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Illegal access read/write When IAEF bit is set in MCE_IASR register IANRW bit captures the access type of the illegal access detected.
    #[inline(always)]
    pub fn ianrw(&self) -> IANRW_R {
        IANRW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IAESR")
            .field("iapriv", &self.iapriv())
            .field("ianrw", &self.ianrw())
            .finish()
    }
}
/**MCE illegal access error status register

You can [`read`](crate::Reg::read) this register and get [`iaesr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#MCE1:IAESR)*/
pub struct IAESRrs;
impl crate::RegisterSpec for IAESRrs {
    type Ux = u32;
}
///`read()` method returns [`iaesr::R`](R) reader structure
impl crate::Readable for IAESRrs {}
///`reset()` method sets IAESR to value 0
impl crate::Resettable for IAESRrs {}
