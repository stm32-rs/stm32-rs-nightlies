///Register `INTERRUPT2REG` reader
pub type R = crate::R<INTERRUPT2REGrs>;
///Register `INTERRUPT2REG` writer
pub type W = crate::W<INTERRUPT2REGrs>;
///Field `AESMANENCINT` reader - AES manual encryption.
pub type AESMANENCINT_R = crate::BitReader;
///Field `AESMANENCINT` writer - AES manual encryption.
pub type AESMANENCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESLEPRIVINT` reader - AES LE privacy engine.
pub type AESLEPRIVINT_R = crate::BitReader;
///Field `AESLEPRIVINT` writer - AES LE privacy engine.
pub type AESLEPRIVINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES manual encryption.
    #[inline(always)]
    pub fn aesmanencint(&self) -> AESMANENCINT_R {
        AESMANENCINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES LE privacy engine.
    #[inline(always)]
    pub fn aesleprivint(&self) -> AESLEPRIVINT_R {
        AESLEPRIVINT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT2REG")
            .field("aesmanencint", &self.aesmanencint())
            .field("aesleprivint", &self.aesleprivint())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES manual encryption.
    #[inline(always)]
    pub fn aesmanencint(&mut self) -> AESMANENCINT_W<'_, INTERRUPT2REGrs> {
        AESMANENCINT_W::new(self, 0)
    }
    ///Bit 1 - AES LE privacy engine.
    #[inline(always)]
    pub fn aesleprivint(&mut self) -> AESLEPRIVINT_W<'_, INTERRUPT2REGrs> {
        AESLEPRIVINT_W::new(self, 1)
    }
}
/**INTERRUPT2REG register

You can [`read`](crate::Reg::read) this register and get [`interrupt2reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt2reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#BLUE:INTERRUPT2REG)*/
pub struct INTERRUPT2REGrs;
impl crate::RegisterSpec for INTERRUPT2REGrs {
    type Ux = u32;
}
///`read()` method returns [`interrupt2reg::R`](R) reader structure
impl crate::Readable for INTERRUPT2REGrs {}
///`write(|w| ..)` method takes [`interrupt2reg::W`](W) writer structure
impl crate::Writable for INTERRUPT2REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTERRUPT2REG to value 0
impl crate::Resettable for INTERRUPT2REGrs {}
