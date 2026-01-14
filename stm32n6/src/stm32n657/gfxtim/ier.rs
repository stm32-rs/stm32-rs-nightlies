///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `AFCOIE` reader - absolute frame counter overflow interrupt enable
pub type AFCOIE_R = crate::BitReader;
///Field `AFCOIE` writer - absolute frame counter overflow interrupt enable
pub type AFCOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALCOIE` reader - absolute line counter overflow interrupt enable
pub type ALCOIE_R = crate::BitReader;
///Field `ALCOIE` writer - absolute line counter overflow interrupt enable
pub type ALCOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TEIE` reader - tearing-effect interrupt enable
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - tearing-effect interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFCC1IE` reader - absolute frame counter compare 1 interrupt enable
pub type AFCC1IE_R = crate::BitReader;
///Field `AFCC1IE` writer - absolute frame counter compare 1 interrupt enable
pub type AFCC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALCC1IE` reader - absolute line counter compare 1 interrupt enable
pub type ALCC1IE_R = crate::BitReader;
///Field `ALCC1IE` writer - absolute line counter compare 1 interrupt enable
pub type ALCC1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALCC2IE` reader - absolute line counter compare 2 interrupt enable
pub type ALCC2IE_R = crate::BitReader;
///Field `ALCC2IE` writer - absolute line counter compare 2 interrupt enable
pub type ALCC2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC1RIE` reader - relative frame counter 1 reload interrupt enable
pub type RFC1RIE_R = crate::BitReader;
///Field `RFC1RIE` writer - relative frame counter 1 reload interrupt enable
pub type RFC1RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RFC2RIE` reader - relative frame counter 2 reload interrupt enable
pub type RFC2RIE_R = crate::BitReader;
///Field `RFC2RIE` writer - relative frame counter 2 reload interrupt enable
pub type RFC2RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV1IE` reader - event 1 interrupt enable
pub type EV1IE_R = crate::BitReader;
///Field `EV1IE` writer - event 1 interrupt enable
pub type EV1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV2IE` reader - event 2 interrupt enable
pub type EV2IE_R = crate::BitReader;
///Field `EV2IE` writer - event 2 interrupt enable
pub type EV2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV3IE` reader - event 3 interrupt enable
pub type EV3IE_R = crate::BitReader;
///Field `EV3IE` writer - event 3 interrupt enable
pub type EV3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EV4IE` reader - event 4 interrupt enable
pub type EV4IE_R = crate::BitReader;
///Field `EV4IE` writer - event 4 interrupt enable
pub type EV4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGAIE` reader - watchdog alarm interrupt enable
pub type WDGAIE_R = crate::BitReader;
///Field `WDGAIE` writer - watchdog alarm interrupt enable
pub type WDGAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGPIE` reader - watchdog pre-alarm interrupt enable
pub type WDGPIE_R = crate::BitReader;
///Field `WDGPIE` writer - watchdog pre-alarm interrupt enable
pub type WDGPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - absolute frame counter overflow interrupt enable
    #[inline(always)]
    pub fn afcoie(&self) -> AFCOIE_R {
        AFCOIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - absolute line counter overflow interrupt enable
    #[inline(always)]
    pub fn alcoie(&self) -> ALCOIE_R {
        ALCOIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - tearing-effect interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - absolute frame counter compare 1 interrupt enable
    #[inline(always)]
    pub fn afcc1ie(&self) -> AFCC1IE_R {
        AFCC1IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - absolute line counter compare 1 interrupt enable
    #[inline(always)]
    pub fn alcc1ie(&self) -> ALCC1IE_R {
        ALCC1IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - absolute line counter compare 2 interrupt enable
    #[inline(always)]
    pub fn alcc2ie(&self) -> ALCC2IE_R {
        ALCC2IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - relative frame counter 1 reload interrupt enable
    #[inline(always)]
    pub fn rfc1rie(&self) -> RFC1RIE_R {
        RFC1RIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - relative frame counter 2 reload interrupt enable
    #[inline(always)]
    pub fn rfc2rie(&self) -> RFC2RIE_R {
        RFC2RIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - event 1 interrupt enable
    #[inline(always)]
    pub fn ev1ie(&self) -> EV1IE_R {
        EV1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - event 2 interrupt enable
    #[inline(always)]
    pub fn ev2ie(&self) -> EV2IE_R {
        EV2IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - event 3 interrupt enable
    #[inline(always)]
    pub fn ev3ie(&self) -> EV3IE_R {
        EV3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - event 4 interrupt enable
    #[inline(always)]
    pub fn ev4ie(&self) -> EV4IE_R {
        EV4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - watchdog alarm interrupt enable
    #[inline(always)]
    pub fn wdgaie(&self) -> WDGAIE_R {
        WDGAIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - watchdog pre-alarm interrupt enable
    #[inline(always)]
    pub fn wdgpie(&self) -> WDGPIE_R {
        WDGPIE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("afcoie", &self.afcoie())
            .field("alcoie", &self.alcoie())
            .field("teie", &self.teie())
            .field("afcc1ie", &self.afcc1ie())
            .field("alcc1ie", &self.alcc1ie())
            .field("alcc2ie", &self.alcc2ie())
            .field("rfc1rie", &self.rfc1rie())
            .field("rfc2rie", &self.rfc2rie())
            .field("ev1ie", &self.ev1ie())
            .field("ev2ie", &self.ev2ie())
            .field("ev3ie", &self.ev3ie())
            .field("ev4ie", &self.ev4ie())
            .field("wdgaie", &self.wdgaie())
            .field("wdgpie", &self.wdgpie())
            .finish()
    }
}
impl W {
    ///Bit 0 - absolute frame counter overflow interrupt enable
    #[inline(always)]
    pub fn afcoie(&mut self) -> AFCOIE_W<'_, IERrs> {
        AFCOIE_W::new(self, 0)
    }
    ///Bit 1 - absolute line counter overflow interrupt enable
    #[inline(always)]
    pub fn alcoie(&mut self) -> ALCOIE_W<'_, IERrs> {
        ALCOIE_W::new(self, 1)
    }
    ///Bit 2 - tearing-effect interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, IERrs> {
        TEIE_W::new(self, 2)
    }
    ///Bit 4 - absolute frame counter compare 1 interrupt enable
    #[inline(always)]
    pub fn afcc1ie(&mut self) -> AFCC1IE_W<'_, IERrs> {
        AFCC1IE_W::new(self, 4)
    }
    ///Bit 8 - absolute line counter compare 1 interrupt enable
    #[inline(always)]
    pub fn alcc1ie(&mut self) -> ALCC1IE_W<'_, IERrs> {
        ALCC1IE_W::new(self, 8)
    }
    ///Bit 9 - absolute line counter compare 2 interrupt enable
    #[inline(always)]
    pub fn alcc2ie(&mut self) -> ALCC2IE_W<'_, IERrs> {
        ALCC2IE_W::new(self, 9)
    }
    ///Bit 12 - relative frame counter 1 reload interrupt enable
    #[inline(always)]
    pub fn rfc1rie(&mut self) -> RFC1RIE_W<'_, IERrs> {
        RFC1RIE_W::new(self, 12)
    }
    ///Bit 13 - relative frame counter 2 reload interrupt enable
    #[inline(always)]
    pub fn rfc2rie(&mut self) -> RFC2RIE_W<'_, IERrs> {
        RFC2RIE_W::new(self, 13)
    }
    ///Bit 16 - event 1 interrupt enable
    #[inline(always)]
    pub fn ev1ie(&mut self) -> EV1IE_W<'_, IERrs> {
        EV1IE_W::new(self, 16)
    }
    ///Bit 17 - event 2 interrupt enable
    #[inline(always)]
    pub fn ev2ie(&mut self) -> EV2IE_W<'_, IERrs> {
        EV2IE_W::new(self, 17)
    }
    ///Bit 18 - event 3 interrupt enable
    #[inline(always)]
    pub fn ev3ie(&mut self) -> EV3IE_W<'_, IERrs> {
        EV3IE_W::new(self, 18)
    }
    ///Bit 19 - event 4 interrupt enable
    #[inline(always)]
    pub fn ev4ie(&mut self) -> EV4IE_W<'_, IERrs> {
        EV4IE_W::new(self, 19)
    }
    ///Bit 24 - watchdog alarm interrupt enable
    #[inline(always)]
    pub fn wdgaie(&mut self) -> WDGAIE_W<'_, IERrs> {
        WDGAIE_W::new(self, 24)
    }
    ///Bit 25 - watchdog pre-alarm interrupt enable
    #[inline(always)]
    pub fn wdgpie(&mut self) -> WDGPIE_W<'_, IERrs> {
        WDGPIE_W::new(self, 25)
    }
}
/**GFXTIM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#GFXTIM:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
