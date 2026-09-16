///Register `DEBUGCMDREG` reader
pub type R = crate::R<DEBUGCMDREGrs>;
///Register `DEBUGCMDREG` writer
pub type W = crate::W<DEBUGCMDREGrs>;
///Field `CLEARDEBUGINT` reader - CLEARDEBUGINT
pub type CLEARDEBUGINT_R = crate::BitReader;
///Field `CLEARDEBUGINT` writer - CLEARDEBUGINT
pub type CLEARDEBUGINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQDEBUGMODE` reader - SEQDEBUGMODE
pub type SEQDEBUGMODE_R = crate::BitReader;
///Field `SEQDEBUGMODE` writer - SEQDEBUGMODE
pub type SEQDEBUGMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQDEBUGBUSSEL` reader - SEQDEBUGBUSSEL
pub type SEQDEBUGBUSSEL_R = crate::FieldReader;
///Field `SEQDEBUGBUSSEL` writer - SEQDEBUGBUSSEL
pub type SEQDEBUGBUSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `AESDEBUGMODE` reader - AESDEBUGMODE
pub type AESDEBUGMODE_R = crate::FieldReader;
///Field `AESDEBUGMODE` writer - AESDEBUGMODE
pub type AESDEBUGMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - CLEARDEBUGINT
    #[inline(always)]
    pub fn cleardebugint(&self) -> CLEARDEBUGINT_R {
        CLEARDEBUGINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SEQDEBUGMODE
    #[inline(always)]
    pub fn seqdebugmode(&self) -> SEQDEBUGMODE_R {
        SEQDEBUGMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - SEQDEBUGBUSSEL
    #[inline(always)]
    pub fn seqdebugbussel(&self) -> SEQDEBUGBUSSEL_R {
        SEQDEBUGBUSSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 16:19 - AESDEBUGMODE
    #[inline(always)]
    pub fn aesdebugmode(&self) -> AESDEBUGMODE_R {
        AESDEBUGMODE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUGCMDREG")
            .field("cleardebugint", &self.cleardebugint())
            .field("seqdebugmode", &self.seqdebugmode())
            .field("seqdebugbussel", &self.seqdebugbussel())
            .field("aesdebugmode", &self.aesdebugmode())
            .finish()
    }
}
impl W {
    ///Bit 0 - CLEARDEBUGINT
    #[inline(always)]
    pub fn cleardebugint(&mut self) -> CLEARDEBUGINT_W<'_, DEBUGCMDREGrs> {
        CLEARDEBUGINT_W::new(self, 0)
    }
    ///Bit 1 - SEQDEBUGMODE
    #[inline(always)]
    pub fn seqdebugmode(&mut self) -> SEQDEBUGMODE_W<'_, DEBUGCMDREGrs> {
        SEQDEBUGMODE_W::new(self, 1)
    }
    ///Bits 2:5 - SEQDEBUGBUSSEL
    #[inline(always)]
    pub fn seqdebugbussel(&mut self) -> SEQDEBUGBUSSEL_W<'_, DEBUGCMDREGrs> {
        SEQDEBUGBUSSEL_W::new(self, 2)
    }
    ///Bits 16:19 - AESDEBUGMODE
    #[inline(always)]
    pub fn aesdebugmode(&mut self) -> AESDEBUGMODE_W<'_, DEBUGCMDREGrs> {
        AESDEBUGMODE_W::new(self, 16)
    }
}
/**DebugCmd register

You can [`read`](crate::Reg::read) this register and get [`debugcmdreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugcmdreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#BLUE:DEBUGCMDREG)*/
pub struct DEBUGCMDREGrs;
impl crate::RegisterSpec for DEBUGCMDREGrs {
    type Ux = u32;
}
///`read()` method returns [`debugcmdreg::R`](R) reader structure
impl crate::Readable for DEBUGCMDREGrs {}
///`write(|w| ..)` method takes [`debugcmdreg::W`](W) writer structure
impl crate::Writable for DEBUGCMDREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEBUGCMDREG to value 0
impl crate::Resettable for DEBUGCMDREGrs {}
