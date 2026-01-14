///Register `DCTL` reader
pub type R = crate::R<DCTLrs>;
///Register `DCTL` writer
pub type W = crate::W<DCTLrs>;
///Field `RWUSIG` reader - Remote wakeup signaling
pub type RWUSIG_R = crate::BitReader;
///Field `RWUSIG` writer - Remote wakeup signaling
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIS` reader - Soft disconnect
pub type SDIS_R = crate::BitReader;
///Field `SDIS` writer - Soft disconnect
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GINSTS` reader - Global IN NAK status
pub type GINSTS_R = crate::BitReader;
///Field `GONSTS` reader - Global OUT NAK status
pub type GONSTS_R = crate::BitReader;
///Field `TCTL` reader - Test control
pub type TCTL_R = crate::FieldReader;
///Field `TCTL` writer - Test control
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SGINAK` writer - Set global IN NAK
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGINAK` writer - Clear global IN NAK
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SGONAK` writer - Set global OUT NAK
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGONAK` writer - Clear global OUT NAK
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POPRGDNE` reader - Power-on programming done
pub type POPRGDNE_R = crate::BitReader;
///Field `POPRGDNE` writer - Power-on programming done
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Remote wakeup signaling
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Soft disconnect
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Global IN NAK status
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Global OUT NAK status
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Test control
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 11 - Power-on programming done
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rwusig", &self.rwusig())
            .field("sdis", &self.sdis())
            .field("ginsts", &self.ginsts())
            .field("gonsts", &self.gonsts())
            .field("tctl", &self.tctl())
            .field("poprgdne", &self.poprgdne())
            .finish()
    }
}
impl W {
    ///Bit 0 - Remote wakeup signaling
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W<'_, DCTLrs> {
        RWUSIG_W::new(self, 0)
    }
    ///Bit 1 - Soft disconnect
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W<'_, DCTLrs> {
        SDIS_W::new(self, 1)
    }
    ///Bits 4:6 - Test control
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W<'_, DCTLrs> {
        TCTL_W::new(self, 4)
    }
    ///Bit 7 - Set global IN NAK
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W<'_, DCTLrs> {
        SGINAK_W::new(self, 7)
    }
    ///Bit 8 - Clear global IN NAK
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W<'_, DCTLrs> {
        CGINAK_W::new(self, 8)
    }
    ///Bit 9 - Set global OUT NAK
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W<'_, DCTLrs> {
        SGONAK_W::new(self, 9)
    }
    ///Bit 10 - Clear global OUT NAK
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W<'_, DCTLrs> {
        CGONAK_W::new(self, 10)
    }
    ///Bit 11 - Power-on programming done
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<'_, DCTLrs> {
        POPRGDNE_W::new(self, 11)
    }
}
/**OTG_HS device control register

You can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#OTG1_HS_DEVICE:DCTL)*/
pub struct DCTLrs;
impl crate::RegisterSpec for DCTLrs {
    type Ux = u32;
}
///`read()` method returns [`dctl::R`](R) reader structure
impl crate::Readable for DCTLrs {}
///`write(|w| ..)` method takes [`dctl::W`](W) writer structure
impl crate::Writable for DCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DCTL to value 0
impl crate::Resettable for DCTLrs {}
