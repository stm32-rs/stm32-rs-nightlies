#[doc = "Register `DDRCTRL_DFIUPD1` reader"]
pub type R = crate::R<DDRCTRL_DFIUPD1rs>;
#[doc = "Register `DDRCTRL_DFIUPD1` writer"]
pub type W = crate::W<DDRCTRL_DFIUPD1rs>;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MAX_X1024` reader - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R = crate::FieldReader;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MAX_X1024` writer - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MIN_X1024` reader - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R = crate::FieldReader;
#[doc = "Field `DFI_T_CTRLUPD_INTERVAL_MIN_X1024` writer - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
pub type DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_max_x1024(&self) -> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R {
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_min_x1024(&self) -> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R {
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlupd_interval_max_x1024(
        &mut self,
    ) -> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W<DDRCTRL_DFIUPD1rs> {
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlupd_interval_min_x1024(
        &mut self,
    ) -> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W<DDRCTRL_DFIUPD1rs> {
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W::new(self, 16)
    }
}
#[doc = "DDRCTRL DFI update register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfiupd1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfiupd1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DFIUPD1rs;
impl crate::RegisterSpec for DDRCTRL_DFIUPD1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dfiupd1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dfiupd1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFIUPD1 to value 0x0001_0001"]
impl crate::Resettable for DDRCTRL_DFIUPD1rs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
