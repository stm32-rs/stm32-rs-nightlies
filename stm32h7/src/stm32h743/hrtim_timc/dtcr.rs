///Register `DTCR` reader
pub type R = crate::R<DTCRrs>;
///Register `DTCR` writer
pub type W = crate::W<DTCRrs>;
///Field `DTRx` reader - Deadtime Rising value
pub type DTRX_R = crate::FieldReader<u16>;
///Field `DTRx` writer - Deadtime Rising value
pub type DTRX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SDTRx` reader - Sign Deadtime Rising value
pub type SDTRX_R = crate::BitReader;
///Field `SDTRx` writer - Sign Deadtime Rising value
pub type SDTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTPRSC` reader - Deadtime Prescaler
pub type DTPRSC_R = crate::FieldReader;
///Field `DTPRSC` writer - Deadtime Prescaler
pub type DTPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DTRSLKx` reader - Deadtime Rising Sign Lock
pub type DTRSLKX_R = crate::BitReader;
///Field `DTRSLKx` writer - Deadtime Rising Sign Lock
pub type DTRSLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTRLKx` reader - Deadtime Rising Lock
pub type DTRLKX_R = crate::BitReader;
///Field `DTRLKx` writer - Deadtime Rising Lock
pub type DTRLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTFx` reader - Deadtime Falling value
pub type DTFX_R = crate::FieldReader<u16>;
///Field `DTFx` writer - Deadtime Falling value
pub type DTFX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SDTFx` reader - Sign Deadtime Falling value
pub type SDTFX_R = crate::BitReader;
///Field `SDTFx` writer - Sign Deadtime Falling value
pub type SDTFX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTFSLKx` reader - Deadtime Falling Sign Lock
pub type DTFSLKX_R = crate::BitReader;
///Field `DTFSLKx` writer - Deadtime Falling Sign Lock
pub type DTFSLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTFLKx` reader - Deadtime Falling Lock
pub type DTFLKX_R = crate::BitReader;
///Field `DTFLKx` writer - Deadtime Falling Lock
pub type DTFLKX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCR")
            .field("dtflkx", &self.dtflkx())
            .field("dtfslkx", &self.dtfslkx())
            .field("sdtfx", &self.sdtfx())
            .field("dtfx", &self.dtfx())
            .field("dtrlkx", &self.dtrlkx())
            .field("dtrslkx", &self.dtrslkx())
            .field("dtprsc", &self.dtprsc())
            .field("sdtrx", &self.sdtrx())
            .field("dtrx", &self.dtrx())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<DTCRrs> {
        DTRX_W::new(self, 0)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<DTCRrs> {
        SDTRX_W::new(self, 9)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<DTCRrs> {
        DTPRSC_W::new(self, 10)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<DTCRrs> {
        DTRSLKX_W::new(self, 14)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<DTCRrs> {
        DTRLKX_W::new(self, 15)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<DTCRrs> {
        DTFX_W::new(self, 16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<DTCRrs> {
        SDTFX_W::new(self, 25)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<DTCRrs> {
        DTFSLKX_W::new(self, 30)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<DTCRrs> {
        DTFLKX_W::new(self, 31)
    }
}
/**Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_TIMC:DTCR)*/
pub struct DTCRrs;
impl crate::RegisterSpec for DTCRrs {
    type Ux = u32;
}
///`read()` method returns [`dtcr::R`](R) reader structure
impl crate::Readable for DTCRrs {}
///`write(|w| ..)` method takes [`dtcr::W`](W) writer structure
impl crate::Writable for DTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTCR to value 0
impl crate::Resettable for DTCRrs {
    const RESET_VALUE: u32 = 0;
}
