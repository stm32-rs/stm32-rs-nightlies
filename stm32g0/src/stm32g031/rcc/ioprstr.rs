#[doc = "Register `IOPRSTR` reader"]
pub type R = crate::R<IOPRSTRrs>;
#[doc = "Register `IOPRSTR` writer"]
pub type W = crate::W<IOPRSTRrs>;
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IOPARST_R = crate::BitReader;
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub type IOPARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub type IOPBRST_R = crate::BitReader;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub type IOPBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPCRST` reader - I/O port C reset"]
pub type IOPCRST_R = crate::BitReader;
#[doc = "Field `IOPCRST` writer - I/O port C reset"]
pub type IOPCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub type IOPDRST_R = crate::BitReader;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub type IOPDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOPFRST` reader - I/O port F reset"]
pub type IOPFRST_R = crate::BitReader;
#[doc = "Field `IOPFRST` writer - I/O port F reset"]
pub type IOPFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioparst(&mut self) -> IOPARST_W<IOPRSTRrs> {
        IOPARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopbrst(&mut self) -> IOPBRST_W<IOPRSTRrs> {
        IOPBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopcrst(&mut self) -> IOPCRST_W<IOPRSTRrs> {
        IOPCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopdrst(&mut self) -> IOPDRST_W<IOPRSTRrs> {
        IOPDRST_W::new(self, 3)
    }
    #[doc = "Bit 5 - I/O port F reset"]
    #[inline(always)]
    #[must_use]
    pub fn iopfrst(&mut self) -> IOPFRST_W<IOPRSTRrs> {
        IOPFRST_W::new(self, 5)
    }
}
#[doc = "GPIO reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioprstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPRSTRrs;
impl crate::RegisterSpec for IOPRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioprstr::R`](R) reader structure"]
impl crate::Readable for IOPRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`ioprstr::W`](W) writer structure"]
impl crate::Writable for IOPRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IOPRSTRrs {
    const RESET_VALUE: u32 = 0;
}
