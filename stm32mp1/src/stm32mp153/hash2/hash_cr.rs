#[doc = "Register `HASH_CR` reader"]
pub type R = crate::R<HASH_CRrs>;
#[doc = "Register `HASH_CR` writer"]
pub type W = crate::W<HASH_CRrs>;
#[doc = "Field `INIT` writer - INIT"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` reader - DMAE"]
pub type DMAE_R = crate::BitReader;
#[doc = "Field `DMAE` writer - DMAE"]
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - DATATYPE"]
pub type DATATYPE_R = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - DATATYPE"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGO0` reader - ALGO0"]
pub type ALGO0_R = crate::BitReader;
#[doc = "Field `ALGO0` writer - ALGO0"]
pub type ALGO0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBW` reader - NBW"]
pub type NBW_R = crate::FieldReader;
#[doc = "Field `DINNE` reader - DINNE"]
pub type DINNE_R = crate::BitReader;
#[doc = "Field `MDMAT` reader - MDMAT"]
pub type MDMAT_R = crate::BitReader;
#[doc = "Field `MDMAT` writer - MDMAT"]
pub type MDMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAA` writer - DMAA"]
pub type DMAA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKEY` reader - LKEY"]
pub type LKEY_R = crate::BitReader;
#[doc = "Field `LKEY` writer - LKEY"]
pub type LKEY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGO1` reader - ALGO1"]
pub type ALGO1_R = crate::BitReader;
#[doc = "Field `ALGO1` writer - ALGO1"]
pub type ALGO1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - DMAE"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DATATYPE"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ALGO0"]
    #[inline(always)]
    pub fn algo0(&self) -> ALGO0_R {
        ALGO0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - NBW"]
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DINNE"]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MDMAT"]
    #[inline(always)]
    pub fn mdmat(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ALGO1"]
    #[inline(always)]
    pub fn algo1(&self) -> ALGO1_R {
        ALGO1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<HASH_CRrs> {
        INIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAE"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<HASH_CRrs> {
        DMAE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - DATATYPE"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<HASH_CRrs> {
        DATATYPE_W::new(self, 4)
    }
    #[doc = "Bit 6 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<HASH_CRrs> {
        MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - ALGO0"]
    #[inline(always)]
    #[must_use]
    pub fn algo0(&mut self) -> ALGO0_W<HASH_CRrs> {
        ALGO0_W::new(self, 7)
    }
    #[doc = "Bit 13 - MDMAT"]
    #[inline(always)]
    #[must_use]
    pub fn mdmat(&mut self) -> MDMAT_W<HASH_CRrs> {
        MDMAT_W::new(self, 13)
    }
    #[doc = "Bit 14 - DMAA"]
    #[inline(always)]
    #[must_use]
    pub fn dmaa(&mut self) -> DMAA_W<HASH_CRrs> {
        DMAA_W::new(self, 14)
    }
    #[doc = "Bit 16 - LKEY"]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LKEY_W<HASH_CRrs> {
        LKEY_W::new(self, 16)
    }
    #[doc = "Bit 18 - ALGO1"]
    #[inline(always)]
    #[must_use]
    pub fn algo1(&mut self) -> ALGO1_W<HASH_CRrs> {
        ALGO1_W::new(self, 18)
    }
}
#[doc = "HASH control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_CRrs;
impl crate::RegisterSpec for HASH_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_cr::R`](R) reader structure"]
impl crate::Readable for HASH_CRrs {}
#[doc = "`write(|w| ..)` method takes [`hash_cr::W`](W) writer structure"]
impl crate::Writable for HASH_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CR to value 0"]
impl crate::Resettable for HASH_CRrs {
    const RESET_VALUE: u32 = 0;
}
