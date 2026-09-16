///Register `MPCKDIVR` reader
pub type R = crate::R<MPCKDIVRrs>;
///Register `MPCKDIVR` writer
pub type W = crate::W<MPCKDIVRrs>;
///Field `MPUDIV` reader - MPUDIV
pub type MPUDIV_R = crate::FieldReader;
///Field `MPUDIV` writer - MPUDIV
pub type MPUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MPUDIVRDY` reader - MPUDIVRDY
pub type MPUDIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - MPUDIV
    #[inline(always)]
    pub fn mpudiv(&self) -> MPUDIV_R {
        MPUDIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - MPUDIVRDY
    #[inline(always)]
    pub fn mpudivrdy(&self) -> MPUDIVRDY_R {
        MPUDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCKDIVR")
            .field("mpudiv", &self.mpudiv())
            .field("mpudivrdy", &self.mpudivrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MPUDIV
    #[inline(always)]
    pub fn mpudiv(&mut self) -> MPUDIV_W<'_, MPCKDIVRrs> {
        MPUDIV_W::new(self, 0)
    }
}
/**This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mpckdivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpckdivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MPCKDIVR)*/
pub struct MPCKDIVRrs;
impl crate::RegisterSpec for MPCKDIVRrs {
    type Ux = u32;
}
///`read()` method returns [`mpckdivr::R`](R) reader structure
impl crate::Readable for MPCKDIVRrs {}
///`write(|w| ..)` method takes [`mpckdivr::W`](W) writer structure
impl crate::Writable for MPCKDIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCKDIVR to value 0x8000_0001
impl crate::Resettable for MPCKDIVRrs {
    const RESET_VALUE: u32 = 0x8000_0001;
}
