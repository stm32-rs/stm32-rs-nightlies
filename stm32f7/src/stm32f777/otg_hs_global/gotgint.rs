///Register `GOTGINT` reader
pub type R = crate::R<GOTGINTrs>;
///Register `GOTGINT` writer
pub type W = crate::W<GOTGINTrs>;
///Field `SEDET` reader - Session end detected
pub type SEDET_R = crate::BitReader;
///Field `SEDET` writer - Session end detected
pub type SEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRSSCHG` reader - Session request success status change
pub type SRSSCHG_R = crate::BitReader;
///Field `SRSSCHG` writer - Session request success status change
pub type SRSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNSSCHG` reader - Host negotiation success status change
pub type HNSSCHG_R = crate::BitReader;
///Field `HNSSCHG` writer - Host negotiation success status change
pub type HNSSCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNGDET` reader - Host negotiation detected
pub type HNGDET_R = crate::BitReader;
///Field `HNGDET` writer - Host negotiation detected
pub type HNGDET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADTOCHG` reader - A-device timeout change
pub type ADTOCHG_R = crate::BitReader;
///Field `ADTOCHG` writer - A-device timeout change
pub type ADTOCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBCDNE` reader - Debounce done
pub type DBCDNE_R = crate::BitReader;
///Field `DBCDNE` writer - Debounce done
pub type DBCDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDCHNG` reader - ID input pin changed
pub type IDCHNG_R = crate::BitReader;
///Field `IDCHNG` writer - ID input pin changed
pub type IDCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Session end detected
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Session request success status change
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Host negotiation success status change
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 17 - Host negotiation detected
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - A-device timeout change
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Debounce done
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ID input pin changed
    #[inline(always)]
    pub fn idchng(&self) -> IDCHNG_R {
        IDCHNG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
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
    ///Bit 2 - Session end detected
    #[inline(always)]
    pub fn sedet(&mut self) -> SEDET_W<'_, GOTGINTrs> {
        SEDET_W::new(self, 2)
    }
    ///Bit 8 - Session request success status change
    #[inline(always)]
    pub fn srsschg(&mut self) -> SRSSCHG_W<'_, GOTGINTrs> {
        SRSSCHG_W::new(self, 8)
    }
    ///Bit 9 - Host negotiation success status change
    #[inline(always)]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<'_, GOTGINTrs> {
        HNSSCHG_W::new(self, 9)
    }
    ///Bit 17 - Host negotiation detected
    #[inline(always)]
    pub fn hngdet(&mut self) -> HNGDET_W<'_, GOTGINTrs> {
        HNGDET_W::new(self, 17)
    }
    ///Bit 18 - A-device timeout change
    #[inline(always)]
    pub fn adtochg(&mut self) -> ADTOCHG_W<'_, GOTGINTrs> {
        ADTOCHG_W::new(self, 18)
    }
    ///Bit 19 - Debounce done
    #[inline(always)]
    pub fn dbcdne(&mut self) -> DBCDNE_W<'_, GOTGINTrs> {
        DBCDNE_W::new(self, 19)
    }
    ///Bit 20 - ID input pin changed
    #[inline(always)]
    pub fn idchng(&mut self) -> IDCHNG_W<'_, GOTGINTrs> {
        IDCHNG_W::new(self, 20)
    }
}
/**OTG_HS interrupt register

You can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#OTG_HS_GLOBAL:GOTGINT)*/
pub struct GOTGINTrs;
impl crate::RegisterSpec for GOTGINTrs {
    type Ux = u32;
}
///`read()` method returns [`gotgint::R`](R) reader structure
impl crate::Readable for GOTGINTrs {}
///`write(|w| ..)` method takes [`gotgint::W`](W) writer structure
impl crate::Writable for GOTGINTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GOTGINT to value 0
impl crate::Resettable for GOTGINTrs {}
