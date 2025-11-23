///Register `SPECULATION_CTRL` reader
pub type R = crate::R<SPECULATION_CTRLrs>;
///Register `SPECULATION_CTRL` writer
pub type W = crate::W<SPECULATION_CTRLrs>;
///Field `READSPEC_DISABLE` reader - READSPEC_DISABLE
pub type READSPEC_DISABLE_R = crate::BitReader;
///Field `READSPEC_DISABLE` writer - READSPEC_DISABLE
pub type READSPEC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITESPEC_DISABLE` reader - WRITESPEC_DISABLE
pub type WRITESPEC_DISABLE_R = crate::BitReader;
///Field `WRITESPEC_DISABLE` writer - WRITESPEC_DISABLE
pub type WRITESPEC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - READSPEC_DISABLE
    #[inline(always)]
    pub fn readspec_disable(&self) -> READSPEC_DISABLE_R {
        READSPEC_DISABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WRITESPEC_DISABLE
    #[inline(always)]
    pub fn writespec_disable(&self) -> WRITESPEC_DISABLE_R {
        WRITESPEC_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPECULATION_CTRL")
            .field("readspec_disable", &self.readspec_disable())
            .field("writespec_disable", &self.writespec_disable())
            .finish()
    }
}
impl W {
    ///Bit 0 - READSPEC_DISABLE
    #[inline(always)]
    pub fn readspec_disable(&mut self) -> READSPEC_DISABLE_W<'_, SPECULATION_CTRLrs> {
        READSPEC_DISABLE_W::new(self, 0)
    }
    ///Bit 1 - WRITESPEC_DISABLE
    #[inline(always)]
    pub fn writespec_disable(&mut self) -> WRITESPEC_DISABLE_W<'_, SPECULATION_CTRLrs> {
        WRITESPEC_DISABLE_W::new(self, 1)
    }
}
/**Controls read and write access speculation.

You can [`read`](crate::Reg::read) this register and get [`speculation_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`speculation_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:SPECULATION_CTRL)*/
pub struct SPECULATION_CTRLrs;
impl crate::RegisterSpec for SPECULATION_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`speculation_ctrl::R`](R) reader structure
impl crate::Readable for SPECULATION_CTRLrs {}
///`write(|w| ..)` method takes [`speculation_ctrl::W`](W) writer structure
impl crate::Writable for SPECULATION_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPECULATION_CTRL to value 0
impl crate::Resettable for SPECULATION_CTRLrs {}
