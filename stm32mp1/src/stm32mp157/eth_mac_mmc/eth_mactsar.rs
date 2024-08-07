///Register `ETH_MACTSAR` reader
pub type R = crate::R<ETH_MACTSARrs>;
///Register `ETH_MACTSAR` writer
pub type W = crate::W<ETH_MACTSARrs>;
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
        f.debug_struct("ETH_MACTSAR")
            .field("tsar", &self.tsar())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TSAR
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TSAR_W<ETH_MACTSARrs> {
        TSAR_W::new(self, 0)
    }
}
/**The Timestamp Addend register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input. This register value is used only when the system time is configured for Fine Update mode (TSCFUPDT bit in the ETH_MACTSCR register). The content of this register is added to a 32-bit accumulator in every clock cycle (of HCLK) and the system time is updated whenever the accumulator overflows.

You can [`read`](crate::Reg::read) this register and get [`eth_mactsar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mactsar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACTSAR)*/
pub struct ETH_MACTSARrs;
impl crate::RegisterSpec for ETH_MACTSARrs {
    type Ux = u32;
}
///`read()` method returns [`eth_mactsar::R`](R) reader structure
impl crate::Readable for ETH_MACTSARrs {}
///`write(|w| ..)` method takes [`eth_mactsar::W`](W) writer structure
impl crate::Writable for ETH_MACTSARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACTSAR to value 0
impl crate::Resettable for ETH_MACTSARrs {
    const RESET_VALUE: u32 = 0;
}
