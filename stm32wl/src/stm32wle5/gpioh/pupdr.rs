#[doc = "Register `PUPDR` reader"]
pub type R = crate::R<PUPDRrs>;
#[doc = "Register `PUPDR` writer"]
pub type W = crate::W<PUPDRrs>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPDR3 {
    #[doc = "0: No pull-up, pull-down"]
    Floating = 0,
    #[doc = "1: Pull-up"]
    PullUp = 1,
    #[doc = "2: Pull-down"]
    PullDown = 2,
}
impl From<PUPDR3> for u8 {
    #[inline(always)]
    fn from(variant: PUPDR3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PUPDR3 {
    type Ux = u8;
}
#[doc = "Field `PUPDR3` reader - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_R = crate::FieldReader<PUPDR3>;
impl PUPDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PUPDR3> {
        match self.bits {
            0 => Some(PUPDR3::Floating),
            1 => Some(PUPDR3::PullUp),
            2 => Some(PUPDR3::PullDown),
            _ => None,
        }
    }
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUPDR3::Floating
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPDR3::PullUp
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPDR3::PullDown
    }
}
#[doc = "Field `PUPDR3` writer - Port x configuration bits (y = 0..15)"]
pub type PUPDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PUPDR3>;
impl<'a, REG> PUPDR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pull-up, pull-down"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PUPDR3::Floating)
    }
    #[doc = "Pull-up"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PUPDR3::PullUp)
    }
    #[doc = "Pull-down"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PUPDR3::PullDown)
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR3_W<PUPDRrs> {
        PUPDR3_W::new(self, 6)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pupdr::R`](R) reader structure"]
impl crate::Readable for PUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`pupdr::W`](W) writer structure"]
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUPDR to value 0"]
impl crate::Resettable for PUPDRrs {
    const RESET_VALUE: u32 = 0;
}
