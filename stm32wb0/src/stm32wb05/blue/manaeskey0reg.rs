///Register `MANAESKEY0REG` reader
pub type R = crate::R<MANAESKEY0REGrs>;
///Register `MANAESKEY0REG` writer
pub type W = crate::W<MANAESKEY0REGrs>;
///Field `MANAESKEY_31_0` reader - Manual mode AES key
pub type MANAESKEY_31_0_R = crate::FieldReader<u32>;
///Field `MANAESKEY_31_0` writer - Manual mode AES key
pub type MANAESKEY_31_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_31_0(&self) -> MANAESKEY_31_0_R {
        MANAESKEY_31_0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESKEY0REG")
            .field("manaeskey_31_0", &self.manaeskey_31_0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Manual mode AES key
    #[inline(always)]
    pub fn manaeskey_31_0(&mut self) -> MANAESKEY_31_0_W<'_, MANAESKEY0REGrs> {
        MANAESKEY_31_0_W::new(self, 0)
    }
}
/**MANAESKEY0REG register

You can [`read`](crate::Reg::read) this register and get [`manaeskey0reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`manaeskey0reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:MANAESKEY0REG)*/
pub struct MANAESKEY0REGrs;
impl crate::RegisterSpec for MANAESKEY0REGrs {
    type Ux = u32;
}
///`read()` method returns [`manaeskey0reg::R`](R) reader structure
impl crate::Readable for MANAESKEY0REGrs {}
///`write(|w| ..)` method takes [`manaeskey0reg::W`](W) writer structure
impl crate::Writable for MANAESKEY0REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MANAESKEY0REG to value 0
impl crate::Resettable for MANAESKEY0REGrs {}
