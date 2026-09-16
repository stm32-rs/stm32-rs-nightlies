///Register `CONFCHR2` reader
pub type R = crate::R<CONFCHR2rs>;
///Register `CONFCHR2` writer
pub type W = crate::W<CONFCHR2rs>;
///Field `CONFCH8` reader - Channel 8 configuration
pub type CONFCH8_R = crate::FieldReader;
///Field `CONFCH8` writer - Channel 8 configuration
pub type CONFCH8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Channel 8 configuration
    #[inline(always)]
    pub fn confch8(&self) -> CONFCH8_R {
        CONFCH8_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFCHR2")
            .field("confch8", &self.confch8())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Channel 8 configuration
    #[inline(always)]
    pub fn confch8(&mut self) -> CONFCH8_W<'_, CONFCHR2rs> {
        CONFCH8_W::new(self, 0)
    }
}
/**channel configuration register 2

You can [`read`](crate::Reg::read) this register and get [`confchr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confchr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F373.html#SDADC1:CONFCHR2)*/
pub struct CONFCHR2rs;
impl crate::RegisterSpec for CONFCHR2rs {
    type Ux = u32;
}
///`read()` method returns [`confchr2::R`](R) reader structure
impl crate::Readable for CONFCHR2rs {}
///`write(|w| ..)` method takes [`confchr2::W`](W) writer structure
impl crate::Writable for CONFCHR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFCHR2 to value 0
impl crate::Resettable for CONFCHR2rs {}
