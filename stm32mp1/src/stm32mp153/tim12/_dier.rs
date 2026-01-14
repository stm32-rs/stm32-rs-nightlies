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
///Field `CC2IE` reader - CC2IE
pub type CC2IE_R = crate::BitReader;
///Field `CC2IE` writer - CC2IE
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE` reader - TIE
pub type TIE_R = crate::BitReader;
///Field `TIE` writer - TIE
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - CC2IE
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - TIE
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("cc2ie", &self.cc2ie())
            .field("tie", &self.tie())
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
    ///Bit 2 - CC2IE
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, _DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 6 - TIE
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, _DIERrs> {
        TIE_W::new(self, 6)
    }
}
/**TIM12 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM12:_DIER)*/
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
