///Register `POSTP_CR` reader
pub type R = crate::R<POSTP_CRrs>;
///Register `POSTP_CR` writer
pub type W = crate::W<POSTP_CRrs>;
///Field `AES_RESET` reader - Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests.
pub type AES_RESET_R = crate::BitReader;
///Field `AES_RESET` writer - Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests.
pub type AES_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NB_LOOP_AES` reader - NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed.
pub type NB_LOOP_AES_R = crate::FieldReader;
///Field `NB_LOOP_AES` writer - NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed.
pub type NB_LOOP_AES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `NB_RND_REINIT` reader - Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized.
pub type NB_RND_REINIT_R = crate::FieldReader<u16>;
///Field `NB_RND_REINIT` writer - Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized.
pub type NB_RND_REINIT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests.
    #[inline(always)]
    pub fn aes_reset(&self) -> AES_RESET_R {
        AES_RESET_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed.
    #[inline(always)]
    pub fn nb_loop_aes(&self) -> NB_LOOP_AES_R {
        NB_LOOP_AES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized.
    #[inline(always)]
    pub fn nb_rnd_reinit(&self) -> NB_RND_REINIT_R {
        NB_RND_REINIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POSTP_CR")
            .field("aes_reset", &self.aes_reset())
            .field("nb_loop_aes", &self.nb_loop_aes())
            .field("nb_rnd_reinit", &self.nb_rnd_reinit())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reset AES post processing. When writing a 1, the AES post processing is reinitialized, resulting in a new key and new state generation before 128-bit random words generation. The '1' written is frozen until it is seen by RNG core clock domain after resynchronization. Then it is automatically reset. It also reruns analog source health tests.
    #[inline(always)]
    pub fn aes_reset(&mut self) -> AES_RESET_W<'_, POSTP_CRrs> {
        AES_RESET_W::new(self, 0)
    }
    ///Bits 8:11 - NB_LOOP_AES is the number of 128-bit words got from the noise source that have to be processed by AES for generating a single 128-bit random word. By default, this value is set to 2 (128 bits generated before an AES processing). 0 value means 16 loops. A new AES processing is started only when the previous one is completed.
    #[inline(always)]
    pub fn nb_loop_aes(&mut self) -> NB_LOOP_AES_W<'_, POSTP_CRrs> {
        NB_LOOP_AES_W::new(self, 8)
    }
    ///Bits 16:31 - Number of 128-bit random words generated before AES automatically resets. This number is in the range of 1 to 65535 words. Value 0x0000 means that AES is never reinitialized.
    #[inline(always)]
    pub fn nb_rnd_reinit(&mut self) -> NB_RND_REINIT_W<'_, POSTP_CRrs> {
        NB_RND_REINIT_W::new(self, 16)
    }
}
/**TRNG_POSTP_CR register

You can [`read`](crate::Reg::read) this register and get [`postp_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`postp_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:POSTP_CR)*/
pub struct POSTP_CRrs;
impl crate::RegisterSpec for POSTP_CRrs {
    type Ux = u32;
}
///`read()` method returns [`postp_cr::R`](R) reader structure
impl crate::Readable for POSTP_CRrs {}
///`write(|w| ..)` method takes [`postp_cr::W`](W) writer structure
impl crate::Writable for POSTP_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POSTP_CR to value 0x0f00
impl crate::Resettable for POSTP_CRrs {
    const RESET_VALUE: u32 = 0x0f00;
}
