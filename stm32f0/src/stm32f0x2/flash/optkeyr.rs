///Register `OPTKEYR` writer
pub type W = crate::W<OPTKEYRrs>;
///Field `OPTKEYR` writer - Option byte key
pub type OPTKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<OPTKEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Option byte key
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<'_, OPTKEYRrs> {
        OPTKEYR_W::new(self, 0)
    }
}
/**Flash option key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x2.html#Flash:OPTKEYR)*/
pub struct OPTKEYRrs;
impl crate::RegisterSpec for OPTKEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure
impl crate::Writable for OPTKEYRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets OPTKEYR to value 0
impl crate::Resettable for OPTKEYRrs {}
