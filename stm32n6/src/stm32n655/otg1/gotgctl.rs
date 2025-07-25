///Register `GOTGCTL` reader
pub type R = crate::R<GOTGCTLrs>;
///Register `GOTGCTL` writer
pub type W = crate::W<GOTGCTLrs>;
///Field `VBVALOEN` reader - Vless thansub>BUSless than/sub> valid override enable.
pub type VBVALOEN_R = crate::BitReader;
///Field `VBVALOEN` writer - Vless thansub>BUSless than/sub> valid override enable.
pub type VBVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VBVALOVAL` reader - Vless thansub>BUSless than/sub> valid override value.
pub type VBVALOVAL_R = crate::BitReader;
///Field `VBVALOVAL` writer - Vless thansub>BUSless than/sub> valid override value.
pub type VBVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVALOEN` reader - A-peripheral session valid override enable.
pub type AVALOEN_R = crate::BitReader;
///Field `AVALOEN` writer - A-peripheral session valid override enable.
pub type AVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AVALOVAL` reader - A-peripheral session valid override value.
pub type AVALOVAL_R = crate::BitReader;
///Field `AVALOVAL` writer - A-peripheral session valid override value.
pub type AVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BVALOEN` reader - B-peripheral session valid override enable.
pub type BVALOEN_R = crate::BitReader;
///Field `BVALOEN` writer - B-peripheral session valid override enable.
pub type BVALOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BVALOVAL` reader - B-peripheral session valid override value.
pub type BVALOVAL_R = crate::BitReader;
///Field `BVALOVAL` writer - B-peripheral session valid override value.
pub type BVALOVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EHEN` reader - Embedded host enable
pub type EHEN_R = crate::BitReader;
///Field `EHEN` writer - Embedded host enable
pub type EHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDSTS` reader - Connector ID status
pub type CIDSTS_R = crate::BitReader;
///Field `DBCT` reader - Long/short debounce time
pub type DBCT_R = crate::BitReader;
///Field `ASVLD` reader - A-session valid
pub type ASVLD_R = crate::BitReader;
///Field `BSVLD` reader - B-session valid
pub type BSVLD_R = crate::BitReader;
///Field `OTGVER` reader - OTG version
pub type OTGVER_R = crate::BitReader;
///Field `OTGVER` writer - OTG version
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CURMOD` reader - Current mode of operation
pub type CURMOD_R = crate::BitReader;
impl R {
    ///Bit 2 - Vless thansub>BUSless than/sub> valid override enable.
    #[inline(always)]
    pub fn vbvaloen(&self) -> VBVALOEN_R {
        VBVALOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Vless thansub>BUSless than/sub> valid override value.
    #[inline(always)]
    pub fn vbvaloval(&self) -> VBVALOVAL_R {
        VBVALOVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - A-peripheral session valid override enable.
    #[inline(always)]
    pub fn avaloen(&self) -> AVALOEN_R {
        AVALOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - A-peripheral session valid override value.
    #[inline(always)]
    pub fn avaloval(&self) -> AVALOVAL_R {
        AVALOVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - B-peripheral session valid override enable.
    #[inline(always)]
    pub fn bvaloen(&self) -> BVALOEN_R {
        BVALOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - B-peripheral session valid override value.
    #[inline(always)]
    pub fn bvaloval(&self) -> BVALOVAL_R {
        BVALOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - Embedded host enable
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Connector ID status
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Long/short debounce time
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - A-session valid
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - B-session valid
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - OTG version
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Current mode of operation
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGCTL")
            .field("vbvaloen", &self.vbvaloen())
            .field("vbvaloval", &self.vbvaloval())
            .field("avaloen", &self.avaloen())
            .field("avaloval", &self.avaloval())
            .field("bvaloen", &self.bvaloen())
            .field("bvaloval", &self.bvaloval())
            .field("ehen", &self.ehen())
            .field("cidsts", &self.cidsts())
            .field("dbct", &self.dbct())
            .field("asvld", &self.asvld())
            .field("bsvld", &self.bsvld())
            .field("otgver", &self.otgver())
            .field("curmod", &self.curmod())
            .finish()
    }
}
impl W {
    ///Bit 2 - Vless thansub>BUSless than/sub> valid override enable.
    #[inline(always)]
    pub fn vbvaloen(&mut self) -> VBVALOEN_W<GOTGCTLrs> {
        VBVALOEN_W::new(self, 2)
    }
    ///Bit 3 - Vless thansub>BUSless than/sub> valid override value.
    #[inline(always)]
    pub fn vbvaloval(&mut self) -> VBVALOVAL_W<GOTGCTLrs> {
        VBVALOVAL_W::new(self, 3)
    }
    ///Bit 4 - A-peripheral session valid override enable.
    #[inline(always)]
    pub fn avaloen(&mut self) -> AVALOEN_W<GOTGCTLrs> {
        AVALOEN_W::new(self, 4)
    }
    ///Bit 5 - A-peripheral session valid override value.
    #[inline(always)]
    pub fn avaloval(&mut self) -> AVALOVAL_W<GOTGCTLrs> {
        AVALOVAL_W::new(self, 5)
    }
    ///Bit 6 - B-peripheral session valid override enable.
    #[inline(always)]
    pub fn bvaloen(&mut self) -> BVALOEN_W<GOTGCTLrs> {
        BVALOEN_W::new(self, 6)
    }
    ///Bit 7 - B-peripheral session valid override value.
    #[inline(always)]
    pub fn bvaloval(&mut self) -> BVALOVAL_W<GOTGCTLrs> {
        BVALOVAL_W::new(self, 7)
    }
    ///Bit 12 - Embedded host enable
    #[inline(always)]
    pub fn ehen(&mut self) -> EHEN_W<GOTGCTLrs> {
        EHEN_W::new(self, 12)
    }
    ///Bit 20 - OTG version
    #[inline(always)]
    pub fn otgver(&mut self) -> OTGVER_W<GOTGCTLrs> {
        OTGVER_W::new(self, 20)
    }
}
/**OTG control and status register

You can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#OTG1:GOTGCTL)*/
pub struct GOTGCTLrs;
impl crate::RegisterSpec for GOTGCTLrs {
    type Ux = u32;
}
///`read()` method returns [`gotgctl::R`](R) reader structure
impl crate::Readable for GOTGCTLrs {}
///`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure
impl crate::Writable for GOTGCTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GOTGCTL to value 0x0001_0000
impl crate::Resettable for GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
