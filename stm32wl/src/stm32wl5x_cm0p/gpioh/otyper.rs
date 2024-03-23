#[doc = "Register `OTYPER` reader"]
pub type R = crate::R<OTYPERrs>;
#[doc = "Register `OTYPER` writer"]
pub type W = crate::W<OTYPERrs>;
#[doc = "Port x configuration bits (y = 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT3 {
    #[doc = "0: Output push-pull (reset state)"]
    PushPull = 0,
    #[doc = "1: Output open-drain"]
    OpenDrain = 1,
}
impl From<OT3> for bool {
    #[inline(always)]
    fn from(variant: OT3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OT3` reader - Port x configuration bits (y = 0..15)"]
pub type OT3_R = crate::BitReader<OT3>;
impl OT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OT3 {
        match self.bits {
            false => OT3::PushPull,
            true => OT3::OpenDrain,
        }
    }
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OT3::PushPull
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OT3::OpenDrain
    }
}
#[doc = "Field `OT3` writer - Port x configuration bits (y = 0..15)"]
pub type OT3_W<'a, REG> = crate::BitWriter<'a, REG, OT3>;
impl<'a, REG> OT3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output push-pull (reset state)"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OT3::PushPull)
    }
    #[doc = "Output open-drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OT3::OpenDrain)
    }
}
impl R {
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<OTYPERrs> {
        OT3_W::new(self, 3)
    }
}
#[doc = "GPIO port output type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otyper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otyper::R`](R) reader structure"]
impl crate::Readable for OTYPERrs {}
#[doc = "`write(|w| ..)` method takes [`otyper::W`](W) writer structure"]
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTYPER to value 0"]
impl crate::Resettable for OTYPERrs {
    const RESET_VALUE: u32 = 0;
}
