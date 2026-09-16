///Register `SKR` writer
pub type W = crate::W<SKRrs>;
///Field `KEY` writer - SRAM2 write protection key for software erase
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<SKRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - SRAM2 write protection key for software erase
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, SKRrs> {
        KEY_W::new(self, 0)
    }
}
/**SKR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#SYSCFG:SKR)*/
pub struct SKRrs;
impl crate::RegisterSpec for SKRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`skr::W`](W) writer structure
impl crate::Writable for SKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SKR to value 0
impl crate::Resettable for SKRrs {}
