///Register `OTG_HS_DOEPTSIZ0` reader
pub type R = crate::R<OTG_HS_DOEPTSIZ0rs>;
///Register `OTG_HS_DOEPTSIZ0` writer
pub type W = crate::W<OTG_HS_DOEPTSIZ0rs>;
///Field `XFRSIZ` reader - Transfer size
pub type XFRSIZ_R = crate::FieldReader;
///Field `XFRSIZ` writer - Transfer size
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `PKTCNT` reader - Packet count
pub type PKTCNT_R = crate::BitReader;
///Field `PKTCNT` writer - Packet count
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STUPCNT` reader - SETUP packet count
pub type STUPCNT_R = crate::FieldReader;
///Field `STUPCNT` writer - SETUP packet count
pub type STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:6 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 19 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 29:30 - SETUP packet count
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DOEPTSIZ0")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .field("stupcnt", &self.stupcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<'_, OTG_HS_DOEPTSIZ0rs> {
        XFRSIZ_W::new(self, 0)
    }
    ///Bit 19 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, OTG_HS_DOEPTSIZ0rs> {
        PKTCNT_W::new(self, 19)
    }
    ///Bits 29:30 - SETUP packet count
    #[inline(always)]
    pub fn stupcnt(&mut self) -> STUPCNT_W<'_, OTG_HS_DOEPTSIZ0rs> {
        STUPCNT_W::new(self, 29)
    }
}
/**OTG_HS device endpoint-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doeptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doeptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_DEVICE:OTG_HS_DOEPTSIZ0)*/
pub struct OTG_HS_DOEPTSIZ0rs;
impl crate::RegisterSpec for OTG_HS_DOEPTSIZ0rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_doeptsiz0::R`](R) reader structure
impl crate::Readable for OTG_HS_DOEPTSIZ0rs {}
///`write(|w| ..)` method takes [`otg_hs_doeptsiz0::W`](W) writer structure
impl crate::Writable for OTG_HS_DOEPTSIZ0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DOEPTSIZ0 to value 0
impl crate::Resettable for OTG_HS_DOEPTSIZ0rs {}
