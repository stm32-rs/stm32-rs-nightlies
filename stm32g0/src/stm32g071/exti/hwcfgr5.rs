///Register `HWCFGR5` reader
pub type R = crate::R<HWCFGR5rs>;
///Register `HWCFGR5` writer
pub type W = crate::W<HWCFGR5rs>;
///Field `CPUEVENT` reader - HW configuration CPU event generation
pub type CPUEVENT_R = crate::FieldReader<u32>;
///Field `CPUEVENT` writer - HW configuration CPU event generation
pub type CPUEVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HW configuration CPU event generation
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR5")
            .field("cpuevent", &self.cpuevent())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HW configuration CPU event generation
    #[inline(always)]
    pub fn cpuevent(&mut self) -> CPUEVENT_W<'_, HWCFGR5rs> {
        CPUEVENT_W::new(self, 0)
    }
}
/**Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#EXTI:HWCFGR5)*/
pub struct HWCFGR5rs;
impl crate::RegisterSpec for HWCFGR5rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr5::R`](R) reader structure
impl crate::Readable for HWCFGR5rs {}
///`write(|w| ..)` method takes [`hwcfgr5::W`](W) writer structure
impl crate::Writable for HWCFGR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR5 to value 0xfeaf_ffff
impl crate::Resettable for HWCFGR5rs {
    const RESET_VALUE: u32 = 0xfeaf_ffff;
}
