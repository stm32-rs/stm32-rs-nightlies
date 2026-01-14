///Register `DIEPCTL0_ALTERNATE` reader
pub type R = crate::R<DIEPCTL0_ALTERNATErs>;
///Register `DIEPCTL0_ALTERNATE` writer
pub type W = crate::W<DIEPCTL0_ALTERNATErs>;
///Field `MPSIZ` reader - Maximum packet size
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USB active endpoint
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USB active endpoint
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EONUM` reader - Even/odd frame
pub type EONUM_R = crate::BitReader;
///Field `NAKSTS` reader - NAK status
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - Endpoint type
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - Endpoint type
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STALL` reader - STALL handshake
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - Tx FIFO number
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - Tx FIFO number
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CNAK` writer - Clear NAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEVNFRM` writer - Set even frame
pub type SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SODDFRM` writer - Set odd frame
pub type SODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - Endpoint disable
pub type EPDIS_R = crate::BitReader;
///Field `EPDIS` writer - Endpoint disable
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPENA` reader - Endpoint enable
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - Endpoint enable
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Even/odd frame
    #[inline(always)]
    pub fn eonum(&self) -> EONUM_R {
        EONUM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAK status
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:25 - Tx FIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL0_ALTERNATE")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("eonum", &self.eonum())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, DIEPCTL0_ALTERNATErs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<'_, DIEPCTL0_ALTERNATErs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<'_, DIEPCTL0_ALTERNATErs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DIEPCTL0_ALTERNATErs> {
        STALL_W::new(self, 21)
    }
    ///Bits 22:25 - Tx FIFO number
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, DIEPCTL0_ALTERNATErs> {
        TXFNUM_W::new(self, 22)
    }
    ///Bit 26 - Clear NAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DIEPCTL0_ALTERNATErs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DIEPCTL0_ALTERNATErs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - Set even frame
    #[inline(always)]
    pub fn sevnfrm(&mut self) -> SEVNFRM_W<'_, DIEPCTL0_ALTERNATErs> {
        SEVNFRM_W::new(self, 28)
    }
    ///Bit 29 - Set odd frame
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W<'_, DIEPCTL0_ALTERNATErs> {
        SODDFRM_W::new(self, 29)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<'_, DIEPCTL0_ALTERNATErs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DIEPCTL0_ALTERNATErs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device IN endpoint 0 control register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`diepctl0_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#OTG1:DIEPCTL0_ALTERNATE)*/
pub struct DIEPCTL0_ALTERNATErs;
impl crate::RegisterSpec for DIEPCTL0_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`diepctl0_alternate::R`](R) reader structure
impl crate::Readable for DIEPCTL0_ALTERNATErs {}
///`write(|w| ..)` method takes [`diepctl0_alternate::W`](W) writer structure
impl crate::Writable for DIEPCTL0_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIEPCTL0_ALTERNATE to value 0
impl crate::Resettable for DIEPCTL0_ALTERNATErs {}
