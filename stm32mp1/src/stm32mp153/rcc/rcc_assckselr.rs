///Register `RCC_ASSCKSELR` reader
pub type R = crate::R<RCC_ASSCKSELRrs>;
///Register `RCC_ASSCKSELR` writer
pub type W = crate::W<RCC_ASSCKSELRrs>;
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
        f.debug_struct("RCC_ASSCKSELR")
            .field("axissrc", &self.axissrc())
            .field("axissrcrdy", &self.axissrcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - AXISSRC
    #[inline(always)]
    #[must_use]
    pub fn axissrc(&mut self) -> AXISSRC_W<RCC_ASSCKSELRrs> {
        AXISSRC_W::new(self, 0)
    }
}
/**This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`rcc_assckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_assckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_ASSCKSELR)*/
pub struct RCC_ASSCKSELRrs;
impl crate::RegisterSpec for RCC_ASSCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_assckselr::R`](R) reader structure
impl crate::Readable for RCC_ASSCKSELRrs {}
///`write(|w| ..)` method takes [`rcc_assckselr::W`](W) writer structure
impl crate::Writable for RCC_ASSCKSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_ASSCKSELR to value 0x8000_0000
impl crate::Resettable for RCC_ASSCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
