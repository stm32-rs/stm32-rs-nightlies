///Register `CC1NR1` reader
pub type R = crate::R<CC1NR1rs>;
///Register `CC1NR1` writer
pub type W = crate::W<CC1NR1rs>;
///Field `SCNONCE` reader - Stream cipher nonce, bits \[63:32\]
pub type SCNONCE_R = crate::FieldReader<u32>;
///Field `SCNONCE` writer - Stream cipher nonce, bits \[63:32\]
pub type SCNONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stream cipher nonce, bits \[63:32\]
    #[inline(always)]
    pub fn scnonce(&self) -> SCNONCE_R {
        SCNONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC1NR1")
            .field("scnonce", &self.scnonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stream cipher nonce, bits \[63:32\]
    #[inline(always)]
    pub fn scnonce(&mut self) -> SCNONCE_W<'_, CC1NR1rs> {
        SCNONCE_W::new(self, 0)
    }
}
/**MCE cipher context 1 nonce register 1

You can [`read`](crate::Reg::read) this register and get [`cc1nr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1nr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:CC1NR1)*/
pub struct CC1NR1rs;
impl crate::RegisterSpec for CC1NR1rs {
    type Ux = u32;
}
///`read()` method returns [`cc1nr1::R`](R) reader structure
impl crate::Readable for CC1NR1rs {}
///`write(|w| ..)` method takes [`cc1nr1::W`](W) writer structure
impl crate::Writable for CC1NR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CC1NR1 to value 0
impl crate::Resettable for CC1NR1rs {}
