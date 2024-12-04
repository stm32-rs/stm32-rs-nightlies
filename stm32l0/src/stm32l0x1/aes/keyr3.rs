///Register `KEYR3` reader
pub type R = crate::R<KEYR3rs>;
///Register `KEYR3` writer
pub type W = crate::W<KEYR3rs>;
///Field `KEY3` reader - AES key register (MSB key \[127:96\])
pub type KEY3_R = crate::FieldReader<u32>;
///Field `KEY3` writer - AES key register (MSB key \[127:96\])
pub type KEY3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - AES key register (MSB key \[127:96\])
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR3").field("key3", &self.key3()).finish()
    }
}
impl W {
    ///Bits 0:31 - AES key register (MSB key \[127:96\])
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W<KEYR3rs> {
        KEY3_W::new(self, 0)
    }
}
/**key register 3

You can [`read`](crate::Reg::read) this register and get [`keyr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#AES:KEYR3)*/
pub struct KEYR3rs;
impl crate::RegisterSpec for KEYR3rs {
    type Ux = u32;
}
///`read()` method returns [`keyr3::R`](R) reader structure
impl crate::Readable for KEYR3rs {}
///`write(|w| ..)` method takes [`keyr3::W`](W) writer structure
impl crate::Writable for KEYR3rs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets KEYR3 to value 0
impl crate::Resettable for KEYR3rs {
    const RESET_VALUE: u32 = 0;
}
