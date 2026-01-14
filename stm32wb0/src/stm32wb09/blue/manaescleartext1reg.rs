///Register `MANAESCLEARTEXT1REG` reader
pub type R = crate::R<MANAESCLEARTEXT1REGrs>;
///Register `MANAESCLEARTEXT1REG` writer
pub type W = crate::W<MANAESCLEARTEXT1REGrs>;
///Field `AES` reader - Manual Aes Clear Text
pub type AES_R = crate::FieldReader<u32>;
///Field `AES` writer - Manual Aes Clear Text
pub type AES_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Manual Aes Clear Text
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESCLEARTEXT1REG")
            .field("aes", &self.aes())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Manual Aes Clear Text
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W<'_, MANAESCLEARTEXT1REGrs> {
        AES_W::new(self, 0)
    }
}
/**MANAESCLEARTEXT1REG register

You can [`read`](crate::Reg::read) this register and get [`manaescleartext1reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescleartext1reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#BLUE:MANAESCLEARTEXT1REG)*/
pub struct MANAESCLEARTEXT1REGrs;
impl crate::RegisterSpec for MANAESCLEARTEXT1REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaescleartext1reg::R`](R) reader structure
impl crate::Readable for MANAESCLEARTEXT1REGrs {}
///`write(|w| ..)` method takes [`manaescleartext1reg::W`](W) writer structure
impl crate::Writable for MANAESCLEARTEXT1REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESCLEARTEXT1REG to value 0
impl crate::Resettable for MANAESCLEARTEXT1REGrs {}
