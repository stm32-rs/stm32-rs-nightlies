///Register `VOSCR` reader
pub type R = crate::R<VOSCRrs>;
///Register `VOSCR` writer
pub type W = crate::W<VOSCRrs>;
///Field `VOS` reader - Voltage scaling selection according to performance
pub type VOS_R = crate::BitReader;
///Field `VOS` writer - Voltage scaling selection according to performance
pub type VOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOSRDY` reader - VOS ready bit for V less than sub>CORE less than /sub> voltage scaling output selection
pub type VOSRDY_R = crate::BitReader;
///Field `ACTVOS` reader - VOS currently applied for V less than sub>CORE less than /sub> voltage scaling selection
pub type ACTVOS_R = crate::BitReader;
///Field `ACTVOSRDY` reader - Voltage level ready bit for currently used ACTVOS
pub type ACTVOSRDY_R = crate::BitReader;
impl R {
    ///Bit 0 - Voltage scaling selection according to performance
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VOS ready bit for V less than sub>CORE less than /sub> voltage scaling output selection
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - VOS currently applied for V less than sub>CORE less than /sub> voltage scaling selection
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Voltage level ready bit for currently used ACTVOS
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOSCR")
            .field("vos", &self.vos())
            .field("vosrdy", &self.vosrdy())
            .field("actvos", &self.actvos())
            .field("actvosrdy", &self.actvosrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Voltage scaling selection according to performance
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, VOSCRrs> {
        VOS_W::new(self, 0)
    }
}
/**PWR voltage scaling control register

You can [`read`](crate::Reg::read) this register and get [`voscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#PWR:VOSCR)*/
pub struct VOSCRrs;
impl crate::RegisterSpec for VOSCRrs {
    type Ux = u32;
}
///`read()` method returns [`voscr::R`](R) reader structure
impl crate::Readable for VOSCRrs {}
///`write(|w| ..)` method takes [`voscr::W`](W) writer structure
impl crate::Writable for VOSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VOSCR to value 0x0002_0002
impl crate::Resettable for VOSCRrs {
    const RESET_VALUE: u32 = 0x0002_0002;
}
