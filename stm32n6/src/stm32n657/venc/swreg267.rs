///Register `SWREG267` reader
pub type R = crate::R<SWREG267rs>;
///Register `SWREG267` writer
pub type W = crate::W<SWREG267rs>;
///Field `SWREG_FIELD` reader - segment 1: Deadzone rate for macroblock skip token 0-1, dmv penalty coefficient (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 1: Deadzone rate for macroblock skip token 0-1, dmv penalty coefficient (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 1: Deadzone rate for macroblock skip token 0-1, dmv penalty coefficient (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG267")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 1: Deadzone rate for macroblock skip token 0-1, dmv penalty coefficient (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG267rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 1: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register

You can [`read`](crate::Reg::read) this register and get [`swreg267::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg267::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#VENC:SWREG267)*/
pub struct SWREG267rs;
impl crate::RegisterSpec for SWREG267rs {
    type Ux = u32;
}
///`read()` method returns [`swreg267::R`](R) reader structure
impl crate::Readable for SWREG267rs {}
///`write(|w| ..)` method takes [`swreg267::W`](W) writer structure
impl crate::Writable for SWREG267rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG267 to value 0
impl crate::Resettable for SWREG267rs {}
