///Register `TIM5_OR` reader
pub type R = crate::R<TIM5_ORrs>;
///Register `TIM5_OR` writer
pub type W = crate::W<TIM5_ORrs>;
///Field `IT4_RMP` reader - Timer Input 4 remap
pub type IT4_RMP_R = crate::FieldReader;
///Field `IT4_RMP` writer - Timer Input 4 remap
pub type IT4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 6:7 - Timer Input 4 remap
    #[inline(always)]
    pub fn it4_rmp(&self) -> IT4_RMP_R {
        IT4_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM5_OR")
            .field("it4_rmp", &self.it4_rmp())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - Timer Input 4 remap
    #[inline(always)]
    pub fn it4_rmp(&mut self) -> IT4_RMP_W<'_, TIM5_ORrs> {
        IT4_RMP_W::new(self, 6)
    }
}
/**TIM5 option register

You can [`read`](crate::Reg::read) this register and get [`tim5_or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim5_or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#TIM5:TIM5_OR)*/
pub struct TIM5_ORrs;
impl crate::RegisterSpec for TIM5_ORrs {
    type Ux = u16;
}
///`read()` method returns [`tim5_or::R`](R) reader structure
impl crate::Readable for TIM5_ORrs {}
///`write(|w| ..)` method takes [`tim5_or::W`](W) writer structure
impl crate::Writable for TIM5_ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM5_OR to value 0
impl crate::Resettable for TIM5_ORrs {}
