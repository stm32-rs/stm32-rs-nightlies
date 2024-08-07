///Register `VHSACCR` reader
pub type R = crate::R<VHSACCRrs>;
///Register `VHSACCR` writer
pub type W = crate::W<VHSACCRrs>;
///Field `HSA` reader - Horizontal Synchronism Active duration
pub type HSA_R = crate::FieldReader<u16>;
///Field `HSA` writer - Horizontal Synchronism Active duration
pub type HSA_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - Horizontal Synchronism Active duration
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VHSACCR").field("hsa", &self.hsa()).finish()
    }
}
impl W {
    ///Bits 0:12 - Horizontal Synchronism Active duration
    #[inline(always)]
    #[must_use]
    pub fn hsa(&mut self) -> HSA_W<VHSACCRrs> {
        HSA_W::new(self, 0)
    }
}
/**DSI Host Video HSA Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vhsaccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vhsaccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:VHSACCR)*/
pub struct VHSACCRrs;
impl crate::RegisterSpec for VHSACCRrs {
    type Ux = u32;
}
///`read()` method returns [`vhsaccr::R`](R) reader structure
impl crate::Readable for VHSACCRrs {}
///`write(|w| ..)` method takes [`vhsaccr::W`](W) writer structure
impl crate::Writable for VHSACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VHSACCR to value 0
impl crate::Resettable for VHSACCRrs {
    const RESET_VALUE: u32 = 0;
}
