///Register `OPTKEYR_` writer
pub type W = crate::W<OPTKEYR_rs>;
///Field `OPTKEYR` writer - FLASH option bytes control access unlock key
pub type OPTKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<OPTKEYR_rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - FLASH option bytes control access unlock key
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<'_, OPTKEYR_rs> {
        OPTKEYR_W::new(self, 0)
    }
}
/**FLASH option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#FLASH:OPTKEYR_)*/
pub struct OPTKEYR_rs;
impl crate::RegisterSpec for OPTKEYR_rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`optkeyr_::W`](W) writer structure
impl crate::Writable for OPTKEYR_rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTKEYR_ to value 0
impl crate::Resettable for OPTKEYR_rs {}
