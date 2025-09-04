///Register `MACTSAR` reader
pub type R = crate::R<MACTSARrs>;
///Register `MACTSAR` writer
pub type W = crate::W<MACTSARrs>;
///Field `TSAR` reader - TSAR
pub type TSAR_R = crate::FieldReader<u32>;
///Field `TSAR` writer - TSAR
pub type TSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TSAR
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSAR")
            .field("tsar", &self.tsar())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TSAR
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W<MACTSARrs> {
        TSAR_W::new(self, 0)
    }
}
/**The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.

You can [`read`](crate::Reg::read) this register and get [`mactsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACTSAR)*/
pub struct MACTSARrs;
impl crate::RegisterSpec for MACTSARrs {
    type Ux = u32;
}
///`read()` method returns [`mactsar::R`](R) reader structure
impl crate::Readable for MACTSARrs {}
///`write(|w| ..)` method takes [`mactsar::W`](W) writer structure
impl crate::Writable for MACTSARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSAR to value 0
impl crate::Resettable for MACTSARrs {}
