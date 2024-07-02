///Register `OTG_GOTGINT` reader
pub type R = crate::R<OTG_GOTGINTrs>;
///Register `OTG_GOTGINT` writer
pub type W = crate::W<OTG_GOTGINTrs>;
///Field `SEDET` reader - SEDET
pub type SEDET_R = crate::BitReader;
///Field `SEDET` writer - SEDET
pub type SEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRSSCHG` reader - SRSSCHG
pub type SRSSCHG_R = crate::BitReader;
///Field `SRSSCHG` writer - SRSSCHG
pub type SRSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNSSCHG` reader - HNSSCHG
pub type HNSSCHG_R = crate::BitReader;
///Field `HNSSCHG` writer - HNSSCHG
pub type HNSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNGDET` reader - HNGDET
pub type HNGDET_R = crate::BitReader;
///Field `HNGDET` writer - HNGDET
pub type HNGDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADTOCHG` reader - ADTOCHG
pub type ADTOCHG_R = crate::BitReader;
///Field `ADTOCHG` writer - ADTOCHG
pub type ADTOCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBCDNE` reader - DBCDNE
pub type DBCDNE_R = crate::BitReader;
///Field `DBCDNE` writer - DBCDNE
pub type DBCDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDCHNG` reader - IDCHNG
pub type IDCHNG_R = crate::BitReader;
///Field `IDCHNG` writer - IDCHNG
pub type IDCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - SEDET
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - SRSSCHG
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HNSSCHG
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - HNGDET
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ADTOCHG
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DBCDNE
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IDCHNG
    #[inline(always)]
    pub fn idchng(&self) -> IDCHNG_R {
        IDCHNG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_GOTGINT")
            .field("sedet", &self.sedet())
            .field("srsschg", &self.srsschg())
            .field("hnsschg", &self.hnsschg())
            .field("hngdet", &self.hngdet())
            .field("adtochg", &self.adtochg())
            .field("dbcdne", &self.dbcdne())
            .field("idchng", &self.idchng())
            .finish()
    }
}
impl W {
    ///Bit 2 - SEDET
    #[inline(always)]
    #[must_use]
    pub fn sedet(&mut self) -> SEDET_W<OTG_GOTGINTrs> {
        SEDET_W::new(self, 2)
    }
    ///Bit 8 - SRSSCHG
    #[inline(always)]
    #[must_use]
    pub fn srsschg(&mut self) -> SRSSCHG_W<OTG_GOTGINTrs> {
        SRSSCHG_W::new(self, 8)
    }
    ///Bit 9 - HNSSCHG
    #[inline(always)]
    #[must_use]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<OTG_GOTGINTrs> {
        HNSSCHG_W::new(self, 9)
    }
    ///Bit 17 - HNGDET
    #[inline(always)]
    #[must_use]
    pub fn hngdet(&mut self) -> HNGDET_W<OTG_GOTGINTrs> {
        HNGDET_W::new(self, 17)
    }
    ///Bit 18 - ADTOCHG
    #[inline(always)]
    #[must_use]
    pub fn adtochg(&mut self) -> ADTOCHG_W<OTG_GOTGINTrs> {
        ADTOCHG_W::new(self, 18)
    }
    ///Bit 19 - DBCDNE
    #[inline(always)]
    #[must_use]
    pub fn dbcdne(&mut self) -> DBCDNE_W<OTG_GOTGINTrs> {
        DBCDNE_W::new(self, 19)
    }
    ///Bit 20 - IDCHNG
    #[inline(always)]
    #[must_use]
    pub fn idchng(&mut self) -> IDCHNG_W<OTG_GOTGINTrs> {
        IDCHNG_W::new(self, 20)
    }
}
/**The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.

You can [`read`](crate::Reg::read) this register and get [`otg_gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:OTG_GOTGINT)*/
pub struct OTG_GOTGINTrs;
impl crate::RegisterSpec for OTG_GOTGINTrs {
    type Ux = u32;
}
///`read()` method returns [`otg_gotgint::R`](R) reader structure
impl crate::Readable for OTG_GOTGINTrs {}
///`write(|w| ..)` method takes [`otg_gotgint::W`](W) writer structure
impl crate::Writable for OTG_GOTGINTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTG_GOTGINT to value 0
impl crate::Resettable for OTG_GOTGINTrs {
    const RESET_VALUE: u32 = 0;
}
