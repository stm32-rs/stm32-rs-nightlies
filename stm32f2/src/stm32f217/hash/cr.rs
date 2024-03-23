#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `INIT` writer - Initialize message digest calculation"]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DMAE_R = crate::BitReader;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - Data type selection"]
pub type DATATYPE_R = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - Data type selection"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - Mode selection"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Mode selection"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGO` reader - Algorithm selection"]
pub type ALGO_R = crate::BitReader;
#[doc = "Field `ALGO` writer - Algorithm selection"]
pub type ALGO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBW` reader - Number of words already pushed"]
pub type NBW_R = crate::FieldReader;
#[doc = "Field `DINNE` reader - DIN not empty"]
pub type DINNE_R = crate::BitReader;
#[doc = "Field `LKEY` reader - Long key selection"]
pub type LKEY_R = crate::BitReader;
#[doc = "Field `LKEY` writer - Long key selection"]
pub type LKEY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Algorithm selection"]
    #[inline(always)]
    pub fn algo(&self) -> ALGO_R {
        ALGO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words already pushed"]
    #[inline(always)]
    pub fn nbw(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DIN not empty"]
    #[inline(always)]
    pub fn dinne(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Initialize message digest calculation"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<CRrs> {
        INIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<CRrs> {
        DMAE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Data type selection"]
    #[inline(always)]
    #[must_use]
    pub fn datatype(&mut self) -> DATATYPE_W<CRrs> {
        DATATYPE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CRrs> {
        MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Algorithm selection"]
    #[inline(always)]
    #[must_use]
    pub fn algo(&mut self) -> ALGO_W<CRrs> {
        ALGO_W::new(self, 7)
    }
    #[doc = "Bit 16 - Long key selection"]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LKEY_W<CRrs> {
        LKEY_W::new(self, 16)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
