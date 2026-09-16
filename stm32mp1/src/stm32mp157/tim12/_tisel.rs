///Register `_TISEL` reader
pub type R = crate::R<_TISELrs>;
///Register `_TISEL` writer
pub type W = crate::W<_TISELrs>;
///Field `TI1SEL` reader - TI1SEL
pub type TI1SEL_R = crate::FieldReader;
///Field `TI1SEL` writer - TI1SEL
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TI2SEL` reader - TI2SEL
pub type TI2SEL_R = crate::FieldReader;
///Field `TI2SEL` writer - TI2SEL
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - TI1SEL
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - TI2SEL
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_TISEL")
            .field("ti1sel", &self.ti1sel())
            .field("ti2sel", &self.ti2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TI1SEL
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<'_, _TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    ///Bits 8:11 - TI2SEL
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W<'_, _TISELrs> {
        TI2SEL_W::new(self, 8)
    }
}
/**TIM12 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM12:_TISEL)*/
pub struct _TISELrs;
impl crate::RegisterSpec for _TISELrs {
    type Ux = u32;
}
///`read()` method returns [`_tisel::R`](R) reader structure
impl crate::Readable for _TISELrs {}
///`write(|w| ..)` method takes [`_tisel::W`](W) writer structure
impl crate::Writable for _TISELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _TISEL to value 0
impl crate::Resettable for _TISELrs {}
