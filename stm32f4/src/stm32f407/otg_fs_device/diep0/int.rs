#[doc = "Register `INT` reader"]
pub type R = crate::R<INTrs>;
#[doc = "Register `INT` writer"]
pub type W = crate::W<INTrs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::BitReader;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - ITTXFE"]
pub type ITTXFE_R = crate::BitReader;
#[doc = "Field `ITTXFE` writer - ITTXFE"]
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - INEPNE"]
pub type INEPNE_R = crate::BitReader;
#[doc = "Field `INEPNE` writer - INEPNE"]
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<INTrs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<INTrs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<INTrs> {
        TOC_W::new(self, 3)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> ITTXFE_W<INTrs> {
        ITTXFE_W::new(self, 4)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    #[must_use]
    pub fn inepne(&mut self) -> INEPNE_W<INTrs> {
        INEPNE_W::new(self, 6)
    }
}
#[doc = "device endpoint-x interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTrs;
impl crate::RegisterSpec for INTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for INTrs {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for INTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT to value 0x80"]
impl crate::Resettable for INTrs {
    const RESET_VALUE: u32 = 0x80;
}
