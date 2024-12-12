///Register `KEYR6` reader
pub type R = crate::R<KEYR6rs>;
///Register `KEYR6` writer
pub type W = crate::W<KEYR6rs>;
///Field `AES_KEYR6` reader - AES key register (MSB key \[223:192\])
pub type AES_KEYR6_R = crate::FieldReader<u32>;
///Field `AES_KEYR6` writer - AES key register (MSB key \[223:192\])
pub type AES_KEYR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - AES key register (MSB key \[223:192\])
    #[inline(always)]
    pub fn aes_keyr6(&self) -> AES_KEYR6_R {
        AES_KEYR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR6")
            .field("aes_keyr6", &self.aes_keyr6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - AES key register (MSB key \[223:192\])
    #[inline(always)]
    pub fn aes_keyr6(&mut self) -> AES_KEYR6_W<KEYR6rs> {
        AES_KEYR6_W::new(self, 0)
    }
}
/**key register 6

You can [`read`](crate::Reg::read) this register and get [`keyr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F7x3.html#AES:KEYR6)*/
pub struct KEYR6rs;
impl crate::RegisterSpec for KEYR6rs {
    type Ux = u32;
}
///`read()` method returns [`keyr6::R`](R) reader structure
impl crate::Readable for KEYR6rs {}
///`write(|w| ..)` method takes [`keyr6::W`](W) writer structure
impl crate::Writable for KEYR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets KEYR6 to value 0
impl crate::Resettable for KEYR6rs {
    const RESET_VALUE: u32 = 0;
}
