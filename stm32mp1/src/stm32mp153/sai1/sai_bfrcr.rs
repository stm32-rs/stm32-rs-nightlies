#[doc = "Register `SAI_BFRCR` reader"]
pub type R = crate::R<SAI_BFRCRrs>;
#[doc = "Register `SAI_BFRCR` writer"]
pub type W = crate::W<SAI_BFRCRrs>;
#[doc = "Field `FRL` reader - FRL"]
pub type FRL_R = crate::FieldReader;
#[doc = "Field `FRL` writer - FRL"]
pub type FRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FSALL` reader - FSALL"]
pub type FSALL_R = crate::FieldReader;
#[doc = "Field `FSALL` writer - FSALL"]
pub type FSALL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FSDEF` reader - FSDEF"]
pub type FSDEF_R = crate::BitReader;
#[doc = "Field `FSPOL` reader - FSPOL"]
pub type FSPOL_R = crate::BitReader;
#[doc = "Field `FSPOL` writer - FSPOL"]
pub type FSPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSOFF` reader - FSOFF"]
pub type FSOFF_R = crate::BitReader;
#[doc = "Field `FSOFF` writer - FSOFF"]
pub type FSOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - FRL"]
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - FSALL"]
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - FSDEF"]
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FSPOL"]
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FSOFF"]
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - FRL"]
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<SAI_BFRCRrs> {
        FRL_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - FSALL"]
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<SAI_BFRCRrs> {
        FSALL_W::new(self, 8)
    }
    #[doc = "Bit 17 - FSPOL"]
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<SAI_BFRCRrs> {
        FSPOL_W::new(self, 17)
    }
    #[doc = "Bit 18 - FSOFF"]
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<SAI_BFRCRrs> {
        FSOFF_W::new(self, 18)
    }
}
#[doc = "This register has no meaning in and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bfrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bfrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_BFRCRrs;
impl crate::RegisterSpec for SAI_BFRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_bfrcr::R`](R) reader structure"]
impl crate::Readable for SAI_BFRCRrs {}
#[doc = "`write(|w| ..)` method takes [`sai_bfrcr::W`](W) writer structure"]
impl crate::Writable for SAI_BFRCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAI_BFRCR to value 0x07"]
impl crate::Resettable for SAI_BFRCRrs {
    const RESET_VALUE: u32 = 0x07;
}
