///Register `KEYR4` reader
pub type R = crate::R<KEYR4rs>;
///Register `KEYR4` writer
pub type W = crate::W<KEYR4rs>;
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
        f.debug_struct("KEYR4").field("key", &self.key()).finish()
    }
}
impl W {
    ///Bits 0:31 - AES key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR4rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 4

You can [`read`](crate::Reg::read) this register and get [`keyr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#AES:KEYR4)*/
pub struct KEYR4rs;
impl crate::RegisterSpec for KEYR4rs {
    type Ux = u32;
}
///`read()` method returns [`keyr4::R`](R) reader structure
impl crate::Readable for KEYR4rs {}
///`write(|w| ..)` method takes [`keyr4::W`](W) writer structure
impl crate::Writable for KEYR4rs {
    type Safety = crate::Safe;
}
///`reset()` method sets KEYR4 to value 0
impl crate::Resettable for KEYR4rs {}
