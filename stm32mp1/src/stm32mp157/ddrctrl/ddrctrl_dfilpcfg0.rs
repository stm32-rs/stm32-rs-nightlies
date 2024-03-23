#[doc = "Register `DDRCTRL_DFILPCFG0` reader"]
pub type R = crate::R<DDRCTRL_DFILPCFG0rs>;
#[doc = "Register `DDRCTRL_DFILPCFG0` writer"]
pub type W = crate::W<DDRCTRL_DFILPCFG0rs>;
#[doc = "Field `DFI_LP_EN_PD` reader - DFI_LP_EN_PD"]
pub type DFI_LP_EN_PD_R = crate::BitReader;
#[doc = "Field `DFI_LP_EN_PD` writer - DFI_LP_EN_PD"]
pub type DFI_LP_EN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_LP_WAKEUP_PD` reader - DFI_LP_WAKEUP_PD"]
pub type DFI_LP_WAKEUP_PD_R = crate::FieldReader;
#[doc = "Field `DFI_LP_WAKEUP_PD` writer - DFI_LP_WAKEUP_PD"]
pub type DFI_LP_WAKEUP_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DFI_LP_EN_SR` reader - DFI_LP_EN_SR"]
pub type DFI_LP_EN_SR_R = crate::BitReader;
#[doc = "Field `DFI_LP_EN_SR` writer - DFI_LP_EN_SR"]
pub type DFI_LP_EN_SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_LP_WAKEUP_SR` reader - DFI_LP_WAKEUP_SR"]
pub type DFI_LP_WAKEUP_SR_R = crate::FieldReader;
#[doc = "Field `DFI_LP_WAKEUP_SR` writer - DFI_LP_WAKEUP_SR"]
pub type DFI_LP_WAKEUP_SR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DFI_LP_EN_DPD` reader - DFI_LP_EN_DPD"]
pub type DFI_LP_EN_DPD_R = crate::BitReader;
#[doc = "Field `DFI_LP_EN_DPD` writer - DFI_LP_EN_DPD"]
pub type DFI_LP_EN_DPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_LP_WAKEUP_DPD` reader - DFI_LP_WAKEUP_DPD"]
pub type DFI_LP_WAKEUP_DPD_R = crate::FieldReader;
#[doc = "Field `DFI_LP_WAKEUP_DPD` writer - DFI_LP_WAKEUP_DPD"]
pub type DFI_LP_WAKEUP_DPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DFI_TLP_RESP` reader - DFI_TLP_RESP"]
pub type DFI_TLP_RESP_R = crate::FieldReader;
#[doc = "Field `DFI_TLP_RESP` writer - DFI_TLP_RESP"]
pub type DFI_TLP_RESP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - DFI_LP_EN_PD"]
    #[inline(always)]
    pub fn dfi_lp_en_pd(&self) -> DFI_LP_EN_PD_R {
        DFI_LP_EN_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - DFI_LP_WAKEUP_PD"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_pd(&self) -> DFI_LP_WAKEUP_PD_R {
        DFI_LP_WAKEUP_PD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DFI_LP_EN_SR"]
    #[inline(always)]
    pub fn dfi_lp_en_sr(&self) -> DFI_LP_EN_SR_R {
        DFI_LP_EN_SR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - DFI_LP_WAKEUP_SR"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_sr(&self) -> DFI_LP_WAKEUP_SR_R {
        DFI_LP_WAKEUP_SR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - DFI_LP_EN_DPD"]
    #[inline(always)]
    pub fn dfi_lp_en_dpd(&self) -> DFI_LP_EN_DPD_R {
        DFI_LP_EN_DPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DFI_LP_WAKEUP_DPD"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_dpd(&self) -> DFI_LP_WAKEUP_DPD_R {
        DFI_LP_WAKEUP_DPD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - DFI_TLP_RESP"]
    #[inline(always)]
    pub fn dfi_tlp_resp(&self) -> DFI_TLP_RESP_R {
        DFI_TLP_RESP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_LP_EN_PD"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_lp_en_pd(&mut self) -> DFI_LP_EN_PD_W<DDRCTRL_DFILPCFG0rs> {
        DFI_LP_EN_PD_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - DFI_LP_WAKEUP_PD"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_lp_wakeup_pd(&mut self) -> DFI_LP_WAKEUP_PD_W<DDRCTRL_DFILPCFG0rs> {
        DFI_LP_WAKEUP_PD_W::new(self, 4)
    }
    #[doc = "Bit 8 - DFI_LP_EN_SR"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_lp_en_sr(&mut self) -> DFI_LP_EN_SR_W<DDRCTRL_DFILPCFG0rs> {
        DFI_LP_EN_SR_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - DFI_LP_WAKEUP_SR"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_lp_wakeup_sr(&mut self) -> DFI_LP_WAKEUP_SR_W<DDRCTRL_DFILPCFG0rs> {
        DFI_LP_WAKEUP_SR_W::new(self, 12)
    }
    #[doc = "Bit 16 - DFI_LP_EN_DPD"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_lp_en_dpd(&mut self) -> DFI_LP_EN_DPD_W<DDRCTRL_DFILPCFG0rs> {
        DFI_LP_EN_DPD_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - DFI_LP_WAKEUP_DPD"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_lp_wakeup_dpd(&mut self) -> DFI_LP_WAKEUP_DPD_W<DDRCTRL_DFILPCFG0rs> {
        DFI_LP_WAKEUP_DPD_W::new(self, 20)
    }
    #[doc = "Bits 24:28 - DFI_TLP_RESP"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_tlp_resp(&mut self) -> DFI_TLP_RESP_W<DDRCTRL_DFILPCFG0rs> {
        DFI_TLP_RESP_W::new(self, 24)
    }
}
#[doc = "DDRCTRL low power configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dfilpcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dfilpcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DFILPCFG0rs;
impl crate::RegisterSpec for DDRCTRL_DFILPCFG0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dfilpcfg0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DFILPCFG0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dfilpcfg0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DFILPCFG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DFILPCFG0 to value 0x0700_0000"]
impl crate::Resettable for DDRCTRL_DFILPCFG0rs {
    const RESET_VALUE: u32 = 0x0700_0000;
}
