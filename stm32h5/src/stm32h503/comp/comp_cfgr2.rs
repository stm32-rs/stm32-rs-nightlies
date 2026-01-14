///Register `COMP_CFGR2` reader
pub type R = crate::R<COMP_CFGR2rs>;
///Register `COMP_CFGR2` writer
pub type W = crate::W<COMP_CFGR2rs>;
///Field `INPSEL0` reader - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
pub type INPSEL0_R = crate::BitReader;
///Field `INPSEL0` writer - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
pub type INPSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\[31:0\]
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\[31:0\]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
    #[inline(always)]
    pub fn inpsel0(&self) -> INPSEL0_R {
        INPSEL0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\[31:0\]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_CFGR2")
            .field("inpsel0", &self.inpsel0())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table 145: COMP1 noninverting input assignment for more details.
    #[inline(always)]
    pub fn inpsel0(&mut self) -> INPSEL0_W<'_, COMP_CFGR2rs> {
        INPSEL0_W::new(self, 4)
    }
    ///Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\[31:0\]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, COMP_CFGR2rs> {
        LOCK_W::new(self, 31)
    }
}
/**Comparator configuration register 2

You can [`read`](crate::Reg::read) this register and get [`comp_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_CFGR2)*/
pub struct COMP_CFGR2rs;
impl crate::RegisterSpec for COMP_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`comp_cfgr2::R`](R) reader structure
impl crate::Readable for COMP_CFGR2rs {}
///`write(|w| ..)` method takes [`comp_cfgr2::W`](W) writer structure
impl crate::Writable for COMP_CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP_CFGR2 to value 0
impl crate::Resettable for COMP_CFGR2rs {}
