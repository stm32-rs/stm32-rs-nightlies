///Register `INITSVTORCR` reader
pub type R = crate::R<INITSVTORCRrs>;
///Register `INITSVTORCR` writer
pub type W = crate::W<INITSVTORCRrs>;
///Field `SVTOR_ADDR` reader - Secure vector table base address
pub type SVTOR_ADDR_R = crate::FieldReader<u32>;
///Field `SVTOR_ADDR` writer - Secure vector table base address
pub type SVTOR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 7:31 - Secure vector table base address
    #[inline(always)]
    pub fn svtor_addr(&self) -> SVTOR_ADDR_R {
        SVTOR_ADDR_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INITSVTORCR")
            .field("svtor_addr", &self.svtor_addr())
            .finish()
    }
}
impl W {
    ///Bits 7:31 - Secure vector table base address
    #[inline(always)]
    pub fn svtor_addr(&mut self) -> SVTOR_ADDR_W<'_, INITSVTORCRrs> {
        SVTOR_ADDR_W::new(self, 7)
    }
}
/**SYSCFG Cortex-M55 SVTOR control register

You can [`read`](crate::Reg::read) this register and get [`initsvtorcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initsvtorcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SYSCFG:INITSVTORCR)*/
pub struct INITSVTORCRrs;
impl crate::RegisterSpec for INITSVTORCRrs {
    type Ux = u32;
}
///`read()` method returns [`initsvtorcr::R`](R) reader structure
impl crate::Readable for INITSVTORCRrs {}
///`write(|w| ..)` method takes [`initsvtorcr::W`](W) writer structure
impl crate::Writable for INITSVTORCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INITSVTORCR to value 0x1800_0000
impl crate::Resettable for INITSVTORCRrs {
    const RESET_VALUE: u32 = 0x1800_0000;
}
