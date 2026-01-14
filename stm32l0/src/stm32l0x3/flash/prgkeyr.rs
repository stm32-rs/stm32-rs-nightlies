///Register `PRGKEYR` writer
pub type W = crate::W<PRGKEYRrs>;
///Field `PRGKEYR` writer - Program memory key
pub type PRGKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<PRGKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Program memory key
    #[inline(always)]
    pub fn prgkeyr(&mut self) -> PRGKEYR_W<'_, PRGKEYRrs> {
        PRGKEYR_W::new(self, 0)
    }
}
/**Program memory key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prgkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#FLASH:PRGKEYR)*/
pub struct PRGKEYRrs;
impl crate::RegisterSpec for PRGKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`prgkeyr::W`](W) writer structure
impl crate::Writable for PRGKEYRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets PRGKEYR to value 0
impl crate::Resettable for PRGKEYRrs {}
