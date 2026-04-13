///Register `HWCFGR6` reader
pub type R = crate::R<HWCFGR6rs>;
///Register `HWCFGR6` writer
pub type W = crate::W<HWCFGR6rs>;
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
        f.debug_struct("HWCFGR6")
            .field("cpuevent", &self.cpuevent())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HW configuration CPU event generation
    #[inline(always)]
    pub fn cpuevent(&mut self) -> CPUEVENT_W<'_, HWCFGR6rs> {
        CPUEVENT_W::new(self, 0)
    }
}
/**Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#EXTI:HWCFGR6)*/
pub struct HWCFGR6rs;
impl crate::RegisterSpec for HWCFGR6rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr6::R`](R) reader structure
impl crate::Readable for HWCFGR6rs {}
///`write(|w| ..)` method takes [`hwcfgr6::W`](W) writer structure
impl crate::Writable for HWCFGR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR6 to value 0x03
impl crate::Resettable for HWCFGR6rs {
    const RESET_VALUE: u32 = 0x03;
}
