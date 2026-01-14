///Register `KEYR5` reader
pub type R = crate::R<KEYR5rs>;
///Register `KEYR5` writer
pub type W = crate::W<KEYR5rs>;
///Field `KEYR5` reader - Data output register
pub type KEYR5_R = crate::FieldReader<u32>;
///Field `KEYR5` writer - Data output register
pub type KEYR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr5(&self) -> KEYR5_R {
        KEYR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR5")
            .field("keyr5", &self.keyr5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn keyr5(&mut self) -> KEYR5_W<'_, KEYR5rs> {
        KEYR5_W::new(self, 0)
    }
}
/**key registers

You can [`read`](crate::Reg::read) this register and get [`keyr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F733.html#CRYP:KEYR5)*/
pub struct KEYR5rs;
impl crate::RegisterSpec for KEYR5rs {
    type Ux = u32;
}
///`read()` method returns [`keyr5::R`](R) reader structure
impl crate::Readable for KEYR5rs {}
///`write(|w| ..)` method takes [`keyr5::W`](W) writer structure
impl crate::Writable for KEYR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR5 to value 0
impl crate::Resettable for KEYR5rs {}
