///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader;
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader;
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, DIERrs> {
        CC1IE_W::new(self, 1)
    }
}
/**TIM11 Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM11:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u16;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
