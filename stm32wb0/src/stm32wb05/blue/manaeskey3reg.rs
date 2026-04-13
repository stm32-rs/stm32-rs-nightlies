///Register `MANAESKEY3REG` reader
pub type R = crate::R<MANAESKEY3REGrs>;
///Register `MANAESKEY3REG` writer
pub type W = crate::W<MANAESKEY3REGrs>;
///Field `MANAESKEY_127_96` reader - Manual mode AES key
pub type MANAESKEY_127_96_R = crate::FieldReader<u32>;
///Field `MANAESKEY_127_96` writer - Manual mode AES key
pub type MANAESKEY_127_96_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_127_96(&self) -> MANAESKEY_127_96_R {
        MANAESKEY_127_96_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESKEY3REG")
            .field("manaeskey_127_96", &self.manaeskey_127_96())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_127_96(&mut self) -> MANAESKEY_127_96_W<'_, MANAESKEY3REGrs> {
        MANAESKEY_127_96_W::new(self, 0)
    }
}
/**MANAESKEY3REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey3reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey3reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:MANAESKEY3REG)*/
pub struct MANAESKEY3REGrs;
impl crate::RegisterSpec for MANAESKEY3REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaeskey3reg::R`](R) reader structure
impl crate::Readable for MANAESKEY3REGrs {}
///`write(|w| ..)` method takes [`manaeskey3reg::W`](W) writer structure
impl crate::Writable for MANAESKEY3REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESKEY3REG to value 0
impl crate::Resettable for MANAESKEY3REGrs {}
