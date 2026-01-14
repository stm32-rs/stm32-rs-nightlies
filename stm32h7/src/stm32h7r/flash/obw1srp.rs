///Register `OBW1SRP` reader
pub type R = crate::R<OBW1SRPrs>;
///Register `OBW1SRP` writer
pub type W = crate::W<OBW1SRPrs>;
///Field `BOR_LEV` reader - Brownout level Write to change corresponding bits in FLASH_OBW1SR register.
pub type BOR_LEV_R = crate::FieldReader;
///Field `BOR_LEV` writer - Brownout level Write to change corresponding bits in FLASH_OBW1SR register.
pub type BOR_LEV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IWDG_HW` reader - Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register.
pub type IWDG_HW_R = crate::BitReader;
///Field `IWDG_HW` writer - Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register.
pub type IWDG_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STOP` reader - Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type NRST_STOP_R = crate::BitReader;
///Field `NRST_STOP` writer - Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type NRST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NRST_STBY` reader - Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type NRST_STBY_R = crate::BitReader;
///Field `NRST_STBY` writer - Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type NRST_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO1_HSLV` reader - XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register.
pub type OCTO1_HSLV_R = crate::BitReader;
///Field `OCTO1_HSLV` writer - XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register.
pub type OCTO1_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTO2_HSLV` reader - XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type OCTO2_HSLV_R = crate::BitReader;
///Field `OCTO2_HSLV` writer - XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type OCTO2_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_FZ_STOP` reader - IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register.
pub type IWDG_FZ_STOP_R = crate::BitReader;
///Field `IWDG_FZ_STOP` writer - IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register.
pub type IWDG_FZ_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IWDG_FZ_SDBY` reader - IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type IWDG_FZ_SDBY_R = crate::BitReader;
///Field `IWDG_FZ_SDBY` writer - IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type IWDG_FZ_SDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VDDIO_HSLV` reader - I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type VDDIO_HSLV_R = crate::BitReader;
///Field `VDDIO_HSLV` writer - I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
pub type VDDIO_HSLV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 2:3 - Brownout level Write to change corresponding bits in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn iwdg_hw(&self) -> IWDG_HW_R {
        IWDG_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn nrst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn nrst_stby(&self) -> NRST_STBY_R {
        NRST_STBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn octo1_hslv(&self) -> OCTO1_HSLV_R {
        OCTO1_HSLV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn octo2_hslv(&self) -> OCTO2_HSLV_R {
        OCTO2_HSLV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn iwdg_fz_stop(&self) -> IWDG_FZ_STOP_R {
        IWDG_FZ_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn iwdg_fz_sdby(&self) -> IWDG_FZ_SDBY_R {
        IWDG_FZ_SDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 29 - I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn vddio_hslv(&self) -> VDDIO_HSLV_R {
        VDDIO_HSLV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBW1SRP")
            .field("bor_lev", &self.bor_lev())
            .field("iwdg_hw", &self.iwdg_hw())
            .field("nrst_stop", &self.nrst_stop())
            .field("nrst_stby", &self.nrst_stby())
            .field("octo1_hslv", &self.octo1_hslv())
            .field("octo2_hslv", &self.octo2_hslv())
            .field("iwdg_fz_stop", &self.iwdg_fz_stop())
            .field("iwdg_fz_sdby", &self.iwdg_fz_sdby())
            .field("vddio_hslv", &self.vddio_hslv())
            .finish()
    }
}
impl W {
    ///Bits 2:3 - Brownout level Write to change corresponding bits in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<'_, OBW1SRPrs> {
        BOR_LEV_W::new(self, 2)
    }
    ///Bit 4 - Independent watchdog HW Control Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn iwdg_hw(&mut self) -> IWDG_HW_W<'_, OBW1SRPrs> {
        IWDG_HW_W::new(self, 4)
    }
    ///Bit 6 - Reset on stop mode programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn nrst_stop(&mut self) -> NRST_STOP_W<'_, OBW1SRPrs> {
        NRST_STOP_W::new(self, 6)
    }
    ///Bit 7 - Reset on standby mode programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn nrst_stby(&mut self) -> NRST_STBY_W<'_, OBW1SRPrs> {
        NRST_STBY_W::new(self, 7)
    }
    ///Bit 8 - XSPIM_P1 High-Speed at Low-Voltage Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn octo1_hslv(&mut self) -> OCTO1_HSLV_W<'_, OBW1SRPrs> {
        OCTO1_HSLV_W::new(self, 8)
    }
    ///Bit 9 - XSPIM_P2 High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn octo2_hslv(&mut self) -> OCTO2_HSLV_W<'_, OBW1SRPrs> {
        OCTO2_HSLV_W::new(self, 9)
    }
    ///Bit 17 - IWDG stop mode freeze Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn iwdg_fz_stop(&mut self) -> IWDG_FZ_STOP_W<'_, OBW1SRPrs> {
        IWDG_FZ_STOP_W::new(self, 17)
    }
    ///Bit 18 - IWDG standby mode freeze programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn iwdg_fz_sdby(&mut self) -> IWDG_FZ_SDBY_W<'_, OBW1SRPrs> {
        IWDG_FZ_SDBY_W::new(self, 18)
    }
    ///Bit 29 - I/O High-Speed at Low-Voltage programming Write to change corresponding bit in FLASH_OBW1SR register.
    #[inline(always)]
    pub fn vddio_hslv(&mut self) -> VDDIO_HSLV_W<'_, OBW1SRPrs> {
        VDDIO_HSLV_W::new(self, 29)
    }
}
/**FLASH option byte word 1 status register programming

You can [`read`](crate::Reg::read) this register and get [`obw1srp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obw1srp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#FLASH:OBW1SRP)*/
pub struct OBW1SRPrs;
impl crate::RegisterSpec for OBW1SRPrs {
    type Ux = u32;
}
///`read()` method returns [`obw1srp::R`](R) reader structure
impl crate::Readable for OBW1SRPrs {}
///`write(|w| ..)` method takes [`obw1srp::W`](W) writer structure
impl crate::Writable for OBW1SRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OBW1SRP to value 0
impl crate::Resettable for OBW1SRPrs {}
