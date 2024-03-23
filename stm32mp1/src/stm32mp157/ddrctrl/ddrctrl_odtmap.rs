#[doc = "Register `DDRCTRL_ODTMAP` reader"]
pub type R = crate::R<DDRCTRL_ODTMAPrs>;
#[doc = "Register `DDRCTRL_ODTMAP` writer"]
pub type W = crate::W<DDRCTRL_ODTMAPrs>;
#[doc = "Field `RANK0_WR_ODT` reader - RANK0_WR_ODT"]
pub type RANK0_WR_ODT_R = crate::BitReader;
#[doc = "Field `RANK0_WR_ODT` writer - RANK0_WR_ODT"]
pub type RANK0_WR_ODT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANK0_RD_ODT` reader - RANK0_RD_ODT"]
pub type RANK0_RD_ODT_R = crate::BitReader;
#[doc = "Field `RANK0_RD_ODT` writer - RANK0_RD_ODT"]
pub type RANK0_RD_ODT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RANK0_WR_ODT"]
    #[inline(always)]
    pub fn rank0_wr_odt(&self) -> RANK0_WR_ODT_R {
        RANK0_WR_ODT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - RANK0_RD_ODT"]
    #[inline(always)]
    pub fn rank0_rd_odt(&self) -> RANK0_RD_ODT_R {
        RANK0_RD_ODT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RANK0_WR_ODT"]
    #[inline(always)]
    #[must_use]
    pub fn rank0_wr_odt(&mut self) -> RANK0_WR_ODT_W<DDRCTRL_ODTMAPrs> {
        RANK0_WR_ODT_W::new(self, 0)
    }
    #[doc = "Bit 4 - RANK0_RD_ODT"]
    #[inline(always)]
    #[must_use]
    pub fn rank0_rd_odt(&mut self) -> RANK0_RD_ODT_W<DDRCTRL_ODTMAPrs> {
        RANK0_RD_ODT_W::new(self, 4)
    }
}
#[doc = "DDRCTRL ODT/Rank map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_odtmap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_odtmap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_ODTMAPrs;
impl crate::RegisterSpec for DDRCTRL_ODTMAPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_odtmap::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_ODTMAPrs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_odtmap::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_ODTMAPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ODTMAP to value 0x11"]
impl crate::Resettable for DDRCTRL_ODTMAPrs {
    const RESET_VALUE: u32 = 0x11;
}
