///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `LCK` reader - LCK
pub type LCK_R = crate::BitReader;
///Field `LCK` writer - LCK
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LCK
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR").field("lck", &self.lck()).finish()
    }
}
impl W {
    ///Bit 0 - LCK
    #[inline(always)]
    pub fn lck(&mut self) -> LCK_W<'_, CRrs> {
        LCK_W::new(self, 0)
    }
}
/**TZSC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#GTZC_TZSC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
