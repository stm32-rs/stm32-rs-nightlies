#[doc = "Register `COMP_CFGR2` reader"]
pub type R = crate::R<COMP_CFGR2rs>;
#[doc = "Register `COMP_CFGR2` writer"]
pub type W = crate::W<COMP_CFGR2rs>;
#[doc = "Field `INPSEL0` reader - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL0_R = crate::BitReader;
#[doc = "Field `INPSEL0` writer - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn inpsel0(&self) -> INPSEL0_R {
        INPSEL0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel0(&mut self) -> INPSEL0_W<COMP_CFGR2rs> {
        INPSEL0_W::new(self, 4)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP_CFGR2rs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "Comparator configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_CFGR2rs;
impl crate::RegisterSpec for COMP_CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_cfgr2::R`](R) reader structure"]
impl crate::Readable for COMP_CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`comp_cfgr2::W`](W) writer structure"]
impl crate::Writable for COMP_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP_CFGR2 to value 0"]
impl crate::Resettable for COMP_CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
