#[doc = "Register `DDRCTRL_DFITMG0` reader"]
pub type R = crate::R<DDRCTRL_DFITMG0rs>;
#[doc = "Register `DDRCTRL_DFITMG0` writer"]
pub type W = crate::W<DDRCTRL_DFITMG0rs>;
#[doc = "Field `DFI_TPHY_WRLAT` reader - DFI_TPHY_WRLAT"]
pub type DFI_TPHY_WRLAT_R = crate::FieldReader;
#[doc = "Field `DFI_TPHY_WRLAT` writer - DFI_TPHY_WRLAT"]
pub type DFI_TPHY_WRLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DFI_TPHY_WRDATA` reader - DFI_TPHY_WRDATA"]
pub type DFI_TPHY_WRDATA_R = crate::FieldReader;
#[doc = "Field `DFI_TPHY_WRDATA` writer - DFI_TPHY_WRDATA"]
pub type DFI_TPHY_WRDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DFI_T_RDDATA_EN` reader - DFI_T_RDDATA_EN"]
pub type DFI_T_RDDATA_EN_R = crate::FieldReader;
#[doc = "Field `DFI_T_RDDATA_EN` writer - DFI_T_RDDATA_EN"]
pub type DFI_T_RDDATA_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DFI_T_CTRL_DELAY` reader - DFI_T_CTRL_DELAY"]
pub type DFI_T_CTRL_DELAY_R = crate::FieldReader;
#[doc = "Field `DFI_T_CTRL_DELAY` writer - DFI_T_CTRL_DELAY"]
pub type DFI_T_CTRL_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - DFI_TPHY_WRLAT"]
    #[inline(always)]
    pub fn dfi_tphy_wrlat(&self) -> DFI_TPHY_WRLAT_R {
        DFI_TPHY_WRLAT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - DFI_TPHY_WRDATA"]
    #[inline(always)]
    pub fn dfi_tphy_wrdata(&self) -> DFI_TPHY_WRDATA_R {
        DFI_TPHY_WRDATA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - DFI_T_RDDATA_EN"]
    #[inline(always)]
    pub fn dfi_t_rddata_en(&self) -> DFI_T_RDDATA_EN_R {
        DFI_T_RDDATA_EN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:28 - DFI_T_CTRL_DELAY"]
    #[inline(always)]
    pub fn dfi_t_ctrl_delay(&self) -> DFI_T_CTRL_DELAY_R {
        DFI_T_CTRL_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DFI_TPHY_WRLAT"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_tphy_wrlat(&mut self) -> DFI_TPHY_WRLAT_W<DDRCTRL_DFITMG0rs> {
        DFI_TPHY_WRLAT_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - DFI_TPHY_WRDATA"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_tphy_wrdata(&mut self) -> DFI_TPHY_WRDATA_W<DDRCTRL_DFITMG0rs> {
        DFI_TPHY_WRDATA_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - DFI_T_RDDATA_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_rddata_en(&mut self) -> DFI_T_RDDATA_EN_W<DDRCTRL_DFITMG0rs> {
        DFI_T_RDDATA_EN_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - DFI_T_CTRL_DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrl_delay(&mut self) -> DFI_T_CTRL_DELAY_W<DDRCTRL_DFITMG0rs> {
        DFI_T_CTRL_DELAY_W::new(self, 24)
    }
}
#[doc = "DDRCTRL DFI timing register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfitmg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfitmg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DFITMG0rs;
impl crate::RegisterSpec for DDRCTRL_DFITMG0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dfitmg0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DFITMG0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dfitmg0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DFITMG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFITMG0 to value 0x0702_0002"]
impl crate::Resettable for DDRCTRL_DFITMG0rs {
    const RESET_VALUE: u32 = 0x0702_0002;
}
