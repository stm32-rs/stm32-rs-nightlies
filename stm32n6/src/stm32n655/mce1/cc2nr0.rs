///Register `CC2NR0` reader
pub type R = crate::R<CC2NR0rs>;
///Register `CC2NR0` writer
pub type W = crate::W<CC2NR0rs>;
///Field `SCNONCE` reader - Stream cipher nonce, bits \[31:0\]
pub type SCNONCE_R = crate::FieldReader<u32>;
///Field `SCNONCE` writer - Stream cipher nonce, bits \[31:0\]
pub type SCNONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Stream cipher nonce, bits \[31:0\]
    #[inline(always)]
    pub fn scnonce(&self) -> SCNONCE_R {
        SCNONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC2NR0")
            .field("scnonce", &self.scnonce())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Stream cipher nonce, bits \[31:0\]
    #[inline(always)]
    pub fn scnonce(&mut self) -> SCNONCE_W<'_, CC2NR0rs> {
        SCNONCE_W::new(self, 0)
    }
}
/**MCE cipher context 2 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`cc2nr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2nr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:CC2NR0)*/
pub struct CC2NR0rs;
impl crate::RegisterSpec for CC2NR0rs {
    type Ux = u32;
}
///`read()` method returns [`cc2nr0::R`](R) reader structure
impl crate::Readable for CC2NR0rs {}
///`write(|w| ..)` method takes [`cc2nr0::W`](W) writer structure
impl crate::Writable for CC2NR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CC2NR0 to value 0
impl crate::Resettable for CC2NR0rs {}
