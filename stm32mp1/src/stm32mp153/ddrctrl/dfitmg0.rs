///Register `DFITMG0` reader
pub type R = crate::R<DFITMG0rs>;
///Register `DFITMG0` writer
pub type W = crate::W<DFITMG0rs>;
///Field `DFI_TPHY_WRLAT` reader - DFI_TPHY_WRLAT
pub type DFI_TPHY_WRLAT_R = crate::FieldReader;
///Field `DFI_TPHY_WRLAT` writer - DFI_TPHY_WRLAT
pub type DFI_TPHY_WRLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DFI_TPHY_WRDATA` reader - DFI_TPHY_WRDATA
pub type DFI_TPHY_WRDATA_R = crate::FieldReader;
///Field `DFI_TPHY_WRDATA` writer - DFI_TPHY_WRDATA
pub type DFI_TPHY_WRDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DFI_T_RDDATA_EN` reader - DFI_T_RDDATA_EN
pub type DFI_T_RDDATA_EN_R = crate::FieldReader;
///Field `DFI_T_RDDATA_EN` writer - DFI_T_RDDATA_EN
pub type DFI_T_RDDATA_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DFI_T_CTRL_DELAY` reader - DFI_T_CTRL_DELAY
pub type DFI_T_CTRL_DELAY_R = crate::FieldReader;
///Field `DFI_T_CTRL_DELAY` writer - DFI_T_CTRL_DELAY
pub type DFI_T_CTRL_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:5 - DFI_TPHY_WRLAT
    #[inline(always)]
    pub fn dfi_tphy_wrlat(&self) -> DFI_TPHY_WRLAT_R {
        DFI_TPHY_WRLAT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - DFI_TPHY_WRDATA
    #[inline(always)]
    pub fn dfi_tphy_wrdata(&self) -> DFI_TPHY_WRDATA_R {
        DFI_TPHY_WRDATA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:22 - DFI_T_RDDATA_EN
    #[inline(always)]
    pub fn dfi_t_rddata_en(&self) -> DFI_T_RDDATA_EN_R {
        DFI_T_RDDATA_EN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:28 - DFI_T_CTRL_DELAY
    #[inline(always)]
    pub fn dfi_t_ctrl_delay(&self) -> DFI_T_CTRL_DELAY_R {
        DFI_T_CTRL_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFITMG0")
            .field("dfi_tphy_wrlat", &self.dfi_tphy_wrlat())
            .field("dfi_tphy_wrdata", &self.dfi_tphy_wrdata())
            .field("dfi_t_rddata_en", &self.dfi_t_rddata_en())
            .field("dfi_t_ctrl_delay", &self.dfi_t_ctrl_delay())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - DFI_TPHY_WRLAT
    #[inline(always)]
    pub fn dfi_tphy_wrlat(&mut self) -> DFI_TPHY_WRLAT_W<'_, DFITMG0rs> {
        DFI_TPHY_WRLAT_W::new(self, 0)
    }
    ///Bits 8:13 - DFI_TPHY_WRDATA
    #[inline(always)]
    pub fn dfi_tphy_wrdata(&mut self) -> DFI_TPHY_WRDATA_W<'_, DFITMG0rs> {
        DFI_TPHY_WRDATA_W::new(self, 8)
    }
    ///Bits 16:22 - DFI_T_RDDATA_EN
    #[inline(always)]
    pub fn dfi_t_rddata_en(&mut self) -> DFI_T_RDDATA_EN_W<'_, DFITMG0rs> {
        DFI_T_RDDATA_EN_W::new(self, 16)
    }
    ///Bits 24:28 - DFI_T_CTRL_DELAY
    #[inline(always)]
    pub fn dfi_t_ctrl_delay(&mut self) -> DFI_T_CTRL_DELAY_W<'_, DFITMG0rs> {
        DFI_T_CTRL_DELAY_W::new(self, 24)
    }
}
/**DDRCTRL DFI timing register 0

You can [`read`](crate::Reg::read) this register and get [`dfitmg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfitmg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DFITMG0)*/
pub struct DFITMG0rs;
impl crate::RegisterSpec for DFITMG0rs {
    type Ux = u32;
}
///`read()` method returns [`dfitmg0::R`](R) reader structure
impl crate::Readable for DFITMG0rs {}
///`write(|w| ..)` method takes [`dfitmg0::W`](W) writer structure
impl crate::Writable for DFITMG0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFITMG0 to value 0x0702_0002
impl crate::Resettable for DFITMG0rs {
    const RESET_VALUE: u32 = 0x0702_0002;
}
