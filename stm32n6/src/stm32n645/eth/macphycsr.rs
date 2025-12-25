///Register `MACPHYCSR` reader
pub type R = crate::R<MACPHYCSRrs>;
///Register `MACPHYCSR` writer
pub type W = crate::W<MACPHYCSRrs>;
///Field `TC` reader - Transmit Configuration in RGMII
pub type TC_R = crate::BitReader;
///Field `TC` writer - Transmit Configuration in RGMII
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LUD` reader - Link Up or Down
pub type LUD_R = crate::BitReader;
///Field `LUD` writer - Link Up or Down
pub type LUD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LNKMOD` reader - Link Mode
pub type LNKMOD_R = crate::BitReader;
///Field `LNKSPEED` reader - Link Speed
pub type LNKSPEED_R = crate::FieldReader;
///Field `LNKSTS` reader - Link Status
pub type LNKSTS_R = crate::BitReader;
impl R {
    ///Bit 0 - Transmit Configuration in RGMII
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Link Up or Down
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - Link Mode
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Link Speed
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Link Status
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPHYCSR")
            .field("tc", &self.tc())
            .field("lud", &self.lud())
            .field("lnkmod", &self.lnkmod())
            .field("lnkspeed", &self.lnkspeed())
            .field("lnksts", &self.lnksts())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Configuration in RGMII
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, MACPHYCSRrs> {
        TC_W::new(self, 0)
    }
    ///Bit 1 - Link Up or Down
    #[inline(always)]
    pub fn lud(&mut self) -> LUD_W<'_, MACPHYCSRrs> {
        LUD_W::new(self, 1)
    }
}
/**PHYIF control status register

You can [`read`](crate::Reg::read) this register and get [`macphycsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macphycsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MACPHYCSR)*/
pub struct MACPHYCSRrs;
impl crate::RegisterSpec for MACPHYCSRrs {
    type Ux = u32;
}
///`read()` method returns [`macphycsr::R`](R) reader structure
impl crate::Readable for MACPHYCSRrs {}
///`write(|w| ..)` method takes [`macphycsr::W`](W) writer structure
impl crate::Writable for MACPHYCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPHYCSR to value 0
impl crate::Resettable for MACPHYCSRrs {}
