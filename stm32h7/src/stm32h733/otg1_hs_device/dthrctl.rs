///Register `DTHRCTL` reader
pub type R = crate::R<DTHRCTLrs>;
///Register `DTHRCTL` writer
pub type W = crate::W<DTHRCTLrs>;
///Field `NONISOTHREN` reader - Nonisochronous IN endpoints threshold enable
pub type NONISOTHREN_R = crate::BitReader;
///Field `NONISOTHREN` writer - Nonisochronous IN endpoints threshold enable
pub type NONISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOTHREN` reader - ISO IN endpoint threshold enable
pub type ISOTHREN_R = crate::BitReader;
///Field `ISOTHREN` writer - ISO IN endpoint threshold enable
pub type ISOTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTHRLEN` reader - Transmit threshold length
pub type TXTHRLEN_R = crate::FieldReader<u16>;
///Field `TXTHRLEN` writer - Transmit threshold length
pub type TXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `RXTHREN` reader - Receive threshold enable
pub type RXTHREN_R = crate::BitReader;
///Field `RXTHREN` writer - Receive threshold enable
pub type RXTHREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXTHRLEN` reader - Receive threshold length
pub type RXTHRLEN_R = crate::FieldReader<u16>;
///Field `RXTHRLEN` writer - Receive threshold length
pub type RXTHRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `ARPEN` reader - Arbiter parking enable
pub type ARPEN_R = crate::BitReader;
///Field `ARPEN` writer - Arbiter parking enable
pub type ARPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ISO IN endpoint threshold enable
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:10 - Transmit threshold length
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    ///Bit 16 - Receive threshold enable
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:25 - Receive threshold length
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    ///Bit 27 - Arbiter parking enable
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTHRCTL")
            .field("nonisothren", &self.nonisothren())
            .field("isothren", &self.isothren())
            .field("txthrlen", &self.txthrlen())
            .field("rxthren", &self.rxthren())
            .field("rxthrlen", &self.rxthrlen())
            .field("arpen", &self.arpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Nonisochronous IN endpoints threshold enable
    #[inline(always)]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<'_, DTHRCTLrs> {
        NONISOTHREN_W::new(self, 0)
    }
    ///Bit 1 - ISO IN endpoint threshold enable
    #[inline(always)]
    pub fn isothren(&mut self) -> ISOTHREN_W<'_, DTHRCTLrs> {
        ISOTHREN_W::new(self, 1)
    }
    ///Bits 2:10 - Transmit threshold length
    #[inline(always)]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<'_, DTHRCTLrs> {
        TXTHRLEN_W::new(self, 2)
    }
    ///Bit 16 - Receive threshold enable
    #[inline(always)]
    pub fn rxthren(&mut self) -> RXTHREN_W<'_, DTHRCTLrs> {
        RXTHREN_W::new(self, 16)
    }
    ///Bits 17:25 - Receive threshold length
    #[inline(always)]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<'_, DTHRCTLrs> {
        RXTHRLEN_W::new(self, 17)
    }
    ///Bit 27 - Arbiter parking enable
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W<'_, DTHRCTLrs> {
        ARPEN_W::new(self, 27)
    }
}
/**OTG_HS Device threshold control register

You can [`read`](crate::Reg::read) this register and get [`dthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTG1_HS_DEVICE:DTHRCTL)*/
pub struct DTHRCTLrs;
impl crate::RegisterSpec for DTHRCTLrs {
    type Ux = u32;
}
///`read()` method returns [`dthrctl::R`](R) reader structure
impl crate::Readable for DTHRCTLrs {}
///`write(|w| ..)` method takes [`dthrctl::W`](W) writer structure
impl crate::Writable for DTHRCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTHRCTL to value 0
impl crate::Resettable for DTHRCTLrs {}
