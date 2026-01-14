///Register `OBW1SR` reader
pub type R = crate::R<OBW1SRrs>;
///Field `BOR_LEV` reader - Brownout level These bits reflects the power level that generates a system reset.
pub type BOR_LEV_R = crate::FieldReader;
///Field `IWDG_HW` reader - Independent watchdog HW Control
pub type IWDG_HW_R = crate::BitReader;
///Field `NRST_STOP` reader - Reset on stop mode
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STBY` reader - Reset on standby mode
pub type NRST_STBY_R = crate::BitReader;
///Field `OCTO1_HSLV` reader - XSPIM_P1 High-Speed at Low-Voltage
pub type OCTO1_HSLV_R = crate::BitReader;
///Field `OCTO2_HSLV` reader - XSPIM_P2 High-Speed at Low-Voltage
pub type OCTO2_HSLV_R = crate::BitReader;
///Field `IWDG_FZ_STOP` reader - IWDG stop mode freeze When set the independent watchdog IWDG is frozen in system Stop mode.
pub type IWDG_FZ_STOP_R = crate::BitReader;
///Field `IWDG_FZ_SDBY` reader - IWDG standby mode freeze When set the independent watchdog IWDG is frozen in system Standby mode.
pub type IWDG_FZ_SDBY_R = crate::BitReader;
///Field `PERSO_OK` reader - Personalization OK This bit is set on STMicroelectronics production line.
pub type PERSO_OK_R = crate::BitReader;
///Field `VDDIO_HSLV` reader - I/O High-Speed at Low-Voltage This bit indicates that the product operates below 2.5 V.
pub type VDDIO_HSLV_R = crate::BitReader;
impl R {
    ///Bits 2:3 - Brownout level These bits reflects the power level that generates a system reset.
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Independent watchdog HW Control
    #[inline(always)]
    pub fn iwdg_hw(&self) -> IWDG_HW_R {
        IWDG_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Reset on stop mode
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Reset on standby mode
    #[inline(always)]
    pub fn nrst_stby(&self) -> NRST_STBY_R {
        NRST_STBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - XSPIM_P1 High-Speed at Low-Voltage
    #[inline(always)]
    pub fn octo1_hslv(&self) -> OCTO1_HSLV_R {
        OCTO1_HSLV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - XSPIM_P2 High-Speed at Low-Voltage
    #[inline(always)]
    pub fn octo2_hslv(&self) -> OCTO2_HSLV_R {
        OCTO2_HSLV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - IWDG stop mode freeze When set the independent watchdog IWDG is frozen in system Stop mode.
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG standby mode freeze When set the independent watchdog IWDG is frozen in system Standby mode.
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 28 - Personalization OK This bit is set on STMicroelectronics production line.
    #[inline(always)]
    pub fn perso_ok(&self) -> PERSO_OK_R {
        PERSO_OK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - I/O High-Speed at Low-Voltage This bit indicates that the product operates below 2.5 V.
    #[inline(always)]
    pub fn vddio_hslv(&self) -> VDDIO_HSLV_R {
        VDDIO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBW1SR")
            .field("bor_lev", &self.bor_lev())
            .field("iwdg_hw", &self.iwdg_hw())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stby", &self.nrst_stby())
            .field("octo1_hslv", &self.octo1_hslv())
            .field("octo2_hslv", &self.octo2_hslv())
            .field("iwdg_fz_stop", &self.iwdg_fz_stop())
            .field("iwdg_fz_sdby", &self.iwdg_fz_sdby())
            .field("perso_ok", &self.perso_ok())
            .field("vddio_hslv", &self.vddio_hslv())
            .finish()
    }
}
/**FLASH option byte word 1 status register

You can [`read`](crate::Reg::read) this register and get [`obw1sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OBW1SR)*/
pub struct OBW1SRrs;
impl crate::RegisterSpec for OBW1SRrs {
    type Ux = u32;
}
///`read()` method returns [`obw1sr::R`](R) reader structure
impl crate::Readable for OBW1SRrs {}
///`reset()` method sets OBW1SR to value 0
impl crate::Resettable for OBW1SRrs {}
