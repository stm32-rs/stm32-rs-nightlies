///Register `INITNSVTORCR` reader
pub type R = crate::R<INITNSVTORCRrs>;
///Register `INITNSVTORCR` writer
pub type W = crate::W<INITNSVTORCRrs>;
///Field `NSVTOR_ADDR` reader - Non-secure vector table base address
pub type NSVTOR_ADDR_R = crate::FieldReader<u32>;
///Field `NSVTOR_ADDR` writer - Non-secure vector table base address
pub type NSVTOR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 7:31 - Non-secure vector table base address
    #[inline(always)]
    pub fn nsvtor_addr(&self) -> NSVTOR_ADDR_R {
        NSVTOR_ADDR_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INITNSVTORCR")
            .field("nsvtor_addr", &self.nsvtor_addr())
            .finish()
    }
}
impl W {
    ///Bits 7:31 - Non-secure vector table base address
    #[inline(always)]
    pub fn nsvtor_addr(&mut self) -> NSVTOR_ADDR_W<'_, INITNSVTORCRrs> {
        NSVTOR_ADDR_W::new(self, 7)
    }
}
/**SYSCFG Cortex-M55 NSVTOR control register

You can [`read`](crate::Reg::read) this register and get [`initnsvtorcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initnsvtorcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SYSCFG:INITNSVTORCR)*/
pub struct INITNSVTORCRrs;
impl crate::RegisterSpec for INITNSVTORCRrs {
    type Ux = u32;
}
///`read()` method returns [`initnsvtorcr::R`](R) reader structure
impl crate::Readable for INITNSVTORCRrs {}
///`write(|w| ..)` method takes [`initnsvtorcr::W`](W) writer structure
impl crate::Writable for INITNSVTORCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INITNSVTORCR to value 0x0800_0000
impl crate::Resettable for INITNSVTORCRrs {
    const RESET_VALUE: u32 = 0x0800_0000;
}
