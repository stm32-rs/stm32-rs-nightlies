///Register `GCR` reader
pub type R = crate::R<GCRrs>;
///Register `GCR` writer
pub type W = crate::W<GCRrs>;
///Field `LTDCEN` reader - LTDC global enable
pub type LTDCEN_R = crate::BitReader;
///Field `LTDCEN` writer - LTDC global enable
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GAMEN` reader - Gamma correction enable
pub type GAMEN_R = crate::BitReader;
///Field `GAMEN` writer - Gamma correction enable
pub type GAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBW` reader - dither blue width
pub type DBW_R = crate::FieldReader;
///Field `DGW` reader - dither green width
pub type DGW_R = crate::FieldReader;
///Field `DRW` reader - dither red width
pub type DRW_R = crate::FieldReader;
///Field `DEN` reader - dither enable
pub type DEN_R = crate::BitReader;
///Field `DEN` writer - dither enable
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC enable
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFEN` reader - single-frame mode: mode enable
pub type SFEN_R = crate::BitReader;
///Field `SFEN` writer - single-frame mode: mode enable
pub type SFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SFSWTR` writer - single-frame mode: software trigger
pub type SFSWTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCPOL` reader - pixel clock polarity
pub type PCPOL_R = crate::BitReader;
///Field `PCPOL` writer - pixel clock polarity
pub type PCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEPOL` reader - blanking (no data/pixel) polarity
pub type DEPOL_R = crate::BitReader;
///Field `DEPOL` writer - blanking (no data/pixel) polarity
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - vertical synchronization polarity
pub type VSPOL_R = crate::BitReader;
///Field `VSPOL` writer - vertical synchronization polarity
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPOL` reader - horizontal synchronization polarity
pub type HSPOL_R = crate::BitReader;
///Field `HSPOL` writer - horizontal synchronization polarity
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LTDC global enable
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Gamma correction enable
    #[inline(always)]
    pub fn gamen(&self) -> GAMEN_R {
        GAMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 4:6 - dither blue width
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - dither green width
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - dither red width
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - dither enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - CRC enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - single-frame mode: mode enable
    #[inline(always)]
    pub fn sfen(&self) -> SFEN_R {
        SFEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - pixel clock polarity
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - blanking (no data/pixel) polarity
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("ltdcen", &self.ltdcen())
            .field("gamen", &self.gamen())
            .field("dbw", &self.dbw())
            .field("dgw", &self.dgw())
            .field("drw", &self.drw())
            .field("den", &self.den())
            .field("crcen", &self.crcen())
            .field("sfen", &self.sfen())
            .field("pcpol", &self.pcpol())
            .field("depol", &self.depol())
            .field("vspol", &self.vspol())
            .field("hspol", &self.hspol())
            .finish()
    }
}
impl W {
    ///Bit 0 - LTDC global enable
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<'_, GCRrs> {
        LTDCEN_W::new(self, 0)
    }
    ///Bit 1 - Gamma correction enable
    #[inline(always)]
    pub fn gamen(&mut self) -> GAMEN_W<'_, GCRrs> {
        GAMEN_W::new(self, 1)
    }
    ///Bit 16 - dither enable
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W<'_, GCRrs> {
        DEN_W::new(self, 16)
    }
    ///Bit 19 - CRC enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, GCRrs> {
        CRCEN_W::new(self, 19)
    }
    ///Bit 24 - single-frame mode: mode enable
    #[inline(always)]
    pub fn sfen(&mut self) -> SFEN_W<'_, GCRrs> {
        SFEN_W::new(self, 24)
    }
    ///Bit 25 - single-frame mode: software trigger
    #[inline(always)]
    pub fn sfswtr(&mut self) -> SFSWTR_W<'_, GCRrs> {
        SFSWTR_W::new(self, 25)
    }
    ///Bit 28 - pixel clock polarity
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W<'_, GCRrs> {
        PCPOL_W::new(self, 28)
    }
    ///Bit 29 - blanking (no data/pixel) polarity
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W<'_, GCRrs> {
        DEPOL_W::new(self, 29)
    }
    ///Bit 30 - vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W<'_, GCRrs> {
        VSPOL_W::new(self, 30)
    }
    ///Bit 31 - horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W<'_, GCRrs> {
        HSPOL_W::new(self, 31)
    }
}
/**LTDC global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:GCR)*/
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
///`read()` method returns [`gcr::R`](R) reader structure
impl crate::Readable for GCRrs {}
///`write(|w| ..)` method takes [`gcr::W`](W) writer structure
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GCR to value 0x2220
impl crate::Resettable for GCRrs {
    const RESET_VALUE: u32 = 0x2220;
}
