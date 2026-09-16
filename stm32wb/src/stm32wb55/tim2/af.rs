///Register `AF` reader
pub type R = crate::R<AFrs>;
///Register `AF` writer
pub type W = crate::W<AFrs>;
///Field `ETRSEL` reader - External trigger source selection
pub type ETRSEL_R = crate::FieldReader;
///Field `ETRSEL` writer - External trigger source selection
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 14:16 - External trigger source selection
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF")
            .field("etrsel", &self.etrsel())
            .finish()
    }
}
impl W {
    ///Bits 14:16 - External trigger source selection
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W<'_, AFrs> {
        ETRSEL_W::new(self, 14)
    }
}
/**TIM2 alternate function option register 1

You can [`read`](crate::Reg::read) this register and get [`af::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM2:AF)*/
pub struct AFrs;
impl crate::RegisterSpec for AFrs {
    type Ux = u32;
}
///`read()` method returns [`af::R`](R) reader structure
impl crate::Readable for AFrs {}
///`write(|w| ..)` method takes [`af::W`](W) writer structure
impl crate::Writable for AFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AF to value 0
impl crate::Resettable for AFrs {}
