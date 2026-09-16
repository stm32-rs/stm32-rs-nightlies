///Register `TZMA0_SIZE` reader
pub type R = crate::R<TZMA0_SIZErs>;
///Register `TZMA0_SIZE` writer
pub type W = crate::W<TZMA0_SIZErs>;
///Field `R0SIZE` reader - R0SIZE
pub type R0SIZE_R = crate::FieldReader<u16>;
///Field `R0SIZE` writer - R0SIZE
pub type R0SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - R0SIZE
    #[inline(always)]
    pub fn r0size(&self) -> R0SIZE_R {
        R0SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZMA0_SIZE")
            .field("r0size", &self.r0size())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - R0SIZE
    #[inline(always)]
    pub fn r0size(&mut self) -> R0SIZE_W<'_, TZMA0_SIZErs> {
        R0SIZE_W::new(self, 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, TZMA0_SIZErs> {
        LOCK_W::new(self, 31)
    }
}
/**ETZPC ROM secure size definition

You can [`read`](crate::Reg::read) this register and get [`tzma0_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzma0_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETZPC:TZMA0_SIZE)*/
pub struct TZMA0_SIZErs;
impl crate::RegisterSpec for TZMA0_SIZErs {
    type Ux = u32;
}
///`read()` method returns [`tzma0_size::R`](R) reader structure
impl crate::Readable for TZMA0_SIZErs {}
///`write(|w| ..)` method takes [`tzma0_size::W`](W) writer structure
impl crate::Writable for TZMA0_SIZErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZMA0_SIZE to value 0x03ff
impl crate::Resettable for TZMA0_SIZErs {
    const RESET_VALUE: u32 = 0x03ff;
}
