///Register `TIM3_AF1` reader
pub type R = crate::R<TIM3_AF1rs>;
///Register `TIM3_AF1` writer
pub type W = crate::W<TIM3_AF1rs>;
///Field `ETRSEL` reader - ETR source selection
pub type ETRSEL_R = crate::FieldReader;
///Field `ETRSEL` writer - ETR source selection
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 14:17 - ETR source selection
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_AF1")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 14:17 - ETR source selection
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<TIM3_AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM3 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`tim3_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM3:TIM3_AF1)*/
pub struct TIM3_AF1rs;
impl crate::RegisterSpec for TIM3_AF1rs {
    type Ux = u32;
}
///`read()` method returns [`tim3_af1::R`](R) reader structure
impl crate::Readable for TIM3_AF1rs {}
///`write(|w| ..)` method takes [`tim3_af1::W`](W) writer structure
impl crate::Writable for TIM3_AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIM3_AF1 to value 0
impl crate::Resettable for TIM3_AF1rs {
    const RESET_VALUE: u32 = 0;
}
