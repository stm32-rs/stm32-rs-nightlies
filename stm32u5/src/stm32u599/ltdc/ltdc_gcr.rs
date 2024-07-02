///Register `LTDC_GCR` reader
pub type R = crate::R<LTDC_GCRrs>;
///Register `LTDC_GCR` writer
pub type W = crate::W<LTDC_GCRrs>;
///Field `LTDCEN` reader - LCD-TFT controller enable This bit is set and cleared by software.
pub type LTDCEN_R = crate::BitReader;
///Field `LTDCEN` writer - LCD-TFT controller enable This bit is set and cleared by software.
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBW` reader - dither blue width These bits return the dither blue bits.
pub type DBW_R = crate::FieldReader;
///Field `DGW` reader - dither green width These bits return the dither green bits.
pub type DGW_R = crate::FieldReader;
///Field `DRW` reader - dither red width These bits return the Dither Red Bits.
pub type DRW_R = crate::FieldReader;
///Field `DEN` reader - dither enable This bit is set and cleared by software.
pub type DEN_R = crate::BitReader;
///Field `DEN` writer - dither enable This bit is set and cleared by software.
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCPOL` reader - pixel clock polarity This bit is set and cleared by software.
pub type PCPOL_R = crate::BitReader;
///Field `PCPOL` writer - pixel clock polarity This bit is set and cleared by software.
pub type PCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEPOL` reader - not data enable polarity This bit is set and cleared by software.
pub type DEPOL_R = crate::BitReader;
///Field `DEPOL` writer - not data enable polarity This bit is set and cleared by software.
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSPOL` reader - vertical synchronization polarity This bit is set and cleared by software.
pub type VSPOL_R = crate::BitReader;
///Field `VSPOL` writer - vertical synchronization polarity This bit is set and cleared by software.
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPOL` reader - horizontal synchronization polarity This bit is set and cleared by software.
pub type HSPOL_R = crate::BitReader;
///Field `HSPOL` writer - horizontal synchronization polarity This bit is set and cleared by software.
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LCD-TFT controller enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - dither blue width These bits return the dither blue bits.
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:10 - dither green width These bits return the dither green bits.
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 12:14 - dither red width These bits return the Dither Red Bits.
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 16 - dither enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - pixel clock polarity This bit is set and cleared by software.
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - not data enable polarity This bit is set and cleared by software.
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - vertical synchronization polarity This bit is set and cleared by software.
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - horizontal synchronization polarity This bit is set and cleared by software.
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_GCR")
            .field("ltdcen", &self.ltdcen())
            .field("dbw", &self.dbw())
            .field("dgw", &self.dgw())
            .field("drw", &self.drw())
            .field("den", &self.den())
            .field("pcpol", &self.pcpol())
            .field("depol", &self.depol())
            .field("vspol", &self.vspol())
            .field("hspol", &self.hspol())
            .finish()
    }
}
impl W {
    ///Bit 0 - LCD-TFT controller enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<LTDC_GCRrs> {
        LTDCEN_W::new(self, 0)
    }
    ///Bit 16 - dither enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<LTDC_GCRrs> {
        DEN_W::new(self, 16)
    }
    ///Bit 28 - pixel clock polarity This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PCPOL_W<LTDC_GCRrs> {
        PCPOL_W::new(self, 28)
    }
    ///Bit 29 - not data enable polarity This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<LTDC_GCRrs> {
        DEPOL_W::new(self, 29)
    }
    ///Bit 30 - vertical synchronization polarity This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<LTDC_GCRrs> {
        VSPOL_W::new(self, 30)
    }
    ///Bit 31 - horizontal synchronization polarity This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<LTDC_GCRrs> {
        HSPOL_W::new(self, 31)
    }
}
/**LTDC global control register

You can [`read`](crate::Reg::read) this register and get [`ltdc_gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_GCR)*/
pub struct LTDC_GCRrs;
impl crate::RegisterSpec for LTDC_GCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_gcr::R`](R) reader structure
impl crate::Readable for LTDC_GCRrs {}
///`write(|w| ..)` method takes [`ltdc_gcr::W`](W) writer structure
impl crate::Writable for LTDC_GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_GCR to value 0x2220
impl crate::Resettable for LTDC_GCRrs {
    const RESET_VALUE: u32 = 0x2220;
}
