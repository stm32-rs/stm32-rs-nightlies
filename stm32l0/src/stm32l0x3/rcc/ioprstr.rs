#[doc = "Register `IOPRSTR` reader"]
pub type R = crate::R<IOPRSTRrs>;
#[doc = "Register `IOPRSTR` writer"]
pub type W = crate::W<IOPRSTRrs>;
#[doc = "I/O port A reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPARST {
    #[doc = "1: Reset I/O port"]
    Reset = 1,
}
impl From<IOPARST> for bool {
    #[inline(always)]
    fn from(variant: IOPARST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOPARST` reader - I/O port A reset"]
pub type IOPARST_R = crate::BitReader<IOPARST>;
impl IOPARST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IOPARST> {
        match self.bits {
            true => Some(IOPARST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset I/O port"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IOPARST::Reset
    }
}
#[doc = "Field `IOPARST` writer - I/O port A reset"]
pub type IOPARST_W<'a, REG> = crate::BitWriter<'a, REG, IOPARST>;
impl<'a, REG> IOPARST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset I/O port"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(IOPARST::Reset)
    }
}
#[doc = "Field `IOPBRST` reader - I/O port B reset"]
pub use IOPARST_R as IOPBRST_R;
#[doc = "Field `IOPCRST` reader - I/O port A reset"]
pub use IOPARST_R as IOPCRST_R;
#[doc = "Field `IOPDRST` reader - I/O port D reset"]
pub use IOPARST_R as IOPDRST_R;
#[doc = "Field `IOPERST` reader - I/O port E reset"]
pub use IOPARST_R as IOPERST_R;
#[doc = "Field `IOPHRST` reader - I/O port H reset"]
pub use IOPARST_R as IOPHRST_R;
#[doc = "Field `IOPBRST` writer - I/O port B reset"]
pub use IOPARST_W as IOPBRST_W;
#[doc = "Field `IOPCRST` writer - I/O port A reset"]
pub use IOPARST_W as IOPCRST_W;
#[doc = "Field `IOPDRST` writer - I/O port D reset"]
pub use IOPARST_W as IOPDRST_W;
#[doc = "Field `IOPERST` writer - I/O port E reset"]
pub use IOPARST_W as IOPERST_W;
#[doc = "Field `IOPHRST` writer - I/O port H reset"]
pub use IOPARST_W as IOPHRST_W;
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
    #[doc = "Bit 2 - I/O port A reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E reset"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port H reset"]
    #[inline(always)]
    pub fn iophrst(&self) -> IOPHRST_R {
        IOPHRST_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 2 - I/O port A reset"]
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
    #[doc = "Bit 4 - I/O port E reset"]
    #[inline(always)]
    #[must_use]
    pub fn ioperst(&mut self) -> IOPERST_W<IOPRSTRrs> {
        IOPERST_W::new(self, 4)
    }
    #[doc = "Bit 7 - I/O port H reset"]
    #[inline(always)]
    #[must_use]
    pub fn iophrst(&mut self) -> IOPHRST_W<IOPRSTRrs> {
        IOPHRST_W::new(self, 7)
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
