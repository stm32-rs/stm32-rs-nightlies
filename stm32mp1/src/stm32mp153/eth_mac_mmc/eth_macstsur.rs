///Register `ETH_MACSTSUR` reader
pub type R = crate::R<ETH_MACSTSURrs>;
///Register `ETH_MACSTSUR` writer
pub type W = crate::W<ETH_MACSTSURrs>;
///Field `TSS` reader - TSS
pub type TSS_R = crate::FieldReader<u32>;
///Field `TSS` writer - TSS
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TSS
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACSTSUR")
            .field("tss", &self.tss())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TSS
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<ETH_MACSTSURrs> {
        TSS_W::new(self, 0)
    }
}
/**The System Time Seconds Update register, along with the System Time Nanoseconds Update register, initializes or updates the system time maintained by the MAC. You must write both registers before setting the TSINIT or TSUPDT bits in ETH_MACTSCR register. This register is present only when the IEEE 1588 Timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`eth_macstsur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macstsur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACSTSUR)*/
pub struct ETH_MACSTSURrs;
impl crate::RegisterSpec for ETH_MACSTSURrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macstsur::R`](R) reader structure
impl crate::Readable for ETH_MACSTSURrs {}
///`write(|w| ..)` method takes [`eth_macstsur::W`](W) writer structure
impl crate::Writable for ETH_MACSTSURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACSTSUR to value 0
impl crate::Resettable for ETH_MACSTSURrs {
    const RESET_VALUE: u32 = 0;
}
