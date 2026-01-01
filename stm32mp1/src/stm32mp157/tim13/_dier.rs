///Register `_DIER` reader
pub type R = crate::R<_DIERrs>;
///Register `_DIER` writer
pub type W = crate::W<_DIERrs>;
///Field `UIE` reader - UIE
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - UIE
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IE` reader - CC1IE
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - CC1IE
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UIE
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1IE
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - UIE
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, _DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - CC1IE
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, _DIERrs> {
        CC1IE_W::new(self, 1)
    }
}
/**TIM13 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM13:_DIER)*/
pub struct _DIERrs;
impl crate::RegisterSpec for _DIERrs {
    type Ux = u16;
}
///`read()` method returns [`_dier::R`](R) reader structure
impl crate::Readable for _DIERrs {}
///`write(|w| ..)` method takes [`_dier::W`](W) writer structure
impl crate::Writable for _DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets _DIER to value 0
impl crate::Resettable for _DIERrs {}
