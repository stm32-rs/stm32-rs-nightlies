///Register `MANAESCLEARTEXT2REG` reader
pub type R = crate::R<MANAESCLEARTEXT2REGrs>;
///Register `MANAESCLEARTEXT2REG` writer
pub type W = crate::W<MANAESCLEARTEXT2REGrs>;
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
        f.debug_struct("MANAESCLEARTEXT2REG")
            .field("aes", &self.aes())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Manual Aes Clear Text
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W<'_, MANAESCLEARTEXT2REGrs> {
        AES_W::new(self, 0)
    }
}
/**MANAESCLEARTEXT2REG register

You can [`read`](crate::Reg::read) this register and get [`manaescleartext2reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaescleartext2reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#BLUE:MANAESCLEARTEXT2REG)*/
pub struct MANAESCLEARTEXT2REGrs;
impl crate::RegisterSpec for MANAESCLEARTEXT2REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaescleartext2reg::R`](R) reader structure
impl crate::Readable for MANAESCLEARTEXT2REGrs {}
///`write(|w| ..)` method takes [`manaescleartext2reg::W`](W) writer structure
impl crate::Writable for MANAESCLEARTEXT2REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESCLEARTEXT2REG to value 0
impl crate::Resettable for MANAESCLEARTEXT2REGrs {}
