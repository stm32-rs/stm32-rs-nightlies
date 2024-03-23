#[doc = "Register `DDRCTRL_HWLPCTL` reader"]
pub type R = crate::R<DDRCTRL_HWLPCTLrs>;
#[doc = "Register `DDRCTRL_HWLPCTL` writer"]
pub type W = crate::W<DDRCTRL_HWLPCTLrs>;
#[doc = "Field `HW_LP_EN` reader - HW_LP_EN"]
pub type HW_LP_EN_R = crate::BitReader;
#[doc = "Field `HW_LP_EN` writer - HW_LP_EN"]
pub type HW_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_LP_EXIT_IDLE_EN` reader - HW_LP_EXIT_IDLE_EN"]
pub type HW_LP_EXIT_IDLE_EN_R = crate::BitReader;
#[doc = "Field `HW_LP_EXIT_IDLE_EN` writer - HW_LP_EXIT_IDLE_EN"]
pub type HW_LP_EXIT_IDLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW_LP_IDLE_X32` reader - HW_LP_IDLE_X32"]
pub type HW_LP_IDLE_X32_R = crate::FieldReader<u16>;
#[doc = "Field `HW_LP_IDLE_X32` writer - HW_LP_IDLE_X32"]
pub type HW_LP_IDLE_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    pub fn hw_lp_en(&self) -> HW_LP_EN_R {
        HW_LP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&self) -> HW_LP_EXIT_IDLE_EN_R {
        HW_LP_EXIT_IDLE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    pub fn hw_lp_idle_x32(&self) -> HW_LP_IDLE_X32_R {
        HW_LP_IDLE_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    #[must_use]
    pub fn hw_lp_en(&mut self) -> HW_LP_EN_W<DDRCTRL_HWLPCTLrs> {
        HW_LP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    #[must_use]
    pub fn hw_lp_exit_idle_en(&mut self) -> HW_LP_EXIT_IDLE_EN_W<DDRCTRL_HWLPCTLrs> {
        HW_LP_EXIT_IDLE_EN_W::new(self, 1)
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    #[must_use]
    pub fn hw_lp_idle_x32(&mut self) -> HW_LP_IDLE_X32_W<DDRCTRL_HWLPCTLrs> {
        HW_LP_IDLE_X32_W::new(self, 16)
    }
}
#[doc = "DDRCTRL hardware low power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_hwlpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_hwlpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_HWLPCTLrs;
impl crate::RegisterSpec for DDRCTRL_HWLPCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_hwlpctl::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_HWLPCTLrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_hwlpctl::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_HWLPCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_HWLPCTL to value 0x03"]
impl crate::Resettable for DDRCTRL_HWLPCTLrs {
    const RESET_VALUE: u32 = 0x03;
}
