///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `TSCE` reader - Touch sensing controller enable
pub type TSCE_R = crate::BitReader;
///Field `TSCE` writer - Touch sensing controller enable
pub type TSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START` reader - Start a new acquisition
pub type START_R = crate::BitReader;
///Field `START` writer - Start a new acquisition
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AM` reader - Acquisition mode
pub type AM_R = crate::BitReader;
///Field `AM` writer - Acquisition mode
pub type AM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCPOL` reader - Synchronization pin polarity
pub type SYNCPOL_R = crate::BitReader;
///Field `SYNCPOL` writer - Synchronization pin polarity
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IODEF` reader - I/O Default mode
pub type IODEF_R = crate::BitReader;
///Field `IODEF` writer - I/O Default mode
pub type IODEF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCV` reader - Max count value
pub type MCV_R = crate::FieldReader;
///Field `MCV` writer - Max count value
pub type MCV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PGPSC` reader - pulse generator prescaler
pub type PGPSC_R = crate::FieldReader;
///Field `PGPSC` writer - pulse generator prescaler
pub type PGPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SSPSC` reader - Spread spectrum prescaler
pub type SSPSC_R = crate::BitReader;
///Field `SSPSC` writer - Spread spectrum prescaler
pub type SSPSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSE` reader - Spread spectrum enable
pub type SSE_R = crate::BitReader;
///Field `SSE` writer - Spread spectrum enable
pub type SSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSD` reader - Spread spectrum deviation
pub type SSD_R = crate::FieldReader;
///Field `SSD` writer - Spread spectrum deviation
pub type SSD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CTPL` reader - Charge transfer pulse low
pub type CTPL_R = crate::FieldReader;
///Field `CTPL` writer - Charge transfer pulse low
pub type CTPL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CTPH` reader - Charge transfer pulse high
pub type CTPH_R = crate::FieldReader;
///Field `CTPH` writer - Charge transfer pulse high
pub type CTPH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Touch sensing controller enable
    #[inline(always)]
    pub fn tsce(&self) -> TSCE_R {
        TSCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start a new acquisition
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Acquisition mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization pin polarity
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O Default mode
    #[inline(always)]
    pub fn iodef(&self) -> IODEF_R {
        IODEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Max count value
    #[inline(always)]
    pub fn mcv(&self) -> MCV_R {
        MCV_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 12:14 - pulse generator prescaler
    #[inline(always)]
    pub fn pgpsc(&self) -> PGPSC_R {
        PGPSC_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Spread spectrum prescaler
    #[inline(always)]
    pub fn sspsc(&self) -> SSPSC_R {
        SSPSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Spread spectrum enable
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - Spread spectrum deviation
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bits 24:27 - Charge transfer pulse low
    #[inline(always)]
    pub fn ctpl(&self) -> CTPL_R {
        CTPL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Charge transfer pulse high
    #[inline(always)]
    pub fn ctph(&self) -> CTPH_R {
        CTPH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("ctph", &self.ctph())
            .field("ctpl", &self.ctpl())
            .field("ssd", &self.ssd())
            .field("sse", &self.sse())
            .field("sspsc", &self.sspsc())
            .field("pgpsc", &self.pgpsc())
            .field("mcv", &self.mcv())
            .field("iodef", &self.iodef())
            .field("syncpol", &self.syncpol())
            .field("am", &self.am())
            .field("start", &self.start())
            .field("tsce", &self.tsce())
            .finish()
    }
}
impl W {
    ///Bit 0 - Touch sensing controller enable
    #[inline(always)]
    pub fn tsce(&mut self) -> TSCE_W<CRrs> {
        TSCE_W::new(self, 0)
    }
    ///Bit 1 - Start a new acquisition
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    ///Bit 2 - Acquisition mode
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<CRrs> {
        AM_W::new(self, 2)
    }
    ///Bit 3 - Synchronization pin polarity
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CRrs> {
        SYNCPOL_W::new(self, 3)
    }
    ///Bit 4 - I/O Default mode
    #[inline(always)]
    pub fn iodef(&mut self) -> IODEF_W<CRrs> {
        IODEF_W::new(self, 4)
    }
    ///Bits 5:7 - Max count value
    #[inline(always)]
    pub fn mcv(&mut self) -> MCV_W<CRrs> {
        MCV_W::new(self, 5)
    }
    ///Bits 12:14 - pulse generator prescaler
    #[inline(always)]
    pub fn pgpsc(&mut self) -> PGPSC_W<CRrs> {
        PGPSC_W::new(self, 12)
    }
    ///Bit 15 - Spread spectrum prescaler
    #[inline(always)]
    pub fn sspsc(&mut self) -> SSPSC_W<CRrs> {
        SSPSC_W::new(self, 15)
    }
    ///Bit 16 - Spread spectrum enable
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W<CRrs> {
        SSE_W::new(self, 16)
    }
    ///Bits 17:23 - Spread spectrum deviation
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W<CRrs> {
        SSD_W::new(self, 17)
    }
    ///Bits 24:27 - Charge transfer pulse low
    #[inline(always)]
    pub fn ctpl(&mut self) -> CTPL_W<CRrs> {
        CTPL_W::new(self, 24)
    }
    ///Bits 28:31 - Charge transfer pulse high
    #[inline(always)]
    pub fn ctph(&mut self) -> CTPH_W<CRrs> {
        CTPH_W::new(self, 28)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TSC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
