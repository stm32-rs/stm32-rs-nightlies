///Register `AXISRAM6CR` reader
pub type R = crate::R<AXISRAM6CRrs>;
///Register `AXISRAM6CR` writer
pub type W = crate::W<AXISRAM6CRrs>;
///Field `SRAMER` reader - SRAM erase
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMSD` reader - Shutdown AXISRAMx
pub type SRAMSD_R = crate::BitReader;
///Field `SRAMSD` writer - Shutdown AXISRAMx
pub type SRAMSD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 20 - Shutdown AXISRAMx
    #[inline(always)]
    pub fn sramsd(&self) -> SRAMSD_R {
        SRAMSD_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXISRAM6CR")
            .field("sramer", &self.sramer())
            .field("sramsd", &self.sramsd())
            .finish()
    }
}
impl W {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, AXISRAM6CRrs> {
        SRAMER_W::new(self, 8)
    }
    ///Bit 20 - Shutdown AXISRAMx
    #[inline(always)]
    pub fn sramsd(&mut self) -> SRAMSD_W<'_, AXISRAM6CRrs> {
        SRAMSD_W::new(self, 20)
    }
}
/**RAMCFG AXISRAM6 control register

You can [`read`](crate::Reg::read) this register and get [`axisram6cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram6cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM6CR)*/
pub struct AXISRAM6CRrs;
impl crate::RegisterSpec for AXISRAM6CRrs {
    type Ux = u32;
}
///`read()` method returns [`axisram6cr::R`](R) reader structure
impl crate::Readable for AXISRAM6CRrs {}
///`write(|w| ..)` method takes [`axisram6cr::W`](W) writer structure
impl crate::Writable for AXISRAM6CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AXISRAM6CR to value 0
impl crate::Resettable for AXISRAM6CRrs {}
