///Register `MTLTXQ1HCR` reader
pub type R = crate::R<MTLTXQ1HCRrs>;
///Register `MTLTXQ1HCR` writer
pub type W = crate::W<MTLTXQ1HCRrs>;
///Field `HC` reader - hiCredit Value
pub type HC_R = crate::FieldReader<u32>;
///Field `HC` writer - hiCredit Value
pub type HC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    ///Bits 0:28 - hiCredit Value
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new(self.bits & 0x1fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ1HCR")
            .field("hc", &self.hc())
            .finish()
    }
}
impl W {
    ///Bits 0:28 - hiCredit Value
    #[inline(always)]
    pub fn hc(&mut self) -> HC_W<'_, MTLTXQ1HCRrs> {
        HC_W::new(self, 0)
    }
}
/**Tx Queue 1 hiCredit register

You can [`read`](crate::Reg::read) this register and get [`mtltxq1hcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltxq1hcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:MTLTXQ1HCR)*/
pub struct MTLTXQ1HCRrs;
impl crate::RegisterSpec for MTLTXQ1HCRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq1hcr::R`](R) reader structure
impl crate::Readable for MTLTXQ1HCRrs {}
///`write(|w| ..)` method takes [`mtltxq1hcr::W`](W) writer structure
impl crate::Writable for MTLTXQ1HCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTXQ1HCR to value 0
impl crate::Resettable for MTLTXQ1HCRrs {}
