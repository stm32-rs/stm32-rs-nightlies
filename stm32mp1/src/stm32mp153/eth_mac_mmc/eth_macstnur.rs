///Register `ETH_MACSTNUR` reader
pub type R = crate::R<ETH_MACSTNURrs>;
///Register `ETH_MACSTNUR` writer
pub type W = crate::W<ETH_MACSTNURrs>;
///Field `TSSS` reader - TSSS
pub type TSSS_R = crate::FieldReader<u32>;
///Field `TSSS` writer - TSSS
pub type TSSS_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `ADDSUB` reader - ADDSUB
pub type ADDSUB_R = crate::BitReader;
///Field `ADDSUB` writer - ADDSUB
pub type ADDSUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - TSSS
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - ADDSUB
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACSTNUR")
            .field("tsss", &self.tsss())
            .field("addsub", &self.addsub())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - TSSS
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<ETH_MACSTNURrs> {
        TSSS_W::new(self, 0)
    }
    ///Bit 31 - ADDSUB
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<ETH_MACSTNURrs> {
        ADDSUB_W::new(self, 31)
    }
}
/**This register is present only when the IEEE 1588 timestamp feature is selected without external timestamp input.

You can [`read`](crate::Reg::read) this register and get [`eth_macstnur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macstnur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACSTNUR)*/
pub struct ETH_MACSTNURrs;
impl crate::RegisterSpec for ETH_MACSTNURrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macstnur::R`](R) reader structure
impl crate::Readable for ETH_MACSTNURrs {}
///`write(|w| ..)` method takes [`eth_macstnur::W`](W) writer structure
impl crate::Writable for ETH_MACSTNURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACSTNUR to value 0
impl crate::Resettable for ETH_MACSTNURrs {
    const RESET_VALUE: u32 = 0;
}
