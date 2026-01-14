///Register `MACSTSUR` reader
pub type R = crate::R<MACSTSURrs>;
///Register `MACSTSUR` writer
pub type W = crate::W<MACSTSURrs>;
///Field `TSS` reader - Timestamp Seconds
pub type TSS_R = crate::FieldReader<u32>;
///Field `TSS` writer - Timestamp Seconds
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Timestamp Seconds
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSTSUR")
            .field("tss", &self.tss())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Timestamp Seconds
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<'_, MACSTSURrs> {
        TSS_W::new(self, 0)
    }
}
/**System time seconds update register

You can [`read`](crate::Reg::read) this register and get [`macstsur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macstsur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#Ethernet_MAC:MACSTSUR)*/
pub struct MACSTSURrs;
impl crate::RegisterSpec for MACSTSURrs {
    type Ux = u32;
}
///`read()` method returns [`macstsur::R`](R) reader structure
impl crate::Readable for MACSTSURrs {}
///`write(|w| ..)` method takes [`macstsur::W`](W) writer structure
impl crate::Writable for MACSTSURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSTSUR to value 0
impl crate::Resettable for MACSTSURrs {}
