#[doc = "Register `HTCR` reader"]
pub type R = crate::R<HTCRrs>;
#[doc = "Register `HTCR` writer"]
pub type W = crate::W<HTCRrs>;
#[doc = "health test configuration\n\nValue on reset: 23118"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum HTCFG {
    #[doc = "43636: Recommended value for RNG certification (0x0000_AA74)"]
    Recommended = 43636,
    #[doc = "391711420: Magic number to be written before any write (0x1759_0ABC)"]
    Magic = 391711420,
}
impl From<HTCFG> for u32 {
    #[inline(always)]
    fn from(variant: HTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HTCFG {
    type Ux = u32;
}
#[doc = "Field `HTCFG` reader - health test configuration"]
pub type HTCFG_R = crate::FieldReader<HTCFG>;
impl HTCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HTCFG> {
        match self.bits {
            43636 => Some(HTCFG::Recommended),
            391711420 => Some(HTCFG::Magic),
            _ => None,
        }
    }
    #[doc = "Recommended value for RNG certification (0x0000_AA74)"]
    #[inline(always)]
    pub fn is_recommended(&self) -> bool {
        *self == HTCFG::Recommended
    }
    #[doc = "Magic number to be written before any write (0x1759_0ABC)"]
    #[inline(always)]
    pub fn is_magic(&self) -> bool {
        *self == HTCFG::Magic
    }
}
#[doc = "Field `HTCFG` writer - health test configuration"]
pub type HTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, HTCFG>;
impl<'a, REG> HTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Recommended value for RNG certification (0x0000_AA74)"]
    #[inline(always)]
    pub fn recommended(self) -> &'a mut crate::W<REG> {
        self.variant(HTCFG::Recommended)
    }
    #[doc = "Magic number to be written before any write (0x1759_0ABC)"]
    #[inline(always)]
    pub fn magic(self) -> &'a mut crate::W<REG> {
        self.variant(HTCFG::Magic)
    }
}
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    #[must_use]
    pub fn htcfg(&mut self) -> HTCFG_W<HTCRrs> {
        HTCFG_W::new(self, 0)
    }
}
#[doc = "health test control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTCRrs;
impl crate::RegisterSpec for HTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htcr::R`](R) reader structure"]
impl crate::Readable for HTCRrs {}
#[doc = "`write(|w| ..)` method takes [`htcr::W`](W) writer structure"]
impl crate::Writable for HTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTCR to value 0x5a4e"]
impl crate::Resettable for HTCRrs {
    const RESET_VALUE: u32 = 0x5a4e;
}
