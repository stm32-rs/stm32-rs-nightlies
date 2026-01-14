///Register `MCUDIVR` reader
pub type R = crate::R<MCUDIVRrs>;
///Register `MCUDIVR` writer
pub type W = crate::W<MCUDIVRrs>;
///Field `MCUDIV` reader - MCUDIV
pub type MCUDIV_R = crate::FieldReader;
///Field `MCUDIV` writer - MCUDIV
pub type MCUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MCUDIVRDY` reader - MCUDIVRDY
pub type MCUDIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:3 - MCUDIV
    #[inline(always)]
    pub fn mcudiv(&self) -> MCUDIV_R {
        MCUDIV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 31 - MCUDIVRDY
    #[inline(always)]
    pub fn mcudivrdy(&self) -> MCUDIVRDY_R {
        MCUDIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUDIVR")
            .field("mcudiv", &self.mcudiv())
            .field("mcudivrdy", &self.mcudivrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - MCUDIV
    #[inline(always)]
    pub fn mcudiv(&mut self) -> MCUDIV_W<'_, MCUDIVRrs> {
        MCUDIV_W::new(self, 0)
    }
}
/**This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mcudivr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcudivr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MCUDIVR)*/
pub struct MCUDIVRrs;
impl crate::RegisterSpec for MCUDIVRrs {
    type Ux = u32;
}
///`read()` method returns [`mcudivr::R`](R) reader structure
impl crate::Readable for MCUDIVRrs {}
///`write(|w| ..)` method takes [`mcudivr::W`](W) writer structure
impl crate::Writable for MCUDIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MCUDIVR to value 0x8000_0000
impl crate::Resettable for MCUDIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
