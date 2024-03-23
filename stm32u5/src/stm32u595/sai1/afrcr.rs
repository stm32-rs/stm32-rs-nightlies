#[doc = "Register `AFRCR` reader"]
pub type R = crate::R<AFRCRrs>;
#[doc = "Register `AFRCR` writer"]
pub type W = crate::W<AFRCRrs>;
#[doc = "Field `FRL` reader - Frame length"]
pub type FRL_R = crate::FieldReader;
#[doc = "Field `FRL` writer - Frame length"]
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FSALL` reader - Frame synchronization active level length"]
pub type FSALL_R = crate::FieldReader;
#[doc = "Field `FSALL` writer - Frame synchronization active level length"]
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FSDEF` reader - Frame synchronization definition"]
pub type FSDEF_R = crate::BitReader;
#[doc = "Field `FSPOL` reader - Frame synchronization polarity"]
pub type FSPOL_R = crate::BitReader;
#[doc = "Field `FSPOL` writer - Frame synchronization polarity"]
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSOFF` reader - Frame synchronization offset"]
pub type FSOFF_R = crate::BitReader;
#[doc = "Field `FSOFF` writer - Frame synchronization offset"]
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Frame synchronization definition"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame length"]
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<AFRCRrs> {
        FRL_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Frame synchronization active level length"]
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<AFRCRrs> {
        FSALL_W::new(self, 8)
    }
    #[doc = "Bit 17 - Frame synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<AFRCRrs> {
        FSPOL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Frame synchronization offset"]
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<AFRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
#[doc = "A frame configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRCRrs;
impl crate::RegisterSpec for AFRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrcr::R`](R) reader structure"]
impl crate::Readable for AFRCRrs {}
#[doc = "`write(|w| ..)` method takes [`afrcr::W`](W) writer structure"]
impl crate::Writable for AFRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRCR to value 0x07"]
impl crate::Resettable for AFRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
