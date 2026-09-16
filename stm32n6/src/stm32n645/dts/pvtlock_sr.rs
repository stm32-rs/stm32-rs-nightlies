///Register `PVTLOCK_SR` reader
pub type R = crate::R<PVTLOCK_SRrs>;
///Field `SW_LOCK_STATUS` reader - Software lock input status
pub type SW_LOCK_STATUS_R = crate::BitReader;
///Field `HW_LOCK_STATUS` reader - Hardware lock input status
pub type HW_LOCK_STATUS_R = crate::BitReader;
impl R {
    ///Bit 0 - Software lock input status
    #[inline(always)]
    pub fn sw_lock_status(&self) -> SW_LOCK_STATUS_R {
        SW_LOCK_STATUS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Hardware lock input status
    #[inline(always)]
    pub fn hw_lock_status(&self) -> HW_LOCK_STATUS_R {
        HW_LOCK_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTLOCK_SR")
            .field("sw_lock_status", &self.sw_lock_status())
            .field("hw_lock_status", &self.hw_lock_status())
            .finish()
    }
}
/**DTS PVT lock status register

You can [`read`](crate::Reg::read) this register and get [`pvtlock_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:PVTLOCK_SR)*/
pub struct PVTLOCK_SRrs;
impl crate::RegisterSpec for PVTLOCK_SRrs {
    type Ux = u32;
}
///`read()` method returns [`pvtlock_sr::R`](R) reader structure
impl crate::Readable for PVTLOCK_SRrs {}
///`reset()` method sets PVTLOCK_SR to value 0
impl crate::Resettable for PVTLOCK_SRrs {}
