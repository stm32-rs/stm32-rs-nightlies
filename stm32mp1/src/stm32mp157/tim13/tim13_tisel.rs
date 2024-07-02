///Register `TIM13_TISEL` reader
pub type R = crate::R<TIM13_TISELrs>;
///Register `TIM13_TISEL` writer
pub type W = crate::W<TIM13_TISELrs>;
///Field `TI1SEL` reader - TI1SEL
pub type TI1SEL_R = crate::FieldReader;
///Field `TI1SEL` writer - TI1SEL
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - TI1SEL
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM13_TISEL")
            .field("ti1sel", &self.ti1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TI1SEL
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM13_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
}
/**TIM13 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim13_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim13_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:TIM13_TISEL)*/
pub struct TIM13_TISELrs;
impl crate::RegisterSpec for TIM13_TISELrs {
    type Ux = u16;
}
///`read()` method returns [`tim13_tisel::R`](R) reader structure
impl crate::Readable for TIM13_TISELrs {}
///`write(|w| ..)` method takes [`tim13_tisel::W`](W) writer structure
impl crate::Writable for TIM13_TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM13_TISEL to value 0
impl crate::Resettable for TIM13_TISELrs {
    const RESET_VALUE: u16 = 0;
}
