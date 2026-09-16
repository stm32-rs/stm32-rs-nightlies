///Register `KEYR1` reader
pub type R = crate::R<KEYR1rs>;
///Register `KEYR1` writer
pub type W = crate::W<KEYR1rs>;
///Field `KEY` reader - AES key register (key \[63:32\])
pub type KEY_R = crate::FieldReader<u32>;
///Field `KEY` writer - AES key register (key \[63:32\])
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - AES key register (key \[63:32\])
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR1").field("key", &self.key()).finish()
    }
}
impl W {
    ///Bits 0:31 - AES key register (key \[63:32\])
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR1rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 1

You can [`read`](crate::Reg::read) this register and get [`keyr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#AES:KEYR1)*/
pub struct KEYR1rs;
impl crate::RegisterSpec for KEYR1rs {
    type Ux = u32;
}
///`read()` method returns [`keyr1::R`](R) reader structure
impl crate::Readable for KEYR1rs {}
///`write(|w| ..)` method takes [`keyr1::W`](W) writer structure
impl crate::Writable for KEYR1rs {
    type Safety = crate::Safe;
}
///`reset()` method sets KEYR1 to value 0
impl crate::Resettable for KEYR1rs {}
