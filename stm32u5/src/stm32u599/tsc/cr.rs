#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `TSCE` reader - Touch sensing controller enable"]
pub type TSCE_R = crate::BitReader;
#[doc = "Field `TSCE` writer - Touch sensing controller enable"]
pub type TSCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start a new acquisition"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start a new acquisition"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AM` reader - Acquisition mode"]
pub type AM_R = crate::BitReader;
#[doc = "Field `AM` writer - Acquisition mode"]
pub type AM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCPOL` reader - Synchronization pin polarity"]
pub type SYNCPOL_R = crate::BitReader;
#[doc = "Field `SYNCPOL` writer - Synchronization pin polarity"]
pub type SYNCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IODEF` reader - I/O Default mode"]
pub type IODEF_R = crate::BitReader;
#[doc = "Field `IODEF` writer - I/O Default mode"]
pub type IODEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCV` reader - Max count value"]
pub type MCV_R = crate::FieldReader;
#[doc = "Field `MCV` writer - Max count value"]
pub type MCV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PGPSC` reader - pulse generator prescaler"]
pub type PGPSC_R = crate::FieldReader;
#[doc = "Field `PGPSC` writer - pulse generator prescaler"]
pub type PGPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SSPSC` reader - Spread spectrum prescaler"]
pub type SSPSC_R = crate::BitReader;
#[doc = "Field `SSPSC` writer - Spread spectrum prescaler"]
pub type SSPSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSE` reader - Spread spectrum enable"]
pub type SSE_R = crate::BitReader;
#[doc = "Field `SSE` writer - Spread spectrum enable"]
pub type SSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSD` reader - Spread spectrum deviation"]
pub type SSD_R = crate::FieldReader;
#[doc = "Field `SSD` writer - Spread spectrum deviation"]
pub type SSD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CTPL` reader - Charge transfer pulse low"]
pub type CTPL_R = crate::FieldReader;
#[doc = "Field `CTPL` writer - Charge transfer pulse low"]
pub type CTPL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTPH` reader - Charge transfer pulse high"]
pub type CTPH_R = crate::FieldReader;
#[doc = "Field `CTPH` writer - Charge transfer pulse high"]
pub type CTPH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Touch sensing controller enable"]
    #[inline(always)]
    pub fn tsce(&self) -> TSCE_R {
        TSCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a new acquisition"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acquisition mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization pin polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O Default mode"]
    #[inline(always)]
    pub fn iodef(&self) -> IODEF_R {
        IODEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max count value"]
    #[inline(always)]
    pub fn mcv(&self) -> MCV_R {
        MCV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 12:14 - pulse generator prescaler"]
    #[inline(always)]
    pub fn pgpsc(&self) -> PGPSC_R {
        PGPSC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler"]
    #[inline(always)]
    pub fn sspsc(&self) -> SSPSC_R {
        SSPSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Spread spectrum enable"]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation"]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low"]
    #[inline(always)]
    pub fn ctpl(&self) -> CTPL_R {
        CTPL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Charge transfer pulse high"]
    #[inline(always)]
    pub fn ctph(&self) -> CTPH_R {
        CTPH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Touch sensing controller enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsce(&mut self) -> TSCE_W<CRrs> {
        TSCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start a new acquisition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CRrs> {
        START_W::new(self, 1)
    }
    #[doc = "Bit 2 - Acquisition mode"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<CRrs> {
        AM_W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronization pin polarity"]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<CRrs> {
        SYNCPOL_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O Default mode"]
    #[inline(always)]
    #[must_use]
    pub fn iodef(&mut self) -> IODEF_W<CRrs> {
        IODEF_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Max count value"]
    #[inline(always)]
    #[must_use]
    pub fn mcv(&mut self) -> MCV_W<CRrs> {
        MCV_W::new(self, 5)
    }
    #[doc = "Bits 12:14 - pulse generator prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pgpsc(&mut self) -> PGPSC_W<CRrs> {
        PGPSC_W::new(self, 12)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn sspsc(&mut self) -> SSPSC_W<CRrs> {
        SSPSC_W::new(self, 15)
    }
    #[doc = "Bit 16 - Spread spectrum enable"]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<CRrs> {
        SSE_W::new(self, 16)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation"]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<CRrs> {
        SSD_W::new(self, 17)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low"]
    #[inline(always)]
    #[must_use]
    pub fn ctpl(&mut self) -> CTPL_W<CRrs> {
        CTPL_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Charge transfer pulse high"]
    #[inline(always)]
    #[must_use]
    pub fn ctph(&mut self) -> CTPH_W<CRrs> {
        CTPH_W::new(self, 28)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
