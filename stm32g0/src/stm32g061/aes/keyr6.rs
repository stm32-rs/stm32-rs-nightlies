///Register `KEYR6` reader
pub type R = crate::R<KEYR6rs>;
///Register `KEYR6` writer
pub type W = crate::W<KEYR6rs>;
///Field `KEY` reader - Cryptographic key, bits \[223:192\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield.
pub type KEY_R = crate::FieldReader<u32>;
///Field `KEY` writer - Cryptographic key, bits \[223:192\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield.
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Cryptographic key, bits \[223:192\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield.
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR6").field("key", &self.key()).finish()
    }
}
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[223:192\] Refer to the AES_KEYR0 register for description of the KEY\[255:0\] bitfield.
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<KEYR6rs> {
        KEY_W::new(self, 0)
    }
}
/**AES key register 6

You can [`read`](crate::Reg::read) this register and get [`keyr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G061.html#AES:KEYR6)*/
pub struct KEYR6rs;
impl crate::RegisterSpec for KEYR6rs {
    type Ux = u32;
}
///`read()` method returns [`keyr6::R`](R) reader structure
impl crate::Readable for KEYR6rs {}
///`write(|w| ..)` method takes [`keyr6::W`](W) writer structure
impl crate::Writable for KEYR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR6 to value 0
impl crate::Resettable for KEYR6rs {}
