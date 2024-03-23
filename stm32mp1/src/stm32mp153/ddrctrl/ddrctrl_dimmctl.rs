#[doc = "Register `DDRCTRL_DIMMCTL` reader"]
pub type R = crate::R<DDRCTRL_DIMMCTLrs>;
#[doc = "Register `DDRCTRL_DIMMCTL` writer"]
pub type W = crate::W<DDRCTRL_DIMMCTLrs>;
#[doc = "Field `DIMM_STAGGER_CS_EN` reader - DIMM_STAGGER_CS_EN"]
pub type DIMM_STAGGER_CS_EN_R = crate::BitReader;
#[doc = "Field `DIMM_STAGGER_CS_EN` writer - DIMM_STAGGER_CS_EN"]
pub type DIMM_STAGGER_CS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIMM_ADDR_MIRR_EN` reader - DIMM_ADDR_MIRR_EN"]
pub type DIMM_ADDR_MIRR_EN_R = crate::BitReader;
#[doc = "Field `DIMM_ADDR_MIRR_EN` writer - DIMM_ADDR_MIRR_EN"]
pub type DIMM_ADDR_MIRR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIMM_STAGGER_CS_EN"]
    #[inline(always)]
    pub fn dimm_stagger_cs_en(&self) -> DIMM_STAGGER_CS_EN_R {
        DIMM_STAGGER_CS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIMM_ADDR_MIRR_EN"]
    #[inline(always)]
    pub fn dimm_addr_mirr_en(&self) -> DIMM_ADDR_MIRR_EN_R {
        DIMM_ADDR_MIRR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIMM_STAGGER_CS_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dimm_stagger_cs_en(&mut self) -> DIMM_STAGGER_CS_EN_W<DDRCTRL_DIMMCTLrs> {
        DIMM_STAGGER_CS_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DIMM_ADDR_MIRR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn dimm_addr_mirr_en(&mut self) -> DIMM_ADDR_MIRR_EN_W<DDRCTRL_DIMMCTLrs> {
        DIMM_ADDR_MIRR_EN_W::new(self, 1)
    }
}
#[doc = "DDRCTRL DIMM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dimmctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dimmctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DIMMCTLrs;
impl crate::RegisterSpec for DDRCTRL_DIMMCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dimmctl::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DIMMCTLrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dimmctl::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DIMMCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DIMMCTL to value 0"]
impl crate::Resettable for DDRCTRL_DIMMCTLrs {
    const RESET_VALUE: u32 = 0;
}
