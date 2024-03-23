#[doc = "Register `ACR1` reader"]
pub type R = crate::R<ACR1rs>;
#[doc = "Register `ACR1` writer"]
pub type W = crate::W<ACR1rs>;
#[doc = "Field `MODE` reader - Audio block mode"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Audio block mode"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRTCFG` reader - Protocol configuration"]
pub type PRTCFG_R = crate::FieldReader;
#[doc = "Field `PRTCFG` writer - Protocol configuration"]
pub type PRTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DS` reader - Data size"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `DS` writer - Data size"]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LSBFIRST` reader - Least significant bit first"]
pub type LSBFIRST_R = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - Least significant bit first"]
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTR` reader - Clock strobing edge"]
pub type CKSTR_R = crate::BitReader;
#[doc = "Field `CKSTR` writer - Clock strobing edge"]
pub type CKSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Synchronization enable"]
pub type SYNCEN_R = crate::FieldReader;
#[doc = "Field `SYNCEN` writer - Synchronization enable"]
pub type SYNCEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONO` reader - Mono mode"]
pub type MONO_R = crate::BitReader;
#[doc = "Field `MONO` writer - Mono mode"]
pub type MONO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDRIV` reader - Output drive"]
pub type OUTDRIV_R = crate::BitReader;
#[doc = "Field `OUTDRIV` writer - Output drive"]
pub type OUTDRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAIAEN` reader - Audio block A enable"]
pub type SAIAEN_R = crate::BitReader;
#[doc = "Field `SAIAEN` writer - Audio block A enable"]
pub type SAIAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NODIV` reader - No divider"]
pub type NODIV_R = crate::BitReader;
#[doc = "Field `NODIV` writer - No divider"]
pub type NODIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKDIV` reader - Master clock divider"]
pub type MCKDIV_R = crate::FieldReader;
#[doc = "Field `MCKDIV` writer - Master clock divider"]
pub type MCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `OSR` reader - OSR"]
pub type OSR_R = crate::BitReader;
#[doc = "Field `OSR` writer - OSR"]
pub type OSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKEN` reader - MCKEN"]
pub type MCKEN_R = crate::BitReader;
#[doc = "Field `MCKEN` writer - MCKEN"]
pub type MCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saiaen(&self) -> SAIAEN_R {
        SAIAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - Master clock divider"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - OSR"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MCKEN"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<ACR1rs> {
        MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prtcfg(&mut self) -> PRTCFG_W<ACR1rs> {
        PRTCFG_W::new(self, 2)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<ACR1rs> {
        DS_W::new(self, 5)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<ACR1rs> {
        LSBFIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    #[must_use]
    pub fn ckstr(&mut self) -> CKSTR_W<ACR1rs> {
        CKSTR_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<ACR1rs> {
        SYNCEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<ACR1rs> {
        MONO_W::new(self, 12)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    #[must_use]
    pub fn outdriv(&mut self) -> OUTDRIV_W<ACR1rs> {
        OUTDRIV_W::new(self, 13)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    #[must_use]
    pub fn saiaen(&mut self) -> SAIAEN_W<ACR1rs> {
        SAIAEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<ACR1rs> {
        DMAEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    #[must_use]
    pub fn nodiv(&mut self) -> NODIV_W<ACR1rs> {
        NODIV_W::new(self, 19)
    }
    #[doc = "Bits 20:25 - Master clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<ACR1rs> {
        MCKDIV_W::new(self, 20)
    }
    #[doc = "Bit 26 - OSR"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<ACR1rs> {
        OSR_W::new(self, 26)
    }
    #[doc = "Bit 27 - MCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<ACR1rs> {
        MCKEN_W::new(self, 27)
    }
}
#[doc = "A Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR1rs;
impl crate::RegisterSpec for ACR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr1::R`](R) reader structure"]
impl crate::Readable for ACR1rs {}
#[doc = "`write(|w| ..)` method takes [`acr1::W`](W) writer structure"]
impl crate::Writable for ACR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR1 to value 0x40"]
impl crate::Resettable for ACR1rs {
    const RESET_VALUE: u32 = 0x40;
}
