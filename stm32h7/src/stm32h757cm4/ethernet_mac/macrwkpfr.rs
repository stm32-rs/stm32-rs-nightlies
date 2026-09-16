///Register `MACRWKPFR` reader
pub type R = crate::R<MACRWKPFRrs>;
///Register `MACRWKPFR` writer
pub type W = crate::W<MACRWKPFRrs>;
///Field `WKUPFRMFTR` reader - WKUPFRMFTR
pub type WKUPFRMFTR_R = crate::FieldReader<u32>;
///Field `WKUPFRMFTR` writer - WKUPFRMFTR
pub type WKUPFRMFTR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - WKUPFRMFTR
    #[inline(always)]
    pub fn wkupfrmftr(&self) -> WKUPFRMFTR_R {
        WKUPFRMFTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRWKPFR")
            .field("wkupfrmftr", &self.wkupfrmftr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - WKUPFRMFTR
    #[inline(always)]
    pub fn wkupfrmftr(&mut self) -> WKUPFRMFTR_W<'_, MACRWKPFRrs> {
        WKUPFRMFTR_W::new(self, 0)
    }
}
/**Remove wakeup packet filter register

You can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#Ethernet_MAC:MACRWKPFR)*/
pub struct MACRWKPFRrs;
impl crate::RegisterSpec for MACRWKPFRrs {
    type Ux = u32;
}
///`read()` method returns [`macrwkpfr::R`](R) reader structure
impl crate::Readable for MACRWKPFRrs {}
///`write(|w| ..)` method takes [`macrwkpfr::W`](W) writer structure
impl crate::Writable for MACRWKPFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRWKPFR to value 0
impl crate::Resettable for MACRWKPFRrs {}
