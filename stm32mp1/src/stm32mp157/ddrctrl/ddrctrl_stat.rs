///Register `DDRCTRL_STAT` reader
pub type R = crate::R<DDRCTRL_STATrs>;
///Field `OPERATING_MODE` reader - OPERATING_MODE
pub type OPERATING_MODE_R = crate::FieldReader;
///Field `SELFREF_TYPE` reader - SELFREF_TYPE
pub type SELFREF_TYPE_R = crate::FieldReader;
///Field `SELFREF_CAM_NOT_EMPTY` reader - SELFREF_CAM_NOT_EMPTY
pub type SELFREF_CAM_NOT_EMPTY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - OPERATING_MODE
    #[inline(always)]
    pub fn operating_mode(&self) -> OPERATING_MODE_R {
        OPERATING_MODE_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:5 - SELFREF_TYPE
    #[inline(always)]
    pub fn selfref_type(&self) -> SELFREF_TYPE_R {
        SELFREF_TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 12 - SELFREF_CAM_NOT_EMPTY
    #[inline(always)]
    pub fn selfref_cam_not_empty(&self) -> SELFREF_CAM_NOT_EMPTY_R {
        SELFREF_CAM_NOT_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_STAT")
            .field("operating_mode", &self.operating_mode())
            .field("selfref_type", &self.selfref_type())
            .field("selfref_cam_not_empty", &self.selfref_cam_not_empty())
            .finish()
    }
}
/**DDRCTRL operating mode status register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_STAT)*/
pub struct DDRCTRL_STATrs;
impl crate::RegisterSpec for DDRCTRL_STATrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_stat::R`](R) reader structure
impl crate::Readable for DDRCTRL_STATrs {}
///`reset()` method sets DDRCTRL_STAT to value 0
impl crate::Resettable for DDRCTRL_STATrs {
    const RESET_VALUE: u32 = 0;
}
