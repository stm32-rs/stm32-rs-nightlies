///Register `AXISRAM1CR` reader
pub type R = crate::R<AXISRAM1CRrs>;
///Register `AXISRAM1CR` writer
pub type W = crate::W<AXISRAM1CRrs>;
///Field `SRAMER` reader - SRAM erase.
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase.
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - SRAM erase.
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXISRAM1CR")
            .field("sramer", &self.sramer())
            .finish()
    }
}
impl W {
    ///Bit 8 - SRAM erase.
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, AXISRAM1CRrs> {
        SRAMER_W::new(self, 8)
    }
}
/**RAMCFG AXISRAM1 control register

You can [`read`](crate::Reg::read) this register and get [`axisram1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axisram1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RAMCFG:AXISRAM1CR)*/
pub struct AXISRAM1CRrs;
impl crate::RegisterSpec for AXISRAM1CRrs {
    type Ux = u32;
}
///`read()` method returns [`axisram1cr::R`](R) reader structure
impl crate::Readable for AXISRAM1CRrs {}
///`write(|w| ..)` method takes [`axisram1cr::W`](W) writer structure
impl crate::Writable for AXISRAM1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AXISRAM1CR to value 0
impl crate::Resettable for AXISRAM1CRrs {}
