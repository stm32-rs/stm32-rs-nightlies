///Register `ISPENDR5` reader
pub type R = crate::R<ISPENDR5rs>;
///Register `ISPENDR5` writer
pub type W = crate::W<ISPENDR5rs>;
///Field `ISPENDR5` reader - ISPENDR5
pub type ISPENDR5_R = crate::FieldReader<u32>;
///Field `ISPENDR5` writer - ISPENDR5
pub type ISPENDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR5
    #[inline(always)]
    pub fn ispendr5(&self) -> ISPENDR5_R {
        ISPENDR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR5")
            .field("ispendr5", &self.ispendr5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR5
    #[inline(always)]
    pub fn ispendr5(&mut self) -> ISPENDR5_W<'_, ISPENDR5rs> {
        ISPENDR5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISPENDR5)*/
pub struct ISPENDR5rs;
impl crate::RegisterSpec for ISPENDR5rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr5::R`](R) reader structure
impl crate::Readable for ISPENDR5rs {}
///`write(|w| ..)` method takes [`ispendr5::W`](W) writer structure
impl crate::Writable for ISPENDR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR5 to value 0
impl crate::Resettable for ISPENDR5rs {}
