///Register `MACWTR` reader
pub type R = crate::R<MACWTRrs>;
///Register `MACWTR` writer
pub type W = crate::W<MACWTRrs>;
///Field `WTO` reader - Watchdog Timeout
pub type WTO_R = crate::FieldReader;
///Field `WTO` writer - Watchdog Timeout
pub type WTO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PWE` reader - Programmable Watchdog Enable
pub type PWE_R = crate::BitReader;
///Field `PWE` writer - Programmable Watchdog Enable
pub type PWE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Watchdog Timeout
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Programmable Watchdog Enable
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACWTR")
            .field("wto", &self.wto())
            .field("pwe", &self.pwe())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Watchdog Timeout
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W<'_, MACWTRrs> {
        WTO_W::new(self, 0)
    }
    ///Bit 8 - Programmable Watchdog Enable
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W<'_, MACWTRrs> {
        PWE_W::new(self, 8)
    }
}
/**Watchdog timeout register

You can [`read`](crate::Reg::read) this register and get [`macwtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macwtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#Ethernet_MAC:MACWTR)*/
pub struct MACWTRrs;
impl crate::RegisterSpec for MACWTRrs {
    type Ux = u32;
}
///`read()` method returns [`macwtr::R`](R) reader structure
impl crate::Readable for MACWTRrs {}
///`write(|w| ..)` method takes [`macwtr::W`](W) writer structure
impl crate::Writable for MACWTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACWTR to value 0
impl crate::Resettable for MACWTRrs {}
