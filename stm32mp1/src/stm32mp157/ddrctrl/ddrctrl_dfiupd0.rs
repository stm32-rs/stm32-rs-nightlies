#[doc = "Register `DDRCTRL_DFIUPD0` reader"]
pub type R = crate::R<DDRCTRL_DFIUPD0rs>;
#[doc = "Register `DDRCTRL_DFIUPD0` writer"]
pub type W = crate::W<DDRCTRL_DFIUPD0rs>;
#[doc = "Field `DFI_T_CTRLUP_MIN` reader - DFI_T_CTRLUP_MIN"]
pub type DFI_T_CTRLUP_MIN_R = crate::FieldReader<u16>;
#[doc = "Field `DFI_T_CTRLUP_MIN` writer - DFI_T_CTRLUP_MIN"]
pub type DFI_T_CTRLUP_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DFI_T_CTRLUP_MAX` reader - DFI_T_CTRLUP_MAX"]
pub type DFI_T_CTRLUP_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `DFI_T_CTRLUP_MAX` writer - DFI_T_CTRLUP_MAX"]
pub type DFI_T_CTRLUP_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CTRLUPD_PRE_SRX` reader - CTRLUPD_PRE_SRX"]
pub type CTRLUPD_PRE_SRX_R = crate::BitReader;
#[doc = "Field `CTRLUPD_PRE_SRX` writer - CTRLUPD_PRE_SRX"]
pub type CTRLUPD_PRE_SRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_AUTO_CTRLUPD_SRX` reader - DIS_AUTO_CTRLUPD_SRX"]
pub type DIS_AUTO_CTRLUPD_SRX_R = crate::BitReader;
#[doc = "Field `DIS_AUTO_CTRLUPD_SRX` writer - DIS_AUTO_CTRLUPD_SRX"]
pub type DIS_AUTO_CTRLUPD_SRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_AUTO_CTRLUPD` reader - DIS_AUTO_CTRLUPD"]
pub type DIS_AUTO_CTRLUPD_R = crate::BitReader;
#[doc = "Field `DIS_AUTO_CTRLUPD` writer - DIS_AUTO_CTRLUPD"]
pub type DIS_AUTO_CTRLUPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - DFI_T_CTRLUP_MIN"]
    #[inline(always)]
    pub fn dfi_t_ctrlup_min(&self) -> DFI_T_CTRLUP_MIN_R {
        DFI_T_CTRLUP_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - DFI_T_CTRLUP_MAX"]
    #[inline(always)]
    pub fn dfi_t_ctrlup_max(&self) -> DFI_T_CTRLUP_MAX_R {
        DFI_T_CTRLUP_MAX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - CTRLUPD_PRE_SRX"]
    #[inline(always)]
    pub fn ctrlupd_pre_srx(&self) -> CTRLUPD_PRE_SRX_R {
        CTRLUPD_PRE_SRX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIS_AUTO_CTRLUPD_SRX"]
    #[inline(always)]
    pub fn dis_auto_ctrlupd_srx(&self) -> DIS_AUTO_CTRLUPD_SRX_R {
        DIS_AUTO_CTRLUPD_SRX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIS_AUTO_CTRLUPD"]
    #[inline(always)]
    pub fn dis_auto_ctrlupd(&self) -> DIS_AUTO_CTRLUPD_R {
        DIS_AUTO_CTRLUPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - DFI_T_CTRLUP_MIN"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlup_min(&mut self) -> DFI_T_CTRLUP_MIN_W<DDRCTRL_DFIUPD0rs> {
        DFI_T_CTRLUP_MIN_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - DFI_T_CTRLUP_MAX"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlup_max(&mut self) -> DFI_T_CTRLUP_MAX_W<DDRCTRL_DFIUPD0rs> {
        DFI_T_CTRLUP_MAX_W::new(self, 16)
    }
    #[doc = "Bit 29 - CTRLUPD_PRE_SRX"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_pre_srx(&mut self) -> CTRLUPD_PRE_SRX_W<DDRCTRL_DFIUPD0rs> {
        CTRLUPD_PRE_SRX_W::new(self, 29)
    }
    #[doc = "Bit 30 - DIS_AUTO_CTRLUPD_SRX"]
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_ctrlupd_srx(&mut self) -> DIS_AUTO_CTRLUPD_SRX_W<DDRCTRL_DFIUPD0rs> {
        DIS_AUTO_CTRLUPD_SRX_W::new(self, 30)
    }
    #[doc = "Bit 31 - DIS_AUTO_CTRLUPD"]
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_ctrlupd(&mut self) -> DIS_AUTO_CTRLUPD_W<DDRCTRL_DFIUPD0rs> {
        DIS_AUTO_CTRLUPD_W::new(self, 31)
    }
}
#[doc = "DDRCTRL DFI update register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiupd0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiupd0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DFIUPD0rs;
impl crate::RegisterSpec for DDRCTRL_DFIUPD0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dfiupd0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dfiupd0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFIUPD0 to value 0x0040_0003"]
impl crate::Resettable for DDRCTRL_DFIUPD0rs {
    const RESET_VALUE: u32 = 0x0040_0003;
}
