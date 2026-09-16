///Register `MPCKSELR` reader
pub type R = crate::R<MPCKSELRrs>;
///Register `MPCKSELR` writer
pub type W = crate::W<MPCKSELRrs>;
///Field `MPUSRC` reader - MPUSRC
pub type MPUSRC_R = crate::FieldReader;
///Field `MPUSRC` writer - MPUSRC
pub type MPUSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MPUSRCRDY` reader - MPUSRCRDY
pub type MPUSRCRDY_R = crate::BitReader;
impl R {
    ///Bits 0:1 - MPUSRC
    #[inline(always)]
    pub fn mpusrc(&self) -> MPUSRC_R {
        MPUSRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 31 - MPUSRCRDY
    #[inline(always)]
    pub fn mpusrcrdy(&self) -> MPUSRCRDY_R {
        MPUSRCRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCKSELR")
            .field("mpusrc", &self.mpusrc())
            .field("mpusrcrdy", &self.mpusrcrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MPUSRC
    #[inline(always)]
    pub fn mpusrc(&mut self) -> MPUSRC_W<'_, MPCKSELRrs> {
        MPUSRC_W::new(self, 0)
    }
}
/**This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`mpckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MPCKSELR)*/
pub struct MPCKSELRrs;
impl crate::RegisterSpec for MPCKSELRrs {
    type Ux = u32;
}
///`read()` method returns [`mpckselr::R`](R) reader structure
impl crate::Readable for MPCKSELRrs {}
///`write(|w| ..)` method takes [`mpckselr::W`](W) writer structure
impl crate::Writable for MPCKSELRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCKSELR to value 0x8000_0000
impl crate::Resettable for MPCKSELRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
