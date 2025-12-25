///Register `MCO1CFGR` reader
pub type R = crate::R<MCO1CFGRrs>;
///Register `MCO1CFGR` writer
pub type W = crate::W<MCO1CFGRrs>;
///Field `MCO1SEL` reader - MCO1SEL
pub type MCO1SEL_R = crate::FieldReader;
///Field `MCO1SEL` writer - MCO1SEL
pub type MCO1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MCO1DIV` reader - MCO1DIV
pub type MCO1DIV_R = crate::FieldReader;
///Field `MCO1DIV` writer - MCO1DIV
pub type MCO1DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCO1ON` reader - MCO1ON
pub type MCO1ON_R = crate::BitReader;
///Field `MCO1ON` writer - MCO1ON
pub type MCO1ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - MCO1SEL
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:7 - MCO1DIV
    #[inline(always)]
    pub fn mco1div(&self) -> MCO1DIV_R {
        MCO1DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 12 - MCO1ON
    #[inline(always)]
    pub fn mco1on(&self) -> MCO1ON_R {
        MCO1ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCO1CFGR")
            .field("mco1sel", &self.mco1sel())
            .field("mco1div", &self.mco1div())
            .field("mco1on", &self.mco1on())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MCO1SEL
    #[inline(always)]
    pub fn mco1sel(&mut self) -> MCO1SEL_W<'_, MCO1CFGRrs> {
        MCO1SEL_W::new(self, 0)
    }
    ///Bits 4:7 - MCO1DIV
    #[inline(always)]
    pub fn mco1div(&mut self) -> MCO1DIV_W<'_, MCO1CFGRrs> {
        MCO1DIV_W::new(self, 4)
    }
    ///Bit 12 - MCO1ON
    #[inline(always)]
    pub fn mco1on(&mut self) -> MCO1ON_W<'_, MCO1CFGRrs> {
        MCO1ON_W::new(self, 12)
    }
}
/**This register is used to select the clock generated on MCO1 output.

You can [`read`](crate::Reg::read) this register and get [`mco1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mco1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MCO1CFGR)*/
pub struct MCO1CFGRrs;
impl crate::RegisterSpec for MCO1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`mco1cfgr::R`](R) reader structure
impl crate::Readable for MCO1CFGRrs {}
///`write(|w| ..)` method takes [`mco1cfgr::W`](W) writer structure
impl crate::Writable for MCO1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCO1CFGR to value 0
impl crate::Resettable for MCO1CFGRrs {}
