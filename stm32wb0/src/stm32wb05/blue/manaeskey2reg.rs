///Register `MANAESKEY2REG` reader
pub type R = crate::R<MANAESKEY2REGrs>;
///Register `MANAESKEY2REG` writer
pub type W = crate::W<MANAESKEY2REGrs>;
///Field `MANAESKEY_95_64` reader - Manual mode AES key
pub type MANAESKEY_95_64_R = crate::FieldReader<u32>;
///Field `MANAESKEY_95_64` writer - Manual mode AES key
pub type MANAESKEY_95_64_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_95_64(&self) -> MANAESKEY_95_64_R {
        MANAESKEY_95_64_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESKEY2REG")
            .field("manaeskey_95_64", &self.manaeskey_95_64())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_95_64(&mut self) -> MANAESKEY_95_64_W<'_, MANAESKEY2REGrs> {
        MANAESKEY_95_64_W::new(self, 0)
    }
}
/**MANAESKEY2REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey2reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey2reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:MANAESKEY2REG)*/
pub struct MANAESKEY2REGrs;
impl crate::RegisterSpec for MANAESKEY2REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaeskey2reg::R`](R) reader structure
impl crate::Readable for MANAESKEY2REGrs {}
///`write(|w| ..)` method takes [`manaeskey2reg::W`](W) writer structure
impl crate::Writable for MANAESKEY2REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESKEY2REG to value 0
impl crate::Resettable for MANAESKEY2REGrs {}
