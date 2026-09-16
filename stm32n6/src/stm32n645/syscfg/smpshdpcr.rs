///Register `SMPSHDPCR` reader
pub type R = crate::R<SMPSHDPCRrs>;
///Register `SMPSHDPCR` writer
pub type W = crate::W<SMPSHDPCRrs>;
///Field `SMPSHDPSEL` reader - Others: Reserved
pub type SMPSHDPSEL_R = crate::FieldReader;
///Field `SMPSHDPSEL` writer - Others: Reserved
pub type SMPSHDPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Others: Reserved
    #[inline(always)]
    pub fn smpshdpsel(&self) -> SMPSHDPSEL_R {
        SMPSHDPSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPSHDPCR")
            .field("smpshdpsel", &self.smpshdpsel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Others: Reserved
    #[inline(always)]
    pub fn smpshdpsel(&mut self) -> SMPSHDPSEL_W<'_, SMPSHDPCRrs> {
        SMPSHDPSEL_W::new(self, 0)
    }
}
/**SYSCFG SMPS observable signals through HDP selection configuration register

You can [`read`](crate::Reg::read) this register and get [`smpshdpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpshdpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#SYSCFG:SMPSHDPCR)*/
pub struct SMPSHDPCRrs;
impl crate::RegisterSpec for SMPSHDPCRrs {
    type Ux = u32;
}
///`read()` method returns [`smpshdpcr::R`](R) reader structure
impl crate::Readable for SMPSHDPCRrs {}
///`write(|w| ..)` method takes [`smpshdpcr::W`](W) writer structure
impl crate::Writable for SMPSHDPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMPSHDPCR to value 0
impl crate::Resettable for SMPSHDPCRrs {}
