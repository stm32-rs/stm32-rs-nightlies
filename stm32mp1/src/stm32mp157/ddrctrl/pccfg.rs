///Register `PCCFG` reader
pub type R = crate::R<PCCFGrs>;
///Register `PCCFG` writer
pub type W = crate::W<PCCFGrs>;
///Field `GO2CRITICAL_EN` reader - GO2CRITICAL_EN
pub type GO2CRITICAL_EN_R = crate::BitReader;
///Field `GO2CRITICAL_EN` writer - GO2CRITICAL_EN
pub type GO2CRITICAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PAGEMATCH_LIMIT` reader - PAGEMATCH_LIMIT
pub type PAGEMATCH_LIMIT_R = crate::BitReader;
///Field `PAGEMATCH_LIMIT` writer - PAGEMATCH_LIMIT
pub type PAGEMATCH_LIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BL_EXP_MODE` reader - BL_EXP_MODE
pub type BL_EXP_MODE_R = crate::BitReader;
///Field `BL_EXP_MODE` writer - BL_EXP_MODE
pub type BL_EXP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GO2CRITICAL_EN
    #[inline(always)]
    pub fn go2critical_en(&self) -> GO2CRITICAL_EN_R {
        GO2CRITICAL_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - PAGEMATCH_LIMIT
    #[inline(always)]
    pub fn pagematch_limit(&self) -> PAGEMATCH_LIMIT_R {
        PAGEMATCH_LIMIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - BL_EXP_MODE
    #[inline(always)]
    pub fn bl_exp_mode(&self) -> BL_EXP_MODE_R {
        BL_EXP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCCFG")
            .field("go2critical_en", &self.go2critical_en())
            .field("pagematch_limit", &self.pagematch_limit())
            .field("bl_exp_mode", &self.bl_exp_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - GO2CRITICAL_EN
    #[inline(always)]
    pub fn go2critical_en(&mut self) -> GO2CRITICAL_EN_W<'_, PCCFGrs> {
        GO2CRITICAL_EN_W::new(self, 0)
    }
    ///Bit 4 - PAGEMATCH_LIMIT
    #[inline(always)]
    pub fn pagematch_limit(&mut self) -> PAGEMATCH_LIMIT_W<'_, PCCFGrs> {
        PAGEMATCH_LIMIT_W::new(self, 4)
    }
    ///Bit 8 - BL_EXP_MODE
    #[inline(always)]
    pub fn bl_exp_mode(&mut self) -> BL_EXP_MODE_W<'_, PCCFGrs> {
        BL_EXP_MODE_W::new(self, 8)
    }
}
/**DDRCTRL port common configuration register

You can [`read`](crate::Reg::read) this register and get [`pccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:PCCFG)*/
pub struct PCCFGrs;
impl crate::RegisterSpec for PCCFGrs {
    type Ux = u32;
}
///`read()` method returns [`pccfg::R`](R) reader structure
impl crate::Readable for PCCFGrs {}
///`write(|w| ..)` method takes [`pccfg::W`](W) writer structure
impl crate::Writable for PCCFGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCCFG to value 0
impl crate::Resettable for PCCFGrs {}
