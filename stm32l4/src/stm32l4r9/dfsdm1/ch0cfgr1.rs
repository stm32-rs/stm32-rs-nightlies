#[doc = "Register `CH0CFGR1` reader"]
pub type R = crate::R<CH0CFGR1rs>;
#[doc = "Register `CH0CFGR1` writer"]
pub type W = crate::W<CH0CFGR1rs>;
#[doc = "Field `SITP` reader - SITP"]
pub type SITP_R = crate::FieldReader;
#[doc = "Field `SITP` writer - SITP"]
pub type SITP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPICKSEL` reader - SPICKSEL"]
pub type SPICKSEL_R = crate::FieldReader;
#[doc = "Field `SPICKSEL` writer - SPICKSEL"]
pub type SPICKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCDEN` reader - SCDEN"]
pub type SCDEN_R = crate::BitReader;
#[doc = "Field `SCDEN` writer - SCDEN"]
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABEN` reader - CKABEN"]
pub type CKABEN_R = crate::BitReader;
#[doc = "Field `CKABEN` writer - CKABEN"]
pub type CKABEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHEN` reader - CHEN"]
pub type CHEN_R = crate::BitReader;
#[doc = "Field `CHEN` writer - CHEN"]
pub type CHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHINSEL` reader - CHINSEL"]
pub type CHINSEL_R = crate::BitReader;
#[doc = "Field `CHINSEL` writer - CHINSEL"]
pub type CHINSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATMPX` reader - DATMPX"]
pub type DATMPX_R = crate::FieldReader;
#[doc = "Field `DATMPX` writer - DATMPX"]
pub type DATMPX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATPACK` reader - DATPACK"]
pub type DATPACK_R = crate::FieldReader;
#[doc = "Field `DATPACK` writer - DATPACK"]
pub type DATPACK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKOUTDIV` reader - CKOUTDIV"]
pub type CKOUTDIV_R = crate::FieldReader;
#[doc = "Field `CKOUTDIV` writer - CKOUTDIV"]
pub type CKOUTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CKOUTSRC_R = crate::BitReader;
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CKOUTSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub type DFSDMEN_R = crate::BitReader;
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub type DFSDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<CH0CFGR1rs> {
        SITP_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<CH0CFGR1rs> {
        SPICKSEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<CH0CFGR1rs> {
        SCDEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<CH0CFGR1rs> {
        CKABEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<CH0CFGR1rs> {
        CHEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<CH0CFGR1rs> {
        CHINSEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<CH0CFGR1rs> {
        DATMPX_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<CH0CFGR1rs> {
        DATPACK_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<CH0CFGR1rs> {
        CKOUTDIV_W::new(self, 16)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<CH0CFGR1rs> {
        CKOUTSRC_W::new(self, 30)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<CH0CFGR1rs> {
        DFSDMEN_W::new(self, 31)
    }
}
#[doc = "channel configuration y register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cfgr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cfgr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0CFGR1rs;
impl crate::RegisterSpec for CH0CFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0cfgr1::R`](R) reader structure"]
impl crate::Readable for CH0CFGR1rs {}
#[doc = "`write(|w| ..)` method takes [`ch0cfgr1::W`](W) writer structure"]
impl crate::Writable for CH0CFGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0CFGR1 to value 0"]
impl crate::Resettable for CH0CFGR1rs {
    const RESET_VALUE: u32 = 0;
}
