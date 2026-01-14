///Register `MACRWKPFR` reader
pub type R = crate::R<MACRWKPFRrs>;
///Register `MACRWKPFR` writer
pub type W = crate::W<MACRWKPFRrs>;
///Field `MACRWKPFR` reader - Remote wakeup packet filter Refer to Table 532, Table 533 and Table 534 for details on register content and programming sequence.
pub type MACRWKPFR_R = crate::FieldReader<u32>;
///Field `MACRWKPFR` writer - Remote wakeup packet filter Refer to Table 532, Table 533 and Table 534 for details on register content and programming sequence.
pub type MACRWKPFR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Remote wakeup packet filter Refer to Table 532, Table 533 and Table 534 for details on register content and programming sequence.
    #[inline(always)]
    pub fn macrwkpfr(&self) -> MACRWKPFR_R {
        MACRWKPFR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRWKPFR")
            .field("macrwkpfr", &self.macrwkpfr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Remote wakeup packet filter Refer to Table 532, Table 533 and Table 534 for details on register content and programming sequence.
    #[inline(always)]
    pub fn macrwkpfr(&mut self) -> MACRWKPFR_W<'_, MACRWKPFRrs> {
        MACRWKPFR_W::new(self, 0)
    }
}
/**Remote wakeup packet filter register

You can [`read`](crate::Reg::read) this register and get [`macrwkpfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrwkpfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MACRWKPFR)*/
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
