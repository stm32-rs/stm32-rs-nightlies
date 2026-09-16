///Register `KEYR0` reader
pub type R = crate::R<KEYR0rs>;
///Register `KEYR0` writer
pub type W = crate::W<KEYR0rs>;
///Field `KEY` reader - Data Output Register (LSB key \[31:0\])
pub type KEY_R = crate::FieldReader<u32>;
///Field `KEY` writer - Data Output Register (LSB key \[31:0\])
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Data Output Register (LSB key \[31:0\])
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR0").field("key", &self.key()).finish()
    }
}
impl W {
    ///Bits 0:31 - Data Output Register (LSB key \[31:0\])
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYR0rs> {
        KEY_W::new(self, 0)
    }
}
/**key register 0

You can [`read`](crate::Reg::read) this register and get [`keyr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#AES:KEYR0)*/
pub struct KEYR0rs;
impl crate::RegisterSpec for KEYR0rs {
    type Ux = u32;
}
///`read()` method returns [`keyr0::R`](R) reader structure
impl crate::Readable for KEYR0rs {}
///`write(|w| ..)` method takes [`keyr0::W`](W) writer structure
impl crate::Writable for KEYR0rs {
    type Safety = crate::Safe;
}
///`reset()` method sets KEYR0 to value 0
impl crate::Resettable for KEYR0rs {}
