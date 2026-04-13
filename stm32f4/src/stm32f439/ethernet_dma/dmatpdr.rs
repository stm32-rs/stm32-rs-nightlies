///Register `DMATPDR` reader
pub type R = crate::R<DMATPDRrs>;
///Register `DMATPDR` writer
pub type W = crate::W<DMATPDRrs>;
///Field `TPD` reader - TPD
pub type TPD_R = crate::FieldReader<u32>;
///Field `TPD` writer - TPD
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TPD
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATPDR").field("tpd", &self.tpd()).finish()
    }
}
impl W {
    ///Bits 0:31 - TPD
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<'_, DMATPDRrs> {
        TPD_W::new(self, 0)
    }
}
/**Ethernet DMA transmit poll demand register

You can [`read`](crate::Reg::read) this register and get [`dmatpdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatpdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#Ethernet_DMA:DMATPDR)*/
pub struct DMATPDRrs;
impl crate::RegisterSpec for DMATPDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmatpdr::R`](R) reader structure
impl crate::Readable for DMATPDRrs {}
///`write(|w| ..)` method takes [`dmatpdr::W`](W) writer structure
impl crate::Writable for DMATPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMATPDR to value 0
impl crate::Resettable for DMATPDRrs {}
