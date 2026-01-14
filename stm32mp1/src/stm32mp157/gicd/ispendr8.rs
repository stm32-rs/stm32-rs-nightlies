///Register `ISPENDR8` reader
pub type R = crate::R<ISPENDR8rs>;
///Register `ISPENDR8` writer
pub type W = crate::W<ISPENDR8rs>;
///Field `ISPENDR8` reader - ISPENDR8
pub type ISPENDR8_R = crate::FieldReader<u32>;
///Field `ISPENDR8` writer - ISPENDR8
pub type ISPENDR8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR8
    #[inline(always)]
    pub fn ispendr8(&self) -> ISPENDR8_R {
        ISPENDR8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR8")
            .field("ispendr8", &self.ispendr8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR8
    #[inline(always)]
    pub fn ispendr8(&mut self) -> ISPENDR8_W<'_, ISPENDR8rs> {
        ISPENDR8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR8)*/
pub struct ISPENDR8rs;
impl crate::RegisterSpec for ISPENDR8rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr8::R`](R) reader structure
impl crate::Readable for ISPENDR8rs {}
///`write(|w| ..)` method takes [`ispendr8::W`](W) writer structure
impl crate::Writable for ISPENDR8rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR8 to value 0
impl crate::Resettable for ISPENDR8rs {}
