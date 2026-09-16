///Register `FDCAN_XIDAM` reader
pub type R = crate::R<FDCAN_XIDAMrs>;
///Register `FDCAN_XIDAM` writer
pub type W = crate::W<FDCAN_XIDAMrs>;
///Field `EIDM` reader - Extended ID mask
pub type EIDM_R = crate::FieldReader<u32>;
///Field `EIDM` writer - Extended ID mask
pub type EIDM_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - Extended ID mask
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_XIDAM")
            .field("eidm", &self.eidm())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - Extended ID mask
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W<'_, FDCAN_XIDAMrs> {
        EIDM_W::new(self, 0)
    }
}
/**FDCAN extended ID and mask register

You can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_XIDAM)*/
pub struct FDCAN_XIDAMrs;
impl crate::RegisterSpec for FDCAN_XIDAMrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_xidam::R`](R) reader structure
impl crate::Readable for FDCAN_XIDAMrs {}
///`write(|w| ..)` method takes [`fdcan_xidam::W`](W) writer structure
impl crate::Writable for FDCAN_XIDAMrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_XIDAM to value 0x1fff_ffff
impl crate::Resettable for FDCAN_XIDAMrs {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
