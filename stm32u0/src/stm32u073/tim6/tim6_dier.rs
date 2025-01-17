///Register `TIM6_DIER` reader
pub type R = crate::R<TIM6_DIERrs>;
///Register `TIM6_DIER` writer
pub type W = crate::W<TIM6_DIERrs>;
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDE` reader - Update DMA request enable
pub type UDE_R = crate::BitReader;
///Field `UDE` writer - Update DMA request enable
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM6_DIER")
            .field("uie", &self.uie())
            .field("ude", &self.ude())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<TIM6_DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<TIM6_DIERrs> {
        UDE_W::new(self, 8)
    }
}
/**TIM6 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim6_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TIM6:TIM6_DIER)*/
pub struct TIM6_DIERrs;
impl crate::RegisterSpec for TIM6_DIERrs {
    type Ux = u16;
}
///`read()` method returns [`tim6_dier::R`](R) reader structure
impl crate::Readable for TIM6_DIERrs {}
///`write(|w| ..)` method takes [`tim6_dier::W`](W) writer structure
impl crate::Writable for TIM6_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM6_DIER to value 0
impl crate::Resettable for TIM6_DIERrs {
    const RESET_VALUE: u16 = 0;
}
