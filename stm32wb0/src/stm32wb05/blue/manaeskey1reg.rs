///Register `MANAESKEY1REG` reader
pub type R = crate::R<MANAESKEY1REGrs>;
///Register `MANAESKEY1REG` writer
pub type W = crate::W<MANAESKEY1REGrs>;
///Field `MANAESKEY_63_32` reader - Manual mode AES key
pub type MANAESKEY_63_32_R = crate::FieldReader<u32>;
///Field `MANAESKEY_63_32` writer - Manual mode AES key
pub type MANAESKEY_63_32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_63_32(&self) -> MANAESKEY_63_32_R {
        MANAESKEY_63_32_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESKEY1REG")
            .field("manaeskey_63_32", &self.manaeskey_63_32())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_63_32(&mut self) -> MANAESKEY_63_32_W<'_, MANAESKEY1REGrs> {
        MANAESKEY_63_32_W::new(self, 0)
    }
}
/**MANAESKEY1REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey1reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey1reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:MANAESKEY1REG)*/
pub struct MANAESKEY1REGrs;
impl crate::RegisterSpec for MANAESKEY1REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaeskey1reg::R`](R) reader structure
impl crate::Readable for MANAESKEY1REGrs {}
///`write(|w| ..)` method takes [`manaeskey1reg::W`](W) writer structure
impl crate::Writable for MANAESKEY1REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESKEY1REG to value 0
impl crate::Resettable for MANAESKEY1REGrs {}
