///Register `TISEL` reader
pub type R = crate::R<TISELrs>;
///Register `TISEL` writer
pub type W = crate::W<TISELrs>;
/**Field `TI1SEL` reader - selects TI1\[0\]
to TI1\[15\]
input Others: Reserved*/
pub type TI1SEL_R = crate::FieldReader;
/**Field `TI1SEL` writer - selects TI1\[0\]
to TI1\[15\]
input Others: Reserved*/
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    /**Bits 0:3 - selects TI1\[0\]
    to TI1\[15\]
    input Others: Reserved*/
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TISEL")
            .field("ti1sel", &self.ti1sel())
            .finish()
    }
}
impl W {
    /**Bits 0:3 - selects TI1\[0\]
    to TI1\[15\]
    input Others: Reserved*/
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TISELrs> {
        TI1SEL_W::new(self, 0)
    }
}
/**TIM14 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#TIM14:TISEL)*/
pub struct TISELrs;
impl crate::RegisterSpec for TISELrs {
    type Ux = u16;
}
///`read()` method returns [`tisel::R`](R) reader structure
impl crate::Readable for TISELrs {}
///`write(|w| ..)` method takes [`tisel::W`](W) writer structure
impl crate::Writable for TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TISEL to value 0
impl crate::Resettable for TISELrs {
    const RESET_VALUE: u16 = 0;
}
