///Register `RCC_MPCKDIVR` reader
pub type R = crate::R<RCC_MPCKDIVRrs>;
///Register `RCC_MPCKDIVR` writer
pub type W = crate::W<RCC_MPCKDIVRrs>;
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
        f.debug_struct("RCC_MPCKDIVR")
            .field("mpudiv", &self.mpudiv())
            .field("mpudivrdy", &self.mpudivrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - MPUDIV
    #[inline(always)]
    #[must_use]
    pub fn mpudiv(&mut self) -> MPUDIV_W<RCC_MPCKDIVRrs> {
        MPUDIV_W::new(self, 0)
    }
}
/**This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`rcc_mpckdivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_mpckdivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_MPCKDIVR)*/
pub struct RCC_MPCKDIVRrs;
impl crate::RegisterSpec for RCC_MPCKDIVRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_mpckdivr::R`](R) reader structure
impl crate::Readable for RCC_MPCKDIVRrs {}
///`write(|w| ..)` method takes [`rcc_mpckdivr::W`](W) writer structure
impl crate::Writable for RCC_MPCKDIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_MPCKDIVR to value 0x8000_0001
impl crate::Resettable for RCC_MPCKDIVRrs {
    const RESET_VALUE: u32 = 0x8000_0001;
}
