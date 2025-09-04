///Register `AMTCR` reader
pub type R = crate::R<AMTCRrs>;
///Register `AMTCR` writer
pub type W = crate::W<AMTCRrs>;
///Field `EN` reader - Dead-time functionality enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - Dead-time functionality enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DT` reader - Dead time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Dead time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Dead-time functionality enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - Dead time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMTCR")
            .field("en", &self.en())
            .field("dt", &self.dt())
            .finish()
    }
}
impl W {
    ///Bit 0 - Dead-time functionality enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<AMTCRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 8:15 - Dead time Dead time value in the AXI clock cycle inserted between two consecutive accesses on the AXI master port. These bits represent the minimum guaranteed number of cycles between two consecutive AXI accesses.
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<AMTCRrs> {
        DT_W::new(self, 8)
    }
}
/**DMA2D AXI master timer configuration register

You can [`read`](crate::Reg::read) this register and get [`amtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DMA2D:AMTCR)*/
pub struct AMTCRrs;
impl crate::RegisterSpec for AMTCRrs {
    type Ux = u32;
}
///`read()` method returns [`amtcr::R`](R) reader structure
impl crate::Readable for AMTCRrs {}
///`write(|w| ..)` method takes [`amtcr::W`](W) writer structure
impl crate::Writable for AMTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AMTCR to value 0
impl crate::Resettable for AMTCRrs {}
