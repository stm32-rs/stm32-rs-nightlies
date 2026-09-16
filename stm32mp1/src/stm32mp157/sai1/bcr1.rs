///Register `BCR1` reader
pub type R = crate::R<BCR1rs>;
///Register `BCR1` writer
pub type W = crate::W<BCR1rs>;
///Field `MODE` reader - MODE
pub type MODE_R = crate::FieldReader;
///Field `MODE` writer - MODE
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRTCFG` reader - PRTCFG
pub type PRTCFG_R = crate::FieldReader;
///Field `PRTCFG` writer - PRTCFG
pub type PRTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DS` reader - DS
pub type DS_R = crate::FieldReader;
///Field `DS` writer - DS
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LSBFIRST` reader - LSBFIRST
pub type LSBFIRST_R = crate::BitReader;
///Field `LSBFIRST` writer - LSBFIRST
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKSTR` reader - CKSTR
pub type CKSTR_R = crate::BitReader;
///Field `CKSTR` writer - CKSTR
pub type CKSTR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCEN` reader - SYNCEN
pub type SYNCEN_R = crate::FieldReader;
///Field `SYNCEN` writer - SYNCEN
pub type SYNCEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MONO` reader - MONO
pub type MONO_R = crate::BitReader;
///Field `MONO` writer - MONO
pub type MONO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTDRIV` reader - OUTDRIV
pub type OUTDRIV_R = crate::BitReader;
///Field `OUTDRIV` writer - OUTDRIV
pub type OUTDRIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAIEN` reader - SAIEN
pub type SAIEN_R = crate::BitReader;
///Field `SAIEN` writer - SAIEN
pub type SAIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAEN` reader - DMAEN
pub type DMAEN_R = crate::BitReader;
///Field `DMAEN` writer - DMAEN
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NODIV` reader - NODIV
pub type NODIV_R = crate::BitReader;
///Field `NODIV` writer - NODIV
pub type NODIV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKDIV` reader - MCKDIV
pub type MCKDIV_R = crate::FieldReader;
///Field `MCKDIV` writer - MCKDIV
pub type MCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `OSR` reader - OSR
pub type OSR_R = crate::BitReader;
///Field `OSR` writer - OSR
pub type OSR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCKEN` reader - MCKEN
pub type MCKEN_R = crate::BitReader;
///Field `MCKEN` writer - MCKEN
pub type MCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - MODE
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PRTCFG
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:7 - DS
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - LSBFIRST
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CKSTR
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - SYNCEN
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - MONO
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - OUTDRIV
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - SAIEN
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - NODIV
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - MCKDIV
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - OSR
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MCKEN
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR1")
            .field("mode", &self.mode())
            .field("prtcfg", &self.prtcfg())
            .field("ds", &self.ds())
            .field("lsbfirst", &self.lsbfirst())
            .field("ckstr", &self.ckstr())
            .field("syncen", &self.syncen())
            .field("mono", &self.mono())
            .field("outdriv", &self.outdriv())
            .field("saien", &self.saien())
            .field("dmaen", &self.dmaen())
            .field("nodiv", &self.nodiv())
            .field("mckdiv", &self.mckdiv())
            .field("osr", &self.osr())
            .field("mcken", &self.mcken())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - MODE
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<'_, BCR1rs> {
        MODE_W::new(self, 0)
    }
    ///Bits 2:3 - PRTCFG
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PRTCFG_W<'_, BCR1rs> {
        PRTCFG_W::new(self, 2)
    }
    ///Bits 5:7 - DS
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<'_, BCR1rs> {
        DS_W::new(self, 5)
    }
    ///Bit 8 - LSBFIRST
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<'_, BCR1rs> {
        LSBFIRST_W::new(self, 8)
    }
    ///Bit 9 - CKSTR
    #[inline(always)]
    pub fn ckstr(&mut self) -> CKSTR_W<'_, BCR1rs> {
        CKSTR_W::new(self, 9)
    }
    ///Bits 10:11 - SYNCEN
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W<'_, BCR1rs> {
        SYNCEN_W::new(self, 10)
    }
    ///Bit 12 - MONO
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W<'_, BCR1rs> {
        MONO_W::new(self, 12)
    }
    ///Bit 13 - OUTDRIV
    #[inline(always)]
    pub fn outdriv(&mut self) -> OUTDRIV_W<'_, BCR1rs> {
        OUTDRIV_W::new(self, 13)
    }
    ///Bit 16 - SAIEN
    #[inline(always)]
    pub fn saien(&mut self) -> SAIEN_W<'_, BCR1rs> {
        SAIEN_W::new(self, 16)
    }
    ///Bit 17 - DMAEN
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, BCR1rs> {
        DMAEN_W::new(self, 17)
    }
    ///Bit 19 - NODIV
    #[inline(always)]
    pub fn nodiv(&mut self) -> NODIV_W<'_, BCR1rs> {
        NODIV_W::new(self, 19)
    }
    ///Bits 20:25 - MCKDIV
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MCKDIV_W<'_, BCR1rs> {
        MCKDIV_W::new(self, 20)
    }
    ///Bit 26 - OSR
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W<'_, BCR1rs> {
        OSR_W::new(self, 26)
    }
    ///Bit 27 - MCKEN
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W<'_, BCR1rs> {
        MCKEN_W::new(self, 27)
    }
}
/**Configuration register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SAI1:BCR1)*/
pub struct BCR1rs;
impl crate::RegisterSpec for BCR1rs {
    type Ux = u32;
}
///`read()` method returns [`bcr1::R`](R) reader structure
impl crate::Readable for BCR1rs {}
///`write(|w| ..)` method takes [`bcr1::W`](W) writer structure
impl crate::Writable for BCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR1 to value 0x40
impl crate::Resettable for BCR1rs {
    const RESET_VALUE: u32 = 0x40;
}
