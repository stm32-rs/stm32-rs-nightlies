///Register `AUTOCR` reader
pub type R = crate::R<AUTOCRrs>;
///Register `AUTOCR` writer
pub type W = crate::W<AUTOCRrs>;
///Field `TRIGSEL` reader - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
pub type TRIGSEL_R = crate::FieldReader;
///Field `TRIGSEL` writer - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TRIGPOL` reader - trigger polarity Note: This bit can be written only when SPE = 0.
pub type TRIGPOL_R = crate::BitReader;
///Field `TRIGPOL` writer - trigger polarity Note: This bit can be written only when SPE = 0.
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRIGEN` reader - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
pub type TRIGEN_R = crate::BitReader;
///Field `TRIGEN` writer - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:19 - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - trigger polarity Note: This bit can be written only when SPE = 0.
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOCR")
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("trigen", &self.trigen())
            .finish()
    }
}
impl W {
    ///Bits 16:19 - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<AUTOCRrs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bit 20 - trigger polarity Note: This bit can be written only when SPE = 0.
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<AUTOCRrs> {
        TRIGPOL_W::new(self, 20)
    }
    ///Bit 21 - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<AUTOCRrs> {
        TRIGEN_W::new(self, 21)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`autocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#SPI1:AUTOCR)*/
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
///`read()` method returns [`autocr::R`](R) reader structure
impl crate::Readable for AUTOCRrs {}
///`write(|w| ..)` method takes [`autocr::W`](W) writer structure
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AUTOCR to value 0
impl crate::Resettable for AUTOCRrs {}
