#[doc = "Register `BCR4` reader"]
pub type R = crate::R<BCR4rs>;
#[doc = "Register `BCR4` writer"]
pub type W = crate::W<BCR4rs>;
#[doc = "Field `MBKEN` reader - MBKEN"]
pub type MBKEN_R = crate::BitReader;
#[doc = "Field `MBKEN` writer - MBKEN"]
pub type MBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUXEN` reader - MUXEN"]
pub type MUXEN_R = crate::BitReader;
#[doc = "Field `MUXEN` writer - MUXEN"]
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTYP` reader - MTYP"]
pub type MTYP_R = crate::FieldReader;
#[doc = "Field `MTYP` writer - MTYP"]
pub type MTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWID` reader - MWID"]
pub type MWID_R = crate::FieldReader;
#[doc = "Field `MWID` writer - MWID"]
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FACCEN` reader - FACCEN"]
pub type FACCEN_R = crate::BitReader;
#[doc = "Field `FACCEN` writer - FACCEN"]
pub type FACCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURSTEN` reader - BURSTEN"]
pub type BURSTEN_R = crate::BitReader;
#[doc = "Field `BURSTEN` writer - BURSTEN"]
pub type BURSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITPOL` reader - WAITPOL"]
pub type WAITPOL_R = crate::BitReader;
#[doc = "Field `WAITPOL` writer - WAITPOL"]
pub type WAITPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRAPMOD` reader - WRAPMOD"]
pub type WRAPMOD_R = crate::BitReader;
#[doc = "Field `WRAPMOD` writer - WRAPMOD"]
pub type WRAPMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITCFG` reader - WAITCFG"]
pub type WAITCFG_R = crate::BitReader;
#[doc = "Field `WAITCFG` writer - WAITCFG"]
pub type WAITCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WREN` reader - WREN"]
pub type WREN_R = crate::BitReader;
#[doc = "Field `WREN` writer - WREN"]
pub type WREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAITEN` reader - WAITEN"]
pub type WAITEN_R = crate::BitReader;
#[doc = "Field `WAITEN` writer - WAITEN"]
pub type WAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTMOD` reader - EXTMOD"]
pub type EXTMOD_R = crate::BitReader;
#[doc = "Field `EXTMOD` writer - EXTMOD"]
pub type EXTMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCWAIT` reader - ASYNCWAIT"]
pub type ASYNCWAIT_R = crate::BitReader;
#[doc = "Field `ASYNCWAIT` writer - ASYNCWAIT"]
pub type ASYNCWAIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBURSTRW` reader - CBURSTRW"]
pub type CBURSTRW_R = crate::BitReader;
#[doc = "Field `CBURSTRW` writer - CBURSTRW"]
pub type CBURSTRW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<BCR4rs> {
        MBKEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<BCR4rs> {
        MUXEN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<BCR4rs> {
        MTYP_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<BCR4rs> {
        MWID_W::new(self, 4)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<BCR4rs> {
        FACCEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR4rs> {
        BURSTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR4rs> {
        WAITPOL_W::new(self, 9)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn wrapmod(&mut self) -> WRAPMOD_W<BCR4rs> {
        WRAPMOD_W::new(self, 10)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR4rs> {
        WAITCFG_W::new(self, 11)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<BCR4rs> {
        WREN_W::new(self, 12)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<BCR4rs> {
        WAITEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR4rs> {
        EXTMOD_W::new(self, 14)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR4rs> {
        ASYNCWAIT_W::new(self, 15)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR4rs> {
        CBURSTRW_W::new(self, 19)
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR4rs;
impl crate::RegisterSpec for BCR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr4::R`](R) reader structure"]
impl crate::Readable for BCR4rs {}
#[doc = "`write(|w| ..)` method takes [`bcr4::W`](W) writer structure"]
impl crate::Writable for BCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCR4 to value 0x30d0"]
impl crate::Resettable for BCR4rs {
    const RESET_VALUE: u32 = 0x30d0;
}
