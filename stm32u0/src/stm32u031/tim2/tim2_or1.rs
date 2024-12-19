///Register `TIM2_OR1` reader
pub type R = crate::R<TIM2_OR1rs>;
///Register `TIM2_OR1` writer
pub type W = crate::W<TIM2_OR1rs>;
///Field `OCREF_CLR` reader - Ocref_clr source selection
pub type OCREF_CLR_R = crate::FieldReader;
///Field `OCREF_CLR` writer - Ocref_clr source selection
pub type OCREF_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Ocref_clr source selection
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_OR1")
            .field("ocref_clr", &self.ocref_clr())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Ocref_clr source selection
    #[inline(always)]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W<TIM2_OR1rs> {
        OCREF_CLR_W::new(self, 0)
    }
}
/**TIM2 option register 1

You can [`read`](crate::Reg::read) this register and get [`tim2_or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#TIM2:TIM2_OR1)*/
pub struct TIM2_OR1rs;
impl crate::RegisterSpec for TIM2_OR1rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_or1::R`](R) reader structure
impl crate::Readable for TIM2_OR1rs {}
///`write(|w| ..)` method takes [`tim2_or1::W`](W) writer structure
impl crate::Writable for TIM2_OR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM2_OR1 to value 0
impl crate::Resettable for TIM2_OR1rs {
    const RESET_VALUE: u32 = 0;
}
