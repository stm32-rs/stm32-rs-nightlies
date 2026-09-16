///Register `ISACTIVER8` reader
pub type R = crate::R<ISACTIVER8rs>;
///Register `ISACTIVER8` writer
pub type W = crate::W<ISACTIVER8rs>;
///Field `ISACTIVER8` reader - ISACTIVER8
pub type ISACTIVER8_R = crate::FieldReader<u32>;
///Field `ISACTIVER8` writer - ISACTIVER8
pub type ISACTIVER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER8
    #[inline(always)]
    pub fn isactiver8(&self) -> ISACTIVER8_R {
        ISACTIVER8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER8")
            .field("isactiver8", &self.isactiver8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER8
    #[inline(always)]
    pub fn isactiver8(&mut self) -> ISACTIVER8_W<'_, ISACTIVER8rs> {
        ISACTIVER8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER8)*/
pub struct ISACTIVER8rs;
impl crate::RegisterSpec for ISACTIVER8rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver8::R`](R) reader structure
impl crate::Readable for ISACTIVER8rs {}
///`write(|w| ..)` method takes [`isactiver8::W`](W) writer structure
impl crate::Writable for ISACTIVER8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER8 to value 0
impl crate::Resettable for ISACTIVER8rs {}
