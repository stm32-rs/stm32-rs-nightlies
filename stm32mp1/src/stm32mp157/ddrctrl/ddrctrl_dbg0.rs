#[doc = "Register `DDRCTRL_DBG0` reader"]
pub type R = crate::R<DDRCTRL_DBG0rs>;
#[doc = "Register `DDRCTRL_DBG0` writer"]
pub type W = crate::W<DDRCTRL_DBG0rs>;
#[doc = "Field `DIS_WC` reader - DIS_WC"]
pub type DIS_WC_R = crate::BitReader;
#[doc = "Field `DIS_WC` writer - DIS_WC"]
pub type DIS_WC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_COLLISION_PAGE_OPT` reader - DIS_COLLISION_PAGE_OPT"]
pub type DIS_COLLISION_PAGE_OPT_R = crate::BitReader;
#[doc = "Field `DIS_COLLISION_PAGE_OPT` writer - DIS_COLLISION_PAGE_OPT"]
pub type DIS_COLLISION_PAGE_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIS_WC"]
    #[inline(always)]
    pub fn dis_wc(&self) -> DIS_WC_R {
        DIS_WC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DIS_COLLISION_PAGE_OPT"]
    #[inline(always)]
    pub fn dis_collision_page_opt(&self) -> DIS_COLLISION_PAGE_OPT_R {
        DIS_COLLISION_PAGE_OPT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_WC"]
    #[inline(always)]
    #[must_use]
    pub fn dis_wc(&mut self) -> DIS_WC_W<DDRCTRL_DBG0rs> {
        DIS_WC_W::new(self, 0)
    }
    #[doc = "Bit 4 - DIS_COLLISION_PAGE_OPT"]
    #[inline(always)]
    #[must_use]
    pub fn dis_collision_page_opt(&mut self) -> DIS_COLLISION_PAGE_OPT_W<DDRCTRL_DBG0rs> {
        DIS_COLLISION_PAGE_OPT_W::new(self, 4)
    }
}
#[doc = "DDRCTRL debug register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_dbg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_dbg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_DBG0rs;
impl crate::RegisterSpec for DDRCTRL_DBG0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_dbg0::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_DBG0rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_dbg0::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_DBG0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DBG0 to value 0"]
impl crate::Resettable for DDRCTRL_DBG0rs {
    const RESET_VALUE: u32 = 0;
}
