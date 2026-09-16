///Register `PLL1CFGR1` reader
pub type R = crate::R<PLL1CFGR1rs>;
///Register `PLL1CFGR1` writer
pub type W = crate::W<PLL1CFGR1rs>;
///Field `DIVN` reader - DIVN
pub type DIVN_R = crate::FieldReader<u16>;
///Field `DIVN` writer - DIVN
pub type DIVN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DIVM1` reader - DIVM1
pub type DIVM1_R = crate::FieldReader;
///Field `DIVM1` writer - DIVM1
pub type DIVM1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:21 - DIVM1
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR1")
            .field("divn", &self.divn())
            .field("divm1", &self.divm1())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - DIVN
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W<'_, PLL1CFGR1rs> {
        DIVN_W::new(self, 0)
    }
    ///Bits 16:21 - DIVM1
    #[inline(always)]
    pub fn divm1(&mut self) -> DIVM1_W<'_, PLL1CFGR1rs> {
        DIVM1_W::new(self, 16)
    }
}
/**This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:PLL1CFGR1)*/
pub struct PLL1CFGR1rs;
impl crate::RegisterSpec for PLL1CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr1::R`](R) reader structure
impl crate::Readable for PLL1CFGR1rs {}
///`write(|w| ..)` method takes [`pll1cfgr1::W`](W) writer structure
impl crate::Writable for PLL1CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR1 to value 0x0001_0031
impl crate::Resettable for PLL1CFGR1rs {
    const RESET_VALUE: u32 = 0x0001_0031;
}
