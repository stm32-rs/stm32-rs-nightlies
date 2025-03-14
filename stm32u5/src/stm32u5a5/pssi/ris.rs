///Register `RIS` reader
pub type R = crate::R<RISrs>;
///Field `OVR_RIS` reader - OVR_RIS
pub type OVR_RIS_R = crate::BitReader;
impl R {
    ///Bit 1 - OVR_RIS
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("ovr_ris", &self.ovr_ris())
            .finish()
    }
}
/**PSSI raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#PSSI:RIS)*/
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RISrs {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RISrs {}
