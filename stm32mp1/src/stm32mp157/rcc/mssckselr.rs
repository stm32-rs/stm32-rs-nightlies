///Register `MSSCKSELR` reader
pub type R = crate::R<MSSCKSELRrs>;
///Register `MSSCKSELR` writer
pub type W = crate::W<MSSCKSELRrs>;
///Field `MCUSSRC` reader - MCUSSRC
pub type MCUSSRC_R = crate::FieldReader;
///Field `MCUSSRC` writer - MCUSSRC
pub type MCUSSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCUSSRCRDY` reader - MCUSSRCRDY
pub type MCUSSRCRDY_R = crate::BitReader;
impl R {
    ///Bits 0:1 - MCUSSRC
    #[inline(always)]
    pub fn mcussrc(&self) -> MCUSSRC_R {
        MCUSSRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 31 - MCUSSRCRDY
    #[inline(always)]
    pub fn mcussrcrdy(&self) -> MCUSSRCRDY_R {
        MCUSSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSSCKSELR")
            .field("mcussrc", &self.mcussrc())
            .field("mcussrcrdy", &self.mcussrcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MCUSSRC
    #[inline(always)]
    pub fn mcussrc(&mut self) -> MCUSSRC_W<'_, MSSCKSELRrs> {
        MCUSSRC_W::new(self, 0)
    }
}
/**This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`mssckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mssckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MSSCKSELR)*/
pub struct MSSCKSELRrs;
impl crate::RegisterSpec for MSSCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`mssckselr::R`](R) reader structure
impl crate::Readable for MSSCKSELRrs {}
///`write(|w| ..)` method takes [`mssckselr::W`](W) writer structure
impl crate::Writable for MSSCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MSSCKSELR to value 0x8000_0000
impl crate::Resettable for MSSCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
