#[doc = "Register `NSCR` reader"]
pub type R = crate::R<NSCRrs>;
#[doc = "Register `NSCR` writer"]
pub type W = crate::W<NSCRrs>;
#[doc = "Field `NSPG` reader - NSPG"]
pub type NSPG_R = crate::BitReader;
#[doc = "Field `NSPG` writer - NSPG"]
pub type NSPG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPER` reader - NSPER"]
pub type NSPER_R = crate::BitReader;
#[doc = "Field `NSPER` writer - NSPER"]
pub type NSPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSMER1` reader - NSMER1"]
pub type NSMER1_R = crate::BitReader;
#[doc = "Field `NSMER1` writer - NSMER1"]
pub type NSMER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSPNB` reader - NSPNB"]
pub type NSPNB_R = crate::FieldReader;
#[doc = "Field `NSPNB` writer - NSPNB"]
pub type NSPNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NSBKER` reader - NSBKER"]
pub type NSBKER_R = crate::BitReader;
#[doc = "Field `NSBKER` writer - NSBKER"]
pub type NSBKER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSMER2` reader - NSMER2"]
pub type NSMER2_R = crate::BitReader;
#[doc = "Field `NSMER2` writer - NSMER2"]
pub type NSMER2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSTRT` reader - Options modification start"]
pub type NSSTRT_R = crate::BitReader;
#[doc = "Field `NSSTRT` writer - Options modification start"]
pub type NSSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub type OPTSTRT_R = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSEOPIE` reader - NSEOPIE"]
pub type NSEOPIE_R = crate::BitReader;
#[doc = "Field `NSEOPIE` writer - NSEOPIE"]
pub type NSEOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSERRIE` reader - NSERRIE"]
pub type NSERRIE_R = crate::BitReader;
#[doc = "Field `NSERRIE` writer - NSERRIE"]
pub type NSERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSLOCK` reader - NSLOCK"]
pub type NSLOCK_R = crate::BitReader;
#[doc = "Field `NSLOCK` writer - NSLOCK"]
pub type NSLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    pub fn nspg(&self) -> NSPG_R {
        NSPG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    pub fn nsper(&self) -> NSPER_R {
        NSPER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    pub fn nsmer1(&self) -> NSMER1_R {
        NSMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    pub fn nspnb(&self) -> NSPNB_R {
        NSPNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    pub fn nsbker(&self) -> NSBKER_R {
        NSBKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    pub fn nsmer2(&self) -> NSMER2_R {
        NSMER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    pub fn nsstrt(&self) -> NSSTRT_R {
        NSSTRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    pub fn nseopie(&self) -> NSEOPIE_R {
        NSEOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    pub fn nserrie(&self) -> NSERRIE_R {
        NSERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    pub fn nslock(&self) -> NSLOCK_R {
        NSLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSPG"]
    #[inline(always)]
    #[must_use]
    pub fn nspg(&mut self) -> NSPG_W<NSCRrs> {
        NSPG_W::new(self, 0)
    }
    #[doc = "Bit 1 - NSPER"]
    #[inline(always)]
    #[must_use]
    pub fn nsper(&mut self) -> NSPER_W<NSCRrs> {
        NSPER_W::new(self, 1)
    }
    #[doc = "Bit 2 - NSMER1"]
    #[inline(always)]
    #[must_use]
    pub fn nsmer1(&mut self) -> NSMER1_W<NSCRrs> {
        NSMER1_W::new(self, 2)
    }
    #[doc = "Bits 3:9 - NSPNB"]
    #[inline(always)]
    #[must_use]
    pub fn nspnb(&mut self) -> NSPNB_W<NSCRrs> {
        NSPNB_W::new(self, 3)
    }
    #[doc = "Bit 11 - NSBKER"]
    #[inline(always)]
    #[must_use]
    pub fn nsbker(&mut self) -> NSBKER_W<NSCRrs> {
        NSBKER_W::new(self, 11)
    }
    #[doc = "Bit 15 - NSMER2"]
    #[inline(always)]
    #[must_use]
    pub fn nsmer2(&mut self) -> NSMER2_W<NSCRrs> {
        NSMER2_W::new(self, 15)
    }
    #[doc = "Bit 16 - Options modification start"]
    #[inline(always)]
    #[must_use]
    pub fn nsstrt(&mut self) -> NSSTRT_W<NSCRrs> {
        NSSTRT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<NSCRrs> {
        OPTSTRT_W::new(self, 17)
    }
    #[doc = "Bit 24 - NSEOPIE"]
    #[inline(always)]
    #[must_use]
    pub fn nseopie(&mut self) -> NSEOPIE_W<NSCRrs> {
        NSEOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - NSERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn nserrie(&mut self) -> NSERRIE_W<NSCRrs> {
        NSERRIE_W::new(self, 25)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<NSCRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<NSCRrs> {
        OPTLOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - NSLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn nslock(&mut self) -> NSLOCK_W<NSCRrs> {
        NSLOCK_W::new(self, 31)
    }
}
#[doc = "Flash non-secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSCRrs;
impl crate::RegisterSpec for NSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nscr::R`](R) reader structure"]
impl crate::Readable for NSCRrs {}
#[doc = "`write(|w| ..)` method takes [`nscr::W`](W) writer structure"]
impl crate::Writable for NSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NSCR to value 0xc000_0000"]
impl crate::Resettable for NSCRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
