#[doc = "Register `NBTP` reader"]
pub type R = crate::R<NBTPrs>;
#[doc = "Register `NBTP` writer"]
pub type W = crate::W<NBTPrs>;
#[doc = "Field `NTSEG2` reader - TSEG2"]
pub type NTSEG2_R = crate::FieldReader;
#[doc = "Field `NTSEG2` writer - TSEG2"]
pub type NTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NTSEG1` reader - NTSEG1"]
pub type NTSEG1_R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - NTSEG1"]
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBRP` reader - NBRP"]
pub type NBRP_R = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - NBRP"]
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NSJW` reader - NSJW"]
pub type NSJW_R = crate::FieldReader;
#[doc = "Field `NSJW` writer - NSJW"]
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - TSEG2"]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - NTSEG1"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - NBRP"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - NSJW"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - TSEG2"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<NBTPrs> {
        NTSEG2_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NTSEG1"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<NBTPrs> {
        NTSEG1_W::new(self, 8)
    }
    #[doc = "Bits 16:24 - NBRP"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<NBTPrs> {
        NBRP_W::new(self, 16)
    }
    #[doc = "Bits 25:31 - NSJW"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<NBTPrs> {
        NSJW_W::new(self, 25)
    }
}
#[doc = "FDCAN_NBTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NBTPrs;
impl crate::RegisterSpec for NBTPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtp::R`](R) reader structure"]
impl crate::Readable for NBTPrs {}
#[doc = "`write(|w| ..)` method takes [`nbtp::W`](W) writer structure"]
impl crate::Writable for NBTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NBTP to value 0x0a33"]
impl crate::Resettable for NBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
