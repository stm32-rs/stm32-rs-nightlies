///Register `KEYR5` reader
pub type R = crate::R<KEYR5rs>;
///Register `KEYR5` writer
pub type W = crate::W<KEYR5rs>;
///Field `KEY` reader - AES key
pub type KEY_R = crate::FieldReader<u32>;
///Field `KEY` writer - AES key
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - AES key
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR5").field("key", &self.key()).finish()
    }
}
impl W {
    ///Bits 0:31 - AES key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR5rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 5

You can [`read`](crate::Reg::read) this register and get [`keyr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#AES:KEYR5)*/
pub struct KEYR5rs;
impl crate::RegisterSpec for KEYR5rs {
    type Ux = u32;
}
///`read()` method returns [`keyr5::R`](R) reader structure
impl crate::Readable for KEYR5rs {}
///`write(|w| ..)` method takes [`keyr5::W`](W) writer structure
impl crate::Writable for KEYR5rs {
    type Safety = crate::Safe;
}
///`reset()` method sets KEYR5 to value 0
impl crate::Resettable for KEYR5rs {}
