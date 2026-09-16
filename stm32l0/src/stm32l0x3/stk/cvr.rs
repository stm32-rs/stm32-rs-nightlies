///Register `CVR` reader
pub type R = crate::R<CVRrs>;
///Register `CVR` writer
pub type W = crate::W<CVRrs>;
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
        f.debug_struct("CVR")
            .field("current", &self.current())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W<'_, CVRrs> {
        CURRENT_W::new(self, 0)
    }
}
/**SysTick current value register

You can [`read`](crate::Reg::read) this register and get [`cvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#STK:CVR)*/
pub struct CVRrs;
impl crate::RegisterSpec for CVRrs {
    type Ux = u32;
}
///`read()` method returns [`cvr::R`](R) reader structure
impl crate::Readable for CVRrs {}
///`write(|w| ..)` method takes [`cvr::W`](W) writer structure
impl crate::Writable for CVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CVR to value 0
impl crate::Resettable for CVRrs {}
