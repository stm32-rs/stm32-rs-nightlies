///Register `TIM12_DIER` reader
pub type R = crate::R<TIM12_DIERrs>;
///Register `TIM12_DIER` writer
pub type W = crate::W<TIM12_DIERrs>;
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
        f.debug_struct("TIM12_DIER")
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
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<TIM12_DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - CC1IE
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<TIM12_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - CC2IE
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<TIM12_DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 6 - TIE
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<TIM12_DIERrs> {
        TIE_W::new(self, 6)
    }
}
/**TIM12 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim12_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim12_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM12:TIM12_DIER)*/
pub struct TIM12_DIERrs;
impl crate::RegisterSpec for TIM12_DIERrs {
    type Ux = u16;
}
///`read()` method returns [`tim12_dier::R`](R) reader structure
impl crate::Readable for TIM12_DIERrs {}
///`write(|w| ..)` method takes [`tim12_dier::W`](W) writer structure
impl crate::Writable for TIM12_DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM12_DIER to value 0
impl crate::Resettable for TIM12_DIERrs {
    const RESET_VALUE: u16 = 0;
}
