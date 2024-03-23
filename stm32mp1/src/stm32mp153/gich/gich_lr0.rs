#[doc = "Register `GICH_LR0` reader"]
pub type R = crate::R<GICH_LR0rs>;
#[doc = "Register `GICH_LR0` writer"]
pub type W = crate::W<GICH_LR0rs>;
#[doc = "Field `VIRTUALID` reader - VIRTUALID"]
pub type VIRTUALID_R = crate::FieldReader<u16>;
#[doc = "Field `VIRTUALID` writer - VIRTUALID"]
pub type VIRTUALID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHYSICALID` reader - PHYSICALID"]
pub type PHYSICALID_R = crate::FieldReader<u16>;
#[doc = "Field `PHYSICALID` writer - PHYSICALID"]
pub type PHYSICALID_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub type PRIORITY_R = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - PRIORITY"]
pub type PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `STATE` reader - STATE"]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `STATE` writer - STATE"]
pub type STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GRP1` reader - GRP1"]
pub type GRP1_R = crate::BitReader;
#[doc = "Field `GRP1` writer - GRP1"]
pub type GRP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW` reader - HW"]
pub type HW_R = crate::BitReader;
#[doc = "Field `HW` writer - HW"]
pub type HW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - VIRTUALID"]
    #[inline(always)]
    pub fn virtualid(&self) -> VIRTUALID_R {
        VIRTUALID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - PHYSICALID"]
    #[inline(always)]
    pub fn physicalid(&self) -> PHYSICALID_R {
        PHYSICALID_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 23:27 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - STATE"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - GRP1"]
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - HW"]
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - VIRTUALID"]
    #[inline(always)]
    #[must_use]
    pub fn virtualid(&mut self) -> VIRTUALID_W<GICH_LR0rs> {
        VIRTUALID_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - PHYSICALID"]
    #[inline(always)]
    #[must_use]
    pub fn physicalid(&mut self) -> PHYSICALID_W<GICH_LR0rs> {
        PHYSICALID_W::new(self, 10)
    }
    #[doc = "Bits 23:27 - PRIORITY"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<GICH_LR0rs> {
        PRIORITY_W::new(self, 23)
    }
    #[doc = "Bits 28:29 - STATE"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<GICH_LR0rs> {
        STATE_W::new(self, 28)
    }
    #[doc = "Bit 30 - GRP1"]
    #[inline(always)]
    #[must_use]
    pub fn grp1(&mut self) -> GRP1_W<GICH_LR0rs> {
        GRP1_W::new(self, 30)
    }
    #[doc = "Bit 31 - HW"]
    #[inline(always)]
    #[must_use]
    pub fn hw(&mut self) -> HW_W<GICH_LR0rs> {
        HW_W::new(self, 31)
    }
}
#[doc = "GICH list register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gich_lr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gich_lr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICH_LR0rs;
impl crate::RegisterSpec for GICH_LR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gich_lr0::R`](R) reader structure"]
impl crate::Readable for GICH_LR0rs {}
#[doc = "`write(|w| ..)` method takes [`gich_lr0::W`](W) writer structure"]
impl crate::Writable for GICH_LR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICH_LR0 to value 0"]
impl crate::Resettable for GICH_LR0rs {
    const RESET_VALUE: u32 = 0;
}
