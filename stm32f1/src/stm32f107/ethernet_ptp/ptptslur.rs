///Register `PTPTSLUR` reader
pub type R = crate::R<PTPTSLURrs>;
///Register `PTPTSLUR` writer
pub type W = crate::W<PTPTSLURrs>;
///Field `TSUSS` reader - Time stamp update subseconds
pub type TSUSS_R = crate::FieldReader<u32>;
///Field `TSUSS` writer - Time stamp update subseconds
pub type TSUSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `TSUPNS` reader - Time stamp update positive or negative sign
pub type TSUPNS_R = crate::BitReader;
///Field `TSUPNS` writer - Time stamp update positive or negative sign
pub type TSUPNS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - Time stamp update subseconds
    #[inline(always)]
    pub fn tsuss(&self) -> TSUSS_R {
        TSUSS_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - Time stamp update positive or negative sign
    #[inline(always)]
    pub fn tsupns(&self) -> TSUPNS_R {
        TSUPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSLUR")
            .field("tsuss", &self.tsuss())
            .field("tsupns", &self.tsupns())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - Time stamp update subseconds
    #[inline(always)]
    pub fn tsuss(&mut self) -> TSUSS_W<'_, PTPTSLURrs> {
        TSUSS_W::new(self, 0)
    }
    ///Bit 31 - Time stamp update positive or negative sign
    #[inline(always)]
    pub fn tsupns(&mut self) -> TSUPNS_W<'_, PTPTSLURrs> {
        TSUPNS_W::new(self, 31)
    }
}
/**Ethernet PTP time stamp low update register (ETH_PTPTSLUR)

You can [`read`](crate::Reg::read) this register and get [`ptptslur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptptslur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#Ethernet_PTP:PTPTSLUR)*/
pub struct PTPTSLURrs;
impl crate::RegisterSpec for PTPTSLURrs {
    type Ux = u32;
}
///`read()` method returns [`ptptslur::R`](R) reader structure
impl crate::Readable for PTPTSLURrs {}
///`write(|w| ..)` method takes [`ptptslur::W`](W) writer structure
impl crate::Writable for PTPTSLURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTPTSLUR to value 0
impl crate::Resettable for PTPTSLURrs {}
