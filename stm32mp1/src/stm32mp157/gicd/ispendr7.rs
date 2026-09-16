///Register `ISPENDR7` reader
pub type R = crate::R<ISPENDR7rs>;
///Register `ISPENDR7` writer
pub type W = crate::W<ISPENDR7rs>;
///Field `ISPENDR7` reader - ISPENDR7
pub type ISPENDR7_R = crate::FieldReader<u32>;
///Field `ISPENDR7` writer - ISPENDR7
pub type ISPENDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR7
    #[inline(always)]
    pub fn ispendr7(&self) -> ISPENDR7_R {
        ISPENDR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR7")
            .field("ispendr7", &self.ispendr7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR7
    #[inline(always)]
    pub fn ispendr7(&mut self) -> ISPENDR7_W<'_, ISPENDR7rs> {
        ISPENDR7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISPENDR7)*/
pub struct ISPENDR7rs;
impl crate::RegisterSpec for ISPENDR7rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr7::R`](R) reader structure
impl crate::Readable for ISPENDR7rs {}
///`write(|w| ..)` method takes [`ispendr7::W`](W) writer structure
impl crate::Writable for ISPENDR7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR7 to value 0
impl crate::Resettable for ISPENDR7rs {}
