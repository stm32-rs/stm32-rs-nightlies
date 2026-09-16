///Register `PLL1CFGR2` reader
pub type R = crate::R<PLL1CFGR2rs>;
///Register `PLL1CFGR2` writer
pub type W = crate::W<PLL1CFGR2rs>;
///Field `DIVP` reader - DIVP
pub type DIVP_R = crate::FieldReader;
///Field `DIVP` writer - DIVP
pub type DIVP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVQ` reader - DIVQ
pub type DIVQ_R = crate::FieldReader;
///Field `DIVQ` writer - DIVQ
pub type DIVQ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DIVR` reader - DIVR
pub type DIVR_R = crate::FieldReader;
///Field `DIVR` writer - DIVR
pub type DIVR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - DIVP
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:14 - DIVQ
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 16:22 - DIVR
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR2")
            .field("divp", &self.divp())
            .field("divq", &self.divq())
            .field("divr", &self.divr())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - DIVP
    #[inline(always)]
    pub fn divp(&mut self) -> DIVP_W<'_, PLL1CFGR2rs> {
        DIVP_W::new(self, 0)
    }
    ///Bits 8:14 - DIVQ
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W<'_, PLL1CFGR2rs> {
        DIVQ_W::new(self, 8)
    }
    ///Bits 16:22 - DIVR
    #[inline(always)]
    pub fn divr(&mut self) -> DIVR_W<'_, PLL1CFGR2rs> {
        DIVR_W::new(self, 16)
    }
}
/**This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:PLL1CFGR2)*/
pub struct PLL1CFGR2rs;
impl crate::RegisterSpec for PLL1CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr2::R`](R) reader structure
impl crate::Readable for PLL1CFGR2rs {}
///`write(|w| ..)` method takes [`pll1cfgr2::W`](W) writer structure
impl crate::Writable for PLL1CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR2 to value 0x0001_0100
impl crate::Resettable for PLL1CFGR2rs {
    const RESET_VALUE: u32 = 0x0001_0100;
}
