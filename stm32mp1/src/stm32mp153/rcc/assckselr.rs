///Register `ASSCKSELR` reader
pub type R = crate::R<ASSCKSELRrs>;
///Register `ASSCKSELR` writer
pub type W = crate::W<ASSCKSELRrs>;
///Field `AXISSRC` reader - AXISSRC
pub type AXISSRC_R = crate::FieldReader;
///Field `AXISSRC` writer - AXISSRC
pub type AXISSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AXISSRCRDY` reader - AXISSRCRDY
pub type AXISSRCRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - AXISSRC
    #[inline(always)]
    pub fn axissrc(&self) -> AXISSRC_R {
        AXISSRC_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - AXISSRCRDY
    #[inline(always)]
    pub fn axissrcrdy(&self) -> AXISSRCRDY_R {
        AXISSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSCKSELR")
            .field("axissrc", &self.axissrc())
            .field("axissrcrdy", &self.axissrcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AXISSRC
    #[inline(always)]
    pub fn axissrc(&mut self) -> AXISSRC_W<'_, ASSCKSELRrs> {
        AXISSRC_W::new(self, 0)
    }
}
/**This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`assckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`assckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:ASSCKSELR)*/
pub struct ASSCKSELRrs;
impl crate::RegisterSpec for ASSCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`assckselr::R`](R) reader structure
impl crate::Readable for ASSCKSELRrs {}
///`write(|w| ..)` method takes [`assckselr::W`](W) writer structure
impl crate::Writable for ASSCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASSCKSELR to value 0x8000_0000
impl crate::Resettable for ASSCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
