///Register `VAL` reader
pub type R = crate::R<VALrs>;
///Register `VAL` writer
pub type W = crate::W<VALrs>;
///Field `CURRENT` reader - Current counter value
pub type CURRENT_R = crate::FieldReader<u32>;
///Field `CURRENT` writer - Current counter value
pub type CURRENT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAL")
            .field("current", &self.current())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W<'_, VALrs> {
        CURRENT_W::new(self, 0)
    }
}
/**SysTick current value register

You can [`read`](crate::Reg::read) this register and get [`val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#STK:VAL)*/
pub struct VALrs;
impl crate::RegisterSpec for VALrs {
    type Ux = u32;
}
///`read()` method returns [`val::R`](R) reader structure
impl crate::Readable for VALrs {}
///`write(|w| ..)` method takes [`val::W`](W) writer structure
impl crate::Writable for VALrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VAL to value 0
impl crate::Resettable for VALrs {}
