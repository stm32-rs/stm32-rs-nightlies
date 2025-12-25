///Register `MCO2CFGR` reader
pub type R = crate::R<MCO2CFGRrs>;
///Register `MCO2CFGR` writer
pub type W = crate::W<MCO2CFGRrs>;
///Field `MCO2SEL` reader - MCO2SEL
pub type MCO2SEL_R = crate::FieldReader;
///Field `MCO2SEL` writer - MCO2SEL
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO2DIV` reader - MCO2DIV
pub type MCO2DIV_R = crate::FieldReader;
///Field `MCO2DIV` writer - MCO2DIV
pub type MCO2DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCO2ON` reader - MCO2ON
pub type MCO2ON_R = crate::BitReader;
///Field `MCO2ON` writer - MCO2ON
pub type MCO2ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - MCO2SEL
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:7 - MCO2DIV
    #[inline(always)]
    pub fn mco2div(&self) -> MCO2DIV_R {
        MCO2DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 12 - MCO2ON
    #[inline(always)]
    pub fn mco2on(&self) -> MCO2ON_R {
        MCO2ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCO2CFGR")
            .field("mco2sel", &self.mco2sel())
            .field("mco2div", &self.mco2div())
            .field("mco2on", &self.mco2on())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MCO2SEL
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<'_, MCO2CFGRrs> {
        MCO2SEL_W::new(self, 0)
    }
    ///Bits 4:7 - MCO2DIV
    #[inline(always)]
    pub fn mco2div(&mut self) -> MCO2DIV_W<'_, MCO2CFGRrs> {
        MCO2DIV_W::new(self, 4)
    }
    ///Bit 12 - MCO2ON
    #[inline(always)]
    pub fn mco2on(&mut self) -> MCO2ON_W<'_, MCO2CFGRrs> {
        MCO2ON_W::new(self, 12)
    }
}
/**This register is used to select the clock generated on MCO2 output.

You can [`read`](crate::Reg::read) this register and get [`mco2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MCO2CFGR)*/
pub struct MCO2CFGRrs;
impl crate::RegisterSpec for MCO2CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`mco2cfgr::R`](R) reader structure
impl crate::Readable for MCO2CFGRrs {}
///`write(|w| ..)` method takes [`mco2cfgr::W`](W) writer structure
impl crate::Writable for MCO2CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCO2CFGR to value 0
impl crate::Resettable for MCO2CFGRrs {}
