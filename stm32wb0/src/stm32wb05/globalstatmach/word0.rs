///Register `WORD0` reader
pub type R = crate::R<WORD0rs>;
///Register `WORD0` writer
pub type W = crate::W<WORD0rs>;
///Field `RadioConfigPtr` reader - Radio Configuration address Pointer.
pub type RADIO_CONFIG_PTR_R = crate::FieldReader<u32>;
///Field `RadioConfigPtr` writer - Radio Configuration address Pointer.
pub type RADIO_CONFIG_PTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Radio Configuration address Pointer.
    #[inline(always)]
    pub fn radio_config_ptr(&self) -> RADIO_CONFIG_PTR_R {
        RADIO_CONFIG_PTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD0")
            .field("radio_config_ptr", &self.radio_config_ptr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Radio Configuration address Pointer.
    #[inline(always)]
    pub fn radio_config_ptr(&mut self) -> RADIO_CONFIG_PTR_W<'_, WORD0rs> {
        RADIO_CONFIG_PTR_W::new(self, 0)
    }
}
/**WORD0 register

You can [`read`](crate::Reg::read) this register and get [`word0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#GLOBALSTATMACH:WORD0)*/
pub struct WORD0rs;
impl crate::RegisterSpec for WORD0rs {
    type Ux = u32;
}
///`read()` method returns [`word0::R`](R) reader structure
impl crate::Readable for WORD0rs {}
///`write(|w| ..)` method takes [`word0::W`](W) writer structure
impl crate::Writable for WORD0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WORD0 to value 0
impl crate::Resettable for WORD0rs {}
